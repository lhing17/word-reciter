import { invoke } from '@tauri-apps/api/core'

export interface ImportResult {
  imported: number
  skipped: number
}

export async function importWordList(path: string, source: string): Promise<ImportResult> {
  return invoke('import_word_list', { path, source })
}
