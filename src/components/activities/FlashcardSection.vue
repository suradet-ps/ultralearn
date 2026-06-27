<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlanStore } from '../../stores/plan'
import { Plus, X, ChevronLeft, ChevronRight, RotateCcw } from '@lucide/vue'

const props = defineProps<{
  planId: string
  principleId: number
}>()

const store = usePlanStore()

const front = ref('')
const back = ref('')
const showForm = ref(false)
const currentIndex = ref(0)
const flipped = ref(false)

const cards = computed(() => store.getFlashcards(props.planId, props.principleId))

function addCard() {
  if (!front.value.trim() || !back.value.trim()) return
  store.addFlashcard(props.planId, props.principleId, front.value.trim(), back.value.trim())
  front.value = ''
  back.value = ''
  showForm.value = false
}

function removeCard(id: string) {
  store.removeFlashcard(props.planId, props.principleId, id)
  if (currentIndex.value >= cards.value.length) {
    currentIndex.value = Math.max(0, cards.value.length - 1)
  }
}

function nextCard() {
  flipped.value = false
  currentIndex.value = (currentIndex.value + 1) % cards.value.length
}

function prevCard() {
  flipped.value = false
  currentIndex.value = (currentIndex.value - 1 + cards.value.length) % cards.value.length
}

function flipCard() {
  flipped.value = !flipped.value
}
</script>

<template>
  <div class="flashcard-section">
    <div class="section-header">
      <h3>Flashcards</h3>
      <button class="btn btn-secondary" @click="showForm = !showForm">
        <Plus :size="16" />
        Add Card
      </button>
    </div>

    <div v-if="showForm" class="add-form">
      <input v-model="front" type="text" placeholder="Front (question)" />
      <input v-model="back" type="text" placeholder="Back (answer)" @keyup.enter="addCard" />
      <div class="form-actions">
        <button class="btn btn-primary" @click="addCard" :disabled="!front.trim() || !back.trim()">
          Add
        </button>
        <button class="btn btn-ghost" @click="showForm = false">Cancel</button>
      </div>
    </div>

    <div v-if="cards.length === 0" class="empty">
      No flashcards yet. Add some to practise retrieval.
    </div>

    <div v-else class="card-view">
      <div class="card-counter">{{ currentIndex + 1 }} / {{ cards.length }}</div>
      <div class="flashcard" :class="{ flipped }" @click="flipCard">
        <div class="flashcard-face front">
          <div class="face-label">Question</div>
          <div class="face-content">{{ cards[currentIndex]?.front }}</div>
          <div class="face-hint">Click to flip</div>
        </div>
        <div class="flashcard-face back">
          <div class="face-label">Answer</div>
          <div class="face-content">{{ cards[currentIndex]?.back }}</div>
          <div class="face-hint">Click to flip</div>
        </div>
      </div>
      <div class="card-controls">
        <button class="btn btn-ghost" @click="prevCard" :disabled="cards.length <= 1">
          <ChevronLeft :size="18" />
        </button>
        <button class="btn btn-ghost" @click="flipCard">
          <RotateCcw :size="18" />
        </button>
        <button class="btn btn-ghost" @click="nextCard" :disabled="cards.length <= 1">
          <ChevronRight :size="18" />
        </button>
      </div>
      <div class="card-list">
        <div v-for="card in cards" :key="card.id" class="card-list-item">
          <span class="card-list-front">{{ card.front }}</span>
          <button class="remove-btn" @click="removeCard(card.id)" aria-label="Remove card">
            <X :size="14" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.flashcard-section {
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
  gap: var(--space-sm);
  padding: var(--space-md);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
  margin-bottom: var(--space-md);
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

.card-view {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.card-counter {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-mute);
  margin-bottom: var(--space-md);
}

.flashcard {
  width: 100%;
  max-width: 400px;
  height: 240px;
  perspective: 1000px;
  cursor: pointer;
  margin-bottom: var(--space-md);
}

.flashcard-face {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--space-lg);
  background: var(--color-surface);
  border: 2px solid var(--color-border);
  border-radius: var(--rounded-lg);
  backface-visibility: hidden;
  position: absolute;
  top: 0;
  left: 0;
  transition: transform 0.4s ease;
}

.flashcard {
  position: relative;
}

.flashcard .front {
  transform: rotateY(0);
}

.flashcard .back {
  transform: rotateY(180deg);
}

.flashcard.flipped .front {
  transform: rotateY(-180deg);
}

.flashcard.flipped .back {
  transform: rotateY(0);
}

.face-label {
  font-size: 12px;
  font-weight: 700;
  text-transform: uppercase;
  color: var(--color-mute);
  margin-bottom: var(--space-sm);
}

.face-content {
  font-size: 18px;
  font-weight: 600;
  text-align: center;
  color: var(--color-ink);
}

.face-hint {
  font-size: 12px;
  color: var(--color-placeholder);
  margin-top: var(--space-md);
}

.card-controls {
  display: flex;
  gap: var(--space-sm);
  margin-bottom: var(--space-lg);
}

.card-list {
  width: 100%;
  max-width: 400px;
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
}

.card-list-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-sm) var(--space-md);
  background: var(--color-surface-raised);
  border-radius: var(--rounded-md);
}

.card-list-front {
  font-size: 14px;
  color: var(--color-body);
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
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

.card-list-item:hover .remove-btn {
  opacity: 1;
}

.remove-btn:hover {
  color: var(--color-error);
}
</style>
