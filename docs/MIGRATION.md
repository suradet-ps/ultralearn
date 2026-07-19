# Vue + Supabase → Rust (Leptos CSR) Migration Guide

This document is a **complete, self-contained playbook** for migrating a
Vue 3 (TypeScript) + Supabase single-page application to **Rust compiled to
WebAssembly** using the [Leptos](https://leptos.dev) framework in
**Client-Side Rendering (CSR)** mode.

It was distilled from the first successful migration in our monorepo
(`tome`, a technical reading tracker). Every pattern below is taken from
working, shipped code. An AI agent (or a human) should be able to follow it
top-to-bottom and reproduce an equivalent Rust app for **any** Vue + Supabase
project of the same shape.

> **Golden rule:** keep the Supabase schema, RLS policies, and REST/Auth API
> exactly as they are. We are replacing the *frontend* only. The Rust app talks
> to the same Supabase project over plain HTTPS (`PostgREST` + `GoTrue`).

---

## 0. Mental Model — How Concepts Translate

| Vue / TS concept                     | Leptos / Rust equivalent                                   |
| ------------------------------------ | ---------------------------------------------------------- |
| `.vue` SFC                           | `#[component] fn Name() -> impl IntoView` + `view! {}`     |
| `ref` / `reactive`                   | `RwSignal<T>` / `ReadSignal<T>` / `Signal<T>`              |
| `computed`                           | `Signal::derive(move || ...)` or `Memo::new`               |
| `watch` / `watchEffect`              | `Effect::new(move |_| ...)`                                |
| `props`                              | `#[component]` fn params (`#[prop(...)]`)                  |
| `emit` / event props                 | `Callback<T>` props                                        |
| `slots` / `children`                 | `Children` / `ChildrenFn` prop                             |
| `v-if` / `v-else`                    | `<Show when=.. fallback=..>`                               |
| `v-for`                              | `<For>` or `.iter().map(...).collect_view()`              |
| `v-model`                            | `value=Signal` + `on_input=Callback<String>`              |
| Pinia store                          | `OnceLock<State>` singleton of `RwSignal`s                 |
| `@supabase/supabase-js`              | Hand-rolled `PostgREST` + `GoTrue` client over `gloo-net` |
| Vue Router                           | `leptos_router` (`<Router>`, `<Routes>`, `<Route>`)        |
| `vite` dev server / build            | `trunk serve` / `trunk build --release`                    |
| `axios` / `fetch`                    | `gloo-net::http::Request`                                  |
| `dayjs` / `date-fns`                 | `chrono` (with `wasmbind` feature!)                        |
| `marked` / `markdown-it`             | `pulldown-cmark` + `ammonia` (sanitizer)                   |
| `DOMPurify`                          | `ammonia`                                                  |
| `uuid` (npm)                         | `uuid` crate (with `js` feature!)                          |
| `localStorage` wrapper               | `gloo-storage::LocalStorage`                               |

---

## 1. Target Stack & Scaffolding

- **Framework:** Leptos `0.8`, features `["csr"]` only (no SSR/hydration).
- **Bundler:** Trunk.
- **Toolchain:** stable Rust, edition 2024, target `wasm32-unknown-unknown`.
- **HTTP:** `gloo-net`. **Storage:** `gloo-storage`. **JS interop:** `web-sys`,
  `js-sys`, `wasm-bindgen`, `wasm-bindgen-futures`.

### 1.1 `rust-toolchain.toml`

```toml
[toolchain]
channel = "stable"
targets = ["wasm32-unknown-unknown"]
profile = "minimal"
```

### 1.2 `Cargo.toml`

Use the following as the canonical dependency set. Add/remove `web-sys`
features as the app needs them (the compiler will tell you which ones are
missing).

```toml
[package]
name = "myapp"
version = "0.1.0"
edition = "2024"
rust-version = "1.88"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
leptos = { version = "0.8", default-features = false, features = ["csr"] }
leptos_meta = { version = "0.8", default-features = false }
leptos_router = { version = "0.8", default-features = false }

# HTTP / Supabase
gloo-net = { version = "0.7", default-features = false, features = ["http", "json"] }
gloo-storage = { version = "0.4", default-features = false }
gloo-utils = { version = "0.2", default-features = false, features = ["serde"] }

# Serialization
serde = { version = "1", default-features = false, features = ["derive", "std"] }
serde_json = { version = "1", default-features = false, features = ["std"] }

# Time / IDs  (NOTE the wasm-specific features)
chrono = { version = "0.4", default-features = false, features = ["clock", "serde", "std", "wasmbind"] }
uuid = { version = "1", default-features = false, features = ["v4", "serde", "js"] }
web-time = { version = "1", default-features = false, features = ["serde"] }

# Markdown / Sanitization (only if the app renders markdown)
pulldown-cmark = { version = "0.13", default-features = false, features = ["html"] }
ammonia = { version = "4", default-features = false }

# Logging / panics
console_log = { version = "1", default-features = false }
console_error_panic_hook = { version = "0.1", default-features = false }
log = { version = "0.4", default-features = false }

# Errors / URLs
thiserror = { version = "2", default-features = false }
url = { version = "2", default-features = false, features = ["std"] }

# JS interop
js-sys = { version = "0.3", default-features = false }
wasm-bindgen = { version = "0.2", default-features = false, features = ["std"] }
wasm-bindgen-futures = { version = "0.4", default-features = false }
web-sys = { version = "0.3", default-features = false, features = [
  "Window", "Document", "HtmlElement", "HtmlInputElement", "HtmlTextAreaElement",
  "HtmlSelectElement", "Element", "Node", "console", "Storage", "Location",
  "Event", "EventTarget", "MouseEvent", "SubmitEvent", "Headers", "Request",
  "RequestInit", "Response", "Url", "UrlSearchParams",
] }

[target.'cfg(target_arch = "wasm32")'.dev-dependencies]
wasm-bindgen-test = { version = "0.3", default-features = false }

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.dev]
opt-level = 1
debug = true
incremental = true

[lints.rust]
unsafe_code = "deny"
unused_must_use = "deny"

[lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
# Leptos view! macros are noisy — relax the pedantic UI lints:
module_name_repetitions = "allow"
missing_errors_doc = "allow"
must_use_candidate = "allow"
too_many_lines = "allow"
too_many_arguments = "allow"
needless_pass_by_value = "allow"
future_not_send = "allow"
```

### 1.3 `Trunk.toml`

```toml
[build]
target = "index.html"
release = true
dist = "dist"
public_url = "/"
filehash = true
minify = "on_release"

[serve]
addresses = ["127.0.0.1"]
port = 3000
no_spa = false          # SPA fallback for client-side routing
```

### 1.4 `index.html` (Trunk entry point)

Trunk-processed assets use the `data-trunk` attribute. Static CSS/fonts that
should NOT be hashed use a normal `<link>`.

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>My App</title>

    <link data-trunk rel="icon" type="image/svg+xml" href="public/favicon.svg" />

    <!-- App styles: Trunk copies + hashes these into dist/ -->
    <link data-trunk rel="css" href="public/styles/variables.css" />
    <link data-trunk rel="css" href="public/styles/reset.css" />
    <link data-trunk rel="css" href="public/styles/main.css" />

    <!-- WASM entry point. data-wasm-opt="0" avoids Windows/Rust 1.87+ wasm-opt breakage -->
    <link data-trunk rel="rust" data-weak-refs data-wasm-opt="0" />
  </head>
  <body></body>
</html>
```

> **Gotcha:** `wasm-opt` (via Trunk) can fail on Windows and with newer Rust
> due to bulk-memory/reference-types. `data-wasm-opt="0"` disables it. If you
> want optimization, keep it on in CI/Linux only.

### 1.5 `rustfmt.toml`

Keep it minimal (default style). A `rustfmt.toml` with a couple of preferences
is fine; `cargo fmt --all --check` must pass in CI.

---

## 2. Directory Architecture

Map the Vue tree onto a Rust module tree. Each folder is a Rust module with a
`mod.rs` that re-exports its children.

```text
src/
├── lib.rs               # wasm-bindgen(start) entry + module tree
├── app.rs               # <App/> root: meta + <Router> + shell
├── core/                # framework-agnostic logic (was: src/lib, src/api, utils)
│   ├── mod.rs
│   ├── error.rs         # AppError / AppResult
│   ├── supabase.rs      # SupabaseClient + config detection + AuthSession
│   ├── postgrest.rs     # PostgREST query builder (replaces supabase-js .from())
│   ├── auth.rs          # GoTrue wrappers (sign in/up/out, get_user)
│   ├── markdown.rs      # pulldown-cmark + ammonia (optional)
│   ├── highlight.rs     # code highlighting + sanitizer allowlists (optional)
│   ├── time.rs          # WASM-safe timestamps
│   ├── utils.rs         # formatting helpers
│   └── types/
│       ├── mod.rs
│       └── database.rs  # serde structs mirroring DB rows (snake_case)
├── stores/              # Pinia stores → OnceLock singletons
│   ├── mod.rs
│   ├── auth.rs
│   ├── books.rs         # (your domain entities)
│   ├── progress.rs
│   └── notes.rs
├── composables/         # Vue composables → plain fns returning handles
│   ├── mod.rs
│   ├── use_timer.rs
│   └── use_markdown.rs
├── components/
│   ├── mod.rs
│   ├── icons.rs         # inline Lucide SVGs as components
│   ├── common/          # BaseButton, BaseInput, BaseModal, ...
│   ├── layout/          # Topbar, Sidebar, ...
│   └── <feature>/       # feature components
└── views/               # Page-level components (Vue router views)
    ├── mod.rs
    ├── router.rs        # route guards / loader helpers
    └── *_view.rs
public/
└── styles/              # plain .css files (variables, reset, main, ...)
```

> **Rule:** put anything that does not touch Leptos signals into `core/`. This
> keeps business logic testable with plain `cargo test --lib` and reusable
> across the app.

---

## 3. Entry Point (`lib.rs`)

```rust
#![allow(missing_docs)]

pub mod app;
pub mod components;
pub mod composables;
pub mod core;
pub mod stores;
pub mod views;

pub use app::App;

use console_error_panic_hook::set_once as set_panic_hook;
use console_log::init_with_level;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use log::Level;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
    init_with_level(Level::Debug).ok();

    // CRITICAL: install store singletons INSIDE the mount closure so their
    // RwSignals are created in the mount-root Owner and are never disposed.
    mount_to_body(|| {
        crate::stores::auth::install();
        crate::stores::books::install();
        crate::stores::progress::install();
        crate::stores::notes::install();
        view! { <App /> }
    });
}
```

> **Gotcha (learned the hard way):** creating global signals *outside* the
> reactive owner leads to disposed-signal panics. Either install stores inside
> `mount_to_body`, or wrap them with `Owner::new_root` + `OnceLock`. The
> `OnceLock` + install-inside-mount pattern below is the simplest that works.

---

## 4. Error Handling (`core/error.rs`)

A single `AppError` enum, `Serialize`/`Deserialize`, with `thiserror`. Provide
`From` impls for every foreign error you `?` on, so async chains stay clean.

```rust
use serde::{Deserialize, Serialize};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum AppError {
    #[error("{message}")]
    Config { message: String },
    #[error("{message}")]
    Http { status: u16, message: String },
    #[error("JSON error: {0}")]
    Json(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("No data returned from server.")]
    NoData,
    #[error("{0}")]
    Other(String),
}

