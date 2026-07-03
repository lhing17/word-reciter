pub const MIGRATIONS: &str = r#"
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
"#;
