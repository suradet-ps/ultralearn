import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Plan, PrincipleProgress, Flashcard, FeedbackEntry, Experiment } from '../types/principle'
import { PRINCIPLES } from '../types/principles-data'

const STORAGE_KEY = 'ultralearn-plans'

function generateId(): string {
  return Date.now().toString(36) + Math.random().toString(36).slice(2, 8)
}

function createDefaultProgress(): PrincipleProgress[] {
  return PRINCIPLES.map((p) => ({
    principleId: p.id,
    notes: '',
    checklist: p.prompts.map((text) => ({
      id: generateId(),
      text,
      checked: false,
    })),
    activities: {},
    completed: false,
  }))
}

function loadPlans(): Plan[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    return raw ? JSON.parse(raw) : []
  } catch {
    return []
  }
}

function savePlans(plans: Plan[]) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(plans))
}

export const usePlanStore = defineStore('plan', () => {
  const plans = ref<Plan[]>(loadPlans())

  function getPlan(id: string): Plan | undefined {
    return plans.value.find((p) => p.id === id)
  }

  function createPlan(topic: string, goal: string): Plan {
    const plan: Plan = {
      id: generateId(),
      topic,
      goal,
      createdAt: new Date().toISOString(),
      principles: createDefaultProgress(),
    }
    plans.value.push(plan)
    savePlans(plans.value)
    return plan
  }

  function deletePlan(id: string) {
    plans.value = plans.value.filter((p) => p.id !== id)
    savePlans(plans.value)
  }

  function updatePrincipleNotes(planId: string, principleId: number, notes: string) {
    const plan = getPlan(planId)
    if (!plan) return
    const pp = plan.principles.find((p) => p.principleId === principleId)
    if (pp) {
      pp.notes = notes
      savePlans(plans.value)
    }
  }

  function toggleChecklistItem(planId: string, principleId: number, itemId: string) {
    const plan = getPlan(planId)
    if (!plan) return
    const pp = plan.principles.find((p) => p.principleId === principleId)
    if (pp) {
      const item = pp.checklist.find((c) => c.id === itemId)
      if (item) {
        item.checked = !item.checked
        savePlans(plans.value)
      }
    }
  }

  function addChecklistItem(planId: string, principleId: number, text: string) {
    const plan = getPlan(planId)
    if (!plan) return
    const pp = plan.principles.find((p) => p.principleId === principleId)
    if (pp) {
      pp.checklist.push({ id: generateId(), text, checked: false })
      savePlans(plans.value)
    }
  }

  function removeChecklistItem(planId: string, principleId: number, itemId: string) {
    const plan = getPlan(planId)
    if (!plan) return
    const pp = plan.principles.find((p) => p.principleId === principleId)
    if (pp) {
      pp.checklist = pp.checklist.filter((c) => c.id !== itemId)
      savePlans(plans.value)
    }
  }

  function togglePrincipleCompleted(planId: string, principleId: number) {
    const plan = getPlan(planId)
    if (!plan) return
    const pp = plan.principles.find((p) => p.principleId === principleId)
    if (pp) {
      pp.completed = !pp.completed
      savePlans(plans.value)
    }
  }

  function updateActivity(planId: string, principleId: number, key: string, value: unknown) {
    const plan = getPlan(planId)
    if (!plan) return
    const pp = plan.principles.find((p) => p.principleId === principleId)
    if (pp) {
      pp.activities[key] = value
      savePlans(plans.value)
    }
  }

  function getProgress(planId: string): number {
    const plan = getPlan(planId)
    if (!plan) return 0
    const completed = plan.principles.filter((p) => p.completed).length
    return Math.round((completed / plan.principles.length) * 100)
  }

  function getPrincipleProgress(planId: string, principleId: number): number {
    const plan = getPlan(planId)
    if (!plan) return 0
    const pp = plan.principles.find((p) => p.principleId === principleId)
    if (!pp) return 0
    if (pp.completed) return 100
    const checked = pp.checklist.filter((c) => c.checked).length
    return Math.round((checked / pp.checklist.length) * 100)
  }

  // Flashcard helpers
  function getFlashcards(planId: string, principleId: number): Flashcard[] {
    const plan = getPlan(planId)
    if (!plan) return []
    const pp = plan.principles.find((p) => p.principleId === principleId)
    return (pp?.activities.flashcards as Flashcard[]) || []
  }

  function addFlashcard(planId: string, principleId: number, front: string, back: string) {
    const cards = getFlashcards(planId, principleId)
    cards.push({ id: generateId(), front, back })
    updateActivity(planId, principleId, 'flashcards', cards)
  }

  function removeFlashcard(planId: string, principleId: number, cardId: string) {
    const cards = getFlashcards(planId, principleId).filter((c) => c.id !== cardId)
    updateActivity(planId, principleId, 'flashcards', cards)
  }

  // Feedback helpers
  function getFeedbackLog(planId: string, principleId: number): FeedbackEntry[] {
    const plan = getPlan(planId)
    if (!plan) return []
    const pp = plan.principles.find((p) => p.principleId === principleId)
    return (pp?.activities.feedbackLog as FeedbackEntry[]) || []
  }

  function addFeedback(planId: string, principleId: number, type: FeedbackEntry['type'], text: string) {
    const log = getFeedbackLog(planId, principleId)
    log.push({ id: generateId(), type, text, date: new Date().toISOString() })
    updateActivity(planId, principleId, 'feedbackLog', log)
  }

  // Experiment helpers
  function getExperiments(planId: string, principleId: number): Experiment[] {
    const plan = getPlan(planId)
    if (!plan) return []
    const pp = plan.principles.find((p) => p.principleId === principleId)
    return (pp?.activities.experiments as Experiment[]) || []
  }

  function addExperiment(planId: string, principleId: number, hypothesis: string, method: string) {
    const exps = getExperiments(planId, principleId)
    exps.push({ id: generateId(), hypothesis, method, result: '', status: 'planned' })
    updateActivity(planId, principleId, 'experiments', exps)
  }

  function updateExperiment(planId: string, principleId: number, expId: string, updates: Partial<Experiment>) {
    const exps = getExperiments(planId, principleId)
    const idx = exps.findIndex((e) => e.id === expId)
    if (idx !== -1) {
      exps[idx] = { ...exps[idx], ...updates }
      updateActivity(planId, principleId, 'experiments', exps)
    }
  }

  // Export / Import
  function exportPlan(planId: string): string {
    const plan = getPlan(planId)
    return plan ? JSON.stringify(plan, null, 2) : ''
  }

  function importPlan(json: string): Plan | null {
    try {
      const plan = JSON.parse(json) as Plan
      if (!plan.id || !plan.topic || !plan.principles) return null
      plan.id = generateId()
      plans.value.push(plan)
      savePlans(plans.value)
      return plan
    } catch {
      return null
    }
  }

  return {
    plans,
    getPlan,
    createPlan,
    deletePlan,
    updatePrincipleNotes,
    toggleChecklistItem,
    addChecklistItem,
    removeChecklistItem,
    togglePrincipleCompleted,
    updateActivity,
    getProgress,
    getPrincipleProgress,
    getFlashcards,
    addFlashcard,
    removeFlashcard,
    getFeedbackLog,
    addFeedback,
    getExperiments,
    addExperiment,
    updateExperiment,
    exportPlan,
    importPlan,
  }
})
