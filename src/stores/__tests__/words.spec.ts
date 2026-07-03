import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useWordsStore } from '../words'
import * as tauri from '../../api/tauri'

vi.mock('../../api/tauri', () => ({
  getStats: vi.fn(),
  importWordList: vi.fn(),
}))

describe('useWordsStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.resetAllMocks()
  })

  it('loadStats fetches and stores stats', async () => {
    vi.mocked(tauri.getStats).mockResolvedValue({
      total: 10,
      unknown: 5,
      half: 3,
      known: 2,
    })

    const store = useWordsStore()
    await store.loadStats()

    expect(tauri.getStats).toHaveBeenCalledTimes(1)
    expect(store.stats).toEqual({ total: 10, unknown: 5, half: 3, known: 2 })
  })

  it('ensureDefaultWordListImported imports the default list and refreshes stats', async () => {
    vi.mocked(tauri.importWordList).mockResolvedValue({ imported: 5, skipped: 0 })
    vi.mocked(tauri.getStats).mockResolvedValue({
      total: 5,
      unknown: 5,
      half: 0,
      known: 0,
    })

    const store = useWordsStore()
    await store.ensureDefaultWordListImported()

    expect(tauri.importWordList).toHaveBeenCalledWith('unique_words_with_chinese.txt')
    expect(tauri.getStats).toHaveBeenCalledTimes(1)
    expect(store.stats).toEqual({ total: 5, unknown: 5, half: 0, known: 0 })
  })

  it('concurrent ensureDefaultWordListImported calls share one import promise', async () => {
    let importCalls = 0
    vi.mocked(tauri.importWordList).mockImplementation(async () => {
      importCalls += 1
      await new Promise((resolve) => setTimeout(resolve, 10))
      return { imported: 1, skipped: 0 }
    })
    vi.mocked(tauri.getStats).mockResolvedValue({
      total: 1,
      unknown: 1,
      half: 0,
      known: 0,
    })

    const store = useWordsStore()
    const [first, second] = [
      store.ensureDefaultWordListImported(),
      store.ensureDefaultWordListImported(),
    ]

    await Promise.all([first, second])
    expect(importCalls).toBe(1)
    expect(tauri.getStats).toHaveBeenCalledTimes(1)
  })
})
