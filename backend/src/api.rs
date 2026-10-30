//! Routes HTTP (préfixe /api). Le jeton est l'identifiant utilisateur (UUID aléatoire)
//! envoyé en `Authorization: Bearer`. À remplacer avec le mot de passe (voir auth.rs).

use crate::{auth, srs::CardState};
use axum::{
    Json, Router,
    extract::{FromRequestParts, Path, Query, State},
    http::{StatusCode, request::Parts},
    routing::{get, post},
};
use chrono::Utc;
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{Row, SqlitePool};
use tower_http::cors::CorsLayer;

type ApiResult<T> = Result<Json<T>, (StatusCode, String)>;

fn internal(e: impl std::fmt::Display) -> (StatusCode, String) {
    tracing::error!("{e}");
    (StatusCode::INTERNAL_SERVER_ERROR, "erreur interne".into())
}

/// Utilisateur authentifié, extrait du jeton Bearer.
pub struct User(pub String);

impl FromRequestParts<SqlitePool> for User {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        pool: &SqlitePool,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or((StatusCode::UNAUTHORIZED, "connexion requise".to_string()))?;
        let found = sqlx::query("SELECT id FROM users WHERE id = ?")
            .bind(token)
            .fetch_optional(pool)
            .await
            .map_err(internal)?;
        found
            .map(|_| User(token.to_string()))
            .ok_or((StatusCode::UNAUTHORIZED, "jeton inconnu".to_string()))
    }
}

pub fn router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .route("/api/login", post(login))
        .route("/api/me", get(me).put(update_me))
        .route("/api/sessions", get(sessions))
        .route("/api/fiches", get(list_fiches))
        .route("/api/fiches/{slug}", get(get_fiche))
        .route("/api/flashcards/due", get(due_cards))
        .route("/api/flashcards/{id}/review", post(review_card))
        .route(
            "/api/signalements",
            get(list_signalements).post(create_signalement),
        )
        .layer(CorsLayer::permissive())
        .with_state(pool)
}

#[derive(Deserialize)]
struct LoginReq {
    pseudo: String,
}

async fn login(State(pool): State<SqlitePool>, Json(req): Json<LoginReq>) -> ApiResult<Value> {
    match auth::login_pseudo(&pool, &req.pseudo).await {
        Ok(id) => Ok(Json(json!({ "token": id }))),
        Err(auth::AuthError::InvalidPseudo) => Err((
            StatusCode::BAD_REQUEST,
            auth::AuthError::InvalidPseudo.to_string(),
        )),
        Err(e) => Err(internal(e)),
    }
}

async fn me(State(pool): State<SqlitePool>, User(uid): User) -> ApiResult<Value> {
    let r =
        sqlx::query("SELECT pseudo, majeure, mineure, active_session_id FROM users WHERE id = ?")
            .bind(&uid)
            .fetch_one(&pool)
            .await
            .map_err(internal)?;
    Ok(Json(json!({
        "pseudo": r.get::<String, _>("pseudo"),
        "majeure": r.get::<Option<String>, _>("majeure"),
        "mineure": r.get::<Option<String>, _>("mineure"),
        "session": r.get::<Option<String>, _>("active_session_id"),
    })))
}

#[derive(Deserialize)]
struct MeUpdate {
    majeure: Option<String>,
    session: Option<String>,
}

/// Choisir la majeure fixe la mineure à l'autre matière ; `null` revient à l'égalité.
async fn update_me(
    State(pool): State<SqlitePool>,
    User(uid): User,
    Json(u): Json<MeUpdate>,
) -> ApiResult<Value> {
    let (majeure, mineure) = match u.majeure.as_deref() {
        Some("droit_public") => (Some("droit_public"), Some("economie")),
        Some("economie") => (Some("economie"), Some("droit_public")),
        None => (None, None),
        Some(_) => return Err((StatusCode::BAD_REQUEST, "matière inconnue".into())),
    };
    sqlx::query("UPDATE users SET majeure = ?, mineure = ?, active_session_id = ? WHERE id = ?")
        .bind(majeure)
        .bind(mineure)
        .bind(u.session)
        .bind(&uid)
        .execute(&pool)
        .await
        .map_err(internal)?;
    Ok(Json(json!({ "ok": true })))
}

