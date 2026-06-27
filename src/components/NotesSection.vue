<script setup lang="ts">
import { computed } from 'vue'
import { usePlanStore } from '../stores/plan'

const props = defineProps<{
  planId: string
  principleId: number
}>()

const store = usePlanStore()

const notes = computed({
  get() {
    const plan = store.getPlan(props.planId)
    const pp = plan?.principles.find((p) => p.principleId === props.principleId)
    return pp?.notes || ''
  },
  set(value: string) {
    store.updatePrincipleNotes(props.planId, props.principleId, value)
  },
})
</script>

<template>
  <div class="notes-section">
    <h3>Notes</h3>
    <textarea
      v-model="notes"
      placeholder="Write your notes, reflections, and plans here..."
      rows="6"
    ></textarea>
  </div>
</template>

<style scoped>
.notes-section h3 {
  margin-bottom: var(--space-md);
}
</style>
