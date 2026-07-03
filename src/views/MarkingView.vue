<template>
  <div class="marking-view">
    <div class="header">
      <router-link to="/">← 返回首页</router-link>
      <span>分类模式 — {{ progress }} / {{ stats.total }}</span>
    </div>

    <div v-if="currentWord" class="card">
      <div class="word">{{ currentWord.word }}</div>
      <div class="hint">按 1/2/3 或点击下方按钮标记</div>
      <MarkButtons @mark="onMark" />
      <button class="skip" @click="nextWord">跳过</button>
    </div>

    <div v-else class="empty">
      所有单词已分类完成 🎉
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { getNextUnmarkedWord, markWord } from '../api/tauri'
import { useWordsStore } from '../stores/words'
import MarkButtons from '../components/MarkButtons.vue'
import type { Familiarity, Word } from '../types'

const router = useRouter()
const wordsStore = useWordsStore()
const currentWord = ref<Word | null>(null)
const offset = ref(0)

const stats = computed(() => wordsStore.stats)
const progress = computed(() => wordsStore.stats.unknown + wordsStore.stats.half + wordsStore.stats.known)

async function loadNext() {
  currentWord.value = await getNextUnmarkedWord(offset.value)
}

function nextWord() {
  offset.value += 1
  loadNext()
}

async function onMark(familiarity: Familiarity) {
  if (!currentWord.value) return
  await markWord(currentWord.value.word, familiarity)
  await wordsStore.loadStats()
  offset.value = 0
  loadNext()
}

function onKeyDown(e: KeyboardEvent) {
  if (e.key === '1') onMark('unknown')
  if (e.key === '2') onMark('half')
  if (e.key === '3') onMark('known')
  if (e.key === 'Escape') router.push('/')
}

onMounted(() => {
  loadNext()
  window.addEventListener('keydown', onKeyDown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', onKeyDown)
})
</script>

<style scoped>
.marking-view { padding: 24px; max-width: 700px; margin: 0 auto; }
.header { display: flex; justify-content: space-between; margin-bottom: 40px; }
.card { text-align: center; padding: 40px 0; }
.word { font-size: 48px; font-weight: bold; margin-bottom: 16px; }
.hint { color: #666; margin-bottom: 32px; }
.skip { margin-top: 24px; background: none; border: none; color: #999; cursor: pointer; }
.empty { text-align: center; padding: 80px 0; font-size: 20px; color: #666; }
</style>
