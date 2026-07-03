use rusqlite::OptionalExtension;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Word {
    pub id: i64,
    pub word: String,
    pub source: Option<String>,
    pub meaning: Option<String>,
}

pub fn get_next_unmarked(conn: &rusqlite::Connection, offset: i64) -> Result<Option<Word>, String> {
    let mut stmt = conn
        .prepare(
            r#"
            SELECT w.id, w.word, w.source, w.meaning
            FROM words w
            LEFT JOIN word_states ws ON w.id = ws.word_id
            WHERE ws.word_id IS NULL
            ORDER BY w.id
            LIMIT 1 OFFSET ?
            "#,
        )
        .map_err(|e| e.to_string())?;

    let word = stmt
        .query_row(rusqlite::params![offset], |row| {
            Ok(Word {
                id: row.get(0)?,
                word: row.get(1)?,
                source: row.get(2)?,
                meaning: row.get(3)?,
            })
        })
        .optional()
        .map_err(|e| e.to_string())?;

    Ok(word)
}
