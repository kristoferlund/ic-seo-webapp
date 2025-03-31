# SEO-Optimized Dynamic Routing for Vite Apps on the Internet Computer

> [!NOTE] > **This is an experimental project** — designed to demonstrate how single-page applications can serve SEO-friendly, route-specific content dynamically on the Internet Computer (ICP). This is not a polished library or CLI yet — feel free to fork and experiment.

---

## 🧠 Overview

Traditional SPA setups (e.g., Vite-based apps) typically serve a single `index.html`, which hinders SEO and social link previews. Search engines and social networks need per-route metadata (title, description, OpenGraph tags, etc.) to correctly index pages and generate rich previews.

This project explores a pattern to solve that by:

- Dynamically generating route-specific `index.html` responses on the "server", served by an ICP canister.
- Certifying these responses using ICP's HTTP certification.
- Using file-based routing to map incoming requests to route handlers.
- Optionally supporting full server-side rendering (e.g., with HTMX or other frameworks).

## 👀 Live demo: <https://blx6i-6iaaa-aaaal-qslxq-cai.icp0.io>

---

## 🗂 Structure

The repository contains:

- A **demo Vite front-end app**.
- A **server canister** written in Rust.
- A **router library** that enables dynamic routing and response generation.
- Integration with the **ICP HTTP certification system** for serving certified assets.

The goal is to enable this pattern within a _single-canister ICP setup_.

---

## 🔁 Request Flow

1. On canister init, all static assets are certified.
2. The default root (`/`) index.html is deleted, so a custom one can be generated on demand.
3. Incoming HTTP requests are routed via a dynamic file-based route tree.
4. Each route has an associated handler function.
5. The handler dynamically generates the content (e.g., custom `index.html`) and the router certifies it before serving.
6. On next request, the router checks if the content is already certified and serves it directly if available.

---

## 🧩 Example: Canister Initialization

```rust
// The built frontend assets, the output of `pnpm run build`
static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../dist");

#[init]
fn init() {
    // Certify pre-built static assets
    certify_all_assets(&ASSETS_DIR);

    // Remove default root asset to ensure it's generated dynamically
    delete_assets(vec!["/"]);
}
```

---

## 🌐 Hooking up HTTP Request Handlers

```rust
#[query]
pub fn http_request(req: HttpRequest) -> HttpResponse {
    router_library::http_request(req)
}

#[update]
fn http_request_update(req: HttpRequest) -> HttpResponse {
    ROUTES.with(|routes| router_library::http_request_update(req, routes))
}
```

---

## 🧪 Example: Route Handler

A route handler accepts:

- `HttpRequest`: the incoming request.
- `RouteParams`: extracted path parameters (e.g., from `/subpath/:id`).

It returns an `HttpResponse` struct, as defined by the `ic_http_certification` crate.

### Example: `subpath/:id` Handler

- Loads the pre-built index.html with a {{ title }} placeholder.
- Uses minijinja to render the template with a route-specific title.
- Constructs and returns an HttpResponse with text/html content.

```rust
use std::{borrow::Cow, collections::HashMap};
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use minijinja::Environment;
use router_library::router::RouteParams;

pub fn handler(_: HttpRequest, params: RouteParams) -> HttpResponse<'static> {
    let html = include_str!("../../../../dist/index.html");
    let env = Environment::new();
    let template = env.template_from_str(html).unwrap();
    let mut ctx = HashMap::new();
    ctx.insert("title", format!("Subpage {}", params.get("id").unwrap()));
    let rendered = template.render(ctx).unwrap();
    HttpResponse::builder()
        .with_headers(vec![("Content-Type".into(), "text/html".into())])
        .with_status_code(StatusCode::OK)
        .with_body(Cow::Owned(rendered.into_bytes()))
        .build()
}
```

---

## 🧭 File-Based Routing

The router library expects a statically defined route tree, generated at build time using a build script. The routes connect incoming requests to their respective handler functions.

### Example Route Tree Definition

```rust
use crate::routes;
use router_library::router::{NodeType, RouteNode};

thread_local! {
    pub static ROUTES: RouteNode = {
        let mut root = RouteNode::new(NodeType::Static("".into()));
        root.insert("", routes::index::handler);
        root.insert("*", routes::__any::handler);
        root.insert("/subpath/:id", routes::subpath::id::handler);
        root.insert("index2", routes::index2::handler);
        root
    };
}
```

---

## ⚙️ Route Generation at Build Time

In your `build.rs`:

```rust
use router_library::build::generate_routes;

fn main() {
    generate_routes();
}
```

This scans `src/routes/` using file-based routing conventions and generates the route tree automatically.

### Example Folder Layout

```
src/routes/
├── index.rs             --> "/"
├── index2.rs            --> "/index2"
├── *.rs                 --> wildcard, matches any request
└── subpath/
    └── :id.rs           --> "/subpath/:id"
```

---

## 💡 Usage Concept

You start with a regular Vite SPA. Then:

1. Add a `server/` folder at the root.
2. Inside `server/`, include:
   - Route handler modules (under `src/routes/`)
   - The asset-serving canister logic.
3. Use the router library to:
   - Match paths to handlers.
   - Generate dynamic responses.
   - Certify assets.

This lets you ship a single canister on ICP that supports dynamic, SEO-optimized rendering with route-level granularity.

---

## 🔒 Certified Assets

Dynamic content is certified before being served using ICP's HTTP certification mechanisms. This ensures that clients and search engines can trust the content even when it's dynamically generated.

---

## 🔮 Potential Future Developments

- Fine-grained caching control for dynamically generated assets.
- CLI tool to:
  - Scaffold the server integration into any SPA project.
  - Assist with deploying the project to the Internet Computer.
- Per-handler configuration of:
  - Response headers.
  - Content type.
  - Cache settings.
- Support for ICP HTTP certification features:
  - Asset aliasing.
  - Fallback assets.
  - Content encoding variants.
- Turn the router module into a reusable library crate.

---

## 📣 Feedback Welcome

> 💬 This is an experimental repo — I’m eager to hear your thoughts, use cases, or improvements.

Feel free to file issues, fork it, or ping me for collaboration.
