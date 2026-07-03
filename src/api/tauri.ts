import { invoke } from '@tauri-apps/api/core'
import type { Familiarity, Stats, Word, ImportResult } from '../types'

export async function importWordList(path: string, source: string): Promise<ImportResult> {
  return invoke('import_word_list', { path, source })
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
