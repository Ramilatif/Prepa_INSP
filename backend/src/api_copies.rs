//! Routes des copies : sujet chronométré, dépôt du PDF, auto-évaluation.

use crate::api::{ApiResult, User, internal};
use crate::copies::{PdfLimits, auto_note, is_hors_delai, remaining_seconds, validate_pdf};
use axum::{
    Json, Router,
    body::Bytes,
    extract::{DefaultBodyLimit, Path, State},
    http::StatusCode,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{Row, SqlitePool};
use std::{collections::HashMap, path::PathBuf};

/// Tolérance (secondes) avant de marquer une copie « hors délai » : latence réseau et envoi.
const TOLERANCE_S: i64 = 120;

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/api/copies", get(list_copies))
        .route("/api/copies/start", post(start))
        .route("/api/copies/{id}", get(get_copie))
        .route(
            "/api/copies/{id}/pdf",
            post(upload_pdf).layer(DefaultBodyLimit::max(40 * 1024 * 1024)),
        )
        .route("/api/copies/{id}/autoeval", post(autoeval))
}

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn limits() -> PdfLimits {
    let d = PdfLimits::default();
    PdfLimits {
        max_bytes: env_usize("MAX_PDF_MB", d.max_bytes / (1024 * 1024)) * 1024 * 1024,
        max_pages: env_usize("MAX_PDF_PAGES", d.max_pages),
    }
}

fn bad(msg: impl Into<String>) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, msg.into())
}

