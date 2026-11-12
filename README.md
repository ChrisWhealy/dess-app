# Service simulator

[![CI](https://github.com/ChrisWhealy/dess-app/actions/workflows/ci.yml/badge.svg)](https://github.com/ChrisWhealy/dess-app/actions/workflows/ci.yml) [![Rust](https://img.shields.io/badge/rust-1.85.0%2B-blue.svg?maxAge=3600)](https://github.com/ChrisWhealy/dess-app/blob/main/rust-toolchain.toml)

A discrete-event simulation of a service: a queue, one to five servers, and jobs that arrive, wait, run, time out, retry, fail and are cancelled.
It runs in the browser, and is drawn as one long-lived scene by [`svg-dom-graph`](https://crates.io/crates/svg-dom-graph).

This is an application of its own, built from this repository alone.
The one thing it takes from the svg-dom-graph project is the published crate, from crates.io.
So it shows what a consumer meets: the packaged crate, its dependencies, its API, and its build.

## Layout

| Path | What it is |
|---|---|
| `crates/service-sim-engine` | The pure engine: an integer-tick event heap, a seeded generator and canonical hashes. No browser |
| `crates/service-sim` | The projection of the engine's state, the scene that shows it, the controller and the session coordinator |
| `app` | The page side: the markup, the controls, and the entry point that the page calls |
| `web` | The site: `index.html`, `main.js`, `style.css`, and `pkg/`, which the build writes |
| `data` | The scenarios and the answers that the engine is held to |
| `docs/budgets.md` | How the application bounds the work that its input can ask for |

## Build and run

You need Rust 1.85 or newer, which `rust-toolchain.toml` selects, with the `wasm32-unknown-unknown` target, [wasm-pack](https://rustwasm.github.io/wasm-pack/) 0.13 or newer, and Node 18 or newer to serve the site.

```sh
scripts/build.sh            # a release build into web/pkg (scripts/build.sh --dev is quicker)
node scripts/serve.mjs      # http://127.0.0.1:8080
```

The build passes `--locked`, so it uses `Cargo.lock` as it is, and two builds of one commit use the same dependencies.
`web/` is the whole site after the build.

## Deploy

Any static host will do.
Copy `web/`, without anything else, to it.
The host needs two things:

- `.wasm` files served as `application/wasm`, which the browser needs to compile them while they stream.
- A content security policy, if you set one, that allows the module and the wasm: `default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'`.

Nothing is fetched from another origin, and the page makes no request but for its own files.

## Test

```sh
cargo test --workspace --exclude dess-app               # the engine, the projection, the session and the controller, natively
wasm-pack test --headless --chrome app                  # the page, in Chrome
wasm-pack test --headless --chrome crates/service-sim   # the surface and controller, in Chrome
cargo deny check                                        # advisories, licences and sources
```

Each crate that has browser tests has a `webdriver.json` that starts Chrome with `--no-sandbox`, which a CI runner needs and a desktop does not mind.
CI runs all of these, on Rust 1.85 and on current stable, and checks that `svg-dom-graph` in `Cargo.lock` comes from the registry.

## The svg-dom-graph version

`crates/service-sim/Cargo.toml` and `app/Cargo.toml` name `svg-dom-graph = "0.2.50"`, which is the published crate.
To move to a newer release, run `scripts/adopt.sh <version>`.
It changes the version, updates the lock file, refuses a local patch, and runs everything that CI runs against the registry's copy.
It is the last step of a release of the library.
A release that changes what the application relies on fails them, and that is the point of keeping the application outside the library's repository.

## What building it showed

- The crates needed nothing from the library's workspace.
  They compile against the published 0.2.50 on Rust 1.85, and the public API was enough for a scene, a data node, connectors, selection, stepping and a replaced scene.
- Everything that did not come with the crates was a path into the library's repository: the scenario data, which is now in `data/`.
- `service_sim::host::GalleryHint` is named for the library's own demo gallery, and watches an element for a class change.
  In an application with no gallery it does nothing, and the page's own visibility events do the work.
  That is a name that could be more general.

## Licence

MIT.
