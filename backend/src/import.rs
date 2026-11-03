//! Import des contenus (sessions et fiches) depuis `content/` vers la base.
//! Idempotent : relancé à chaque démarrage, il met à jour ce qui existe déjà.

use crate::fiches::parse_fiche;
use sqlx::SqlitePool;
use std::path::Path;

/// Compteurs renvoyés pour les logs et les tests.
#[derive(Debug, Default, PartialEq)]
pub struct ImportReport {
    pub sessions: usize,
    pub fiches: usize,
    pub flashcards: usize,
    pub annales: usize,
    pub grilles: usize,
    pub erreurs: Vec<String>,
}

pub async fn import_all(pool: &SqlitePool, content_dir: &Path) -> anyhow::Result<ImportReport> {
    let mut report = ImportReport::default();
    import_sessions(pool, &content_dir.join("sessions"), &mut report).await?;
    import_fiches(pool, &content_dir.join("fiches"), &mut report).await?;
    import_annales(pool, &content_dir.join("annales/index.json"), &mut report).await?;
    import_grilles(pool, &content_dir.join("grilles"), &mut report).await?;
    Ok(report)
}

/// Importe le manifeste des annales (produit par tools/annales/fetch_annales.py).
/// Absent, il est simplement ignoré : les annales sont optionnelles au démarrage.
async fn import_annales(
    pool: &SqlitePool,
    index: &Path,
    r: &mut ImportReport,
) -> anyhow::Result<()> {
    let Ok(raw) = std::fs::read_to_string(index) else {
        return Ok(());
    };
    let v: serde_json::Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => {
            r.erreurs.push(format!("{}: {e}", index.display()));
            return Ok(());
        }
    };
    for d in v["documents"].as_array().into_iter().flatten() {
        let (Some(url), Some(type_), Some(epreuve), Some(format)) = (
            d["url"].as_str(),
            d["type"].as_str(),
            d["epreuve"].as_str(),
            d["format"].as_str(),
        ) else {
            r.erreurs.push(format!("annale incomplète : {d}"));
            continue;
        };
        if format != "reforme" && format != "ancien" {
            r.erreurs
                .push(format!("format inconnu « {format} » pour {url}"));
            continue;
        }
        // L'URL source est unique par document : elle sert d'identifiant stable.
        sqlx::query(
            "INSERT INTO annales (id, annee, concours, type, epreuve, epreuve_code, format, url_source, fichier)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET annee = excluded.annee, type = excluded.type, epreuve = excluded.epreuve,
               epreuve_code = excluded.epreuve_code, format = excluded.format, fichier = excluded.fichier",
        )
        .bind(url)
        .bind(d["annee"].as_i64())
        .bind(d["concours"].as_str().unwrap_or("externe"))
        .bind(type_)
        .bind(epreuve)
        .bind(d["epreuve_code"].as_str())
        .bind(format)
        .bind(url)
        .bind(d["fichier"].as_str())
        .execute(pool)
        .await?;
        r.annales += 1;
    }
    Ok(())
}

/// Importe les grilles d'évaluation : la somme des poids doit valoir le total annoncé.
async fn import_grilles(pool: &SqlitePool, dir: &Path, r: &mut ImportReport) -> anyhow::Result<()> {
    for path in files_with_ext(dir, "json") {
        let raw = std::fs::read_to_string(&path)?;
        let v: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                r.erreurs.push(format!("{}: {e}", path.display()));
                continue;
            }
        };
        let (Some(code), Some(nom)) = (v["epreuve_code"].as_str(), v["nom"].as_str()) else {
            r.erreurs
                .push(format!("{}: epreuve_code ou nom manquant", path.display()));
            continue;
        };
        if let Err(msg) = check_poids(&v) {
            r.erreurs.push(format!("{}: {msg}", path.display()));
            continue;
        }
        sqlx::query(
            "INSERT INTO grilles (epreuve_code, nom, config_json) VALUES (?, ?, ?)
             ON CONFLICT(epreuve_code) DO UPDATE SET nom = excluded.nom, config_json = excluded.config_json",
        )
        .bind(code)
        .bind(nom)
        .bind(&raw)
        .execute(pool)
        .await?;
        r.grilles += 1;
    }
    Ok(())
}

/// Vérifie que les poids des critères totalisent bien le total annoncé de la grille.
fn check_poids(grille: &serde_json::Value) -> Result<(), String> {
    let total = grille["total"].as_f64().ok_or("total manquant")?;
    let somme: f64 = grille["criteres"]
        .as_array()
        .ok_or("criteres manquants")?
        .iter()
        .map(|c| c["poids"].as_f64().unwrap_or(0.0))
        .sum();
    if (somme - total).abs() > 1e-9 {
        return Err(format!(
            "somme des poids {somme} différente du total {total}"
        ));
    }
    Ok(())
}

async fn import_sessions(
    pool: &SqlitePool,
    dir: &Path,
    r: &mut ImportReport,
) -> anyhow::Result<()> {
    for path in files_with_ext(dir, "json") {
        let raw = std::fs::read_to_string(&path)?;
        let v: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                r.erreurs.push(format!("{}: {e}", path.display()));
                continue;
            }
        };
        let (Some(id), Some(label)) = (v["id"].as_str(), v["label"].as_str()) else {
            r.erreurs
                .push(format!("{}: id ou label manquant", path.display()));
            continue;
        };
        sqlx::query(
            "INSERT INTO sessions (id, label, config_json) VALUES (?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET label = excluded.label, config_json = excluded.config_json",
        )
        .bind(id)
        .bind(label)
        .bind(&raw)
        .execute(pool)
        .await?;
        r.sessions += 1;
    }
    Ok(())
}

