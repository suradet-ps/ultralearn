<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { usePlanStore } from '../stores/plan'
import { PRINCIPLES } from '../types/principles-data'
import { ArrowLeft, Download, Trash2, CheckCircle2, Circle } from '@lucide/vue'

const route = useRoute()
const router = useRouter()
const store = usePlanStore()

const planId = route.params.id as string
const plan = computed(() => store.getPlan(planId))
const progress = computed(() => store.getProgress(planId))

function goBack() {
  router.push({ name: 'landing' })
}

function goToPrinciple(principleId: number) {
  router.push({ name: 'principle', params: { id: planId, principleId } })
}

function handleDelete() {
  if (confirm('Delete this plan? This cannot be undone.')) {
    store.deletePlan(planId)
    router.push({ name: 'landing' })
  }
}

function handleExport() {
  const json = store.exportPlan(planId)
  const blob = new Blob([json], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `ultralearn-${plan.value?.topic.toLowerCase().replace(/\s+/g, '-')}.json`
  a.click()
  URL.revokeObjectURL(url)
}
</script>

<template>
  <div class="plan-overview" v-if="plan">
    <div class="container">
      <div class="plan-header">
        <button class="btn-ghost" @click="goBack">
          <ArrowLeft :size="18" />
          Back
        </button>
        <div class="plan-actions">
          <button class="btn-icon" @click="handleExport" aria-label="Export plan">
            <Download :size="18" />
          </button>
          <button class="btn-icon" @click="handleDelete" aria-label="Delete plan">
            <Trash2 :size="18" />
          </button>
        </div>
      </div>

      <div class="plan-title-section">
        <h1>{{ plan.topic }}</h1>
        <p v-if="plan.goal" class="plan-goal">{{ plan.goal }}</p>
        <div class="plan-progress">
          <div class="progress-bar">
            <div class="progress-bar-fill" :style="{ width: progress + '%' }"></div>
          </div>
          <span class="progress-text">{{ progress }}% complete</span>
        </div>
      </div>

      <div class="principles-grid">
        <button
          v-for="p in PRINCIPLES"
          :key="p.id"
          class="principle-card"
          @click="goToPrinciple(p.id)"
        >
          <div class="principle-accent" :style="{ background: p.color }"></div>
          <div class="principle-card-body">
            <div class="principle-card-top">
              <div class="principle-badge" :style="{ background: p.color }">
                {{ p.id }}
              </div>
              <component
                :is="plan.principles[p.id - 1]?.completed ? CheckCircle2 : Circle"
                :size="20"
                :style="{ color: plan.principles[p.id - 1]?.completed ? 'var(--color-success)' : 'var(--color-mute)' }"
              />
            </div>
            <h3 class="principle-card-title">{{ p.name }}</h3>
            <p class="principle-card-tagline">{{ p.tagline }}</p>
            <div class="principle-card-progress">
              <div class="progress-bar">
                <div
                  class="progress-bar-fill"
                  :style="{
                    width: store.getPrincipleProgress(planId, p.id) + '%',
                    background: p.color,
                  }"
                ></div>
              </div>
            </div>
          </div>
        </button>
      </div>
    </div>
  </div>

  <div v-else class="not-found">
    <div class="container">
      <p>Plan not found.</p>
      <button class="btn btn-primary" @click="goBack">Go back</button>
    </div>
  </div>
</template>

<style scoped>
.plan-overview {
  padding: var(--space-xl) 0 var(--space-section);
}

.plan-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-xl);
}

.plan-actions {
  display: flex;
  gap: var(--space-sm);
}

.plan-title-section {
  margin-bottom: var(--space-2xl);
}

.plan-title-section h1 {
  margin-bottom: var(--space-sm);
}

.plan-goal {
  color: var(--color-mute);
  font-size: 18px;
  margin-bottom: var(--space-lg);
}

.plan-progress {
  display: flex;
  align-items: center;
  gap: var(--space-md);
}

.plan-progress .progress-bar {
  flex: 1;
  max-width: 400px;
}

.progress-text {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-mute);
}

.principles-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-md);
}

.principle-card {
  text-align: left;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--rounded-md);
  overflow: hidden;
  cursor: pointer;
  transition: box-shadow 0.2s, transform 0.2s;
}

.principle-card:hover {
  box-shadow: var(--elevation-2);
  transform: translateY(-2px);
}

.principle-accent {
  height: 4px;
}

.principle-card-body {
  padding: var(--space-lg);
}

.principle-card-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-md);
}

.principle-badge {
  width: 32px;
  height: 32px;
  border-radius: var(--rounded-full);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 700;
  color: var(--color-on-primary);
}

.principle-card-title {
  font-size: 20px;
  font-weight: 700;
  margin-bottom: var(--space-xs);
}

.principle-card-tagline {
  font-size: 14px;
  color: var(--color-mute);
  margin-bottom: var(--space-md);
}

.principle-card-progress {
  margin-top: auto;
}

.not-found {
  padding: var(--space-section) 0;
  text-align: center;
}

.not-found p {
  margin-bottom: var(--space-lg);
  color: var(--color-mute);
}

@media (max-width: 1023px) {
  .principles-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 767px) {
  .principles-grid {
    grid-template-columns: 1fr;
  }
}
</style>
