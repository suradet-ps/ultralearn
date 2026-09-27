# Ultralearn

[![CI](https://github.com/suradet-ps/ultralearn/actions/workflows/ci.yml/badge.svg)](https://github.com/suradet-ps/ultralearn/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust: stable](https://img.shields.io/badge/rust-stable-orange.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Leptos v0.8](https://img.shields.io/badge/Leptos-v0.8-blue.svg)](https://leptos.dev)
[![PWA](https://img.shields.io/badge/PWA-installable-5A0FC8.svg?logo=pwa&logoColor=white)](https://web.dev/progressive-web-apps/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/suradet-ps/ultralearn/issues)

---

## ◆ PULSE

"Learn Rust", "play guitar", "Japanese" - a topic is not a plan, and a
plan is where ultralearning begins. Ultralearn scaffolds any topic into
a guided structure across Scott Young's nine principles -
metalearning, focus, directness, drill, retrieval, feedback, retention,
intuition, experimentation - each with its own checklists, notes,
flashcards, feedback logs, experiments, and retention schedule. All
client-side, all local, no account, no backend: the plan is yours, the
data is yours, and the compounding is yours to watch.

| P0 ▣ | P1 ▢ | P2-P9 ☐ |
|---|---|---|

*The foundation and nearly all of the core experience are sealed; real
spaced repetition (FSRS), adaptive plans, sync, and the v1.0 gate
stand open.*

> Built with Rust 2024 + Leptos 0.8, kept in `localStorage`, exported
> as JSON - a learning OS with no landlord.
>
> **suradet-ps**, artifact keeper

---

## ◆ IGNITION

One target, one tool, one command.

```
⟫ rustup target add wasm32-unknown-unknown
⟫ cargo install --locked trunk
⟫ trunk serve
```

Open [http://127.0.0.1:3000](http://127.0.0.1:3000).

The release artifact: `⟫ trunk build --release` - static output in
`dist/`, served by any static host.

<details>
<summary>Prerequisites</summary>

- [Rust](https://www.rust-lang.org/tools/install) (stable, 1.97+)
  with the `wasm32-unknown-unknown` target
- [Trunk](https://trunkrs.dev/#install) - installed above

</details>

---

## ◆ ANATOMY

One store, nine principles, a set of honest activities.

- **Generates** - a topic plus an optional goal scaffolds all nine
  principles at once; every principle arrives with its prompts and its
  own working space.
- **Tracks** - per-principle checklists, free-form notes, and the
  completion percentages - the overview shows where the plan stands,
  the principle page shows why.
- **Practices** - flashcards for retrieval, a feedback log that names
  the kind of feedback (outcome, informational, corrective), an
  experiment tracker for hypotheses and results, a Feynman workspace,
  and a Pomodoro timer with a focus-streak indicator.
- **Schedules** - retention reminders at 1/3/7/14/30-day intervals;
  the honest fixed calendar of today, with FSRS waiting in Phase 2.
- **Remembers** - everything persists under `ultralearn-plans` in
  `localStorage`; export and import carry the plan between machines as
  JSON. Clearing the browser clears the plans - export is the backup
  ritual.
- **Wears** - dark or light, persisted; installable offline through
  the web app manifest and service worker.

---

## ◆ RITUALS

**The core ceremony** - the new plan:

1. Enter the topic - "Learn Rust" - and an optional goal.
2. The generator scaffolds all nine principles; the plan is born
   complete, not blank.
3. Work the principle pages: checklists, notes, flashcards, feedback,
   experiments - the day's practice lands where it belongs.
4. Review the retention schedule and watch the percentages move; the
   plan compounds because the work was structured.

**The ceremony of the local page** - no backend, no account, no
telemetry. The plan lives in the browser and leaves it only when the
JSON export says so.

**The ceremony of the keyboard** - digits 1-9 jump to a principle, `c`
toggles complete, `b` or `Esc` goes back - and the shortcuts stand
down while you type. The page stays out of the way of the practice.

---

## ◆ ECHOES

**Where this artifact is heading**

```
P0 ▸ foundation: scaffold, storage, nine principles, CI ────────────── ▸ sealed
P1 ▸ core experience: edit, tags, markdown, a11y, offline ──────────── ▸ forging
P2 ▸ real spaced repetition (FSRS) ──────────────────────────────────── ▸ open
P3 ▸ adaptive plans - the app learns with you ───────────────────────── ▸ open
P4-P8 ▸ interop, sync, commons, hardening, i18n ─────────────────────── ▸ open
P9 ▸ v1.0.0 stable release ──────────────────────────────────────────── ▸ open
```

**Raising the artifact** - the honest path lives in `ROADMAP.md`.
Gates before any PR: `cargo fmt --all --check`, `cargo clippy
--target wasm32-unknown-unknown`, `cargo test`, and the Trunk
production build. Open an issue first to discuss a change.

**Status** - CI runs the full gate on every push and PR.
[Watch the gates](.github/workflows).

---

```
  ─────────────────────────────────────────
   A topic is a wish.
   A plan is the first day of the work.
  ─────────────────────────────────────────
```

Licensed under the [MIT License](LICENSE).