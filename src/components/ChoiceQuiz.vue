<template>
  <div class="choice-quiz">
    <div class="word">{{ quiz.word }}</div>
    <div class="options">
      <button
        v-for="opt in quiz.options"
        :key="opt"
        class="option-btn"
        :class="buttonClass(opt)"
        :disabled="answered"
        @click="select(opt)"
      >
        {{ opt }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { ChoiceQuiz } from '../types'

const props = defineProps<{
  quiz: ChoiceQuiz
  answered: boolean
}>()
const emit = defineEmits<{
  (e: 'answer', correct: boolean): void
}>()

const selected = defineModel<string | null>('selected', { default: null })

function select(opt: string) {
  selected.value = opt
  emit('answer', opt === props.quiz.correct)
}

const buttonClass = computed(() => (opt: string) => {
  if (!props.answered) return ''
  if (opt === props.quiz.correct) return 'correct'
  if (opt === selected.value) return 'wrong'
  return ''
})
</script>

<style scoped>
.word { font-size: 36px; font-weight: bold; text-align: center; margin-bottom: 24px; }
.options { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; max-width: 480px; margin: 0 auto; }
.option-btn { padding: 16px; border: 1px solid #ddd; border-radius: 8px; background: white; cursor: pointer; }
.option-btn:hover:not(:disabled) { background: #f5f5f5; }
.option-btn.correct { background: #e8f5e9; border-color: #2e7d32; }
.option-btn.wrong { background: #ffebee; border-color: #c62828; }
</style>
