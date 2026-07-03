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

  async function loadQuiz() {
    answered.value = false
    result.value = null
    currentQuiz.value = await generateQuiz()
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
    loadQuiz,
    recordAnswer,
    finishQuiz,
  }
})
