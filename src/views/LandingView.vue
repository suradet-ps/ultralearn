<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { usePlanStore } from '../stores/plan'
import { ArrowRight, Sparkles } from '@lucide/vue'

const router = useRouter()
const store = usePlanStore()

const topic = ref('')
const goal = ref('')

const examples = [
  'Rust Programming',
  'Machine Learning',
  'Guitar',
  'Japanese Language',
  'Data Structures & Algorithms',
  'Digital Painting',
]

function handleCreate() {
  if (!topic.value.trim()) return
  const plan = store.createPlan(topic.value.trim(), goal.value.trim())
  router.push({ name: 'plan', params: { id: plan.id } })
}

function useExample(example: string) {
  topic.value = example
}
</script>

<template>
  <div class="landing">
    <section class="hero">
      <div class="hero-inner">
        <div class="hero-badge">
          <Sparkles :size="14" />
          <span>Based on Scott Young's Ultralearning</span>
        </div>
        <h1 class="hero-title">What do you want to learn?</h1>
        <p class="hero-subtitle">
          Enter a topic and build a structured learning plan using the 9 Ultralearning principles.
        </p>

        <form class="hero-form" @submit.prevent="handleCreate">
          <div class="form-group">
            <label for="topic" class="form-label">Topic</label>
            <input
              id="topic"
              v-model="topic"
              type="text"
              placeholder="e.g. Rust Programming, Guitar, Machine Learning..."
              class="form-input"
              required
            />
          </div>

          <div class="form-group">
            <label for="goal" class="form-label">Goal (optional)</label>
            <textarea
              id="goal"
              v-model="goal"
              placeholder="What do you want to achieve? e.g. Build a CLI tool in Rust, Play 3 songs on guitar..."
              rows="3"
            ></textarea>
          </div>

          <button type="submit" class="btn btn-primary btn-lg" :disabled="!topic.trim()">
            Generate Learning Plan
            <ArrowRight :size="18" />
          </button>
        </form>

        <div class="examples">
          <span class="examples-label">Try:</span>
          <button
            v-for="ex in examples"
            :key="ex"
            class="example-chip"
            @click="useExample(ex)"
          >
            {{ ex }}
          </button>
        </div>
      </div>
    </section>

    <section class="principles-preview">
      <div class="container">
        <h2 class="section-title">The 9 Ultralearning Principles</h2>
        <p class="section-subtitle">
          Each plan is structured around these principles from Scott Young's framework.
        </p>
        <div class="principles-grid">
          <div
            v-for="i in 9"
            :key="i"
            class="principle-preview-card"
          >
            <div
              class="principle-dot"
              :style="{ background: `var(--p${i})` }"
            ></div>
            <div>
              <div class="principle-number">P{{ i }}</div>
              <div class="principle-name">{{ getPrincipleName(i) }}</div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script lang="ts">
const names: Record<number, string> = {
  1: 'Metalearning',
  2: 'Focus',
  3: 'Directness',
  4: 'Drill',
  5: 'Retrieval',
  6: 'Feedback',
  7: 'Retention',
  8: 'Intuition',
  9: 'Experimentation',
}

function getPrincipleName(id: number): string {
  return names[id] || ''
}
</script>

<style scoped>
.landing {
  min-height: calc(100vh - 60px);
}

.hero {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-section) 0;
}

.hero-inner {
  max-width: 640px;
  width: 100%;
  padding: 0 var(--space-lg);
  text-align: center;
}

.hero-badge {
  display: inline-flex;
  align-items: center;
  gap: var(--space-xs);
  font-size: 12px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-primary);
  background: var(--color-primary-muted);
  padding: 6px 14px;
  border-radius: var(--rounded-full);
  margin-bottom: var(--space-lg);
}

.hero-title {
  font-size: 48px;
  font-weight: 700;
  line-height: 1.1;
  margin-bottom: var(--space-md);
}

.hero-subtitle {
  font-size: 18px;
  color: var(--color-mute);
  margin-bottom: var(--space-2xl);
}

.hero-form {
  text-align: left;
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
}

.form-label {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-ink);
}

.btn-lg {
  width: 100%;
  padding: 16px 24px;
  font-size: 16px;
}

.btn-lg:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.examples {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: center;
  gap: var(--space-sm);
  margin-top: var(--space-xl);
}

.examples-label {
  font-size: 14px;
  color: var(--color-mute);
}

.example-chip {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-body);
  background: var(--color-surface-raised);
  border: 1px solid var(--color-border);
  border-radius: var(--rounded-full);
  padding: 6px 16px;
  transition: background 0.15s, color 0.15s;
}

.example-chip:hover {
  background: var(--color-primary-muted);
  color: var(--color-primary);
  border-color: var(--color-primary-muted);
}

.principles-preview {
  padding: var(--space-section) 0;
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
}

.section-title {
  text-align: center;
  margin-bottom: var(--space-sm);
}

.section-subtitle {
  text-align: center;
  color: var(--color-mute);
  margin-bottom: var(--space-2xl);
}

.principles-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-md);
}

.principle-preview-card {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-md) var(--space-lg);
  background: var(--color-canvas);
  border: 1px solid var(--color-border);
  border-radius: var(--rounded-md);
}

.principle-dot {
  width: 12px;
  height: 12px;
  border-radius: var(--rounded-full);
  flex-shrink: 0;
}

.principle-number {
  font-size: 12px;
  font-weight: 700;
  color: var(--color-mute);
  text-transform: uppercase;
}

.principle-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-ink);
}

@media (max-width: 1023px) {
  .principles-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 767px) {
  .hero-title {
    font-size: 32px;
  }

  .hero-subtitle {
    font-size: 16px;
  }

  .principles-grid {
    grid-template-columns: 1fr;
  }
}
</style>
