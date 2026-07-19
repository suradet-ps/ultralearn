# Ultralearn Roadmap

This roadmap tracks the journey from the current WASM client-side scaffold to a
fully adaptive, local-first **learning operating system**, and beyond. It
follows the architecture, principles, and data model described in
[README.md](README.md). Nothing here is set in stone — the project is built in
the open, and every phase is up for discussion in issues and pull requests.

> **North Star:** a private-by-default, offline-capable study companion that
> does not just *store* your learning plan but *coaches* you through it — and
> lets learners share hard-won study plans and techniques with each other
> without ever surrendering that data to a server.

## Status & Principles

Ultralearn is a **client-side WASM app** (Rust + Leptos, bundled by Trunk).
There is no backend and no account. All plans live in the browser's
`localStorage` and can be exported or imported as JSON. This shapes every
decision below:

- **Local-first, always.** The device is the source of truth. Network is optional.
- **Private by default.** No telemetry, no tracking, no account.
- **Open format.** Plans are portable JSON. You own your data and can leave.
- **Standards over lock-in.** Where the ecosystem has a good standard (FSRS for
  scheduling, CRDTs for sync), we adopt it rather than inventing our own.

---

## Phase 0: Foundation (done)

- [x] Rust + Leptos 0.8 WASM app, bundled with Trunk, served as a static SPA
- [x] `localStorage` persistence (`gloo-storage`) with export / import to JSON
- [x] Landing → Plan Overview → Principle Detail routing (`leptos_router`)
- [x] The 9 Ultralearning principles as first-class data, with prompts per principle
- [x] Plan generator: topic + optional goal scaffolds all 9 principles
- [x] Per-principle activities: checklist, notes, flashcards, feedback log,
      experiment tracker, Pomodoro timer, Feynman workspace, retention schedule
- [x] Dark / light theme persisted to `localStorage`
- [x] CI: `cargo fmt`, `cargo clippy -D warnings`, `cargo check`, `cargo test`,
      Trunk production build; deployed to Vercel as a static SPA
- [x] MIT-licensed, with a real README, lint discipline, and no `unsafe` on the
      hot path

---

## Phase 1: Solidify the Core Experience (next)

Close the gaps between "works" and "delightful to use every day."

