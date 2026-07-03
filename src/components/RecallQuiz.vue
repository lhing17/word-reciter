<template>
  <div class="recall-quiz">
    <div class="word">{{ quiz.word }}</div>
    <button v-if="!showAnswer" class="show-btn" @click="showAnswer = true">显示答案</button>
    <div v-else class="answer">
      <div>{{ quiz.answer }}</div>
      <div v-if="!answered" class="self-eval">
        <span>是否答对？</span>
        <button @click="emit('answer', true)">对了</button>
        <button @click="emit('answer', false)">错了</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import type { RecallQuiz } from '../types'

const props = defineProps<{ quiz: RecallQuiz; answered: boolean }>()
const emit = defineEmits<{ (e: 'answer', correct: boolean): void }>()

const showAnswer = ref(false)

watch(() => props.quiz, () => {
  showAnswer.value = false
}, { deep: true })
</script>

<style scoped>
.word { font-size: 36px; font-weight: bold; text-align: center; margin-bottom: 24px; }
.show-btn { display: block; margin: 0 auto; padding: 10px 32px; }
.answer { text-align: center; font-size: 18px; }
.self-eval { margin-top: 16px; }
.self-eval button { margin: 0 8px; }
</style>