impl AppError {
    pub fn config(m: impl Into<String>) -> Self { Self::Config { message: m.into() } }
    pub fn http(status: u16, m: impl Into<String>) -> Self { Self::Http { status, message: m.into() } }
    pub fn other(m: impl Into<String>) -> Self { Self::Other(m.into()) }
    pub const fn is_unauthorized(&self) -> bool {
        matches!(self, Self::Unauthorized) ||
        matches!(self, Self::Http { status, .. } if matches!(*status, 401 | 403))
    }
}

impl From<serde_json::Error> for AppError { fn from(e: serde_json::Error) -> Self { Self::Json(e.to_string()) } }
impl From<gloo_net::Error> for AppError { fn from(e: gloo_net::Error) -> Self { Self::Network(e.to_string()) } }
```

---

## 5. The Supabase Client (replaces `@supabase/supabase-js`)

There is no official Rust WASM Supabase SDK, so we implement the two endpoints
we actually use: **`PostgREST`** (`/rest/v1/...`) and **`GoTrue`**
(`/auth/v1/...`). Both are plain HTTPS.

### 5.1 Config detection (`core/supabase.rs`)

Supports two sources, in order:

1. **Compile-time env vars** via `option_env!("SUPABASE_URL")` /
   `option_env!("SUPABASE_ANON_KEY")` (works on Vercel/CI — the values are
   baked into the WASM at build time).
2. **Runtime `localStorage`** fallback via an in-app config form (great for
   local dev without rebuilding).

```rust
const URL_STORAGE_KEY: &str = "myapp_supabase_url";
const ANON_STORAGE_KEY: &str = "myapp_supabase_anon";
const TOKEN_STORAGE_KEY: &str = "myapp_supabase_token";

