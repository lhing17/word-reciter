<template>
  <div class="mark-buttons">
    <button class="btn unknown" @click="emit('mark', 'unknown')">生词 (1)</button>
    <button class="btn half" @click="emit('mark', 'half')">半熟词 (2)</button>
    <button class="btn known" @click="emit('mark', 'known')">熟词 (3)</button>
  </div>
</template>

<script setup lang="ts">
import type { Familiarity } from '../types'

const emit = defineEmits<{
  (e: 'mark', value: Familiarity): void
}>()
</script>

<style scoped>
.mark-buttons {
  display: flex;
  gap: 16px;
  justify-content: center;
}
.btn {
  padding: 12px 24px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
}
.unknown { background: #ffebee; color: #c62828; }
.half { background: #fff3e0; color: #ef6c00; }
.known { background: #e8f5e9; color: #2e7d32; }
</style>