async fn sessions(State(pool): State<SqlitePool>, _u: User) -> ApiResult<Value> {
    let rows = sqlx::query("SELECT id, label, config_json FROM sessions ORDER BY id")
        .fetch_all(&pool)
        .await
        .map_err(internal)?;
    let list: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<String, _>("id"),
                "label": r.get::<String, _>("label"),
                "config": serde_json::from_str::<Value>(&r.get::<String, _>("config_json")).unwrap_or(Value::Null),
            })
        })
        .collect();
    Ok(Json(json!(list)))
}

#[derive(Deserialize)]
struct FicheFilter {
    q: Option<String>,
    matiere: Option<String>,
    epreuve: Option<String>,
}

async fn list_fiches(
    State(pool): State<SqlitePool>,
    _u: User,
    Query(f): Query<FicheFilter>,
) -> ApiResult<Value> {
    let rows = sqlx::query("SELECT slug, titre, matiere, epreuves_json, updated_at, statut, a_verifier FROM fiches ORDER BY titre")
        .fetch_all(&pool)
        .await
        .map_err(internal)?;
    let q = f.q.as_deref().map(str::to_lowercase);
    let list: Vec<Value> = rows
        .iter()
        .filter(|r| {
            f.matiere
                .as_deref()
                .is_none_or(|m| r.get::<String, _>("matiere") == m)
        })
        .filter(|r| {
            f.epreuve.as_deref().is_none_or(|e| {
                r.get::<String, _>("epreuves_json")
                    .contains(&format!("\"{e}\""))
            })
        })
        .filter(|r| {
            q.as_deref()
                .is_none_or(|q| r.get::<String, _>("titre").to_lowercase().contains(q))
        })
        .map(|r| {
            json!({
                "slug": r.get::<String, _>("slug"),
                "titre": r.get::<String, _>("titre"),
                "matiere": r.get::<String, _>("matiere"),
                "maj": r.get::<String, _>("updated_at"),
                "statut": r.get::<String, _>("statut"),
                "a_verifier": r.get::<i64, _>("a_verifier") != 0,
            })
        })
        .collect();
    Ok(Json(json!(list)))
}

async fn get_fiche(
    State(pool): State<SqlitePool>,
    _u: User,
    Path(slug): Path<String>,
) -> ApiResult<Value> {
    let r = sqlx::query("SELECT id, slug, titre, matiere, epreuves_json, updated_at, statut, sources_json, body_md FROM fiches WHERE slug = ?")
        .bind(&slug)
        .fetch_optional(&pool)
        .await
        .map_err(internal)?
        .ok_or((StatusCode::NOT_FOUND, "fiche introuvable".to_string()))?;
    Ok(Json(json!({
        "id": r.get::<String, _>("id"),
        "slug": r.get::<String, _>("slug"),
        "titre": r.get::<String, _>("titre"),
        "matiere": r.get::<String, _>("matiere"),
        "epreuves": serde_json::from_str::<Value>(&r.get::<String, _>("epreuves_json")).unwrap_or(Value::Null),
        "maj": r.get::<String, _>("updated_at"),
        "statut": r.get::<String, _>("statut"),
        "sources": serde_json::from_str::<Value>(&r.get::<String, _>("sources_json")).unwrap_or(Value::Null),
        "body_md": r.get::<String, _>("body_md"),
    })))
}

