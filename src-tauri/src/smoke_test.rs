#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use crate::db::migrations::MIGRATIONS;
    use crate::db::word_states;
    use crate::db::words;
    use crate::services::study;
    use crate::services::word_import;

    /// End-to-end backend smoke test mirroring the manual UI smoke path.
    ///
    /// 1. Imports the default word list (9411 words).
    /// 2. Marks 10 words as unknown.
    /// 3. Verifies the study pool contains those words.
    /// 4. Generates a quiz from the pool.
    /// 5. Marks the studied words as known.
    /// 6. Verifies known words no longer appear in the study pool.
    #[test]
    fn smoke_test_classification_and_study_modes() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(MIGRATIONS).unwrap();

        // Step 1: fresh import
        let result = word_import::import_from_txt(
            &mut conn,
            "../references/unique_words_with_chinese.txt",
            "unique_words_with_chinese.txt",
        )
        .unwrap();
        assert_eq!(result.imported, 9411);
        assert_eq!(result.skipped, 0);

        let stats = word_states::get_stats(&conn).unwrap();
        assert_eq!(stats.total, 9411);
        assert_eq!(stats.unknown, 0);
        assert_eq!(stats.half, 0);
        assert_eq!(stats.known, 0);

        // Step 2: mark 10 words that have meanings as unknown
        let mut stmt = conn
            .prepare(
                "SELECT word FROM words WHERE meaning IS NOT NULL AND TRIM(meaning) <> '' ORDER BY id LIMIT 10",
            )
            .unwrap();
        let words_with_meaning: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        drop(stmt);
        assert!(
            words_with_meaning.len() >= 10,
            "expected at least 10 words with meanings, got {}",
            words_with_meaning.len()
        );

        let mut marked_words = Vec::new();
        for word in &words_with_meaning {
            word_states::mark_word(&conn, word, "unknown").unwrap();
            marked_words.push(word.clone());
        }

        let stats = word_states::get_stats(&conn).unwrap();
        assert_eq!(stats.unknown, 10);
        assert_eq!(stats.total, 9411);

        // Step 3 & 4: study pool should contain the marked words and be quiz-ready
        let pool = words::get_study_pool(&conn).unwrap();
        assert_eq!(
            pool.len(),
            10,
            "study pool should contain exactly the 10 marked words with meanings"
        );
        for word in &marked_words {
            assert!(
                pool.iter().any(|w| &w.word == word),
                "marked word {} should be in the study pool",
                word
            );
        }

        let quiz = study::generate_quiz(&pool);
        assert!(quiz.is_some(), "a quiz should be generated from the pool");

        // Step 5: mark all 10 words as known
        for word in &marked_words {
            word_states::mark_word(&conn, word, "known").unwrap();
        }

        // Step 6: known words must not be in the study pool
        let pool_after = words::get_study_pool(&conn).unwrap();
        for word in &marked_words {
            assert!(
                !pool_after.iter().any(|w| &w.word == word),
                "known word {} should not appear in the study pool",
                word
            );
        }

        let stats = word_states::get_stats(&conn).unwrap();
        assert_eq!(stats.known, 10);
        assert_eq!(stats.unknown, 0);
    }
}
