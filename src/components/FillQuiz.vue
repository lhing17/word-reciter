<template>
  <div class="fill-quiz">
    <div class="hint">{{ quiz.hint }}</div>
    <div class="prompt">
      <span>{{ quiz.first }}</span>
      <input v-model="input" :disabled="answered" @keyup.enter="submit" />
      <span>{{ quiz.last }}</span>
    </div>
    <button v-if="!answered" class="submit-btn" @click="submit">提交</button>
    <div v-else class="answer">正确答案：{{ quiz.word }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { FillQuiz } from '../types'

const props = defineProps<{ quiz: FillQuiz; answered: boolean }>()
const emit = defineEmits<{ (e: 'answer', correct: boolean): void }>()

const input = ref('')

function normalize(s: string) {
  return s.trim().toLowerCase().replace(/[.,!?]$/, '')
}

function submit() {
  const correct = normalize(input.value) === normalize(props.quiz.word)
  emit('answer', correct)
}
</script>

<style scoped>
.hint { font-size: 20px; text-align: center; margin-bottom: 16px; }
.prompt { display: flex; align-items: center; justify-content: center; gap: 8px; font-size: 28px; margin-bottom: 24px; }
.prompt input { width: 160px; text-align: center; font-size: 24px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; text-transform: lowercase; }
.submit-btn { display: block; margin: 0 auto; padding: 10px 32px; font-size: 16px; }
.answer { text-align: center; font-size: 18px; color: #2e7d32; }
</style>
