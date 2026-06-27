export interface Principle {
  id: number
  name: string
  tagline: string
  color: string
  description: string
  prompts: string[]
}

export interface Plan {
  id: string
  topic: string
  goal: string
  createdAt: string
  principles: PrincipleProgress[]
}

export interface PrincipleProgress {
  principleId: number
  notes: string
  checklist: ChecklistItem[]
  activities: Record<string, unknown>
  completed: boolean
}

export interface ChecklistItem {
  id: string
  text: string
  checked: boolean
}

export interface Flashcard {
  id: string
  front: string
  back: string
}

export interface DrillExercise {
  id: string
  description: string
  completed: boolean
}

export interface FeedbackEntry {
  id: string
  type: 'outcome' | 'informational' | 'corrective'
  text: string
  date: string
}

export interface RetentionSchedule {
  id: string
  topic: string
  dates: string[]
  completedDates: string[]
}

export interface Experiment {
  id: string
  hypothesis: string
  method: string
  result: string
  status: 'planned' | 'running' | 'completed'
}
