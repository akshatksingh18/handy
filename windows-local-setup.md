# Windows local setup

This checkout has been built from source on Akshat's Windows x64 machine with Rust 1.98,
Visual Studio Build Tools 2026, Bun 1.4.2, Vulkan SDK 1.4.309.0, ONNX Runtime 1.24.2, and
`spirv-headers:x64-windows` from vcpkg. The SDK and native dependencies are installed below
`%LOCALAPPDATA%\handy-build-deps`; Bun is installed in the user's npm global directory. These
are local build dependencies, not tracked repository content.

## Current source and recovery

The current candidate includes final upstream source `73c095857bc4eadff097f64767c40ad129cc372d`, retaining the
English profile, opt-in full-recording recognition, manual vocabulary matching, history delivery,
startup warm-up and timing-only diagnostics. Native inference now uses upstream's supervised worker;
warm-up uses the same worker without capturing audio or publishing text. Full-recording mode
disables both live streaming and live chunking. The `upstream` remote is removed; `origin` remains.

Local pre-import history, worktree edits, and the imported source tip are retained under `.git/local-recovery/upstream-import-20261026/` and `refs/local-recovery/`. These recovery objects are local only; do not push them.

Targeted local checks pass: the full-recording policy across eight model/setting combinations,
eight converter regressions, six history-delivery checks and parsing of the four edited TypeScript
interfaces. Full frontend build, type-checking and quality checks pass at `9ddfa264`
([run 40418896227](https://github.com/akshatksingh18/handy/actions/runs/40418896227)). The Windows ARM64
release build and CPU-device probe pass at `e5e04507`
([run 40417269897](https://github.com/akshatksingh18/handy/actions/runs/40417269897)); no artifact is installed.
Subsequent application changes are locale fallbacks and formatting; the diagnostic example also
propagates the new fallible capability lookup. Full Rust tests at `9ddfa264` pass: 316 passed,
18 intentionally ignored, no failures
([run 40418893755](https://github.com/akshatksingh18/handy/actions/runs/40418893755)). The three native application
files changed after the Windows build are formatter-equivalent at `e5e04507` and `9ddfa264`;
the standalone diagnostic is compiled by the successful Rust suite. No installed executable, profile, model
or user history is replaced. The accepted idle re-warm on the separate `arm64-rewarm` branch must be ported to the supervised
worker before adopting this source, followed by live microphone/paste acceptance.
Use the current BUILD.md for native prerequisites; the recipe below describes the older x64 baseline.

## Previous x64 rebuild baseline

Install frontend dependencies with `bun install --frozen-lockfile`. The combined upstream
`bun run build` script fails here because Windows Device Guard blocks Bun's generated
`node_modules\.bin\tsc.exe` shim. Run the same two build stages directly:

```powershell
$bun = Join-Path $env:APPDATA 'npm\bun.ps1'
& $bun x tsc --pretty false
& $bun x vite build
```

Then compile the release app without the upstream CI-only signing/bundling step. The temporary
Tauri config override skips only its frontend hook because the two frontend stages were already
run above; it does not change a tracked project file.

```powershell
$deps = Join-Path $env:LOCALAPPDATA 'handy-build-deps'
$env:VULKAN_SDK = Join-Path $deps 'VulkanSDK\1.4.309.0'
$env:ORT_LIB_LOCATION = Join-Path $deps 'onnxruntime-win-x64-1.24.2\lib'
$env:ORT_PREFER_DYNAMIC_LINK = '1'
$env:CARGO_NET_OFFLINE = 'true' # use the installed dependency cache
$env:CMAKE_PREFIX_PATH = (Join-Path $deps 'vcpkg\installed\x64-windows').Replace('\', '/')
$cmakeBin = 'C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin'
$env:PATH = "$(Join-Path $env:APPDATA 'npm');$env:VULKAN_SDK\Bin;$cmakeBin;$env:PATH"
$config = '{"build":{"beforeBuildCommand":""}}'
bun x tauri build --no-bundle --ci --config $config
```

The verified release build produced `src-tauri\target\release\handy.exe` and staged 13
transcription runtime libraries plus `onnxruntime.dll` and the application resources. Launching
that executable with `--start-hidden --debug` kept it alive for 20 seconds and initialized
`%APPDATA%\com.pais.handy\settings_store.json`, `history.db`, and the WebView2 profile. The
smoke-test process was stopped afterward.

Build/startup on this Windows machine are verified. Akshat's initial portable Notepad test did
capture/transcribe/insert speech, but failed latency, Roman-Hinglish script and acronym acceptance.
Whisper Small CPU/NVIDIA/Intel inference and approved Turbo/NVIDIA inference were measured locally;
model integrity and aggregate results are owned by `../voice-workspace/setup.md` and
`../voice-workspace/performance.md`. Turbo's updated live test still took 3–4 seconds and Hindi
output failed; active scope is now English only. Parakeet Unified EN Q8 is the faster candidate,
with the old converter disabled. Akshat accepts this profile's live English speed; no exact
new latency or p95 is reported. Its later live E03 test failed PostgreSQL and punctuation.
Akshat reports the deployed full-recording/three-term retest's technical words/punctuation worked
perfectly, cleanup off. Broader accuracy, quantitative latency and insertion/reliability,
spoken self-correction and signed packaging remain open. No cleanup model is configured.
The full-recording quality update's offline Tauri release build passed with embedded production
assets; all 14 runtime DLL hashes match. The deployed app passes transcript-free headless batch
inference on the saved synthetic E03 clip. The quality GUI remains alive for at least 60 seconds
after NVIDIA load and batch priming. The user-reported quality retest is a one-fixture qualitative
pass, not the full microphone/paste/reliability suite.
TypeScript/frontend build, local ESLint and native diagnostic/regression checks passed. Current
executable/model hashes and timings remain owned by the linked Voice Workspace documents.
Fork-specific implemented behavior and checks are indexed in voice-workspace-features.md.
The upstream [BUILD.md](BUILD.md) remains the general cross-platform guide.

Use the documented Tauri build rather than bare `cargo build --release` for the final GUI:
Tauri enables the embedded production asset protocol; a bare Cargo release can still have
the development URL selected. The formatter/Clippy components were absent from this minimal
Rust toolchain; do not claim those checks passed without installing/running them.

## Windows ARM64 build (GitHub Actions)

Akshat's second laptop is a Snapdragon X running Windows 11 ARM64 with no Rust, Bun or Visual
Studio toolchain, so its `handy.exe` is built in the cloud, not here.
`.github/workflows/windows-arm64-build.yml` is a manual (`workflow_dispatch`) job on the
`windows-11-arm` runner with a read-only token, no secrets, no signing and no release upload. It
pins Bun 1.4.2, Rust 1.98.0 and SHA-pinned actions, builds the static CPU-only
`aarch64-pc-windows-msvc` target with `tauri build --no-bundle`, checks that `--list-devices` shows a
CPU and no GPU, and uploads `handy.exe`, the staged ARM64 VC++ runtime DLLs and `SHA256SUMS.txt` as a
7-day artifact. The ARM64 build has no GPU backend and no sibling `ggml*`/`transcribe` DLLs.

Status: the first run on `main` (commit `12ee212`, run 40252857590) passed. The `build` job took
27 min 34 s on a cold Rust cache (about 24 min of it the release compile) with Rust 1.98.0, Bun
1.4.2 and clang-cl 22.1.8. `handy.exe --list-devices` reported one device, `kind=cpu` (Cobalt 100
on the runner). The artifact (19,166,163 bytes) holds `handy.exe` (SHA-256 `61170cd9…6bad`), the
staged ARM64 VC++ runtime DLLs and `SHA256SUMS.txt`. On the Snapdragon X laptop the exe runs
without an application-control block, binds the Oryon CPU and passed a live Notepad dictation test;
Voice Workspace's `setup.md` and `performance.md` own those results. The workflow also builds
branches: run 40412889343 built `arm64-rewarm` (commit `ff21566`, idle re-warm) in 13 min 18 s on
a warm cache, `handy.exe` SHA-256 `abe07448…3e91`, DLLs unchanged; that exe is now deployed on the
Snapdragon laptop. Pushing a non-`main` branch triggers no workflow. A push to `main` also triggers
`main-build.yml` (macOS, Linux and Windows builds); put `[skip ci]` in the commit message when
pushing only workflow or documentation changes.

## Local ASR diagnostic

`src-tauri/examples/asr_latency.rs` benchmarks batch or real-time-paced native streaming without
microphone, VAD, history or paste. Optional local vocabulary scoring uses the app text matcher
separately from raw ASR. It emits aggregate JSON only, never recognized
text. The tested build on this machine linked the existing release `transcribe_cpp`, `hound` and
`serde_json`, `natural`, `once_cell`, `regex` and `strsim` rlibs with `rustc`, plus the existing native
`transcribe.lib`; no dependency download was required. All 48 standalone diagnostic/text-filter
regressions pass, including critical tokens, dictionary prefix preservation and raw punctuation
boundaries. Optional gold/vocabulary inputs and audio stay local; no transcript text is emitted.

For a conventional rebuild after configuring the same native environment above:

```powershell
Set-Location 'D:\AI Important Files\personal-project\handy\src-tauri'
cargo build --release --offline --example asr_latency
```

This conventional example command has not separately been exercised here; it can compile the
project library as well and take longer. It is not the final GUI build command. Copy the diagnostic
exe from `target/release/examples/` (or the tested direct-rustc output `target/release/asr_latency.exe`)
beside the matching isolated runtime DLLs, named `asr_latency.exe`, then use Voice Workspace's
`scripts/benchmark-asr.ps1`. Do not mix native/library versions. Use explicit device name and inspect
returned `bound_device`: process-local indices/backend labels can change. Gold fixtures and WAVs
stay ignored/local. A synthetic TTS result does not certify user-accent accuracy or actual insertion.
