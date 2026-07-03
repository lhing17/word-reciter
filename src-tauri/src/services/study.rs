use crate::db::words::Word;
use rand::seq::{IteratorRandom, SliceRandom};
use serde::{Deserialize, Serialize};

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

pub fn generate_quiz(pool: &[Word]) -> Option<Quiz> {
    if pool.is_empty() {
        return None;
    }
    let mut rng = rand::thread_rng();
    let target = pool.choose(&mut rng)?;
    let answer = target.meaning.as_ref()?.trim();
    if answer.is_empty() {
        return None;
    }
    let answer = answer.to_string();

    let quiz_type = rand::random::<u8>() % 10;
    if quiz_type < 4 {
        // choice
        let mut options: Vec<String> = pool
            .iter()
            .filter(|w| w.word != target.word)
            .filter_map(|w| {
                let m = w.meaning.as_ref()?.trim();
                if m.is_empty() { None } else { Some(m.to_string()) }
            })
            .choose_multiple(&mut rng, 3);
        options.push(answer.clone());
        options.shuffle(&mut rng);
        Some(Quiz::Choice {
            word: target.word.clone(),
            correct: answer,
            options,
        })
    } else if quiz_type < 7 {
        // fill
        let chars: Vec<char> = target.word.chars().collect();
        if chars.len() < 3 {
            return Some(Quiz::Recall {
                word: target.word.clone(),
                answer,
            });
        }
        let first = chars.first().unwrap().to_string();
        let last = chars.last().unwrap().to_string();
        Some(Quiz::Fill {
            word: target.word.clone(),
            hint: answer,
            first,
            last,
        })
    } else {
        // recall
        Some(Quiz::Recall {
            word: target.word.clone(),
            answer,
        })
    }
}
