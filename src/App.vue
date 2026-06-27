<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Sun, Moon, BookOpen } from '@lucide/vue'

const isDark = ref(false)

function toggleTheme() {
  isDark.value = !isDark.value
  document.documentElement.setAttribute('data-theme', isDark.value ? 'dark' : 'light')
  localStorage.setItem('ultralearn-theme', isDark.value ? 'dark' : 'light')
}

onMounted(() => {
  const saved = localStorage.getItem('ultralearn-theme')
  if (saved === 'dark') {
    isDark.value = true
    document.documentElement.setAttribute('data-theme', 'dark')
  }
})
</script>

<template>
  <nav class="nav-bar">
    <div class="nav-inner">
      <router-link to="/" class="nav-logo">
        <BookOpen :size="24" />
        <span>Ultralearn</span>
      </router-link>
      <div class="nav-actions">
        <button class="btn-icon" @click="toggleTheme" aria-label="Toggle theme">
          <Sun v-if="isDark" :size="18" />
          <Moon v-else :size="18" />
        </button>
      </div>
    </div>
  </nav>
  <main class="main-content">
    <router-view v-slot="{ Component }">
      <transition name="fade" mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
  </main>
</template>

<style scoped>
.nav-bar {
  position: sticky;
  top: 0;
  z-index: 100;
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  height: 60px;
}

.nav-inner {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 var(--space-lg);
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.nav-logo {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  font-size: 18px;
  font-weight: 700;
  color: var(--color-ink);
  text-decoration: none;
}

.nav-logo:hover {
  text-decoration: none;
}

.nav-actions {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
}

.main-content {
  min-height: calc(100vh - 60px);
}
</style>
