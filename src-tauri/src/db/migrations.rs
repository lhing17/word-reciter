use rusqlite::Connection;

const MIGRATIONS: &[(i64, &str)] = &[
    (
        1,
        r#"
CREATE TABLE IF NOT EXISTS words (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word TEXT UNIQUE NOT NULL,
    source TEXT,
    meaning TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_words_word ON words(word);

CREATE TABLE IF NOT EXISTS word_states (
    word_id INTEGER PRIMARY KEY,
    familiarity TEXT CHECK(familiarity IN ('unknown', 'half', 'known')) NOT NULL,
    marked_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (word_id) REFERENCES words(id)
);

CREATE INDEX IF NOT EXISTS idx_word_states_familiarity ON word_states(familiarity);

CREATE TABLE IF NOT EXISTS study_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word_id INTEGER NOT NULL,
    quiz_type TEXT CHECK(quiz_type IN ('choice', 'fill', 'recall')) NOT NULL,
    result TEXT CHECK(result IN ('correct', 'wrong', 'skipped')) NOT NULL,
    familiarity_after TEXT CHECK(familiarity_after IN ('unknown', 'half', 'known')) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (word_id) REFERENCES words(id)
);

CREATE INDEX IF NOT EXISTS idx_study_logs_word_id ON study_logs(word_id);
"#,
    ),
];

/// Runs pending schema migrations in order inside a transaction.
///
/// A `schema_migrations` table tracks which versions have already been applied.
/// Each new migration is executed atomically and recorded before committing.
pub fn run_migrations(conn: &mut Connection) -> Result<(), String> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (version INTEGER PRIMARY KEY)",
        [],
    )
    .map_err(|e| e.to_string())?;

    let applied: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    for (version, sql) in MIGRATIONS {
        if *version > applied {
            tx.execute_batch(sql).map_err(|e| e.to_string())?;
            tx.execute(
                "INSERT INTO schema_migrations (version) VALUES (?)",
                rusqlite::params![version],
            )
            .map_err(|e| e.to_string())?;
        }
    }
    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
}
