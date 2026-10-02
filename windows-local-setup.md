# Windows local setup

This checkout has been built from source on Akshat's Windows x64 machine with Rust 1.98,
Visual Studio Build Tools 2026, Bun 1.4.2, Vulkan SDK 1.4.309.0, ONNX Runtime 1.24.2, and
`spirv-headers:x64-windows` from vcpkg. The SDK and native dependencies are installed below
`%LOCALAPPDATA%\handy-build-deps`; Bun is installed in the user's npm global directory. These
are local build dependencies, not tracked repository content.

## Rebuild on this machine

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
`../voice-workspace/performance.md`. Updated live acceptance and signed packaging remain open.
Fork-specific implemented behavior and checks are indexed in voice-workspace-features.md.
The upstream [BUILD.md](BUILD.md) remains the general cross-platform guide.

Use the documented Tauri build rather than bare `cargo build --release` for the final GUI:
Tauri enables the embedded production asset protocol; a bare Cargo release can still have
the development URL selected. The formatter/Clippy components were absent from this minimal
Rust toolchain; do not claim those checks passed without installing/running them.
