<script setup lang="ts">
import { ref } from 'vue'
import { usePlanStore } from '../stores/plan'
import { Plus, X, Check } from '@lucide/vue'

const props = defineProps<{
  planId: string
  principleId: number
}>()

const store = usePlanStore()
const newItemText = ref('')

function plan() {
  return store.getPlan(props.planId)
}

function progress() {
  return plan()?.principles.find((p) => p.principleId === props.principleId)
}

function addItem() {
  if (!newItemText.value.trim()) return
  store.addChecklistItem(props.planId, props.principleId, newItemText.value.trim())
  newItemText.value = ''
}
</script>

<template>
  <div class="checklist-section" v-if="progress()">
    <h3>Checklist</h3>
    <div class="checklist">
      <div
        v-for="item in progress()!.checklist"
        :key="item.id"
        class="checklist-item"
        :class="{ checked: item.checked }"
      >
        <button
          class="check-btn"
          @click="store.toggleChecklistItem(planId, principleId, item.id)"
          :aria-label="item.checked ? 'Uncheck' : 'Check'"
        >
          <Check v-if="item.checked" :size="14" />
        </button>
        <span class="checklist-text">{{ item.text }}</span>
        <button
          class="remove-btn"
          @click="store.removeChecklistItem(planId, principleId, item.id)"
          aria-label="Remove"
        >
          <X :size="14" />
        </button>
      </div>
    </div>
    <div class="add-item">
      <input
        v-model="newItemText"
        type="text"
        placeholder="Add a custom item..."
        @keyup.enter="addItem"
      />
      <button class="btn btn-secondary" @click="addItem" :disabled="!newItemText.trim()">
        <Plus :size="16" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.checklist-section {
  margin-bottom: var(--space-xl);
}

.checklist-section h3 {
  margin-bottom: var(--space-md);
}

.checklist {
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
  margin-bottom: var(--space-md);
}

.checklist-item {
  display: flex;
  align-items: flex-start;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-md);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
}

.checklist-item.checked .checklist-text {
  text-decoration: line-through;
  color: var(--color-mute);
}

.check-btn {
  width: 22px;
  height: 22px;
  flex-shrink: 0;
  border: 2px solid var(--color-border-strong);
  border-radius: var(--rounded-sm);
  background: var(--color-surface);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: 1px;
  transition: background 0.15s, border-color 0.15s;
}

.checklist-item.checked .check-btn {
  background: var(--color-success);
  border-color: var(--color-success);
  color: white;
}

.checklist-text {
  flex: 1;
  font-size: 14px;
  line-height: 1.5;
}

.remove-btn {
  flex-shrink: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-mute);
  border-radius: var(--rounded-sm);
  opacity: 0;
  transition: opacity 0.15s, color 0.15s;
}

.checklist-item:hover .remove-btn {
  opacity: 1;
}

.remove-btn:hover {
  color: var(--color-error);
}

.add-item {
  display: flex;
  gap: var(--space-sm);
}

.add-item input {
  flex: 1;
}

.add-item .btn {
  flex-shrink: 0;
  padding: 12px 16px;
}
</style>
