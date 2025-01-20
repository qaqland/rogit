use std::{collections::HashMap, sync::Arc};

use crate::config::Settings;
use anyhow::Result;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

#[tokio::main]
pub async fn run(settings: &Settings) -> Result<()> {
    let pairpool_map = Arc::new(HashMap::from(settings));
    let action = Router::new()
        .route("/refs", get(list_refs))
        .route("/tags", get(list_refs))
        .route("/commit/:hash", get(list_refs))
        .route("/patch/:hash", get(list_refs))
        .route("/tags", get(list_refs));
    let app = Router::new()
        .route("/:repo", get(repo_path))
        .nest("/:repo/-", action)
        .with_state(pairpool_map.clone());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

#[debug_handler]
async fn repo_path(
    Path(repo): Path<String>,
    State(pool): State<Arc<HashMap<String, PairPool>>>,
) -> impl IntoResponse {
    let pool_git = &pool.get(&repo).unwrap().git.get().await.unwrap();
    let path: String = pool_git
        .interact(|repo| repo.path().to_string_lossy().into())
        .await
        .unwrap();
    (StatusCode::OK, path)
}

#[debug_handler]
async fn list_refs(
    Path(repo): Path<String>,
    State(pool): State<Arc<HashMap<String, PairPool>>>,
) -> Result<String, (StatusCode, String)> {
    let pool_db = &pool.get(&repo).unwrap().sqlite.get().await.unwrap();
    let refs = pool_db
        .interact(|conn| {
            let mut stmt = conn.prepare("SELECT name FROM branch").unwrap();
            let rows = stmt.query_map([], |row| row.get::<_, String>(0)).unwrap();
            let mut tags = Vec::new();
            for row in rows {
                tags.push(row.unwrap());
            }
            tags
        })
        .await
        .unwrap();
    Ok(refs.join("\n"))
}
