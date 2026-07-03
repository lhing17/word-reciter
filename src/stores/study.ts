import { defineStore } from 'pinia'
import { ref } from 'vue'
import { generateQuiz, submitStudyResult } from '../api/tauri'
import type { Quiz, Familiarity, ResultType } from '../types'

export const useStudyStore = defineStore('study', () => {
  const currentQuiz = ref<Quiz | null>(null)
  const answered = ref(false)
  const result = ref<ResultType | null>(null)
  const sessionTotal = ref(0)
  const sessionCorrect = ref(0)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function loadQuiz() {
    answered.value = false
    result.value = null
    error.value = null
    loading.value = true
    try {
      currentQuiz.value = await generateQuiz()
    } catch (e) {
      currentQuiz.value = null
      error.value = e instanceof Error ? e.message : '加载题目失败，请稍后重试。'
    } finally {
      loading.value = false
    }
  }

  function recordAnswer(isCorrect: boolean) {
    answered.value = true
    result.value = isCorrect ? 'correct' : 'wrong'
    sessionTotal.value += 1
    if (isCorrect) sessionCorrect.value += 1
  }

  async function finishQuiz(familiarityAfter: Familiarity) {
    if (!currentQuiz.value || !result.value) return
    await submitStudyResult({
      word: currentQuiz.value.word,
      quiz_type: currentQuiz.value.type,
      result: result.value,
      familiarity_after: familiarityAfter,
    })
    await loadQuiz()
  }

  return {
    currentQuiz,
    answered,
    result,
    sessionTotal,
    sessionCorrect,
    loading,
    error,
    loadQuiz,
    recordAnswer,
    finishQuiz,
  }
})
