import { invoke } from '@tauri-apps/api/core'
import type { Familiarity, Quiz, Stats, StudyResultPayload, Word, ImportResult } from '../types'

export async function importWordList(source: string): Promise<ImportResult> {
  return invoke('import_word_list', { source })
}

export async function getStats(): Promise<Stats> {
  return invoke('get_stats')
}

export async function getNextUnmarkedWord(offset: number): Promise<Word | null> {
  return invoke('get_next_unmarked_word', { offset })
}

export async function markWord(word: string, familiarity: Familiarity): Promise<void> {
  return invoke('mark_word', { word, familiarity })
}

export async function generateQuiz(): Promise<Quiz | null> {
  return invoke('generate_quiz')
}

export async function submitStudyResult(payload: StudyResultPayload): Promise<void> {
  return invoke('submit_study_result', { payload })
}