/// Durée réelle (minutes) d'une épreuve dans la session active de l'utilisateur,
/// à défaut dans la première session disponible. Jamais codée en dur.
async fn duree_epreuve(
    pool: &SqlitePool,
    uid: &str,
    code: &str,
) -> Result<i64, (StatusCode, String)> {
    let row = sqlx::query(
        "SELECT s.config_json FROM sessions s
         LEFT JOIN users u ON u.active_session_id = s.id AND u.id = ?
         ORDER BY CASE WHEN u.id IS NULL THEN 1 ELSE 0 END, s.id DESC LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
    .map_err(internal)?
    .ok_or_else(|| bad("aucune session configurée"))?;
    let cfg: Value =
        serde_json::from_str(&row.get::<String, _>("config_json")).map_err(internal)?;
    cfg["epreuves"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|e| e["code"] == code)
        .and_then(|e| e["duree_minutes"].as_i64())
        .ok_or_else(|| {
            bad(format!(
                "épreuve « {code} » sans durée dans la session (ancien format ?)"
            ))
        })
}

#[derive(Deserialize)]
struct StartReq {
    epreuve_code: String,
    sujet_url: String,
}

async fn start(
    State(pool): State<SqlitePool>,
    User(uid): User,
    Json(r): Json<StartReq>,
) -> ApiResult<Value> {
    let duree = duree_epreuve(&pool, &uid, &r.epreuve_code).await?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now();
    sqlx::query(
        "INSERT INTO copies (id, user_id, epreuve_code, sujet_url, duree_minutes, started_at, statut)
         VALUES (?, ?, ?, ?, ?, ?, 'en_cours')",
    )
    .bind(&id)
    .bind(&uid)
    .bind(&r.epreuve_code)
    .bind(&r.sujet_url)
    .bind(duree)
    .bind(now.to_rfc3339())
    .execute(&pool)
    .await
    .map_err(internal)?;
    Ok(Json(
        json!({ "id": id, "duree_minutes": duree, "remaining_seconds": duree * 60 }),
    ))
}

fn copie_json(r: &sqlx::sqlite::SqliteRow, now: DateTime<Utc>) -> Value {
    let started = DateTime::parse_from_rfc3339(&r.get::<String, _>("started_at"))
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or(now);
    let statut: String = r.get("statut");
    let duree: i64 = r.get("duree_minutes");
    json!({
        "id": r.get::<String, _>("id"),
        "epreuve_code": r.get::<String, _>("epreuve_code"),
        "sujet_url": r.get::<String, _>("sujet_url"),
        "duree_minutes": duree,
        "started_at": r.get::<String, _>("started_at"),
        "statut": statut,
        "hors_delai": r.get::<i64, _>("hors_delai") != 0,
        "elapsed_seconds": r.get::<Option<i64>, _>("elapsed_seconds"),
        "pdf_pages": r.get::<Option<i64>, _>("pdf_pages"),
        "note": r.get::<Option<f64>, _>("note"),
        "remaining_seconds": (statut == "en_cours").then(|| remaining_seconds(started, duree, now)),
    })
}

/// Requête de lecture d'une copie avec sa note d'auto-évaluation ; assemblée à la compilation
/// (sqlx refuse le SQL construit à l'exécution).
macro_rules! copie_select {
    ($tail:literal) => {
        concat!(
            "SELECT c.*, a.note AS note FROM copies c LEFT JOIN auto_evaluations a ON a.copie_id = c.id ",
            $tail
        )
    };
}

async fn list_copies(State(pool): State<SqlitePool>, User(uid): User) -> ApiResult<Value> {
    let rows = sqlx::query(copie_select!(
        "WHERE c.user_id = ? ORDER BY c.started_at DESC"
    ))
    .bind(&uid)
    .fetch_all(&pool)
    .await
    .map_err(internal)?;
    let now = Utc::now();
    Ok(Json(json!(
        rows.iter().map(|r| copie_json(r, now)).collect::<Vec<_>>()
    )))
}

/// Charge une copie en vérifiant qu'elle appartient bien à l'utilisateur.
async fn own_copie(
    pool: &SqlitePool,
    uid: &str,
    id: &str,
) -> Result<sqlx::sqlite::SqliteRow, (StatusCode, String)> {
    sqlx::query(copie_select!("WHERE c.id = ? AND c.user_id = ?"))
        .bind(id)
        .bind(uid)
        .fetch_optional(pool)
        .await
        .map_err(internal)?
        .ok_or((StatusCode::NOT_FOUND, "copie introuvable".into()))
}

async fn get_copie(
    State(pool): State<SqlitePool>,
    User(uid): User,
    Path(id): Path<String>,
) -> ApiResult<Value> {
    let r = own_copie(&pool, &uid, &id).await?;
    Ok(Json(copie_json(&r, Utc::now())))
}

async fn upload_pdf(
    State(pool): State<SqlitePool>,
    User(uid): User,
    Path(id): Path<String>,
    body: Bytes,
) -> ApiResult<Value> {
    let r = own_copie(&pool, &uid, &id).await?;
    if r.get::<String, _>("statut") != "en_cours" {
        return Err(bad("cette copie a déjà été déposée"));
    }
    let pages = validate_pdf(&body, limits()).map_err(|e| bad(e.to_string()))?;
    let started = DateTime::parse_from_rfc3339(&r.get::<String, _>("started_at"))
        .map_err(internal)?
        .with_timezone(&Utc);
    let now = Utc::now();
    let duree: i64 = r.get("duree_minutes");
    let hors = is_hors_delai(started, duree, now, TOLERANCE_S);

    // Le fichier est rangé par utilisateur ; le nom ne vient jamais du client.
    let dir = PathBuf::from(std::env::var("DATA_DIR").unwrap_or_else(|_| "data".into()))
        .join("copies")
        .join(&uid);
    std::fs::create_dir_all(&dir).map_err(internal)?;
    let path = dir.join(format!("{id}.pdf"));
    std::fs::write(&path, &body).map_err(internal)?;

    sqlx::query(
        "UPDATE copies SET statut = 'deposee', submitted_at = ?, elapsed_seconds = ?, hors_delai = ?,
         pdf_path = ?, pdf_pages = ?, pdf_bytes = ? WHERE id = ?",
    )
    .bind(now.to_rfc3339())
    .bind((now - started).num_seconds())
    .bind(hors as i64)
    .bind(path.to_string_lossy().to_string())
    .bind(pages as i64)
    .bind(body.len() as i64)
    .bind(&id)
    .execute(&pool)
    .await
    .map_err(internal)?;
    Ok(Json(json!({ "pages": pages, "hors_delai": hors })))
}

#[derive(Deserialize)]
struct EvalReq {
    scores: HashMap<String, f64>,
}

async fn autoeval(
    State(pool): State<SqlitePool>,
    User(uid): User,
    Path(id): Path<String>,
    Json(req): Json<EvalReq>,
) -> ApiResult<Value> {
    let r = own_copie(&pool, &uid, &id).await?;
    if r.get::<String, _>("statut") == "en_cours" {
        return Err(bad("dépose d'abord la copie"));
    }
    let code: String = r.get("epreuve_code");
    let g = sqlx::query("SELECT config_json FROM grilles WHERE epreuve_code = ?")
        .bind(&code)
        .fetch_optional(&pool)
        .await
        .map_err(internal)?
        .ok_or_else(|| bad(format!("pas de grille pour « {code} »")))?;
    let grille: Value =
        serde_json::from_str(&g.get::<String, _>("config_json")).map_err(internal)?;
    let note = auto_note(&grille, &req.scores).map_err(bad)?;
    sqlx::query(
        "INSERT INTO auto_evaluations (copie_id, scores_json, note, created_at) VALUES (?, ?, ?, ?)
         ON CONFLICT(copie_id) DO UPDATE SET scores_json = excluded.scores_json, note = excluded.note, created_at = excluded.created_at",
    )
    .bind(&id)
    .bind(serde_json::to_string(&req.scores).map_err(internal)?)
    .bind(note)
    .bind(Utc::now().to_rfc3339())
    .execute(&pool)
    .await
    .map_err(internal)?;
    sqlx::query("UPDATE copies SET statut = 'auto_evaluee' WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(internal)?;
    Ok(Json(json!({ "note": note, "sur": 20, "indicative": true })))
}
