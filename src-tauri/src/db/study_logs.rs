pub fn log_study(
    conn: &rusqlite::Connection,
    word: &str,
    quiz_type: &str,
    result: &str,
    familiarity_after: &str,
) -> Result<(), String> {
    const VALID_QUIZ_TYPES: [&str; 3] = ["choice", "fill", "recall"];
    const VALID_RESULTS: [&str; 3] = ["correct", "wrong", "skipped"];

    if !VALID_QUIZ_TYPES.contains(&quiz_type) {
        return Err(format!("invalid quiz_type: {}", quiz_type));
    }
    if !VALID_RESULTS.contains(&result) {
        return Err(format!("invalid result: {}", result));
    }

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
