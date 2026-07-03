use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Word {
    pub id: i64,
    pub word: String,
    pub source: Option<String>,
    pub meaning: Option<String>,
}

pub fn get_unmarked_queue(conn: &rusqlite::Connection) -> Result<Vec<Word>, String> {
    let mut stmt = conn
        .prepare(
            r#"
            SELECT w.id, w.word, w.source, w.meaning
            FROM words w
            LEFT JOIN word_states ws ON w.id = ws.word_id
            WHERE ws.word_id IS NULL
            ORDER BY RANDOM()
            "#,
        )
        .map_err(|e| e.to_string())?;

    let words = stmt
        .query_map([], |row| {
            Ok(Word {
                id: row.get(0)?,
                word: row.get(1)?,
                source: row.get(2)?,
                meaning: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(words)
}

pub fn get_study_pool(conn: &rusqlite::Connection) -> Result<Vec<Word>, String> {
    let mut stmt = conn
        .prepare(
            r#"
            SELECT w.id, w.word, w.source, w.meaning
            FROM words w
            JOIN word_states ws ON w.id = ws.word_id
            WHERE ws.familiarity IN ('unknown', 'half')
              AND w.meaning IS NOT NULL
              AND TRIM(w.meaning) <> ''
            "#,
        )
        .map_err(|e| e.to_string())?;

    let words = stmt
        .query_map([], |row| {
            Ok(Word {
                id: row.get(0)?,
                word: row.get(1)?,
                source: row.get(2)?,
                meaning: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(words)
}
