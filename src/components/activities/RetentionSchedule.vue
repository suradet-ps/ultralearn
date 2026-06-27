<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlanStore } from '../../stores/plan'
import { Calendar, Check } from '@lucide/vue'

const props = defineProps<{
  planId: string
  principleId: number
}>()

const store = usePlanStore()

const topic = ref('')

interface ScheduleItem {
  id: string
  topic: string
  dates: string[]
  completedDates: string[]
}

function getSchedules(): ScheduleItem[] {
  const plan = store.getPlan(props.planId)
  const pp = plan?.principles.find((p) => p.principleId === props.principleId)
  return (pp?.activities.schedules as ScheduleItem[]) || []
}

function saveSchedules(items: ScheduleItem[]) {
  store.updateActivity(props.planId, props.principleId, 'schedules', items)
}

const schedules = computed(() => getSchedules())

function generateSchedule() {
  if (!topic.value.trim()) return
  const base = new Date()
  const offsets = [1, 3, 7, 14, 30]
  const dates = offsets.map((days) => {
    const d = new Date(base)
    d.setDate(d.getDate() + days)
    return d.toISOString().split('T')[0]
  })
  const items = getSchedules()
  items.push({
    id: Date.now().toString(36),
    topic: topic.value.trim(),
    dates,
    completedDates: [],
  })
  saveSchedules(items)
  topic.value = ''
}

function toggleDate(scheduleId: string, date: string) {
  const items = getSchedules()
  const s = items.find((i) => i.id === scheduleId)
  if (!s) return
  const idx = s.completedDates.indexOf(date)
  if (idx === -1) {
    s.completedDates.push(date)
  } else {
    s.completedDates.splice(idx, 1)
  }
  saveSchedules(items)
}

function isPast(dateStr: string): boolean {
  return new Date(dateStr) <= new Date()
}

function formatDate(dateStr: string): string {
  return new Date(dateStr).toLocaleDateString(undefined, { month: 'short', day: 'numeric' })
}
</script>

<template>
  <div class="retention-section">
    <h3>Spaced Repetition Schedule</h3>
    <p class="desc">Create review schedules: Day 1, 3, 7, 14, 30</p>

    <div class="add-form">
      <input
        v-model="topic"
        type="text"
        placeholder="Topic or concept to schedule for review"
        @keyup.enter="generateSchedule"
      />
      <button class="btn btn-primary" @click="generateSchedule" :disabled="!topic.trim()">
        <Calendar :size="16" />
        Generate Schedule
      </button>
    </div>

    <div v-if="schedules.length === 0" class="empty">
      No schedules yet. Add topics to track your spaced repetition.
    </div>

    <div v-else class="schedule-list">
      <div v-for="s in schedules" :key="s.id" class="schedule-card">
        <div class="schedule-topic">{{ s.topic }}</div>
        <div class="schedule-dates">
          <button
            v-for="date in s.dates"
            :key="date"
            class="date-btn"
            :class="{
              completed: s.completedDates.includes(date),
              due: isPast(date) && !s.completedDates.includes(date),
            }"
            @click="toggleDate(s.id, date)"
          >
            <Check v-if="s.completedDates.includes(date)" :size="14" />
            {{ formatDate(date) }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.retention-section h3 {
  margin-bottom: var(--space-xs);
}

.desc {
  font-size: 14px;
  color: var(--color-mute);
  margin-bottom: var(--space-md);
}

.add-form {
  display: flex;
  gap: var(--space-sm);
  margin-bottom: var(--space-lg);
}

.add-form input {
  flex: 1;
}

.add-form .btn {
  flex-shrink: 0;
  white-space: nowrap;
}

.empty {
  text-align: center;
  padding: var(--space-xl);
  color: var(--color-mute);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
}

.schedule-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.schedule-card {
  padding: var(--space-md);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
}

.schedule-topic {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-ink);
  margin-bottom: var(--space-md);
}

.schedule-dates {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-sm);
}

.date-btn {
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

.date-btn.due {
  color: var(--color-warning);
  border-color: var(--color-warning);
}

.date-btn.completed {
  color: white;
  background: var(--color-success);
  border-color: var(--color-success);
}
</style>
