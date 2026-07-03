use rusqlite::Connection;
use serde::Serialize;
use std::fs;
use std::io::{BufRead, BufReader};

/// Result of importing words from a text file.
#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub imported: usize,
    pub skipped: usize,
}

/// Imports words from a `word|meaning` text file into SQLite.
///
/// Each line is expected to be in the format `word|meaning`. If the line does
/// not contain `|`, the meaning is stored as an empty string. Lines that are
/// empty after trimming are ignored.
///
/// Words already present in the `words` table are skipped (`INSERT OR IGNORE`).
pub fn import_from_txt(conn: &mut Connection, path: &str, source: &str) -> Result<ImportResult, String> {
    let file = fs::File::open(path).map_err(|e| format!("Failed to open {}: {}", path, e))?;
    let reader = BufReader::new(file);

    let tx = conn
        .transaction()
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    let mut stmt = tx
        .prepare("INSERT OR IGNORE INTO words (word, source, meaning) VALUES (?, ?, ?)")
        .map_err(|e| format!("Failed to prepare insert statement: {}", e))?;

    let mut imported = 0usize;
    let mut skipped = 0usize;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (word, meaning) = match trimmed.split_once('|') {
            Some((w, m)) => (w.trim(), m.trim()),
            None => (trimmed, ""),
        };

        if word.is_empty() {
            continue;
        }

        let rows = stmt
            .execute(rusqlite::params![word, source, meaning])
            .map_err(|e| format!("Failed to insert word '{}': {}", word, e))?;

        if rows == 1 {
            imported += 1;
        } else {
            skipped += 1;
        }
    }

    drop(stmt);
    tx.commit()
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    Ok(ImportResult { imported, skipped })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::run_migrations;
    use std::io::Write;

    #[test]
    fn imports_default_reference_file() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();

        let result = import_from_txt(
            &mut conn,
            "../references/unique_words_with_chinese.txt",
            "unique_words_with_chinese.txt",
        )
        .unwrap();

        assert_eq!(result.imported, 9411);
        assert_eq!(result.skipped, 0);

        let second = import_from_txt(
            &mut conn,
            "../references/unique_words_with_chinese.txt",
            "unique_words_with_chinese.txt",
        )
        .unwrap();

        assert_eq!(second.imported, 0);
        assert_eq!(second.skipped, 9411);
    }

    #[test]
    fn imports_words_and_skips_duplicates() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("words.txt");
        {
            let mut file = fs::File::create(&file_path).unwrap();
            writeln!(file, "hello|你好").unwrap();
            writeln!(file, "world|世界").unwrap();
            writeln!(file, "hello|你好").unwrap();
            writeln!(file, "  spaced | 有间隔  ").unwrap();
            writeln!(file, "empty-meaning").unwrap();
            writeln!(file, "   ").unwrap();
        }

        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();

        let first = import_from_txt(
            &mut conn,
            file_path.to_str().unwrap(),
            "test-source",
        )
        .unwrap();
        assert_eq!(first.imported, 4);
        assert_eq!(first.skipped, 1);

        let second = import_from_txt(
            &mut conn,
            file_path.to_str().unwrap(),
            "test-source",
        )
        .unwrap();
        assert_eq!(second.imported, 0);
        assert_eq!(second.skipped, 5);
    }
}
