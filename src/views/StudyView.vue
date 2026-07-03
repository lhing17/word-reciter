<template>
  <div class="study-view">
    <div class="header">
      <router-link to="/">← 返回首页</router-link>
      <span>背诵模式 — 第 {{ studyStore.sessionTotal }} 题 / 正确率 {{ accuracy }}</span>
    </div>

    <div v-if="!studyStore.loading && studyStore.currentQuiz" class="quiz-area">
      <ChoiceQuiz
        v-if="studyStore.currentQuiz.type === 'choice'"
        :key="studyStore.currentQuiz.word"
        :quiz="studyStore.currentQuiz"
        v-model:selected="selectedOption"
        :answered="studyStore.answered"
        @answer="onAnswer"
      />
      <FillQuiz
        v-else-if="studyStore.currentQuiz.type === 'fill'"
        :key="studyStore.currentQuiz.word"
        :quiz="studyStore.currentQuiz"
        :answered="studyStore.answered"
        @answer="onAnswer"
      />
      <RecallQuiz
        v-else
        :key="studyStore.currentQuiz.word"
        :quiz="studyStore.currentQuiz"
        :answered="studyStore.answered"
        @answer="onAnswer"
      />

      <StudyResultPanel
        v-if="studyStore.answered && studyStore.currentQuiz"
        :answer="correctAnswer"
        :disabled="studyStore.submitting"
        @finish="onFinish"
      />
      <div v-if="studyStore.error" class="error" style="margin-top: 16px; text-align: center;">{{ studyStore.error }}</div>
    </div>

    <div v-else-if="studyStore.loading" class="empty">加载中……</div>
    <div v-else-if="studyStore.error" class="empty error">
      <div>{{ studyStore.error }}</div>
      <button class="retry-btn" @click="studyStore.loadQuiz()">重试</button>
    </div>
    <div v-else class="empty">
      当前没有可背诵的单词。请先标记一些生词/半熟词，或检查词库是否包含中文释义。
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useStudyStore } from '../stores/study'
import { useWordsStore } from '../stores/words'
import ChoiceQuiz from '../components/ChoiceQuiz.vue'
import FillQuiz from '../components/FillQuiz.vue'
import RecallQuiz from '../components/RecallQuiz.vue'
import StudyResultPanel from '../components/StudyResultPanel.vue'
import type { Familiarity } from '../types'

const studyStore = useStudyStore()
const wordsStore = useWordsStore()
const selectedOption = ref<string | null>(null)

const accuracy = computed(() => {
  if (studyStore.sessionTotal === 0) return '0%'
  return `${Math.round((studyStore.sessionCorrect / studyStore.sessionTotal) * 100)}%`
})

const correctAnswer = computed(() => {
  const q = studyStore.currentQuiz
  if (!q) return ''
  if (q.type === 'choice') return q.correct
  if (q.type === 'recall') return q.answer
  return q.word
})

function onAnswer(correct: boolean) {
  studyStore.recordAnswer(correct)
}

async function onFinish(familiarity: Familiarity) {
  const success = await studyStore.finishQuiz(familiarity)
  if (success) {
    selectedOption.value = null
    await wordsStore.loadStats()
  }
}

onMounted(() => {
  studyStore.loadQuiz()
})
</script>

<style scoped>
.study-view { padding: 24px; max-width: 700px; margin: 0 auto; }
.header { display: flex; justify-content: space-between; margin-bottom: 32px; }
.empty { text-align: center; padding: 80px 0; font-size: 18px; color: #666; }
.empty.error { color: #c62828; }
.retry-btn { margin-top: 16px; padding: 8px 24px; border: 1px solid #c62828; border-radius: 6px; background: #c62828; color: #fff; cursor: pointer; }
.retry-btn:hover { background: #a71d1d; }
</style>