/// Cartes à réviser : celles dont l'échéance est passée, puis les cartes jamais vues.
async fn due_cards(State(pool): State<SqlitePool>, User(uid): User) -> ApiResult<Value> {
    let now = Utc::now().to_rfc3339();
    let rows = sqlx::query(
        "SELECT f.id, f.recto, f.verso, f.kind, f.source_ref FROM flashcards f
         LEFT JOIN card_states s ON s.card_id = f.id AND s.user_id = ?
         WHERE s.card_id IS NULL OR s.due_at <= ?
         ORDER BY s.due_at LIMIT 50",
    )
    .bind(&uid)
    .bind(&now)
    .fetch_all(&pool)
    .await
    .map_err(internal)?;
    let list: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<String, _>("id"), "recto": r.get::<String, _>("recto"),
                "verso": r.get::<String, _>("verso"), "kind": r.get::<String, _>("kind"),
                "source_ref": r.get::<String, _>("source_ref"),
            })
        })
        .collect();
    Ok(Json(json!(list)))
}

#[derive(Deserialize)]
struct ReviewReq {
    quality: u8,
}

async fn review_card(
    State(pool): State<SqlitePool>,
    User(uid): User,
    Path(card_id): Path<String>,
    Json(req): Json<ReviewReq>,
) -> ApiResult<Value> {
    if req.quality > 5 {
        return Err((StatusCode::BAD_REQUEST, "qualité entre 0 et 5".into()));
    }
    let now = Utc::now();
    let existing = sqlx::query("SELECT ease, interval_days, repetitions, due_at FROM card_states WHERE user_id = ? AND card_id = ?")
        .bind(&uid)
        .bind(&card_id)
        .fetch_optional(&pool)
        .await
        .map_err(internal)?;
    let state = match existing {
        Some(r) => CardState {
            ease: r.get("ease"),
            interval_days: r.get("interval_days"),
            repetitions: r.get("repetitions"),
            due_at: now,
        },
        None => CardState::new(now),
    };
    let next = state.review(req.quality, now);
    sqlx::query(
        "INSERT INTO card_states (user_id, card_id, ease, interval_days, repetitions, due_at, last_reviewed)
         VALUES (?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(user_id, card_id) DO UPDATE SET ease = excluded.ease, interval_days = excluded.interval_days,
           repetitions = excluded.repetitions, due_at = excluded.due_at, last_reviewed = excluded.last_reviewed",
    )
    .bind(&uid)
    .bind(&card_id)
    .bind(next.ease)
    .bind(next.interval_days)
    .bind(next.repetitions)
    .bind(next.due_at.to_rfc3339())
    .bind(now.to_rfc3339())
    .execute(&pool)
    .await
    .map_err(internal)?;
    Ok(Json(
        json!({ "due_at": next.due_at.to_rfc3339(), "interval_days": next.interval_days }),
    ))
}

#[derive(Deserialize)]
struct SignalReq {
    cible_type: String,
    cible_id: String,
    message: String,
}

async fn create_signalement(
    State(pool): State<SqlitePool>,
    User(uid): User,
    Json(s): Json<SignalReq>,
) -> ApiResult<Value> {
    if !["fiche", "flashcard", "corrige"].contains(&s.cible_type.as_str())
        || s.message.trim().is_empty()
    {
        return Err((StatusCode::BAD_REQUEST, "signalement invalide".into()));
    }
    sqlx::query("INSERT INTO signalements (id, user_id, cible_type, cible_id, message, created_at) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&uid)
        .bind(&s.cible_type)
        .bind(&s.cible_id)
        .bind(s.message.trim())
        .bind(Utc::now().to_rfc3339())
        .execute(&pool)
        .await
        .map_err(internal)?;
    Ok(Json(json!({ "ok": true })))
}

async fn list_signalements(State(pool): State<SqlitePool>, _u: User) -> ApiResult<Value> {
    let rows = sqlx::query(
        "SELECT s.id, s.cible_type, s.cible_id, s.message, s.status, s.created_at, u.pseudo
         FROM signalements s JOIN users u ON u.id = s.user_id ORDER BY s.created_at DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(internal)?;
    let list: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<String, _>("id"), "cible_type": r.get::<String, _>("cible_type"),
                "cible_id": r.get::<String, _>("cible_id"), "message": r.get::<String, _>("message"),
                "status": r.get::<String, _>("status"), "created_at": r.get::<String, _>("created_at"),
                "pseudo": r.get::<String, _>("pseudo"),
            })
        })
        .collect();
    Ok(Json(json!(list)))
}
