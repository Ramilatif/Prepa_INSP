//! Répétition espacée, algorithme SM-2.
//! Logique pure (sans base de données) pour rester facilement testable.

use chrono::{DateTime, Duration, Utc};

/// État de révision d'une carte pour un utilisateur.
#[derive(Debug, Clone, PartialEq)]
pub struct CardState {
    pub ease: f64,
    pub interval_days: i64,
    pub repetitions: i64,
    pub due_at: DateTime<Utc>,
}

/// Facilité minimale imposée par SM-2.
const MIN_EASE: f64 = 1.3;

impl CardState {
    /// Carte jamais révisée : due immédiatement.
    pub fn new(now: DateTime<Utc>) -> Self {
        Self {
            ease: 2.5,
            interval_days: 0,
            repetitions: 0,
            due_at: now,
        }
    }

    /// Applique une note de qualité `quality` (0 à 5) et renvoie le nouvel état.
    /// En dessous de 3, la carte est réapprise : répétitions remises à zéro, revue le lendemain.
    pub fn review(&self, quality: u8, now: DateTime<Utc>) -> Self {
        let q = quality.min(5) as f64;
        let ease = (self.ease + 0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02)).max(MIN_EASE);
        let (repetitions, interval_days) = if quality < 3 {
            (0, 1)
        } else {
            let reps = self.repetitions + 1;
            let interval = match reps {
                1 => 1,
                2 => 6,
                _ => (self.interval_days as f64 * ease).round() as i64,
            };
            (reps, interval)
        };
        Self {
            ease,
            interval_days,
            repetitions,
            due_at: now + Duration::days(interval_days),
        }
    }

    /// Utilisée par les tests ; l'API filtre directement en SQL sur `due_at`.
    #[cfg(test)]
    pub fn is_due(&self, now: DateTime<Utc>) -> bool {
        self.due_at <= now
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t0() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-11-01T08:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    #[test]
    fn nouvelle_carte_est_due() {
        assert!(CardState::new(t0()).is_due(t0()));
    }

    #[test]
    fn intervalles_1_6_puis_multiplie_par_la_facilite() {
        let s1 = CardState::new(t0()).review(4, t0());
        assert_eq!((s1.repetitions, s1.interval_days), (1, 1));
        let s2 = s1.review(4, t0());
        assert_eq!((s2.repetitions, s2.interval_days), (2, 6));
        let s3 = s2.review(4, t0());
        assert_eq!(s3.interval_days, (6.0 * s3.ease).round() as i64);
    }

    #[test]
    fn echec_remet_a_zero() {
        let s = CardState::new(t0())
            .review(5, t0())
            .review(5, t0())
            .review(1, t0());
        assert_eq!((s.repetitions, s.interval_days), (0, 1));
    }

    #[test]
    fn facilite_jamais_sous_le_plancher() {
        let mut s = CardState::new(t0());
        for _ in 0..20 {
            s = s.review(0, t0());
        }
        assert!((s.ease - MIN_EASE).abs() < f64::EPSILON);
    }

    #[test]
    fn la_date_d_echeance_suit_l_intervalle() {
        let s = CardState::new(t0()).review(5, t0());
        assert_eq!(s.due_at, t0() + Duration::days(1));
        assert!(!s.is_due(t0()));
    }
}
