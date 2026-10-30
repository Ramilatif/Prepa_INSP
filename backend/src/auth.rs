//! Authentification, isolée dans ce module.
//! Aujourd'hui : pseudo seul. Avant la mise en ligne, remplacer `PseudoAuth`
//! par une implémentation avec mot de passe sans toucher au reste de l'API.

use sqlx::SqlitePool;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("pseudo invalide (2 à 32 caractères alphanumériques, - ou _)")]
    InvalidPseudo,
    #[error("base de données : {0}")]
    Db(#[from] sqlx::Error),
}

/// Vérifie la forme d'un pseudo.
pub fn validate_pseudo(p: &str) -> Result<String, AuthError> {
    let p = p.trim();
    let ok = (2..=32).contains(&p.chars().count())
        && p.chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_');
    if ok {
        Ok(p.to_lowercase())
    } else {
        Err(AuthError::InvalidPseudo)
    }
}

/// Connexion par pseudo : crée le compte au premier passage. Renvoie l'identifiant utilisateur.
pub async fn login_pseudo(pool: &SqlitePool, pseudo: &str) -> Result<String, AuthError> {
    let pseudo = validate_pseudo(pseudo)?;
    if let Some((id,)) = sqlx::query_as::<_, (String,)>("SELECT id FROM users WHERE pseudo = ?")
        .bind(&pseudo)
        .fetch_optional(pool)
        .await?
    {
        return Ok(id);
    }
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO users (id, pseudo, created_at) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(&pseudo)
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(pool)
        .await?;
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pseudo_valides_et_normalises() {
        assert_eq!(validate_pseudo("  Rami_18 ").unwrap(), "rami_18");
    }

    #[test]
    fn pseudo_invalides() {
        assert!(validate_pseudo("a").is_err());
        assert!(validate_pseudo("a b").is_err());
        assert!(validate_pseudo(&"x".repeat(33)).is_err());
    }

    #[tokio::test]
    async fn login_cree_puis_retrouve() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        let a = login_pseudo(&pool, "Alice").await.unwrap();
        let b = login_pseudo(&pool, "alice").await.unwrap();
        assert_eq!(a, b);
    }
}
