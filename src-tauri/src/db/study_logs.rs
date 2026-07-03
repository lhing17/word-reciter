pub fn log_study(
    conn: &rusqlite::Connection,
    word: &str,
    quiz_type: &str,
    result: &str,
    familiarity_after: &str,
) -> Result<(), String> {
    let word_id: i64 = conn
        .query_row(
            "SELECT id FROM words WHERE word = ?",
            rusqlite::params![word],
            |row| row.get(0),
        )
        .map_err(|_| format!("word not found: {}", word))?;

    let sql = r#"
        INSERT INTO study_logs (word_id, quiz_type, result, familiarity_after)
        VALUES (?, ?, ?, ?)
    "#;
    conn.execute(
        sql,
        rusqlite::params![word_id, quiz_type, result, familiarity_after],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
