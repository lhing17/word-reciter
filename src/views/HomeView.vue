<template>
  <div class="home">
    <h1>Word Reciter</h1>

    <div v-if="error" class="error">{{ error }}</div>

    <div class="stats">
      <div class="stat-card">
        <div class="number">{{ wordsStore.stats.total }}</div>
        <div class="label">总单词</div>
      </div>
      <div class="stat-card">
        <div class="number">{{ wordsStore.stats.unknown }}</div>
        <div class="label">生词</div>
      </div>
      <div class="stat-card">
        <div class="number">{{ wordsStore.stats.half }}</div>
        <div class="label">半熟词</div>
      </div>
      <div class="stat-card">
        <div class="number">{{ wordsStore.stats.known }}</div>
        <div class="label">熟词</div>
      </div>
    </div>

    <div class="actions">
      <router-link class="btn" to="/marking">开始分类模式</router-link>
      <router-link class="btn" to="/study">开始背诵模式</router-link>
    </div>

    <p class="hint">当前词库：朗文 9000 词</p>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useWordsStore } from '../stores/words'

const wordsStore = useWordsStore()
const error = ref<string | null>(null)

onMounted(async () => {
  try {
    await wordsStore.ensureDefaultWordListImported()
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e)
  }
})
</script>

<style scoped>
.home {
  padding: 32px;
  max-width: 600px;
  margin: 0 auto;
}
.stats {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
  margin: 24px 0;
}
.stat-card {
  background: #f5f5f5;
  border-radius: 8px;
  padding: 16px;
  text-align: center;
}
.number {
  font-size: 24px;
  font-weight: bold;
}
.label {
  color: #666;
  font-size: 14px;
  margin-top: 4px;
}
.actions {
  display: flex;
  gap: 16px;
  margin-top: 24px;
}
.btn {
  flex: 1;
  display: block;
  text-align: center;
  padding: 12px;
  background: #1976d2;
  color: white;
  text-decoration: none;
  border-radius: 6px;
  border: none;
  cursor: pointer;
}
.hint {
  color: #999;
  margin-top: 24px;
  text-align: center;
}
.error {
  color: #c00;
  background: #ffeaea;
  padding: 12px;
  border-radius: 6px;
  margin-bottom: 16px;
}
</style>
