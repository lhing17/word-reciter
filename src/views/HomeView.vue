<template>
  <div style="padding: 24px">
    <h1>Word Reciter</h1>
    <button @click="importDefault">导入默认词库</button>
    <p v-if="result">导入 {{ result.imported }} 个，跳过 {{ result.skipped }} 个</p>
    <router-link to="/marking">分类模式</router-link>
    <router-link to="/study">背诵模式</router-link>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { importWordList, type ImportResult } from '../api/tauri'

const result = ref<ImportResult | null>(null)

async function importDefault() {
  result.value = await importWordList('references/unique_words_with_chinese.txt', 'unique_words_with_chinese.txt')
}
</script>
