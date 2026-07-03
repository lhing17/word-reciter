import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getStats, importWordList } from '../api/tauri'
import type { Stats } from '../types'

export const useWordsStore = defineStore('words', () => {
  const stats = ref<Stats>({ total: 0, unknown: 0, half: 0, known: 0 })
  let importPromise: Promise<void> | null = null

  async function loadStats() {
    stats.value = await getStats()
  }

  async function ensureDefaultWordListImported() {
    if (importPromise) return importPromise

    importPromise = (async () => {
      await importWordList('references/unique_words_with_chinese.txt', 'unique_words_with_chinese.txt')
      await loadStats()
    })().finally(() => {
      importPromise = null
    })

    return importPromise
  }

  return { stats, loadStats, ensureDefaultWordListImported }
})
