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
    pub erreurs: Vec<String>,
}

pub async fn import_all(pool: &SqlitePool, content_dir: &Path) -> anyhow::Result<ImportReport> {
    let mut report = ImportReport::default();
    import_sessions(pool, &content_dir.join("sessions"), &mut report).await?;
    import_fiches(pool, &content_dir.join("fiches"), &mut report).await?;
    Ok(report)
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
            "INSERT INTO fiches (id, slug, titre, matiere, epreuves_json, updated_at, statut, sources_json, body_md)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(slug) DO UPDATE SET titre = excluded.titre, matiere = excluded.matiere,
               epreuves_json = excluded.epreuves_json, updated_at = excluded.updated_at,
               statut = excluded.statut, sources_json = excluded.sources_json, body_md = excluded.body_md",
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
}
