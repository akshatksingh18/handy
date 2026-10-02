# Handy

@upstream-instructions.md

Akshat's independently maintained copy of the MIT-licensed Handy desktop dictation application.
Handy is the production-quality Windows, macOS, and Linux foundation replacing the earlier
WhimprFlow proof of concept; mobile clients are not part of the current implementation.

**Status:** Active desktop foundation — the Windows source build and local startup are verified,
and this imported history is published in Akshat's private `akshatksingh18/handy` repository.
Akshat's Turbo live test still took 3–4 seconds and Hindi output failed. Active product scope is
now English only; the converter is off, while native streaming warm-up and diagnostics support
the faster Parakeet test profile. English live acceptance and signed packaging remain unverified.

## Files

- `../voice-workspace/CLAUDE.md` — owning context for the standalone integrated voice app;
  requirements/research/evaluation live there, while this fork remains the desktop base.
- `README.md` — upstream product behavior, installation, limitations, release verification, and
  troubleshooting; read before changing user-facing behavior or support claims.
- `BUILD.md` — canonical platform prerequisites and source-build workflow; read before building or
  changing dependencies, packaging, or release behavior.
- `windows-local-setup.md` — this machine's verified Windows toolchain, build commands, runtime
  smoke-test evidence, and remaining verification limits; read before rebuilding here.
- `voice-workspace-features.md` — active English profile, dormant Roman-Hinglish delivery, startup warm-up,
  latency diagnostics, regression evidence and limitations; read before dictation changes.
- `upstream-instructions.md` — imported upstream development contract, architecture map, commands,
  code style, platform notes, and contribution rules; follow it for all code changes.
- `AGENTS.md` — compatibility pointer to this canonical `CLAUDE.md`; do not store independent
  project context there.
- `src-tauri/` — Rust/Tauri backend, native integrations, transcription pipeline, resources, and
  platform packaging configuration.
- `src/` — React/TypeScript interface, settings, onboarding, overlay, stores, and translations.
- `tests/` — end-to-end and regression coverage.
- `.github/workflows/` — upstream quality, build, packaging, and release automation.
- `package.json` and `bun.lock` — Bun-managed frontend/tooling dependency definitions.
- `LICENSE` — upstream MIT license, which must remain with redistributed copies.

## Working agreement

- Treat `cjpais/Handy` as the read-only `upstream` remote. Publish project work only to Akshat's
  private GitHub repository unless he explicitly changes that policy.
- Preserve upstream Git history, copyright notices, attribution, and the MIT license.
- Keep implemented, locally verified, and merely documented behavior distinct. Do not claim mobile
  support; the accepted production base is the cross-platform desktop application.
- English only is the active Voice Workspace scope. Keep translation, Roman Hinglish and AI
  cleanup off in its test profile; preserve dormant converter/data for recovery, not a Hindi readiness claim.
- The initial Windows acceptance gate requires dependency installation, a successful source build,
  and a launched local process that remains alive long enough to initialize its runtime. A live
  microphone-to-text test still requires Akshat's interactive permission and speech input.
- Keep the archived WhimprFlow repository as a recoverable historical reference; Handy is the
  active desktop base.
- Follow the workspace material-change documentation gate: synchronize affected current-state
  sources, including `README.md`, `BUILD.md`, applicable release/configuration documents, and this
  status whenever code, architecture, platforms, build/release behavior, privacy, or integrations
  change.
- Whenever a new project-owned working file or top-level source area is added, add a bullet under
  `## Files` in the same edit. Generated, dependency, cache, and build-output directories do not
  need individual entries.
