import { invoke } from '@tauri-apps/api/core'
import type { Stats, ImportResult } from '../types'

export async function importWordList(path: string, source: string): Promise<ImportResult> {
  return invoke('import_word_list', { path, source })
}

export async function getStats(): Promise<Stats> {
  return invoke('get_stats')
}
