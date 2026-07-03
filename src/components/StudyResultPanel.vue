<template>
  <div class="result-panel">
    <p>正确答案：{{ answer }}</p>
    <div class="mark-buttons">
      <button class="btn unknown" :disabled="disabled" @click="emit('finish', 'unknown')">仍为生词</button>
      <button class="btn half" :disabled="disabled" @click="emit('finish', 'half')">半熟词</button>
      <button class="btn known" :disabled="disabled" @click="emit('finish', 'known')">熟词</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Familiarity } from '../types'

defineProps<{ answer: string; disabled?: boolean }>()
const emit = defineEmits<{ (e: 'finish', value: Familiarity): void }>()
</script>

<style scoped>
.result-panel { text-align: center; margin-top: 24px; }
.mark-buttons { display: flex; gap: 12px; justify-content: center; margin-top: 12px; }
.btn { padding: 10px 20px; border: none; border-radius: 6px; cursor: pointer; }
.btn:disabled { opacity: 0.6; cursor: not-allowed; }
.unknown { background: #ffebee; color: #c62828; }
.half { background: #fff3e0; color: #ef6c00; }
.known { background: #e8f5e9; color: #2e7d32; }
</style>