fn read_config() -> (String, String) {
    let build_url = option_env!("SUPABASE_URL").unwrap_or_default().to_string();
    let build_anon = option_env!("SUPABASE_ANON_KEY").unwrap_or_default().to_string();
    if !build_url.is_empty() && !build_anon.is_empty() {
        return (build_url, build_anon);
    }
    let url = LocalStorage::get::<String>(URL_STORAGE_KEY).unwrap_or_default();
    let anon = LocalStorage::get::<String>(ANON_STORAGE_KEY).unwrap_or_default();
    (url, anon)
}
```

The `SupabaseClient` bundles url + anon key + optional bearer token, and hands
out a `PostgrestClient` and a `SupabaseAuth`:

```rust
#[derive(Debug, Clone)]
pub struct SupabaseClient { url: String, anon_key: String, token: Option<String> }

impl SupabaseClient {
    pub fn postgrest(&self) -> PostgrestClient {
        let mut c = PostgrestClient::new(&self.url).with_api_key(&self.anon_key);
        if let Some(t) = &self.token { c = c.with_token(t); }
        c
    }
    pub fn auth(&self) -> SupabaseAuth<'_> {
        SupabaseAuth::new(&self.url, &self.anon_key, self.token.as_deref())
    }
    pub fn load_persisted_token() -> Option<String> {
        LocalStorage::get::<String>(TOKEN_STORAGE_KEY).ok()
    }
    pub fn persist_token(token: Option<&str>) {
        match token {
            Some(v) => { let _ = LocalStorage::set(TOKEN_STORAGE_KEY, v); }
            None => LocalStorage::delete(TOKEN_STORAGE_KEY),
        }
    }
}

