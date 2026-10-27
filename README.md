# Voice Workspace

A private, offline desktop dictation app. Hold a shortcut, speak, let go, and
the text is pasted where your cursor is. Speech is recognized on your own
computer; nothing is uploaded.

This is the desktop app of Akshat's Voice Workspace project. It started from
[Handy](https://github.com/cjpais/Handy) by CJ Pais (MIT License) and is now
maintained on its own, with its own name, icons and release process. It no
longer tracks Handy's code or downloads anything from Handy's servers.

## What it does

- **Push-to-talk dictation** into any app, with Escape to cancel.
- **Local speech recognition** through
  [transcribe.cpp](https://github.com/handy-computer/transcribe.cpp) and ggml.
  The main model is Parakeet Unified EN 0.6B; Whisper and other models can be
  downloaded from Hugging Face inside the app.
- **Full-recording mode** recognizes the whole recording after release, which
  keeps technical words and punctuation more accurate than live streaming.
- **Fast first dictation after idle:** pressing the shortcut after 15 s
  without dictation warms the model while you speak.
- **Early phrase recognition** (opt-in, `early_chunk_recognition`): in a long
  dictation, each finished phrase is recognized at a pause while you are still
  holding the key, so release only has the last phrase left to process.
- **Custom words** for names and technical terms, and a local history with
  copy and re-transcribe.
- Optional AI cleanup through a provider you configure; off by default.

The interface is English only. There is no self-updater: new versions are
built on GitHub Actions and installed by hand.

## Platforms

Windows is the tested platform, on two laptops: x64 with an NVIDIA GPU
(Vulkan) and Snapdragon X ARM64 (CPU). The macOS and Linux code is kept from
Handy but neither is a target: CI compiles and unit-tests the app on Linux,
nothing is packaged or run there, and macOS is not built.

## Build and run

- **Windows (either laptop):** each
  [GitHub Release](https://github.com/akshatksingh18/voice-workspace-desktop/releases)
  has `voice-workspace-windows-arm64.zip` (Snapdragon) and
  `voice-workspace-windows-x64.zip` (Intel/AMD, NVIDIA through Vulkan), each a
  ready-to-run app folder, plus `SHA256SUMS.txt`. Voice Workspace's
  `scripts/install-desktop.ps1` picks the right one, checks it and installs it
  with a rollback copy. A new release is made by bumping the version and
  running `release.yml`; `windows-arm64-build.yml` and `windows-x64-build.yml`
  build one-off test copies.
- **Other targets and development:** see [BUILD.md](BUILD.md) and
  [windows-local-setup.md](windows-local-setup.md).
- **Portable mode:** a file named `portable` next to the exe keeps settings,
  models, history and logs in a `Data` folder beside it.

Pushes to `main` run code-quality checks and Rust tests; Playwright runs on
pull requests or by hand.

## Documentation

- [docs/troubleshooting.md](docs/troubleshooting.md): platform notes, paste
  and model-download problems, command-line flags and debug mode.
- [development.md](development.md): architecture map and coding conventions.
- [voice-workspace-features.md](voice-workspace-features.md): this project's
  own features and their test evidence.

## License and credits

MIT License; see [LICENSE](LICENSE). The original copyright notice is kept.

- [Handy](https://github.com/cjpais/Handy) by CJ Pais and contributors, the
  app this project started from.
- The model catalog library in `src/lib/model-catalog/` comes from
  `@handy-computer/models` (MIT).
- [ggml](https://github.com/ggml-org/ggml) and transcribe.cpp for on-device
  inference, Whisper by OpenAI, Parakeet by NVIDIA, Silero for voice activity
  detection, and [Tauri](https://tauri.app) for the app framework.
