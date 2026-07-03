import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useStudyStore } from '../study'
import * as tauri from '../../api/tauri'
import type { Quiz } from '../../types'

vi.mock('../../api/tauri', () => ({
  generateQuiz: vi.fn(),
  submitStudyResult: vi.fn(),
}))

const choiceQuiz: Quiz = {
  type: 'choice',
  word: 'example',
  correct: '例子',
  options: ['例子', '样品', '例外', '经验'],
}

describe('useStudyStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.resetAllMocks()
  })

  it('loadQuiz resets state and stores the generated quiz', async () => {
    vi.mocked(tauri.generateQuiz).mockResolvedValue(choiceQuiz)

    const store = useStudyStore()
    const promise = store.loadQuiz()

    expect(store.loading).toBe(true)
    expect(store.answered).toBe(false)
    expect(store.result).toBeNull()

    await promise

    expect(store.loading).toBe(false)
    expect(store.currentQuiz).toEqual(choiceQuiz)
    expect(store.error).toBeNull()
  })

  it('loadQuiz records an error when generation fails', async () => {
    vi.mocked(tauri.generateQuiz).mockRejectedValue(new Error('boom'))

    const store = useStudyStore()
    await store.loadQuiz()

    expect(store.loading).toBe(false)
    expect(store.currentQuiz).toBeNull()
    expect(store.error).toBe('boom')
  })

  it('recordAnswer updates session state and prevents duplicate answers', () => {
    vi.mocked(tauri.generateQuiz).mockResolvedValue(choiceQuiz)

    const store = useStudyStore()
    store.currentQuiz = choiceQuiz
    store.recordAnswer(true)

    expect(store.answered).toBe(true)
    expect(store.result).toBe('correct')
    expect(store.sessionTotal).toBe(1)
    expect(store.sessionCorrect).toBe(1)

    store.recordAnswer(true)
    expect(store.sessionTotal).toBe(1)
    expect(store.sessionCorrect).toBe(1)
  })

  it('finishQuiz submits the result and loads the next quiz', async () => {
    vi.mocked(tauri.generateQuiz).mockResolvedValue(choiceQuiz)
    vi.mocked(tauri.submitStudyResult).mockResolvedValue(undefined)

    const store = useStudyStore()
    await store.loadQuiz()
    store.recordAnswer(true)

    const finished = await store.finishQuiz('known')

    expect(finished).toBe(true)
    expect(tauri.submitStudyResult).toHaveBeenCalledWith({
      word: 'example',
      quiz_type: 'choice',
      result: 'correct',
      familiarity_after: 'known',
    })
    expect(tauri.generateQuiz).toHaveBeenCalledTimes(2)
    expect(store.submitting).toBe(false)
  })

  it('finishQuiz returns false when no quiz or answer is present', async () => {
    const store = useStudyStore()
    expect(await store.finishQuiz('known')).toBe(false)
  })
})