/// Build a fresh client with the persisted token attached.
pub fn supabase() -> AppResult<SupabaseClient> { /* read_config + set_token */ }
```

The token is persisted to `localStorage` on login and re-attached on every
`supabase()` call — this is our session persistence (equivalent to
supabase-js `persistSession`).

### 5.2 The `PostgREST` query builder (`core/postgrest.rs`)

This is the workhorse that replaces `supabase.from('table').select()...`. It is
a small fluent builder over `gloo-net` that constructs `PostgREST` URLs and
headers. Mirror only the operations you use.

Supported operations (extend as needed):

- `.from(table)` → `QueryBuilder`
- `.select(cols)`, `.eq/.lte/.gte/.is_in(col, val)`, `.order(col, asc)`,
  `.range(offset, limit)`, `.on_conflict(cols)`
- `.get::<T>()` → `Vec<T>`, `.get_one::<T>()` → `Option<T>`
  (sets `Accept: application/vnd.pgrst.object+json`, treats HTTP 406 as `None`)
- `.insert(&body)` / `.insert_one(&body)` (adds `Prefer: return=representation`)
- `.update(&body)` (PATCH), `.delete()`
- `.upsert(&body, "col_a,col_b")` (`Prefer: resolution=merge-duplicates,return=representation`)
- `client.rpc(name, &body)` → `POST /rest/v1/rpc/{name}`

Auth headers applied to every request:

```rust
fn apply_auth(&self, mut b: RequestBuilder) -> RequestBuilder {
    if let Some(k) = &self.api_key { b = b.header("apikey", k); }
    if let Some(t) = &self.token { b = b.header("Authorization", &format!("Bearer {t}")); }
    b
}
```

Range/pagination uses the `Range` + `Range-Unit: items` headers (Supabase’s
`offset-limit` convention: `Range: 0-999`).

Response parsing centralizes error mapping (401/403 → `Unauthorized`; parse the
`message` / `error_description` JSON fields for a friendly message).

**Translation examples:**

```ts
// Vue (supabase-js)
const { data } = await supabase
  .from('reading_books')
  .select('*')
  .eq('user_id', uid)
  .order('created_at', { ascending: false });
