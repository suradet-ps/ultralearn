<div align="center">

# Ultralearn

**Build structured, self-directed learning plans grounded in the 9 Ultralearning principles.**

[![CI](https://github.com/sursdet-ps/ultralearn/actions/workflows/ci.yml/badge.svg)](https://github.com/sursdet-ps/ultralearn/actions/workflows/ci.yml)
[![Vue 3](https://img.shields.io/badge/Vue-3-42b883?logo=vue.js&logoColor=white)](https://vuejs.org/)
[![Vite](https://img.shields.io/badge/Vite-8-646cff?logo=vite&logoColor=white)](https://vite.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-6-3178c6?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Bun](https://img.shields.io/badge/Bun-1.3-000000?logo=bun&logoColor=white)](https://bun.sh/)
[![License](https://img.shields.io/badge/license-MIT-4b32c3)](LICENSE)

</div>

---

> Turn any topic — *"Learn Rust"*, *"Play guitar"*, *"Japanese"* — into a guided plan organized around the nine ultralearning principles from [Scott Young](https://www.scotthyoung.com/blog/the-ultralearning-book/).

Ultralearn is a client-side web app that turns any topic — *"Learn Rust"*, *"Play guitar"*, *"Japanese"* — into a guided plan organized around the nine ultralearning principles. Track notes, checklists, flashcards, feedback logs, and experiments per principle, and watch your progress compound.

Everything runs in the browser. There is **no backend and no account** — your plans are stored locally in `localStorage` and can be exported or imported as JSON.

---

## Features

| | |
|---|---|
| 🧭 **Plan generator** | Enter a topic and an optional goal to scaffold a plan across all 9 principles. |
| 📚 **9 principles** | Metalearning, Focus, Directness, Drill, Retrieval, Feedback, Retention, Intuition, Experimentation. |
| ✅ **Checklists & notes** | Per-principle prompts, custom checklists, and free-form notes. |
| 🃏 **Flashcards** | Create retrieval practice cards for any principle. |
| 📝 **Feedback log** | Capture outcome / informational / corrective feedback. |
| 🧪 **Experiments** | Track learning hypotheses, methods, and results. |
| 📊 **Progress tracking** | Per-principle and overall completion percentages. |
| 🌗 **Dark mode** | Theme toggle persisted to `localStorage`. |
| 💾 **Export / Import** | Backup or share plans as JSON. |
| ⚡ **Zero-config** | No server, no database, no build step required to run locally. |

---

## Tech Stack

- **[Vue 3](https://vuejs.org/)** — Composition API with `<script setup>` SFCs
- **[Vite](https://vite.dev/)** — build tooling and dev server
- **[TypeScript](https://www.typescriptlang.org/)** — type-safe source (`vue-tsc` for checking)
- **[Pinia](https://pinia.vuejs.org/)** — state management
- **[Vue Router](https://router.vuejs.org/)** — client-side routing
- **[Lucide](https://lucide.dev/)** — icon set
- **[Bun](https://bun.sh/)** — package manager & runtime

---

## Getting Started

### Prerequisites

- [Bun](https://bun.sh/) (v1.3+)

### Installation

```bash
bun install
```

### Development

Start the Vite dev server with hot module replacement:

```bash
bun run dev
```

The app is served at `http://localhost:5173` by default.

### Production Build

Type-check and build to `dist/`:

```bash
bun run build
```

### Preview the Build

```bash
bun run preview
```

---

## Project Structure

```text
ultralearn/
├── .github/workflows/ci.yml   # CI: type-check & build
├── public/                    # Static assets (favicon, icons)
├── src/
│   ├── components/            # Reusable UI & activity components
│   │   └── activities/        # Pomodoro, flashcards, feedback, etc.
│   ├── router/                # Vue Router configuration
│   ├── stores/                # Pinia stores (plan state)
│   ├── types/                 # TypeScript models & principle data
│   ├── views/                 # Route-level pages
│   ├── App.vue                # Root component (nav + theme)
│   ├── main.ts                # App bootstrap
│   └── style.css              # Global styles & design tokens
├── index.html
├── vite.config.ts
└── tsconfig*.json
```

---

## Data & Privacy

Ultralearn is fully client-side. All plans, notes, and progress live in your browser's `localStorage` under the key `ultralearn-plans` (and theme preference under `ultralearn-theme`). Clearing your browser data removes your plans — use **Export** in a plan to back up as JSON.

---

## Continuous Integration

GitHub Actions runs on every push and pull request to `main`. The workflow installs dependencies with `bun install --frozen-lockfile` and runs `bun run build` (type-check + production build). See [`.github/workflows/ci.yml`](.github/workflows/ci.yml).

---

## Scripts

| Script | Description |
| --- | --- |
| `bun run dev` | Start the development server. |
| `bun run build` | Type-check (`vue-tsc`) and build for production. |
| `bun run preview` | Preview the production build locally. |

---

## License

This project is provided as-is for personal learning use.
