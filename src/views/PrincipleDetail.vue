<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { usePlanStore } from '../stores/plan'
import { PRINCIPLES } from '../types/principles-data'
import { ArrowLeft, CheckCircle2, Circle } from '@lucide/vue'

import ChecklistSection from '../components/ChecklistSection.vue'
import NotesSection from '../components/NotesSection.vue'
import PomodoroTimer from '../components/activities/PomodoroTimer.vue'
import FlashcardSection from '../components/activities/FlashcardSection.vue'
import FeynmanWorkspace from '../components/activities/FeynmanWorkspace.vue'
import FeedbackLog from '../components/activities/FeedbackLog.vue'
import ExperimentTracker from '../components/activities/ExperimentTracker.vue'
import RetentionSchedule from '../components/activities/RetentionSchedule.vue'

const route = useRoute()
const router = useRouter()
const store = usePlanStore()

const planId = route.params.id as string
const principleId = Number(route.params.principleId)

const plan = computed(() => store.getPlan(planId))
const principle = computed(() => PRINCIPLES.find((p) => p.id === principleId))
const progress = computed(() => {
  const pp = plan.value?.principles.find((p) => p.principleId === principleId)
  return pp
})

function goBack() {
  router.push({ name: 'plan', params: { id: planId } })
}

function toggleCompleted() {
  store.togglePrincipleCompleted(planId, principleId)
}
</script>

<template>
  <div class="principle-detail" v-if="plan && principle">
    <div class="container">
      <div class="detail-header">
        <button class="btn-ghost" @click="goBack">
          <ArrowLeft :size="18" />
          Back to Plan
        </button>
      </div>

      <div class="principle-hero" :style="{ borderColor: principle.color }">
        <div class="principle-badge-lg" :style="{ background: principle.color }">
          {{ principle.id }}
        </div>
        <div class="principle-info">
          <div class="principle-tag" :style="{ color: principle.color }">
            Principle {{ principle.id }}
          </div>
          <h1>{{ principle.name }}</h1>
          <p class="principle-tagline">{{ principle.tagline }}</p>
        </div>
        <button
          class="complete-btn"
          :class="{ completed: progress?.completed }"
          @click="toggleCompleted"
        >
          <CheckCircle2 v-if="progress?.completed" :size="20" />
          <Circle v-else :size="20" />
          {{ progress?.completed ? 'Completed' : 'Mark Complete' }}
        </button>
      </div>

      <p class="principle-desc">{{ principle.description }}</p>

      <div class="detail-body">
        <div class="detail-main">
          <!-- Principle-specific activities -->
          <PomodoroTimer v-if="principleId === 2" />
          <FlashcardSection v-if="principleId === 5" :planId="planId" :principleId="principleId" />
          <FeynmanWorkspace v-if="principleId === 8" :planId="planId" :principleId="principleId" />
          <FeedbackLog v-if="principleId === 6" :planId="planId" :principleId="principleId" />
          <ExperimentTracker v-if="principleId === 9" :planId="planId" :principleId="principleId" />
          <RetentionSchedule v-if="principleId === 7" :planId="planId" :principleId="principleId" />

          <ChecklistSection :planId="planId" :principleId="principleId" />
          <NotesSection :planId="planId" :principleId="principleId" />
        </div>
      </div>
    </div>
  </div>

  <div v-else class="not-found">
    <div class="container">
      <p>Principle not found.</p>
      <button class="btn btn-primary" @click="goBack">Go back</button>
    </div>
  </div>
</template>

<style scoped>
.principle-detail {
  padding: var(--space-xl) 0 var(--space-section);
}

.detail-header {
  margin-bottom: var(--space-xl);
}

.principle-hero {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  padding: var(--space-lg);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-left: 4px solid;
  border-radius: var(--rounded-md);
  margin-bottom: var(--space-lg);
}

.principle-badge-lg {
  width: 56px;
  height: 56px;
  flex-shrink: 0;
  border-radius: var(--rounded-full);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  font-weight: 700;
  color: var(--color-on-primary);
}

.principle-info {
  flex: 1;
}

.principle-tag {
  font-size: 12px;
  font-weight: 700;
  text-transform: uppercase;
  margin-bottom: var(--space-xs);
}

.principle-info h1 {
  margin-bottom: var(--space-xs);
}

.principle-tagline {
  font-size: 16px;
  color: var(--color-mute);
}

.complete-btn {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  font-size: 14px;
  font-weight: 600;
  padding: 10px 20px;
  border-radius: var(--rounded-md);
  color: var(--color-mute);
  background: var(--color-surface-raised);
  border: 1px solid var(--color-border);
  transition: all 0.15s;
}

.complete-btn:hover {
  border-color: var(--color-success);
  color: var(--color-success);
}

.complete-btn.completed {
  background: var(--color-success);
  border-color: var(--color-success);
  color: white;
}

.principle-desc {
  font-size: 18px;
  line-height: 1.6;
  color: var(--color-body);
  margin-bottom: var(--space-2xl);
  max-width: 800px;
}

.detail-body {
  max-width: 800px;
}

.detail-main {
  display: flex;
  flex-direction: column;
  gap: var(--space-2xl);
}

.not-found {
  padding: var(--space-section) 0;
  text-align: center;
}

.not-found p {
  margin-bottom: var(--space-lg);
  color: var(--color-mute);
}

@media (max-width: 767px) {
  .principle-hero {
    flex-direction: column;
    align-items: flex-start;
    text-align: left;
  }

  .complete-btn {
    align-self: stretch;
    justify-content: center;
  }
}
</style>