```

```rust
// Rust
let books: Vec<Book> = client.postgrest()
    .from("reading_books")
    .select("*")
    .eq("user_id", uid.to_string())
    .order("created_at", false)
    .range(0, 999)
    .get()
    .await?;
```

```ts
// upsert
await supabase.from('reading_progress')
  .upsert(row, { onConflict: 'user_id,chapter_id' });
```

```rust
let saved: Progress = client.postgrest()
    .from("reading_progress")
    .upsert_one(&row, "user_id,chapter_id")
    .await?;
```

### 5.3 Auth / GoTrue (`core/auth.rs`)

Wrap the GoTrue REST endpoints directly:

- `POST /auth/v1/token?grant_type=password` → sign in
- `POST /auth/v1/signup` (with `data` for user metadata) → sign up
- `POST /auth/v1/logout` → sign out
- `GET  /auth/v1/user` → restore session from a token

```rust
pub async fn sign_in_with_password(&self, email: &str, password: &str) -> AppResult<AuthSession> {
    let url = format!("{}/auth/v1/token?grant_type=password", self.url);
    let body = serde_json::json!({ "email": email, "password": password });
    self.post_session(&url, &body).await
}
```

`AuthSession` deserializes `access_token`, `refresh_token`, `expires_in`, and
the nested `user`. On success, persist the access token.

---

## 6. Database Types (`core/types/database.rs`)

One `serde` struct per table, named exactly like the Vue TS interfaces. Keep
field names in **`snake_case`** to match PostgREST columns → **no rename layer
needed**. Enums map to Postgres enums via `#[serde(rename_all = "snake_case")]`.

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ReadingStatus {
    #[default] NotStarted,
    InProgress,
    Completed,
    ReviewNeeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub author: Option<String>,
    pub total_chapters: i32,
    pub created_at: DateTime<Utc>,
}
```

Guidelines:
- Nullable columns → `Option<T>`.
- `integer` → `i32`, `bigint`/counts from RPC → `i64`, `decimal`/`double
  precision` → `f64`.
- Client-only fields (e.g. a computed `children` tree) → add
  `#[serde(default)]` so they don’t need to be present in API responses.

---

## 7. State Management — Pinia → `OnceLock` Singletons

Each Pinia store becomes a `Copy` struct of `RwSignal`s, stored in a module
`OnceLock`, installed once at startup, and accessed via a `use_*()` helper.
Actions become `async` methods (they perform the fetch and update signals).

```rust
use leptos::prelude::*;
use std::sync::OnceLock;

static AUTH: OnceLock<AuthState> = OnceLock::new();

pub fn install() { let _ = AUTH.set(AuthState::new()); }
pub fn use_auth() -> AuthState { *AUTH.get().expect("AuthState not initialized") }

#[derive(Debug, Clone, Copy)]
pub struct AuthState {
    pub user: RwSignal<Option<uuid::Uuid>>,
    pub profile: RwSignal<Option<Profile>>,
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}

impl AuthState {
    pub async fn sign_in(&self, email: &str, password: &str) -> AppResult<()> {
        self.loading.set(true);
        self.error.set(None);
        let result = async {
            let mut c = supabase::supabase()?;
            let session = c.auth().sign_in_with_password(email, password).await?;
            session.persist();
            c.set_token(Some(session.access_token));
            let profile = fetch_profile(&c, session.user.id).await?;
            AppResult::Ok((session.user.id, profile))
        }.await;
        self.loading.set(false);
        // ...set signals or error...
        Ok(())
    }
}
```

Notes:
- `RwSignal` is `Copy`, so `AuthState` is `Copy` — you can freely move it into
  closures without cloning.
- In async blocks, read signals with `get_untracked()` to avoid accidental
  reactive subscriptions.
- Keep the `loading` / `error` fields to mirror Pinia’s common ergonomics.
- Store an in-memory tree + provide a `reset()` for sign-out.

Stores may depend on each other by calling `use_auth()` etc. (e.g. `books`
reads the current `user` id before fetching).

---

## 8. Components — Vue SFC → Leptos `#[component]`

### 8.1 Anatomy

