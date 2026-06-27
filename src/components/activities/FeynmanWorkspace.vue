<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlanStore } from '../../stores/plan'
import { PenLine, AlertTriangle, BookOpen } from '@lucide/vue'

const props = defineProps<{
  planId: string
  principleId: number
}>()

const store = usePlanStore()

const step = ref<1 | 2 | 3>(1)

function getActivity(key: string): string {
  const plan = store.getPlan(props.planId)
  const pp = plan?.principles.find((p) => p.principleId === props.principleId)
  return (pp?.activities[key] as string) || ''
}

function setActivity(key: string, value: string) {
  store.updateActivity(props.planId, props.principleId, key, value)
}

const concept = computed({
  get: () => getActivity('feynman_concept'),
  set: (v: string) => setActivity('feynman_concept', v),
})

const explanation = computed({
  get: () => getActivity('feynman_explanation'),
  set: (v: string) => setActivity('feynman_explanation', v),
})

const gaps = computed({
  get: () => getActivity('feynman_gaps'),
  set: (v: string) => setActivity('feynman_gaps', v),
})
</script>

<template>
  <div class="feynman">
    <h3>Feynman Technique</h3>
    <div class="steps">
      <button class="step-btn" :class="{ active: step === 1 }" @click="step = 1">
        <PenLine :size="16" />
        1. Explain
      </button>
      <button class="step-btn" :class="{ active: step === 2 }" @click="step = 2">
        <AlertTriangle :size="16" />
        2. Find Gaps
      </button>
      <button class="step-btn" :class="{ active: step === 3 }" @click="step = 3">
        <BookOpen :size="16" />
        3. Review
      </button>
    </div>

    <div v-if="step === 1" class="step-content">
      <div class="form-group">
        <label>What concept do you want to understand?</label>
        <input v-model="concept" type="text" placeholder="e.g. Ownership in Rust, Backpropagation..." />
      </div>
      <div class="form-group">
        <label>Explain it as if teaching a complete beginner</label>
        <textarea
          v-model="explanation"
          placeholder="Write your explanation here. Use simple language. If you get stuck, that's a gap you need to fill..."
          rows="8"
        ></textarea>
      </div>
      <button class="btn btn-primary" @click="step = 2">I've explained it — find my gaps</button>
    </div>

    <div v-if="step === 2" class="step-content">
      <div class="gap-prompt">
        <AlertTriangle :size="20" />
        <p>
          Where did you get stuck? What parts were hard to explain? What did you have to look up?
          These are your understanding gaps.
        </p>
      </div>
      <div class="form-group">
        <label>What gaps did you find?</label>
        <textarea
          v-model="gaps"
          placeholder="List the specific parts where your understanding broke down..."
          rows="6"
        ></textarea>
      </div>
      <button class="btn btn-primary" @click="step = 3">Review and fill gaps</button>
    </div>

    <div v-if="step === 3" class="step-content">
      <div class="review-summary">
        <div class="review-block">
          <h4>Concept</h4>
          <p>{{ concept || '(not filled)' }}</p>
        </div>
        <div class="review-block">
          <h4>Your Explanation</h4>
          <p class="preserve-whitespace">{{ explanation || '(not filled)' }}</p>
        </div>
        <div class="review-block">
          <h4>Understanding Gaps</h4>
          <p class="preserve-whitespace">{{ gaps || '(not filled)' }}</p>
        </div>
      </div>
      <p class="review-hint">
        Go back to your resources, fill the gaps, then try explaining again from scratch.
      </p>
      <button class="btn btn-secondary" @click="step = 1">Start over</button>
    </div>
  </div>
</template>

<style scoped>
.feynman {
  padding: var(--space-lg);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
}

.feynman h3 {
  margin-bottom: var(--space-lg);
}

.steps {
  display: flex;
  gap: var(--space-sm);
  margin-bottom: var(--space-lg);
}

.step-btn {
  display: flex;
  align-items: center;
  gap: var(--space-xs);
  font-size: 13px;
  font-weight: 600;
  padding: 8px 16px;
  border-radius: var(--rounded-full);
  color: var(--color-mute);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  transition: all 0.15s;
}

.step-btn.active {
  color: var(--color-on-primary);
  background: var(--color-primary);
  border-color: var(--color-primary);
}

.step-content {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
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

.gap-prompt {
  display: flex;
  gap: var(--space-md);
  padding: var(--space-md);
  background: var(--color-warning-muted);
  border-radius: var(--rounded-md);
  color: var(--color-warning);
}

.gap-prompt p {
  font-size: 14px;
  color: var(--color-body);
}

.review-summary {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.review-block {
  padding: var(--space-md);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--rounded-md);
}

.review-block h4 {
  font-size: 12px;
  font-weight: 700;
  text-transform: uppercase;
  color: var(--color-mute);
  margin-bottom: var(--space-xs);
}

.review-block p {
  font-size: 14px;
  color: var(--color-body);
}

.preserve-whitespace {
  white-space: pre-wrap;
}

.review-hint {
  font-size: 14px;
  color: var(--color-mute);
  font-style: italic;
}
</style>
