//! Import des fiches de cours : Markdown avec front matter YAML.
//!
//! Format : `---` YAML `---` puis corps Markdown. Les flashcards liées sont les lignes
//! `- recto :: verso` situées sous le titre `## Flashcards`.

use serde::{Deserialize, Serialize};

/// Statuts autorisés, dans l'ordre croissant de fiabilité.
pub const STATUTS: [&str; 4] = [
    "brouillon",
    "sources officielles vérifiées",
    "concordance avec les ouvrages vérifiée",
    "relue par un humain",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrontMatter {
    pub slug: String,
    pub titre: String,
    pub matiere: String,
    pub epreuves: Vec<String>,
    pub maj: String,
    pub statut: String,
    pub sources: Vec<String>,
    /// « fiche » (par défaut) ou « cours ».
    #[serde(default = "default_type", rename = "type")]
    pub kind: String,
    /// Parcours auquel appartient un cours (ex. « droit-public »).
    #[serde(default)]
    pub parcours: Option<String>,
    /// Rang du cours dans son parcours.
    #[serde(default)]
    pub ordre: Option<i64>,
}

fn default_type() -> String {
    "fiche".to_string()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flashcard {
    pub kind: String,
    pub recto: String,
    pub verso: String,
}

#[derive(Debug, Clone)]
pub struct Fiche {
    pub meta: FrontMatter,
    pub body: String,
    pub flashcards: Vec<Flashcard>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FicheError {
    #[error("front matter absent ou mal délimité")]
    NoFrontMatter,
    #[error("front matter invalide : {0}")]
    BadYaml(String),
    #[error("statut inconnu : {0}")]
    BadStatut(String),
    #[error("au moins une source est obligatoire")]
    NoSource,
    #[error("notes de bas de page incohérentes : {0}")]
    Footnotes(String),
}

/// Analyse le contenu d'un fichier de fiche.
pub fn parse_fiche(raw: &str) -> Result<Fiche, FicheError> {
    let raw = raw.trim_start_matches('\u{feff}').replace("\r\n", "\n");
    let rest = raw.strip_prefix("---\n").ok_or(FicheError::NoFrontMatter)?;
    let (yaml, body) = rest.split_once("\n---").ok_or(FicheError::NoFrontMatter)?;
    let meta: FrontMatter =
        serde_yaml::from_str(yaml).map_err(|e| FicheError::BadYaml(e.to_string()))?;
    if !STATUTS.contains(&meta.statut.as_str()) {
        return Err(FicheError::BadStatut(meta.statut));
    }
    if meta.sources.is_empty() {
        return Err(FicheError::NoSource);
    }
    let body = body.trim_start_matches('\n').to_string();
    check_footnotes(&body)?;
    let flashcards = extract_flashcards(&body);
    Ok(Fiche {
        meta,
        body,
        flashcards,
    })
}

/// Vérifie que chaque renvoi `[^n]` a une définition `[^n]:` en début de ligne, et inversement.
/// Une note mal écrite (ex. `[1]` au lieu de `[^1]`) ne se relierait pas à sa source à l'affichage.
fn check_footnotes(body: &str) -> Result<(), FicheError> {
    let mut refs = std::collections::BTreeSet::new();
    let mut defs = std::collections::BTreeSet::new();
    for line in body.lines() {
        let mut rest = line;
        let mut at_start = true;
        while let Some(i) = rest.find("[^") {
            let after = &rest[i + 2..];
            let Some(j) = after.find(']') else { break };
            let label = &after[..j];
            let is_def = at_start && i == 0 && after[j + 1..].starts_with(':');
            if !label.is_empty() && label.chars().all(|c| c.is_alphanumeric() || c == '-') {
                if is_def {
                    defs.insert(label.to_string());
                } else {
                    refs.insert(label.to_string());
                }
            }
            at_start = false;
            rest = &after[j + 1..];
        }
    }
    let sans_def: Vec<_> = refs.difference(&defs).cloned().collect();
    let non_citees: Vec<_> = defs.difference(&refs).cloned().collect();
    if sans_def.is_empty() && non_citees.is_empty() {
        return Ok(());
    }
    Err(FicheError::Footnotes(format!(
        "renvois sans définition : {sans_def:?} ; définitions jamais citées : {non_citees:?}"
    )))
}

/// Extrait les flashcards de la section `## Flashcards`.
/// Une ligne `- [type] recto :: verso` fixe le type ; sans crochets, le type est « definition ».
fn extract_flashcards(body: &str) -> Vec<Flashcard> {
    let mut in_section = false;
    let mut cards = Vec::new();
    for line in body.lines() {
        if line.starts_with("## ") {
            in_section = line.trim().eq_ignore_ascii_case("## flashcards");
            continue;
        }
        if !in_section {
            continue;
        }
        let Some(item) = line.trim().strip_prefix("- ") else {
            continue;
        };
        let Some((left, verso)) = item.split_once("::") else {
            continue;
        };
        let (kind, recto) = match left
            .trim()
            .strip_prefix('[')
            .and_then(|l| l.split_once(']'))
        {
            Some((k, r)) => (k.trim().to_string(), r.trim().to_string()),
            None => ("definition".to_string(), left.trim().to_string()),
        };
        if !recto.is_empty() && !verso.trim().is_empty() {
            cards.push(Flashcard {
                kind,
                recto,
                verso: verso.trim().to_string(),
            });
        }
    }
    cards
}

#[cfg(test)]
mod tests {
    use super::*;

    const OK: &str = "---\nslug: test\ntitre: Test\nmatiere: droit_public\nepreuves: [op_droit_public]\nmaj: 2026-10-30\nstatut: brouillon\nsources:\n  - Constitution, art. 5\n---\n# Test\n\n## Flashcards\n- Qui est garant de l'indépendance nationale ? :: Le Président (art. 5)\n- [date] Adoption de la Constitution de 1958 :: 4 octobre 1958\n";

    #[test]
    fn parse_fiche_complete() {
        let f = parse_fiche(OK).unwrap();
        assert_eq!(f.meta.slug, "test");
        assert_eq!(f.flashcards.len(), 2);
        assert_eq!(f.flashcards[0].kind, "definition");
        assert_eq!(f.flashcards[1].kind, "date");
    }

    #[test]
    fn type_par_defaut_fiche_et_cours_avec_parcours() {
        assert_eq!(parse_fiche(OK).unwrap().meta.kind, "fiche");
        let cours = OK.replace(
            "statut: brouillon",
            "statut: brouillon
type: cours
parcours: droit-public
ordre: 2",
        );
        let m = parse_fiche(&cours).unwrap().meta;
        assert_eq!(
            (m.kind.as_str(), m.parcours.as_deref(), m.ordre),
            ("cours", Some("droit-public"), Some(2))
        );
    }

    #[test]
    fn notes_de_bas_de_page_coherentes_ou_refusees() {
        let ok = OK.replace(
            "# Test",
            "# Test

Texte[^1].

[^1]: Source A",
        );
        assert!(parse_fiche(&ok).is_ok());
        let sans_def = OK.replace(
            "# Test",
            "# Test

Texte[^2].",
        );
        assert!(matches!(
            parse_fiche(&sans_def),
            Err(FicheError::Footnotes(_))
        ));
        let non_citee = OK.replace(
            "# Test",
            "# Test

[^3]: Jamais citee",
        );
        assert!(matches!(
            parse_fiche(&non_citee),
            Err(FicheError::Footnotes(_))
        ));
    }

    #[test]
    fn refuse_sans_front_matter() {
        assert_eq!(
            parse_fiche("# rien").unwrap_err(),
            FicheError::NoFrontMatter
        );
    }

    #[test]
    fn refuse_statut_inconnu() {
        let bad = OK.replace("brouillon", "valide");
        assert_eq!(
            parse_fiche(&bad).unwrap_err(),
            FicheError::BadStatut("valide".into())
        );
    }

    #[test]
    fn refuse_sans_source() {
        let bad = OK.replace("sources:\n  - Constitution, art. 5", "sources: []");
        assert_eq!(parse_fiche(&bad).unwrap_err(), FicheError::NoSource);
    }

    #[test]
    fn flashcards_hors_section_ignorees() {
        let raw = OK.replace("## Flashcards", "## Autre");
        assert!(parse_fiche(&raw).unwrap().flashcards.is_empty());
    }
}