```rust
use leptos::prelude::*;

#[component]
pub fn BaseButton(
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = false)] loading: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = "button")] button_type: &'static str,
    #[prop(default = false)] block: bool,
    #[prop(optional, into)] on_click: Option<Callback<web_sys::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let class = format!("btn btn--{}{}", variant.class(), if block { " btn--block" } else { "" });
    let is_disabled = disabled || loading;
    view! {
        <button type=button_type class=class disabled=is_disabled
            on:click=move |ev| {
                if !is_disabled { if let Some(h) = on_click.as_ref() { h.run(ev); } }
            }>
            {children()}
        </button>
    }
}
```

Prop translation table:

| Vue prop pattern              | Leptos                                             |
| ----------------------------- | -------------------------------------------------- |
| `prop: { default: x }`        | `#[prop(default = x)] name: T`                     |
| optional prop                 | `#[prop(optional)] name: Option<T>`                |
| accepts value OR signal       | `#[prop(into)] name: Signal<T>`                    |
| `emit('click')`               | `#[prop(optional, into)] on_click: Option<Callback<...>>` |
| default slot                  | `children: Children`                               |
| named/reused slot             | `children: ChildrenFn`                             |

### 8.2 `v-model` two-way binding

Split into a value `Signal` and an `on_input` `Callback<String>`:

```rust
// Parent
let email = RwSignal::new(String::new());
view! {
    <BaseInput
        value=Signal::derive(move || email.get())
        on_input=Callback::new(move |v: String| email.set(v))
        label="Email" input_type="email"
    />
}
```

### 8.3 Conditionals & lists

```rust
<Show when=move || user.get().is_some() fallback=|| view! { <LoginView/> }>
    <Dashboard/>
</Show>

// list
{move || items.get().into_iter()
    .map(|item| view! { <li>{item.title}</li> })
    .collect_view()}
```

### 8.4 Icons

Rather than an icon crate, inline Lucide SVG paths as a small generic `Icon`
component + a `macro_rules! icon` to declare each named icon. This keeps the
bundle small and guarantees a 1:1 match with the original Vue icons.

---

## 9. Views & Routing (`leptos_router`)

`app.rs` sets up meta + `<Router>` and a shell that swaps authenticated vs.
unauthenticated routes with `<Show>` (Leptos 0.8 has no built-in route guards).

```rust
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let auth = use_auth();
    Effect::new(move |_| {
        if !auth.initialized.get_untracked() {
            leptos::task::spawn_local(async move { auth.init_auth().await; });
        }
    });
    view! {
        <Title text="My App" />
        <Router><Shell/></Router>
    }
}

#[component]
fn Shell() -> impl IntoView {
    let user = use_auth().user;
    let fallback = || view! { <NotFound/> };
    view! {
        <Show when=move || user.get().is_some() fallback=move || view! {
            <Routes fallback=fallback>
                <Route path=path!("/login") view=LoginView/>
                <Route path=path!("/") view=LoginView/>
            </Routes>
        }>
            <AppTopbar/>
            <Routes fallback=fallback>
                <Route path=path!("/") view=DashboardView/>
                <Route path=path!("/books/:id") view=BookView/>
                <Route path=WildcardSegment("") view=NotFound/>
            </Routes>
        </Show>
    }
}
```

- Navigation: `use_navigate()` → `navigate("/", Default::default())`.
- Links: `<A href="/register">`.
- Route params: `use_params_map()`.
- Run async work in handlers with `leptos::task::spawn_local`.

> **Gotcha:** if you navigate right after an async action, capture the value you
> need before `await`, and guard signal writes after `await` with an
> `on_cleanup`-set `disposed` flag to avoid writing to a disposed view.

---

## 10. Composables

Vue composables become plain functions returning a handle struct of signals +
callbacks. `on_cleanup` replaces `onUnmounted`.

```rust
pub struct TimerHandle {
    pub seconds: Signal<i64>,
    pub running: ReadSignal<bool>,
    pub start: Callback<()>,
    pub pause: Callback<()>,
    pub reset: Callback<()>,
}

pub fn use_timer() -> TimerHandle {
    let seconds = RwSignal::new(0_i64);
    // web_sys setInterval; store the handle id in an RwSignal<Option<i32>>
    // clear it in on_cleanup(...)
    // ...
}
```

Use `web-sys` (`set_interval_with_callback_and_timeout_and_arguments_0`,
`clear_interval_with_handle`) and `wasm_bindgen::closure::Closure` for timers /
DOM APIs.

---

## 11. Markdown & Sanitization (optional feature)

