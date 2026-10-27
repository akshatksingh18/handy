# Voice Workspace Desktop

Akshat's desktop dictation app: the Tauri (Rust + React) application behind the Voice Workspace
project. It started from the MIT-licensed Handy app and is now maintained as its own project, with
its own name ("Voice Workspace"), icons, CI and release process. There is no upstream remote, no
updater and no runtime download from Handy's servers; nothing pulls Handy's code in automatically.
Product requirements, evaluation and test results live in `../voice-workspace/`.

**Status:** Active desktop app, Windows-first. `main` contains Handy's final imported source
(`73c09585`), this project's dictation features and the standalone makeover. The Snapdragon X
laptop runs the CI-built ARM64 exe from `main`; the combined live retest (idle re-warm, early
phrases, rename) is the next acceptance step. Both laptops install GitHub Releases (ARM64 and
x64 zips) with Voice Workspace's `install-desktop.ps1`; neither needs a local compiler. The x64
laptop still runs an older build until that script is run there.

## Files

- `../voice-workspace/CLAUDE.md` — owning context for requirements, evaluation and test results.
- `README.md` — public entry point: what the app does, platforms, build/run, credits.
- `development.md` — architecture map, commands, code conventions and the library pins still
  maintained by Handy's authors; read before code changes.
- `BUILD.md` — platform prerequisites and source builds; read before building or changing
  dependencies or packaging.
- `windows-local-setup.md` — CI workflows, Windows build recipes, verified runs and limits; read
  before rebuilding.
- `voice-workspace-features.md` — this project's dictation features (English profile,
  full-recording mode, idle re-warm, early phrases, diagnostics) and their evidence; read before
  dictation changes.
- `docs/troubleshooting.md` — platform notes and troubleshooting carried over from Handy's README;
  the in-app Linux and Secure Input help links point here.
- `src-tauri/` — Rust/Tauri backend, transcription pipeline, packaging config and icons.
- `src/` — React/TypeScript interface; `src/lib/model-catalog/` is the vendored model catalog
  library (see its README for provenance and license).
- `tests/` — Playwright smoke tests.
- `.github/workflows/` — code quality, Rust tests (on Linux), Playwright, the manual Windows
  ARM64 and x64 builds, and `release.yml`, which builds both and publishes a GitHub Release.
- `package.json` and `bun.lock` — Bun-managed frontend dependencies.
- `LICENSE` — MIT license with Handy's original copyright notice; keep it.

## Working agreement

- Follow the workspace and `personal-project` CLAUDE chain. Publish only to Akshat's public
  `akshatksingh18/voice-workspace-desktop` repository; review every push as public content.
- Keep the MIT license, the original copyright notice and the Handy credit in README and the
  About page. Do not reintroduce Handy's name, logo or brand assets as this app's branding.
- No upstream remote and no automatic upstream sync. Pulling a later Handy change is a manual,
  reviewed port. Library pins (`transcribe-cpp`, `handy-recorder`, the `cjpais` git pins) change
  only on purpose; `development.md` lists them.
- The portable marker text stays `Handy Portable Mode` so existing portable `Data/` folders keep
  loading; do not change it without a migration. Other internal "handy" names are also kept on
  purpose because nothing user-visible depends on them and renaming risks existing data:
  the `handy_app_lib` crate name, `HANDY_*` environment variables, the `handy_keys` module,
  `handy-*.wav` recording names, the `handy.png` tray resource names and the x64 laptop's
  `%LOCALAPPDATA%\handy-build-deps` folder. The pink palette is inherited and kept; only the logos
  are this project's own.
- Full builds and test suites run on GitHub Actions; locally run only quick targeted checks.
- Releases: bump the version in `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml` (and its
  lock entry) and `package.json` together, then run `release.yml`; it refuses an existing tag.
  Release zips are public and must never contain settings, models, history or recordings.
- English only is the active scope. Translation, Roman Hinglish and AI cleanup stay off in the
  test profile; keep the dormant converter for recovery, not as a Hindi readiness claim.
- Keep implemented, CI-verified and live-tested behavior distinct. A live microphone/paste test
  needs Akshat's speech; record aggregate results in the Voice Workspace documents.
- Whenever a new project-owned working file or top-level source area is added, add it under
  `## Files` in the same edit. Synchronize README, BUILD, development and setup documents when
  code, build, privacy or platform behavior changes.
