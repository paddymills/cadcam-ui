use crate::{ApiState, Result};
use axum::{Json, extract::State};
use serde_json::{Value, json};
use std::sync::Arc;

/// get all active programs
pub async fn active(State(state): ApiState) -> Result<Json<Value>> {
    let state = Arc::clone(&state);
    let mut conn = state.pool.get().await?;

    let programs: Vec<Value> = conn
        .simple_query(include_str!("schema/active_programs.sql"))
        .await?
        .into_first_result()
        .await?
        .into_iter()
        .map(|row| super::parse_db_row(&row))
        .collect();

    Ok(Json(json!(programs)))
}
