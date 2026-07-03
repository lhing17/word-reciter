use crate::db::words::Word;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Quiz {
    Choice {
        word: String,
        correct: String,
        options: Vec<String>,
    },
    Fill {
        word: String,
        hint: String,
        first: String,
        last: String,
    },
    Recall {
        word: String,
        answer: String,
    },
}

fn fetch_meaningful_words_excluding(
    conn: &Connection,
    exclude_word: &str,
) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT meaning FROM words WHERE meaning IS NOT NULL AND TRIM(meaning) <> '' AND word <> ?",
        )
        .map_err(|e| e.to_string())?;
    let meanings = stmt
        .query_map(rusqlite::params![exclude_word], |row| {
            let m: String = row.get(0)?;
            Ok(m.trim().to_string())
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(meanings)
}

pub fn generate_quiz(conn: &Connection, pool: &[Word]) -> Result<Option<Quiz>, String> {
    let mut rng = rand::thread_rng();
    generate_quiz_with_rng(conn, pool, &mut rng)
}

pub fn generate_quiz_with_rng<R: Rng>(
    conn: &Connection,
    pool: &[Word],
    rng: &mut R,
) -> Result<Option<Quiz>, String> {
    if pool.is_empty() {
        return Ok(None);
    }
    let target = pool
        .choose(rng)
        .ok_or_else(|| "empty pool".to_string())?;
    let answer = target
        .meaning
        .as_ref()
        .map(|m| m.trim())
        .filter(|m| !m.is_empty())
        .ok_or_else(|| "target has no meaning".to_string())?
        .to_string();

    let quiz_type = rng.gen::<u8>() % 10;
    if quiz_type < 4 {
        // choice: ensure 4 unique options (1 correct + 3 distractors)
        let target_meaning = answer.clone();
        let unique_distractor_meanings: HashSet<String> = pool
            .iter()
            .filter(|w| w.word != target.word)
            .filter_map(|w| {
                let m = w.meaning.as_ref()?.trim();
                if m.is_empty() || m == target_meaning { None } else { Some(m.to_string()) }
            })
            .collect();
        let mut distractors: Vec<String> = unique_distractor_meanings
            .iter()
            .cloned()
            .choose_multiple(rng, 3);

        if distractors.len() < 3 {
            let fallback = fetch_meaningful_words_excluding(conn, &target.word)?;
            let used: HashSet<String> = distractors
                .iter()
                .cloned()
                .chain(std::iter::once(target_meaning.clone()))
                .collect();
            let needed = 3 - distractors.len();
            let extra: HashSet<String> = fallback
                .into_iter()
                .filter(|m| !used.contains(m))
                .collect();
            let extra_sample: Vec<String> = extra
                .iter()
                .cloned()
                .choose_multiple(rng, needed);
            distractors.extend(extra_sample);
        }

        if distractors.len() < 3 {
            // Not enough meaningful words anywhere for a choice quiz.
            return Ok(Some(Quiz::Recall {
                word: target.word.clone(),
                answer,
            }));
        }

        let mut options = distractors;
        options.push(answer.clone());
        options.shuffle(rng);
        let unique_options: HashSet<String> = options.iter().cloned().collect();
        if unique_options.len() != 4 {
            return Ok(Some(Quiz::Recall {
                word: target.word.clone(),
                answer,
            }));
        }
        Ok(Some(Quiz::Choice {
            word: target.word.clone(),
            correct: answer,
            options,
        }))
    } else if quiz_type < 7 {
        // fill
        let chars: Vec<char> = target.word.chars().collect();
        if chars.len() < 3 {
            return Ok(Some(Quiz::Recall {
                word: target.word.clone(),
                answer,
            }));
        }
        let first = chars.first().unwrap().to_string();
        let last = chars.last().unwrap().to_string();
        Ok(Some(Quiz::Fill {
            word: target.word.clone(),
            hint: answer,
            first,
            last,
        }))
    } else {
        // recall
        Ok(Some(Quiz::Recall {
            word: target.word.clone(),
            answer,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::run_migrations;
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use rusqlite::Connection;

    fn make_word(id: i64, word: &str, meaning: &str) -> Word {
        Word {
            id,
            word: word.to_string(),
            source: None,
            meaning: Some(meaning.to_string()),
        }
    }

    fn insert_word(conn: &Connection, word: &str, meaning: &str) {
        conn.execute(
            "INSERT INTO words (word, meaning) VALUES (?, ?)",
            rusqlite::params![word, meaning],
        )
        .unwrap();
    }

    /// Generates quizzes until a choice quiz is produced.
    fn first_choice_quiz(conn: &Connection, pool: &[Word], rng: &mut StdRng) -> Quiz {
        for _ in 0..200 {
            if let Some(Quiz::Choice {
                word,
                correct,
                options,
            }) = generate_quiz_with_rng(conn, pool, rng).unwrap()
            {
                return Quiz::Choice {
                    word,
                    correct,
                    options,
                };
            }
        }
        panic!("failed to generate a choice quiz after 200 attempts");
    }

    /// Generates quizzes until a recall quiz is produced.
    fn first_recall_quiz(conn: &Connection, pool: &[Word], rng: &mut StdRng) -> Quiz {
        for _ in 0..200 {
            if let Some(Quiz::Recall { word, answer }) = generate_quiz_with_rng(conn, pool, rng).unwrap() {
                return Quiz::Recall { word, answer };
            }
        }
        panic!("failed to generate a recall quiz after 200 attempts");
    }

    #[test]
    fn empty_pool_returns_none() {
        let conn = Connection::open_in_memory().unwrap();
        let mut rng = StdRng::seed_from_u64(1);
        let quiz = generate_quiz_with_rng(&conn, &[], &mut rng).unwrap();
        assert!(quiz.is_none());
    }

    #[test]
    fn single_word_pool_falls_back_to_recall() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let pool = vec![make_word(1, "hello", "你好")];
        let mut rng = StdRng::seed_from_u64(1);

        let quiz = first_recall_quiz(&conn, &pool, &mut rng);
        match quiz {
            Quiz::Recall { word, answer } => {
                assert_eq!(word, "hello");
                assert_eq!(answer, "你好");
            }
            _ => panic!("expected recall quiz when no distractors are available"),
        }
    }

    #[test]
    fn pool_with_enough_distractors_produces_choice_with_four_options() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let pool = vec![
            make_word(1, "apple", "苹果"),
            make_word(2, "banana", "香蕉"),
            make_word(3, "cherry", "樱桃"),
            make_word(4, "date", "枣"),
        ];
        let mut rng = StdRng::seed_from_u64(1);

        let quiz = first_choice_quiz(&conn, &pool, &mut rng);
        match quiz {
            Quiz::Choice { correct, options, .. } => {
                assert_eq!(options.len(), 4);
                assert!(options.contains(&correct));
            }
            _ => panic!("expected choice quiz when distractors are available"),
        }
    }

    #[test]
    fn pool_requiring_global_fallback_produces_four_options() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        insert_word(&conn, "lemon", "柠檬");
        insert_word(&conn, "mango", "芒果");

        let pool = vec![
            make_word(1, "apple", "苹果"),
            make_word(2, "banana", "香蕉"),
        ];
        let mut rng = StdRng::seed_from_u64(1);

        let quiz = first_choice_quiz(&conn, &pool, &mut rng);
        match quiz {
            Quiz::Choice { correct, options, .. } => {
                assert_eq!(options.len(), 4);
                assert!(options.contains(&correct));
            }
            _ => panic!("expected choice quiz after global fallback"),
        }
    }

    #[test]
    fn pool_with_duplicate_meanings_produces_unique_options() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        // Two words share the same meaning; that meaning must not be duplicated in options.
        let pool = vec![
            make_word(1, "apple", "苹果"),
            make_word(2, "banana", "香蕉"),
            make_word(3, "cherry", "樱桃"),
            make_word(4, "date", "枣"),
            make_word(5, "fake_apple", "苹果"), // duplicate meaning
        ];
        let mut rng = StdRng::seed_from_u64(1);

        let quiz = first_choice_quiz(&conn, &pool, &mut rng);
        match quiz {
            Quiz::Choice { word: _word, correct, options } => {
                assert_eq!(options.len(), 4);
                let unique_options: HashSet<String> = options.iter().cloned().collect();
                assert_eq!(unique_options.len(), 4, "choice options must be unique");
                let correct_count = options.iter().filter(|o| *o == &correct).count();
                // The correct answer must appear exactly once and be among the options.
                assert_eq!(correct_count, 1, "correct answer must appear exactly once");
                assert!(options.contains(&correct));
            }
            _ => panic!("expected choice quiz when enough unique distractor meanings exist"),
        }
    }

    #[test]
    fn pool_with_insufficient_meanings_falls_back_to_recall() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let pool = vec![
            make_word(1, "apple", "苹果"),
            make_word(2, "banana", "香蕉"),
        ];
        let mut rng = StdRng::seed_from_u64(1);

        let quiz = first_recall_quiz(&conn, &pool, &mut rng);
        match quiz {
            Quiz::Recall { .. } => {}
            _ => panic!("expected recall when no meaningful distractors exist anywhere"),
        }
    }
}
