//! database connections, deserializating and schema

pub mod program;

/// Tiberius Error
pub type SqlError = tiberius::error::Error;

/// Convenience export of database Pool type
pub type DbPool = bb8::Pool<bb8_tiberius::ConnectionManager>;

/// Convenience export of Client type
pub type DbClient = bb8_tiberius::rt::Client;

/// Builds a connection pool for a database
pub async fn init_pool(config: tiberius::Config, size: u32) -> crate::Result<DbPool> {
    log::debug!("** init db pool");

    let mgr = bb8_tiberius::ConnectionManager::build(config)?;
    log::debug!("** > db connection Manager built");

    let pool = bb8::Pool::builder().max_size(size).build(mgr).await?;
    log::debug!("** > db pool built");

    Ok(pool)
}

pub fn parse_db_row(row: &tiberius::Row) -> serde_json::Value {
    let mut json = serde_json::Map::new();

    for column in row.columns() {
        let col = column.name();
        let value = try_map_value(row, column);

        if let Some(value) = value {
            json.insert(col.to_string(), value);
        }
    }

    serde_json::Value::Object(json)
}

pub fn try_map_value(row: &tiberius::Row, column: &tiberius::Column) -> Option<serde_json::Value> {
    let col = column.name();

    // integer
    if let Ok(Some(val)) = row.try_get::<i64, _>(col) {
        return serde_json::to_value(val).ok();
    }

    // float
    if let Ok(Some(val)) = row.try_get::<f64, _>(col) {
        return serde_json::to_value(val).ok();
    }

    // datetime
    if let Ok(Some(val)) = row.try_get::<chrono::NaiveDateTime, _>(col) {
        return serde_json::to_value(val).ok();
    }

    // string
    if let Ok(Some(val)) = row.try_get::<&str, _>(col) {
        return serde_json::to_value(val).ok();
    }

    None
}
