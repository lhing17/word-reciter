use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Stats {
    pub total: i64,
    pub unknown: i64,
    pub half: i64,
    pub known: i64,
}

pub fn get_stats(conn: &rusqlite::Connection) -> Result<Stats, String> {
    let total: i64 = conn
        .query_row("SELECT COUNT(*) FROM words", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    let mut unknown = 0i64;
    let mut half = 0i64;
    let mut known = 0i64;

    let mut stmt = conn
        .prepare("SELECT familiarity, COUNT(*) FROM word_states GROUP BY familiarity")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            let familiarity: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok((familiarity, count))
        })
        .map_err(|e| e.to_string())?;

    for row in rows {
        let (fam, count) = row.map_err(|e| e.to_string())?;
        match fam.as_str() {
            "unknown" => unknown = count,
            "half" => half = count,
            "known" => known = count,
            _ => {}
        }
    }

    Ok(Stats {
        total,
        unknown,
        half,
        known,
    })
}

pub fn mark_word(
    conn: &rusqlite::Connection,
    word: &str,
    familiarity: &str,
) -> Result<(), String> {
    let word_id: i64 = conn
        .query_row(
            "SELECT id FROM words WHERE word = ?",
            rusqlite::params![word],
            |row| row.get(0),
        )
        .map_err(|_| format!("word not found: {}", word))?;

    let sql = r#"
        INSERT INTO word_states (word_id, familiarity, marked_at, updated_at)
        VALUES (?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ON CONFLICT(word_id) DO UPDATE SET
            familiarity = excluded.familiarity,
            updated_at = CURRENT_TIMESTAMP
    "#;
    conn.execute(sql, rusqlite::params![word_id, familiarity])
        .map_err(|e| e.to_string())?;
    Ok(())
}
