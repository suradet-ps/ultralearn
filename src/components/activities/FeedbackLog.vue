<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlanStore } from '../../stores/plan'
import { Plus } from '@lucide/vue'
import type { FeedbackEntry } from '../../types/principle'

const props = defineProps<{
  planId: string
  principleId: number
}>()

const store = usePlanStore()

const type = ref<FeedbackEntry['type']>('outcome')
const text = ref('')

const log = computed(() => store.getFeedbackLog(props.planId, props.principleId))

const typeLabels: Record<FeedbackEntry['type'], string> = {
  outcome: 'Outcome',
  informational: 'Informational',
  corrective: 'Corrective',
}

const typeColors: Record<FeedbackEntry['type'], string> = {
  outcome: 'var(--color-primary)',
  informational: 'var(--color-warning)',
  corrective: 'var(--color-success)',
}

function addEntry() {
  if (!text.value.trim()) return
  store.addFeedback(props.planId, props.principleId, type.value, text.value.trim())
  text.value = ''
}

function formatDate(iso: string) {
  return new Date(iso).toLocaleDateString(undefined, {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}
</script>

<template>
  <div class="feedback-section">
    <h3>Feedback Log</h3>

    <div class="add-form">
      <div class="type-select">
        <button
          v-for="(label, key) in typeLabels"
          :key="key"
          class="type-btn"
          :class="{ active: type === key }"
          :style="type === key ? { background: typeColors[key as FeedbackEntry['type']], color: 'white' } : {}"
          @click="type = key as FeedbackEntry['type']"
        >
          {{ label }}
        </button>
      </div>
      <div class="form-row">
        <input
          v-model="text"
          type="text"
          placeholder="What feedback did you get?"
          @keyup.enter="addEntry"
        />
        <button class="btn btn-primary" @click="addEntry" :disabled="!text.trim()">
          <Plus :size="16" />
        </button>
      </div>
    </div>

    <div v-if="log.length === 0" class="empty">No feedback logged yet.</div>

    <div v-else class="log-list">
      <div v-for="entry in log" :key="entry.id" class="log-entry">
        <div class="log-type" :style="{ background: typeColors[entry.type] }">
          {{ typeLabels[entry.type] }}
        </div>
        <div class="log-content">
          <p>{{ entry.text }}</p>
          <span class="log-date">{{ formatDate(entry.date) }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.feedback-section {
  margin-bottom: var(--space-xl);
}

.feedback-section h3 {
  margin-bottom: var(--space-md);
}

.add-form {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
  padding: var(--space-md);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
  margin-bottom: var(--space-md);
}

.type-select {
  display: flex;
  gap: var(--space-xs);
}

.type-btn {
  font-size: 12px;
  font-weight: 600;
  padding: 6px 12px;
  border-radius: var(--rounded-full);
  color: var(--color-mute);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  transition: all 0.15s;
}

.type-btn.active {
  border-color: transparent;
}

.form-row {
  display: flex;
  gap: var(--space-sm);
}

.form-row input {
  flex: 1;
}

.form-row .btn {
  flex-shrink: 0;
  padding: 12px 16px;
}

.empty {
  text-align: center;
  padding: var(--space-xl);
  color: var(--color-mute);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
}

.log-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.log-entry {
  display: flex;
  gap: var(--space-md);
  padding: var(--space-md);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
}

.log-type {
  flex-shrink: 0;
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  padding: 4px 10px;
  border-radius: var(--rounded-full);
  color: white;
  height: fit-content;
}

.log-content {
  flex: 1;
}

.log-content p {
  font-size: 14px;
  color: var(--color-body);
  margin-bottom: var(--space-xs);
}

.log-date {
  font-size: 12px;
  color: var(--color-mute);
}
</style>