async fn import_fiches(pool: &SqlitePool, dir: &Path, r: &mut ImportReport) -> anyhow::Result<()> {
    for path in files_with_ext(dir, "md") {
        let fiche = match parse_fiche(&std::fs::read_to_string(&path)?) {
            Ok(f) => f,
            Err(e) => {
                r.erreurs.push(format!("{}: {e}", path.display()));
                continue;
            }
        };
        let m = &fiche.meta;
        let fiche_id = format!("fiche:{}", m.slug);
        sqlx::query(
            "INSERT INTO fiches (id, slug, titre, matiere, epreuves_json, updated_at, statut, sources_json, body_md, type, parcours, ordre)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(slug) DO UPDATE SET titre = excluded.titre, matiere = excluded.matiere,
               epreuves_json = excluded.epreuves_json, updated_at = excluded.updated_at,
               statut = excluded.statut, sources_json = excluded.sources_json, body_md = excluded.body_md,
               type = excluded.type, parcours = excluded.parcours, ordre = excluded.ordre",
        )
        .bind(&fiche_id)
        .bind(&m.slug)
        .bind(&m.titre)
        .bind(&m.matiere)
        .bind(serde_json::to_string(&m.epreuves)?)
        .bind(&m.maj)
        .bind(&m.statut)
        .bind(serde_json::to_string(&m.sources)?)
        .bind(&fiche.body)
        .bind(&m.kind)
        .bind(&m.parcours)
        .bind(m.ordre)
        .execute(pool)
        .await?;
        r.fiches += 1;
        for (i, c) in fiche.flashcards.iter().enumerate() {
            // Identifiant stable : une carte garde son historique tant que sa position ne change pas.
            sqlx::query(
                "INSERT INTO flashcards (id, fiche_id, kind, recto, verso, source_ref) VALUES (?, ?, ?, ?, ?, ?)
                 ON CONFLICT(id) DO UPDATE SET kind = excluded.kind, recto = excluded.recto, verso = excluded.verso",
            )
            .bind(format!("{fiche_id}#{i}"))
            .bind(&fiche_id)
            .bind(&c.kind)
            .bind(&c.recto)
            .bind(&c.verso)
            .bind(m.sources.first().cloned().unwrap_or_default())
            .execute(pool)
            .await?;
            r.flashcards += 1;
        }
    }
    Ok(())
}

fn files_with_ext(dir: &Path, ext: &str) -> Vec<std::path::PathBuf> {
    let mut v: Vec<_> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == ext))
        .collect();
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn import_est_idempotent_et_signale_les_erreurs() {
        let dir = std::env::temp_dir().join(format!("insp_import_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(dir.join("sessions")).unwrap();
        std::fs::create_dir_all(dir.join("fiches")).unwrap();
        std::fs::write(dir.join("sessions/a.json"), r#"{"id":"s","label":"S"}"#).unwrap();
        std::fs::write(dir.join("fiches/mauvaise.md"), "pas de front matter").unwrap();
        std::fs::write(
            dir.join("fiches/ok.md"),
            "---\nslug: ok\ntitre: Ok\nmatiere: droit_public\nepreuves: []\nmaj: 2026-10-30\nstatut: brouillon\nsources: [x]\n---\n## Flashcards\n- a :: b\n",
        )
        .unwrap();
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        let r1 = import_all(&pool, &dir).await.unwrap();
        let r2 = import_all(&pool, &dir).await.unwrap();
        assert_eq!(
            (r1.sessions, r1.fiches, r1.flashcards, r1.erreurs.len()),
            (1, 1, 1, 1)
        );
        assert_eq!(r2.fiches, 1);
        let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM flashcards")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(n, 1);
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn poids_doivent_totaliser_le_total() {
        let ok = serde_json::json!({"total": 20, "criteres": [{"poids": 5}, {"poids": 15}]});
        let ko = serde_json::json!({"total": 20, "criteres": [{"poids": 5}, {"poids": 10}]});
        assert!(check_poids(&ok).is_ok());
        assert!(check_poids(&ko).is_err());
        assert!(check_poids(&serde_json::json!({"criteres": []})).is_err());
    }

    #[tokio::test]
    async fn import_annales_et_grilles() {
        let dir = std::env::temp_dir().join(format!("insp_import_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(dir.join("annales")).unwrap();
        std::fs::create_dir_all(dir.join("grilles")).unwrap();
        std::fs::write(
            dir.join("annales/index.json"),
            r#"{"documents":[
              {"annee":2025,"concours":"externe","type":"sujet","epreuve":"economie","epreuve_code":"op_economie","format":"reforme","url":"https://x/a.pdf","fichier":"files/a.pdf"},
              {"annee":2023,"concours":"externe","type":"sujet","epreuve":"economie","epreuve_code":"op_economie","format":"ancien","url":"https://x/b.pdf"},
              {"annee":2024,"type":"sujet","epreuve":"x","format":"bizarre","url":"https://x/c.pdf"}]}"#,
        )
        .unwrap();
        std::fs::write(
            dir.join("grilles/ok.json"),
            r#"{"epreuve_code":"e","nom":"E","total":20,"criteres":[{"poids":20}]}"#,
        )
        .unwrap();
        std::fs::write(
            dir.join("grilles/ko.json"),
            r#"{"epreuve_code":"f","nom":"F","total":20,"criteres":[{"poids":3}]}"#,
        )
        .unwrap();
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        let r = import_all(&pool, &dir).await.unwrap();
        assert_eq!((r.annales, r.grilles), (2, 1));
        // Le format inconnu et la grille aux poids faux sont signalés, pas importés.
        assert_eq!(r.erreurs.len(), 2);
        let (anciens,): (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM annales WHERE format = 'ancien'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(anciens, 1);
        std::fs::remove_dir_all(dir).ok();
    }
}
