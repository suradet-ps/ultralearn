<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue'
import { Play, Pause, RotateCcw } from '@lucide/vue'

const DURATIONS = { work: 25 * 60, shortBreak: 5 * 60, longBreak: 15 * 60 }
const LABELS = { work: 'Focus', shortBreak: 'Short Break', longBreak: 'Long Break' }

const mode = ref<'work' | 'shortBreak' | 'longBreak'>('work')
const seconds = ref(DURATIONS.work)
const isRunning = ref(false)
const sessions = ref(0)
let interval: ReturnType<typeof setInterval> | null = null

const display = computed(() => {
  const m = Math.floor(seconds.value / 60)
  const s = seconds.value % 60
  return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`
})

const progress = computed(() => {
  const total = DURATIONS[mode.value]
  return ((total - seconds.value) / total) * 100
})

function start() {
  if (isRunning.value) return
  isRunning.value = true
  interval = setInterval(() => {
    if (seconds.value > 0) {
      seconds.value--
    } else {
      stop()
      if (mode.value === 'work') {
        sessions.value++
        if (sessions.value % 4 === 0) {
          switchMode('longBreak')
        } else {
          switchMode('shortBreak')
        }
      } else {
        switchMode('work')
      }
    }
  }, 1000)
}

function stop() {
  isRunning.value = false
  if (interval) {
    clearInterval(interval)
    interval = null
  }
}

function reset() {
  stop()
  seconds.value = DURATIONS[mode.value]
}

function switchMode(newMode: 'work' | 'shortBreak' | 'longBreak') {
  stop()
  mode.value = newMode
  seconds.value = DURATIONS[newMode]
}

onUnmounted(() => {
  if (interval) clearInterval(interval)
})
</script>

<template>
  <div class="pomodoro">
    <h3>Pomodoro Timer</h3>
    <div class="timer-modes">
      <button
        v-for="(label, key) in LABELS"
        :key="key"
        class="mode-btn"
        :class="{ active: mode === key }"
        @click="switchMode(key as typeof mode)"
      >
        {{ label }}
      </button>
    </div>
    <div class="timer-display">{{ display }}</div>
    <div class="timer-progress">
      <div class="timer-progress-fill" :style="{ width: progress + '%' }"></div>
    </div>
    <div class="timer-controls">
      <button class="btn btn-primary" @click="isRunning ? stop() : start()">
        <Pause v-if="isRunning" :size="18" />
        <Play v-else :size="18" />
        {{ isRunning ? 'Pause' : 'Start' }}
      </button>
      <button class="btn btn-ghost" @click="reset">
        <RotateCcw :size="18" />
        Reset
      </button>
    </div>
    <div class="sessions">Sessions completed: {{ sessions }}</div>
  </div>
</template>

<style scoped>
.pomodoro {
  text-align: center;
  padding: var(--space-lg);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
}

.pomodoro h3 {
  margin-bottom: var(--space-lg);
}

.timer-modes {
  display: flex;
  justify-content: center;
  gap: var(--space-xs);
  margin-bottom: var(--space-lg);
}

.mode-btn {
  font-size: 13px;
  font-weight: 600;
  padding: 8px 16px;
  border-radius: var(--rounded-full);
  color: var(--color-mute);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  transition: all 0.15s;
}

.mode-btn.active {
  color: var(--color-on-primary);
  background: var(--color-primary);
  border-color: var(--color-primary);
}

.timer-display {
  font-size: 72px;
  font-weight: 700;
  font-variant-numeric: tabular-nums;
  letter-spacing: -2px;
  color: var(--color-ink);
  margin-bottom: var(--space-md);
}

.timer-progress {
  height: 4px;
  border-radius: var(--rounded-full);
  background: var(--color-border);
  margin-bottom: var(--space-lg);
  overflow: hidden;
}

.timer-progress-fill {
  height: 100%;
  background: var(--color-primary);
  border-radius: var(--rounded-full);
  transition: width 1s linear;
}

.timer-controls {
  display: flex;
  justify-content: center;
  gap: var(--space-sm);
  margin-bottom: var(--space-md);
}

.sessions {
  font-size: 14px;
  color: var(--color-mute);
}
</style>
