<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlanStore } from '../../stores/plan'
import { Plus, FlaskConical } from '@lucide/vue'
import type { Experiment } from '../../types/principle'

const props = defineProps<{
  planId: string
  principleId: number
}>()

const store = usePlanStore()

const hypothesis = ref('')
const method = ref('')
const showForm = ref(false)

const experiments = computed(() => store.getExperiments(props.planId, props.principleId))

const statusColors: Record<Experiment['status'], string> = {
  planned: 'var(--color-mute)',
  running: 'var(--color-warning)',
  completed: 'var(--color-success)',
}

function addExp() {
  if (!hypothesis.value.trim() || !method.value.trim()) return
  store.addExperiment(props.planId, props.principleId, hypothesis.value.trim(), method.value.trim())
  hypothesis.value = ''
  method.value = ''
  showForm.value = false
}

function cycleStatus(exp: Experiment) {
  const next: Record<Experiment['status'], Experiment['status']> = {
    planned: 'running',
    running: 'completed',
    completed: 'planned',
  }
  store.updateExperiment(props.planId, props.principleId, exp.id, { status: next[exp.status] })
}
</script>

<template>
  <div class="experiment-section">
    <div class="section-header">
      <h3>Experiments</h3>
      <button class="btn btn-secondary" @click="showForm = !showForm">
        <Plus :size="16" />
        New Experiment
      </button>
    </div>

    <div v-if="showForm" class="add-form">
      <div class="form-group">
        <label>Hypothesis — What do you want to try?</label>
        <input v-model="hypothesis" type="text" placeholder="e.g. Learning from video tutorials works better than reading docs" />
      </div>
      <div class="form-group">
        <label>Method — How will you test it?</label>
        <textarea v-model="method" placeholder="e.g. Spend 1 week on video tutorials, then 1 week on docs. Compare retention and speed." rows="3"></textarea>
      </div>
      <div class="form-actions">
        <button class="btn btn-primary" @click="addExp" :disabled="!hypothesis.trim() || !method.trim()">
          Add Experiment
        </button>
        <button class="btn btn-ghost" @click="showForm = false">Cancel</button>
      </div>
    </div>

    <div v-if="experiments.length === 0" class="empty">
      <FlaskConical :size="32" />
      <p>No experiments yet. Try something new and track the results.</p>
    </div>

    <div v-else class="exp-list">
      <div v-for="exp in experiments" :key="exp.id" class="exp-card">
        <div class="exp-header">
          <button
            class="status-badge"
            :style="{ background: statusColors[exp.status] }"
            @click="cycleStatus(exp)"
          >
            {{ exp.status }}
          </button>
        </div>
        <div class="exp-hypothesis">{{ exp.hypothesis }}</div>
        <div class="exp-method">{{ exp.method }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.experiment-section {
  margin-bottom: var(--space-xl);
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-md);
}

.add-form {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
  padding: var(--space-md);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
  margin-bottom: var(--space-md);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
}

.form-group label {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-ink);
}

.form-actions {
  display: flex;
  gap: var(--space-sm);
}

.empty {
  text-align: center;
  padding: var(--space-2xl);
  color: var(--color-mute);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
}

.empty p {
  margin-top: var(--space-sm);
  font-size: 14px;
}

.exp-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.exp-card {
  padding: var(--space-md);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
}

.exp-header {
  margin-bottom: var(--space-sm);
}

.status-badge {
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  padding: 4px 12px;
  border-radius: var(--rounded-full);
  color: white;
  cursor: pointer;
}

.exp-hypothesis {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-ink);
  margin-bottom: var(--space-xs);
}

.exp-method {
  font-size: 14px;
  color: var(--color-body);
}
</style>
