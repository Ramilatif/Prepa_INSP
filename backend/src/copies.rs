//! Logique métier des copies : chronomètre, validation du PDF, auto-évaluation.
//! Fonctions pures (sans base de données ni HTTP) pour rester testables.

use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

/// Limites du PDF accepté. Elles sont alignées sur celles de l'API Claude (taille et pages),
/// qui seront utilisées en phase 4 ; configurables dans `.env` (voir PROGRESS.md : à reconfirmer).
#[derive(Debug, Clone, Copy)]
pub struct PdfLimits {
    pub max_bytes: usize,
    pub max_pages: usize,
}

impl Default for PdfLimits {
    fn default() -> Self {
        Self {
            max_bytes: 32 * 1024 * 1024,
            max_pages: 100,
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum CopieError {
    #[error("le fichier n'est pas un PDF")]
    NotPdf,
    #[error("PDF trop volumineux ({0} Mo ; maximum {1} Mo)")]
    TooBig(usize, usize),
    #[error("PDF trop long ({0} pages ; maximum {1})")]
    TooManyPages(usize, usize),
    #[error("PDF illisible ou protégé : {0}")]
    Unreadable(String),
    #[error("PDF sans page")]
    Empty,
}

/// Valide un PDF déposé et renvoie son nombre de pages.
pub fn validate_pdf(data: &[u8], limits: PdfLimits) -> Result<usize, CopieError> {
    if !data.starts_with(b"%PDF") {
        return Err(CopieError::NotPdf);
    }
    if data.len() > limits.max_bytes {
        let mo = |n: usize| n.div_ceil(1024 * 1024);
        return Err(CopieError::TooBig(mo(data.len()), mo(limits.max_bytes)));
    }
    let doc = lopdf::Document::load_mem(data).map_err(|e| CopieError::Unreadable(e.to_string()))?;
    if doc.is_encrypted() {
        return Err(CopieError::Unreadable("fichier chiffré".into()));
    }
    let pages = doc.get_pages().len();
    if pages == 0 {
        return Err(CopieError::Empty);
    }
    if pages > limits.max_pages {
        return Err(CopieError::TooManyPages(pages, limits.max_pages));
    }
    Ok(pages)
}

/// Secondes restantes (jamais négatives) pour une épreuve commencée à `started`.
pub fn remaining_seconds(started: DateTime<Utc>, duree_minutes: i64, now: DateTime<Utc>) -> i64 {
    (duree_minutes * 60 - (now - started).num_seconds()).max(0)
}

/// Une copie est hors délai si elle est remise plus de `tolerance_s` secondes après la fin.
pub fn is_hors_delai(
    started: DateTime<Utc>,
    duree_minutes: i64,
    submitted: DateTime<Utc>,
    tolerance_s: i64,
) -> bool {
    (submitted - started).num_seconds() > duree_minutes * 60 + tolerance_s
}

/// Calcule la note sur 20 d'une auto-évaluation : somme des points par critère.
/// Chaque critère de la grille doit être noté, entre 0 et son poids. La note est ramenée
/// sur 20 si le total de la grille est différent.
pub fn auto_note(grille: &Value, scores: &HashMap<String, f64>) -> Result<f64, String> {
    let total = grille["total"].as_f64().ok_or("grille sans total")?;
    let criteres = grille["criteres"]
        .as_array()
        .ok_or("grille sans critères")?;
    let mut somme = 0.0;
    for c in criteres {
        let id = c["id"].as_str().ok_or("critère sans id")?;
        let poids = c["poids"].as_f64().ok_or("critère sans poids")?;
        let s = *scores
            .get(id)
            .ok_or_else(|| format!("critère non noté : {id}"))?;
        if !(0.0..=poids).contains(&s) {
            return Err(format!("{id} : {s} hors de [0 ; {poids}]"));
        }
        somme += s;
    }
    if let Some(inconnu) = scores
        .keys()
        .find(|k| !criteres.iter().any(|c| c["id"] == k.as_str()))
    {
        return Err(format!("critère inconnu : {inconnu}"));
    }
    // Arrondi au demi-point, comme une note de copie.
    Ok((somme / total * 20.0 * 2.0).round() / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn t(s: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
    }

    /// Construit un PDF minimal valide de `n` pages.
    fn pdf(n: usize) -> Vec<u8> {
        use lopdf::{Document, Object, Stream, dictionary};
        let mut doc = Document::with_version("1.5");
        let pages_id = doc.new_object_id();
        let mut kids = vec![];
        for _ in 0..n {
            let content = doc.add_object(Stream::new(dictionary! {}, b"".to_vec()));
            let page = doc.add_object(dictionary! {
                "Type" => "Page", "Parent" => pages_id, "Contents" => content,
                "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
            });
            kids.push(Object::Reference(page));
        }
        doc.objects.insert(
            pages_id,
            Object::Dictionary(
                dictionary! {"Type" => "Pages", "Kids" => kids, "Count" => n as i64},
            ),
        );
        let catalog = doc.add_object(dictionary! {"Type" => "Catalog", "Pages" => pages_id});
        doc.trailer.set("Root", catalog);
        let mut out = vec![];
        doc.save_to(&mut out).unwrap();
        out
    }

    #[test]
    fn pdf_valide_compte_les_pages() {
        assert_eq!(validate_pdf(&pdf(3), PdfLimits::default()), Ok(3));
    }

    #[test]
    fn pdf_refuse_non_pdf_trop_gros_trop_long() {
        let l = PdfLimits {
            max_bytes: 10_000_000,
            max_pages: 2,
        };
        assert_eq!(validate_pdf(b"hello", l), Err(CopieError::NotPdf));
        assert_eq!(
            validate_pdf(&pdf(3), l),
            Err(CopieError::TooManyPages(3, 2))
        );
        let petit = PdfLimits {
            max_bytes: 10,
            max_pages: 100,
        };
        assert!(matches!(
            validate_pdf(&pdf(1), petit),
            Err(CopieError::TooBig(..))
        ));
        assert!(matches!(
            validate_pdf(b"%PDF-1.4 casse", PdfLimits::default()),
            Err(CopieError::Unreadable(_))
        ));
    }

    #[test]
    fn chronometre() {
        let d = t("2027-03-15T09:00:00Z");
        assert_eq!(
            remaining_seconds(d, 300, t("2027-03-15T09:01:00Z")),
            300 * 60 - 60
        );
        assert_eq!(remaining_seconds(d, 300, t("2027-03-15T20:00:00Z")), 0);
    }

    #[test]
    fn hors_delai_avec_tolerance() {
        let d = t("2027-03-15T09:00:00Z");
        assert!(!is_hors_delai(d, 240, t("2027-03-15T13:00:30Z"), 60));
        assert!(is_hors_delai(d, 240, t("2027-03-15T13:02:00Z"), 60));
    }

    fn grille() -> Value {
        json!({"total": 20, "criteres": [{"id": "a", "poids": 5}, {"id": "b", "poids": 15}]})
    }

    #[test]
    fn note_somme_des_criteres() {
        let s = HashMap::from([("a".to_string(), 4.0), ("b".to_string(), 9.0)]);
        assert_eq!(auto_note(&grille(), &s), Ok(13.0));
    }

    #[test]
    fn note_arrondie_au_demi_point_et_ramenee_sur_20() {
        let g = json!({"total": 10, "criteres": [{"id": "a", "poids": 10}]});
        let s = HashMap::from([("a".to_string(), 6.3)]);
        assert_eq!(auto_note(&g, &s), Ok(12.5));
    }

    #[test]
    fn note_refuse_scores_invalides() {
        let hors = HashMap::from([("a".to_string(), 6.0), ("b".to_string(), 1.0)]);
        assert!(auto_note(&grille(), &hors).is_err());
        let manquant = HashMap::from([("a".to_string(), 1.0)]);
        assert!(auto_note(&grille(), &manquant).is_err());
        let inconnu = HashMap::from([
            ("a".to_string(), 1.0),
            ("b".to_string(), 1.0),
            ("z".to_string(), 1.0),
        ]);
        assert!(auto_note(&grille(), &inconnu).is_err());
    }
}