- [x] Plan renaming and editing from the overview screen (not just create/delete)
- [x] Per-plan tags and a searchable plan list on the landing page
- [x] Duplicate / template a plan ("start a new Rust plan from my old one")
- [x] Markdown notes with a safe in-WASM renderer (`pulldown-cmark`, HTML
      escaped by default so notes can't inject scripts) — mirrors the
      "rich-text / markdown with sanitizer" goal without the `ammonia` weight
- [ ] Flashcard improvements: images, LaTeX/MathML, and an SM-2-style review loop
      driven by the existing `RetentionSchedule` *(deferred to Phase 2 / FSRS)*
- [x] Pomodoro session history and a simple "focus this week" streak indicator
- [x] Keyboard shortcuts across the app (digits 1-9 jump to a principle on
      the overview, `c` toggles complete / `b` or `Esc` goes back on a
      principle, shortcuts stand down while typing)
- [x] Accessibility pass: semantic HTML, ARIA labels, focus-management
      friendly markup, and `prefers-reduced-motion` to disable animations
- [x] Offline installability: a web app manifest + service worker so Ultralearn
      opens and works with no network like a native app

---

## Phase 2: Real Spaced Repetition (FSRS)

The current retention scheduler is a fixed 1/3/7/14/30-day calendar. The
spaced-repetition research community has moved well past that — **FSRS**
(Free Spaced Repetition Scheduler) is the open, validated standard used by
Anki, SiYuan, and many others, with a growing Rust ecosystem
(`open-spaced-repetition/rs-fsrs`).

- [ ] Replace the fixed retention calendar with an FSRS scheduler (`rs-fsrs`)
      running entirely in-WASM — no server, no account
- [ ] Per-flashcard review: rate recall (Again / Hard / Good / Easy); intervals
      and difficulty adapt to the learner, not a fixed table
- [ ] A daily "Due reviews" queue across all principles of a plan
- [ ] Optional FSRS parameter optimizer run locally over the learner's own history
- [ ] Import / export decks in a portable format (`.csv` and Anki-compatible where
      feasible) so knowledge is never trapped
- [ ] Respect `prefers-reduced-motion` and offer a low-stimulation review mode

---

## Phase 3: Adaptive Plans (the app learns with you)

Today every plan looks the same: 9 principles, same prompts. A real ultralearning
coach adapts. This phase makes plans respond to what the learner actually does.

- [ ] Progress analytics: completion, review retention, focus time, and
      experiment outcomes visualized per plan and per principle
- [ ] "Rate-limiting factor" detector: surface the principle with the lowest
      sub-skill completion and suggest drilling it (mirrors the **Drill**
      principle directly)
- [ ] Adaptive prompts: checklists and suggestions reorder by what the learner
      tends to skip or fail
- [ ] Streak / momentum signals that nudge without nagging (configurable, off by default)
- [ ] Self-reflection prompts tied to the **Feedback** and **Intuition**
      principles, scheduled by the learner's own rhythm
- [ ] Exportable study report (JSON + printable view) for attaching to a portfolio

---

## Phase 4: Interop & Extensibility

Ultralearn should not be an island. The plan JSON is already portable; this
phase makes the *content* portable too.

- [ ] Public, versioned **plan schema** documented in `docs/PLAN_SCHEMA.md`
      with forward/backward compatibility rules (additive fields only)
- [ ] Import from / export to common formats where mapping is lossless
      (e.g. Obsidian markdown notes, Anki `.csv` decks)
- [ ] A tiny, stable URL / deep-link scheme so a plan (or one principle) can be
      shared as a read-only link that imports on open — still fully client-side
- [ ] Optional local "skill graph": link related plans ("Japanese" ↔ "Kanji" ↔
      "Memory techniques") so principles and flashcards can be reused
- [ ] Plugin hook (documented, sandboxed) for custom activity types beyond the
      built-in nine — still rendered client-side, no native build required

---

## Phase 5: Local-First Sync & Sharing (the leap)

The biggest unmet need in study tools is **sharing without surveillance**.
Anki locks sync behind AnkiWeb; Quizlet locks content behind a subscription.
Ultralearn can do better: keep the data on-device and sync peer-to-peer.

This phase adopts the **local-first** stack the OSS community has converged on
(CRDTs + P2P transport), which fits Ultralearn's no-backend design exactly.

- [ ] Model plans as **CRDT documents** so two devices can edit offline and merge
      deterministically (no last-writer-wins, no lost edits)
- [ ] Local transport first: sync across the same machine / LAN via
      mDNS + WebRTC data channels, browser-to-browser, no server
- [ ] Optional relay fallback (user-operated or a public community relay) when
      direct P2P is blocked by a firewall — relay never sees plaintext
- [ ] End-to-end encryption for synced plans (X25519 + a user passphrase; the
      key never leaves the device)
- [ ] "Share a plan" as a signed, encrypted bundle a learner can send over any
      channel (USB, email, chat) and import into their own local-first store
- [ ] Conflict surfacing in the UI: when a merge reconciles two edits, show what
      changed rather than hiding it

---

## Phase 6: A Learning Commons (opt-in, decentralized)

Once plans can sync and be shared, the natural next step is a *commons*:
learners contributing study plans and technique libraries for any topic.

- [ ] A signed, content-addressed plan format (hash-addressed, like IPFS
      blocks) so a shared plan is verifiable and tamper-evident
- [ ] Optional publish to a self-hosted or community **plan registry** — a
      plain static host or a small relay, never a proprietary silo
- [ ] Discoverability: browse/copy public plans by topic without an account;
      copying forks into your own local-first store (you own the copy)
- [ ] Provenance & attribution: every shared plan carries its author signature
      and a "derived from" chain, all client-side verifiable
- [ ] Quality signals from the community (stars, "helped me") stored as CRDT
      counters — aggregated locally, never sold

---

## Phase 7: Performance & Platform Hardening

Keep the WASM bundle small, fast, and reachable on modest hardware.

- [ ] Bundle-size budget as a CI gate (fail the build if `dist/` exceeds a budget)
- [ ] Cold-start and interaction-latency benchmarks in CI (criterion-style, in-WASM)
- [ ] Full `unsafe` audit: every block justified, isolated, and tested; zero otherwise
- [ ] Reproducible builds (`SOURCE_DATE_EPOCH`) so a given commit hashes identically
- [ ] `cargo-audit` + `cargo-deny` stay green as dependencies grow
- [ ] Desktop builds via Tauri (same Rust core, native window, offline installer)
- [ ] ARM / low-power targets for Raspberry-Pi-class study kiosks

---

## Phase 8: Internationalization & Accessibility at Scale

Ultralearning is for everyone; the interface should be too.

- [ ] i18n framework with the 9 principles and UI strings externalized
- [ ] Community translation docs + a contribution guide for translators
- [ ] Right-to-left layout support (Arabic, Hebrew, Dzongkha)
- [ ] High-contrast and dyslexia-friendly themes
- [ ] Screen-reader tour of the learning loop, not just static labels

---

## Phase 9: First Stable Release (v1.0.0)

- [ ] Frozen, documented plan schema with a migration path from current files
- [ ] FSRS scheduling and adaptive prompts shipped and battle-tested
- [ ] Local-first sync + E2E encryption stable for same-machine and LAN use
- [ ] `v1.0.0` tag, release notes, and prebuilt static + desktop artifacts
- [ ] A short, honest "how it works / what it never does" privacy section front
      and center in the README

---

## Future / Ecosystem (dreaming out loud)

- [ ] **On-device AI coach** (optional, opt-in, fully local via WebGPU/llama.cpp
      in-WASM): suggests drills from your notes, generates retrieval questions
      from your Feynman explanations, never sends your data anywhere
- [ ] **Study-group mode**: a small P2P circle of learners sharing one plan,
      each with private notes but shared checklists and decks
- [ ] **Spaced everything**: apply FSRS not just to flashcards but to experiments,
      feedback reflections, and intuition explanations
- [ ] **Wearable / CLI companions**: review due cards from a terminal or a tiny
      companion app, same CRDT store, same encryption
- [ ] **Research mode**: with explicit consent, export anonymized, aggregated
      learning-pattern stats (no content) to advance ultralearning science —
      opt-in, revocable, zero-knowledge by design

---

## How to Help

Ultralearn is built in the open. Pick any unchecked box above, open an issue to
align, and send a PR. Phases 1–4 are the highest-leverage places to start; the
local-first sync work in Phase 5 is where the project becomes something no
mainstream study app will ship. Come build the learning OS with us.
