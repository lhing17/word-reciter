import { defineStore } from 'pinia'
import { ref } from 'vue'
import { getStats, importWordList } from '../api/tauri'
import type { Stats } from '../types'

export const useWordsStore = defineStore('words', () => {
  const stats = ref<Stats>({ total: 0, unknown: 0, half: 0, known: 0 })

  async function loadStats() {
    stats.value = await getStats()
  }

  async function ensureDefaultWordListImported() {
    await importWordList('references/unique_words_with_chinese.txt', 'unique_words_with_chinese.txt')
    await loadStats()
  }

  return { stats, loadStats, ensureDefaultWordListImported }
})
