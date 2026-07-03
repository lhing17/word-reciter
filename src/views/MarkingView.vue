<template>
  <div class="marking-view">
    <div class="header">
      <router-link to="/">← 返回首页</router-link>
      <span>分类模式 — {{ progress }} / {{ stats.total }}</span>
    </div>

    <div v-if="error" class="error">
      {{ error }}
      <button class="retry" @click="loadNext">重试</button>
    </div>

    <div v-if="currentWord" class="card">
      <div class="word">{{ currentWord.word }}</div>
      <div class="hint">按 1/2/3 或点击下方按钮标记</div>
      <MarkButtons :disabled="isProcessing" @mark="onMark" />
      <button class="skip" :disabled="isProcessing" @click="nextWord">跳过</button>
    </div>

    <div v-else-if="isInitializing || isLoading" class="empty">加载中……</div>
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
const isProcessing = ref(false)
const isLoading = ref(false)
const isInitializing = ref(true)
const error = ref<string | null>(null)

const stats = computed(() => wordsStore.stats)
const progress = computed(() => wordsStore.stats.unknown + wordsStore.stats.half + wordsStore.stats.known)

async function loadNext() {
  isLoading.value = true
  error.value = null
  try {
    let word = await getNextUnmarkedWord(offset.value)
    if (!word && offset.value > 0) {
      offset.value = 0
      word = await getNextUnmarkedWord(0)
    }
    currentWord.value = word
  } catch (e) {
    error.value = e instanceof Error ? e.message : '加载单词失败'
  } finally {
    isLoading.value = false
  }
}

async function nextWord() {
  if (isProcessing.value) return
  offset.value += 1
  await loadNext()
}

async function onMark(familiarity: Familiarity) {
  if (!currentWord.value || isProcessing.value) return
  isProcessing.value = true
  error.value = null
  try {
    await markWord(currentWord.value.word, familiarity)
    await wordsStore.loadStats()
    offset.value = 0
    await loadNext()
  } catch (e) {
    error.value = e instanceof Error ? e.message : '标记失败'
  } finally {
    isProcessing.value = false
  }
}

function onKeyDown(e: KeyboardEvent) {
  if (isProcessing.value) return
  if (e.key === '1') onMark('unknown')
  if (e.key === '2') onMark('half')
  if (e.key === '3') onMark('known')
  if (e.key === 'Escape') router.push('/')
}

onMounted(async () => {
  try {
    await wordsStore.ensureDefaultWordListImported()
    await loadNext()
  } catch (e) {
    error.value = e instanceof Error ? e.message : '初始化失败'
  } finally {
    isInitializing.value = false
  }
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
.skip:disabled { color: #ccc; cursor: not-allowed; }
.empty { text-align: center; padding: 80px 0; font-size: 20px; color: #666; }
.error { color: #c62828; background: #ffebee; padding: 16px; border-radius: 6px; margin-bottom: 24px; display: flex; justify-content: space-between; align-items: center; }
.retry { background: #c62828; color: #fff; border: none; border-radius: 4px; padding: 6px 12px; cursor: pointer; }
</style>
