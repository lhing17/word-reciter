use crate::db::words::Word;
use rand::seq::{IteratorRandom, SliceRandom};
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
    if pool.is_empty() {
        return Ok(None);
    }
    let mut rng = rand::thread_rng();
    let target = pool
        .choose(&mut rng)
        .ok_or_else(|| "empty pool".to_string())?;
    let answer = target
        .meaning
        .as_ref()
        .map(|m| m.trim())
        .filter(|m| !m.is_empty())
        .ok_or_else(|| "target has no meaning".to_string())?
        .to_string();

    let quiz_type = rand::random::<u8>() % 10;
    if quiz_type < 4 {
        // choice: ensure 4 options (1 correct + 3 distractors)
        let mut distractors: Vec<String> = pool
            .iter()
            .filter(|w| w.word != target.word)
            .filter_map(|w| {
                let m = w.meaning.as_ref()?.trim();
                if m.is_empty() { None } else { Some(m.to_string()) }
            })
            .choose_multiple(&mut rng, 3);

        if distractors.len() < 3 {
            let fallback = fetch_meaningful_words_excluding(conn, &target.word)?;
            let used: HashSet<String> = distractors
                .iter()
                .cloned()
                .chain(std::iter::once(answer.clone()))
                .collect();
            let needed = 3 - distractors.len();
            let extra: Vec<String> = fallback
                .into_iter()
                .filter(|m| !used.contains(m))
                .choose_multiple(&mut rng, needed);
            distractors.extend(extra);
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
        options.shuffle(&mut rng);
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