Replace `marked`/`markdown-it` + `DOMPurify` with `pulldown-cmark` + `ammonia`:

- Parse with GFM options (tables, strikethrough, task lists, footnotes).
- Intercept fenced code blocks to run them through your highlighter, emitting
  raw `Event::Html`.
- Sanitize the final HTML with `ammonia::Builder` using explicit allowlists for
  tags, attributes, and URL schemes; force `rel="noopener noreferrer nofollow"`
  on links.

Everything runs in-WASM, no JS dependency.

---

## 12. Time on WASM (important)

`std::time::SystemTime::now()` **panics** on `wasm32-unknown-unknown`. Fixes:

- Enable `chrono`’s **`wasmbind`** feature (already in the Cargo.toml above) so
  `Utc::now()` works via JS `Date`.
- Enable `uuid`’s **`js`** feature so `Uuid::new_v4()` has an RNG.
- For monotonic timing use the `web-time` crate.
- When sending timestamps to Supabase, format with a **fixed ISO-8601 / RFC3339**
  string (`to_rfc3339()`), not a locale-dependent format, to avoid drift/parse
  errors on the server.

Put a `core/time.rs` helper that returns `chrono::Utc::now()` and formats to
RFC3339, and use it everywhere instead of raw time calls.

---

## 13. Styling

Pure CSS, unchanged from the Vue project’s design tokens:

- Put global stylesheets in `public/styles/` (`variables.css` with `:root`
  custom properties, `reset.css`, `main.css`, optional `highlight.css`).
- Reference them from `index.html` via `data-trunk rel="css"`.
- For component-scoped styles you can also use Leptos `<style>` blocks, but the
  simplest 1:1 port is to keep the existing global CSS + BEM class names and
  just emit the same `class="..."` strings from `view!`.

---

## 14. CI (`.github/workflows/ci.yml`)

Five jobs; `build` depends on the rest passing:

```yaml
name: CI
on:
  push: { branches: [main] }
  pull_request: { branches: [main] }
permissions: { contents: read }
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
        with: { persist-credentials: false }
      - uses: dtolnay/rust-toolchain@stable
        with: { targets: wasm32-unknown-unknown }
      - uses: Swatinem/rust-cache@v2
      - run: cargo check --target wasm32-unknown-unknown
  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
        with: { persist-credentials: false }
      - uses: dtolnay/rust-toolchain@stable
        with: { components: clippy, targets: wasm32-unknown-unknown }
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --target wasm32-unknown-unknown -- -D clippy::correctness -D clippy::suspicious
  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
        with: { persist-credentials: false }
      - uses: dtolnay/rust-toolchain@stable
        with: { components: rustfmt }
      - run: cargo fmt --all --check
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
        with: { persist-credentials: false }
      - uses: dtolnay/rust-toolchain@stable
        with: { targets: wasm32-unknown-unknown }
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --lib
  build:
    runs-on: ubuntu-latest
    needs: [check, clippy, fmt, test]
    steps:
      - uses: actions/checkout@v6
        with: { persist-credentials: false }
      - uses: dtolnay/rust-toolchain@stable
        with: { targets: wasm32-unknown-unknown }
      - uses: Swatinem/rust-cache@v2
      - run: cargo install trunk --locked
      - run: trunk build --release
```

---

## 15. Deployment (Vercel) — `vercel.json`

Vercel builds the WASM and serves `dist/` as a static SPA.

```json
{
  "buildCommand": "cargo install trunk --locked && rustup target add wasm32-unknown-unknown && trunk build --release",
  "outputDirectory": "dist",
  "rewrites": [{ "source": "/(.*)", "destination": "/index.html" }],
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        { "key": "Content-Security-Policy",
          "value": "default-src 'self'; script-src 'self' 'wasm-unsafe-eval' 'unsafe-inline'; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com data:; img-src 'self' data: blob: https:; connect-src 'self' https://*.supabase.co wss://*.supabase.co; frame-ancestors 'none'; base-uri 'self'; object-src 'none'; upgrade-insecure-requests" }
      ]
    },
    { "source": "/(.*).wasm", "headers": [{ "key": "Content-Type", "value": "application/wasm" }] }
  ]
}
```

Deployment gotchas:
- **CSP must allow `'wasm-unsafe-eval'`** in `script-src` or the module won’t
  instantiate. Trunk’s bootstrap also needs `'unsafe-inline'` for its inline
  loader script.
