pub fn log_study(
    conn: &rusqlite::Connection,
    word: &str,
    quiz_type: &str,
    result: &str,
    familiarity_after: &str,
) -> Result<(), String> {
    let sql = r#"
        INSERT INTO study_logs (word_id, quiz_type, result, familiarity_after)
        VALUES ((SELECT id FROM words WHERE word = ?), ?, ?, ?)
    "#;
    conn.execute(
        sql,
        rusqlite::params![word, quiz_type, result, familiarity_after],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