- **`connect-src` must list your Supabase domains** (`https://*.supabase.co`
  and `wss://*.supabase.co`).
- Use a catch-all rewrite `"/(.*)" → "/index.html"` for client-side deep links.
  Do **not** enable `cleanUrls` — it interferes with the SPA fallback.
- Set `SUPABASE_URL` / `SUPABASE_ANON_KEY` as Vercel build-time env vars so
  `option_env!` bakes them in.
- Serve `.wasm` with `Content-Type: application/wasm` and long immutable cache;
  keep `index.html` at `no-cache`.

---

## 16. Migration Order (Step-by-Step Checklist)

Follow this order — each step compiles/passes before moving on.

1. **Scaffold:** `rust-toolchain.toml`, `Cargo.toml`, `Trunk.toml`,
   `index.html`, `rustfmt.toml`, empty module tree. Confirm `trunk serve`
   renders a "Hello" `<App/>`.
2. **Copy CSS** from the Vue project into `public/styles/` unchanged; wire in
   `index.html`.
3. **`core/error.rs`** — `AppError` / `AppResult` + `From` impls.
4. **`core/postgrest.rs`** — the query builder (only the ops you use).
5. **`core/auth.rs` + `core/supabase.rs`** — client, config detection, session
   persistence, GoTrue wrappers.
6. **`core/types/database.rs`** — one struct per table (mirror the TS types),
   plus any RPC response structs.
7. **`core/` extras** — `time.rs`, `utils.rs`, and `markdown.rs`/`highlight.rs`
   if the app renders markdown. Add `cargo test --lib` unit tests here.
8. **Stores** — port Pinia stores to `OnceLock` singletons; wire `install()`
   calls in `lib.rs` inside `mount_to_body`.
9. **Common components** — `BaseButton`, `BaseInput`, `BaseModal`, loaders,
   `icons.rs`.
10. **Layout + auth views** — topbar, `LoginView`, `RegisterView`; get the
    end-to-end auth flow working against the real Supabase project.
11. **Feature views & components** — port page by page, matching classes/markup.
12. **Router** — assemble `<App/>`/`<Shell/>` with the authed/unauthed split.
13. **Composables** — `use_timer`, etc.
14. **Green build:** `cargo fmt --all --check`, `cargo clippy --target
    wasm32-unknown-unknown`, `cargo test --lib`, `trunk build --release`.
15. **CI + Vercel** — add the workflow and `vercel.json`; set env vars; deploy.

---

## 17. WASM Pitfalls Cheat-Sheet (real bugs we hit)

| Symptom                                            | Cause / Fix                                                                 |
| -------------------------------------------------- | -------------------------------------------------------------------------- |
| Panic on `SystemTime::now()`                       | Use `chrono` `wasmbind`; never call `std::time` directly.                   |
| `Uuid::new_v4()` panics / no RNG                   | Enable `uuid` `js` feature.                                                 |
| Disposed-signal panic for global state             | Create store signals inside `mount_to_body` / `Owner::new_root` + `OnceLock`. |
| Context missing across `<Router>` boundary         | `provide_context` inside the mount closure and inside each route view.      |
| `wasm-opt` build fails (Windows / Rust 1.87+)      | `data-wasm-opt="0"` in `index.html`, or restrict wasm-opt to Linux CI.     |
| Blank page / module won’t load in prod             | CSP missing `'wasm-unsafe-eval'`; add it to `script-src`.                   |
| Supabase calls blocked in prod                     | Add `https://*.supabase.co` (+ `wss://`) to CSP `connect-src`.              |
| 404 on deep-link refresh                           | SPA rewrite `"/(.*)" → "/index.html"`; do not use `cleanUrls`.             |
| Timestamps rejected/parsed wrong by Postgres       | Send fixed `to_rfc3339()` strings, not locale formats.                     |
| Infinite reactive loop (e.g. select → fetch)       | Read with `get_untracked()` inside effects/handlers that also write.        |
| `get_one` returns error instead of `None`          | Treat HTTP 406 as `Ok(None)` when using `pgrst.object` accept header.       |

---

*Generated from the `tome` reference migration. Keep this file in the repo root
of each project you migrate and update the table in §0 whenever you introduce a
new cross-language pattern.*
