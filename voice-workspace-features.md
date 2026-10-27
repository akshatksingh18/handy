# Voice Workspace dictation features

This project's own dictation behavior on top of the code inherited from Handy. Test results and
timings belong in `../voice-workspace/performance.md` and `../voice-workspace/setup.md`; this file
says what is implemented and how it was checked. The integrated notes/meeting app is still planned;
no restricted Granola/Natively code is used here.

## Active profile

English only, Parakeet Unified EN 0.6B Q8. English is selected explicitly; Roman Hinglish,
translation and AI cleanup are off. The Hindi/Hinglish requirement is superseded; the converter
stays dormant for recovery, and its synthetic tests never proved acoustic or meaning accuracy.

## Implemented behavior

- **Settings (General → Voice Workspace):** _Use full-recording recognition_, _Hindi in Roman
  Hinglish_ and _Warm model on startup_, all off by default. The test profile turns on
  full-recording mode and warm-up, not Roman conversion.
- **Full-recording mode** (`prefer_batch_transcription`) recognizes the whole recording after
  release, with offline VAD and no live preview. It suppresses live streaming and upstream's
  long-form live chunking (`policy.rs`, unit-tested over every model/setting combination). Restart
  after changing it so warm-up primes the chosen path.
- **Startup warm-up** (`preload_model`) loads the selected model and runs one second of discarded
  silence through the batch or stream path before reporting loading complete. It never opens the
  microphone or stores the result. INFO logs `dictation_warmup_ms`.
- **Idle re-warm:** a dictation that starts after 15 s without use (warm-up on) repeats that
  warm-up in a background thread while the user speaks. It queues on the engine like any other
  run, so a release during it waits rather than fails; a failure only costs the speed it would
  have saved. INFO logs `dictation_rewarm_ms` and `idle_s`, no content. Live result on the
  previous engine: first dictation after 26.8 min idle 0.743 s (was 3.192 s).
- **Early phrase recognition** (opt-in `early_chunk_recognition`, no UI toggle; needs
  full-recording mode, VAD on and Model Unload not _Immediately_): a third live decoder beside
  upstream's stream and long-form chunk decoders (`transcription/phrases.rs`). The recorder passes
  only frames its VAD keeps, so frames stopping for 200 ms (after the 450 ms VAD hangover) mark a
  pause. Once at least 3 s is undecoded, that phrase is recognized while the key is still held;
  release decodes only the rest. Phrases join like long-form chunks. If the frames seen differ
  from the recording, or a phrase fails, the whole recording is transcribed instead; Escape
  cancels like the other live decoders (a phrase decoding at that moment is stopped by restarting
  the engine worker, so the next dictation reloads the model). INFO logs `early_chunks` counts and
  audio/decode milliseconds, no content. Possible side effect: a cut mid-sentence can add a period.
- **Custom Words** stay post-ASR fuzzy matching on Parakeet. A prefix fix keeps ordinary preceding
  words when a better suffix match exists ("a post-Gree SQL" → "a PostgreSQL"). The test profile
  seeds PostgreSQL, Kubernetes and REST API at threshold 0.18; this is manual, not learning.
- **Roman Hinglish (dormant):** local Hindi-script conversion before and after optional cleanup,
  fail-closed on unsupported symbols, never combined with translation. History shows delivery
  text; _Show raw recognition (recovery only)_ reveals the preserved ASR.
- **Diagnostics:** release INFO logs `dictation_latency` (capture, recognition, output, paste and
  stop-to-paste ms), never transcript text. Headless `--stats-only --json` omits text;
  `--list-devices --json` gives stable device selectors. `src-tauri/examples/asr_latency.rs` is a
  content-free native benchmark (batch or paced streaming, optional local gold/vocabulary scoring);
  it does not cover the GUI, microphone, VAD, history or paste.

## Verification

At commit `a61d1b18` (the feature port) the full Rust suite passed
([run 40485230453](https://github.com/akshatksingh18/voice-workspace-desktop/actions/runs/40485230453))
and the Windows ARM64 release build with its CPU-only device check passed
([run 40485233651](https://github.com/akshatksingh18/voice-workspace-desktop/actions/runs/40485233651)).
The standalone makeover then passed Rust tests, code quality (lint, format, model-language
coverage, keyboard and history-delivery tests, production build) and Playwright; exact runs and
the deployed exe are in `windows-local-setup.md`. Re-warm and early phrases are covered by the
policy unit tests and compile checks only; their behavior on this engine is not yet live-tested.

Earlier evidence still applies to unchanged behavior: eight converter tests, six history-delivery
checks and 48 diagnostic/text-filter regressions pass. Akshat accepted the Parakeet profile's live
English speed and reported the full-recording/three-term retest's technical words and punctuation
correct (one fixture, cleanup off). On the Snapdragon laptop, list/clause commas and sentence
breaks inside one hold remain weak. Nothing here claims guaranteed instant dictation, complete
accuracy, Hindi readiness, automatic learning, meeting integration, macOS/Linux verification or
signed packaging.

## Spoken correction boundary

There is no rule that turns "Let's meet at 7 pm. Sorry, let's meet at 6 pm." into one corrected
sentence. The pipeline finalizes ASR, handles output/history, then pastes once. The proposed first
slice is same-recording correction before paste, with raw recovery and latency tests; editing a
previous insertion is a separate target-validated mode. Optional LLM cleanup exists but is not
configured; the separate local Qwen3-0.6B experiment failed time-repair and clause checks, so no
local processor is enabled. Design and fixtures live in the Voice Workspace documents.

## Maintenance

Converter source: `src-tauri/src/audio_toolkit/hinglish.rs`. Keep fixtures synthetic and add
spellings only as exact entries. Do not re-enable it for English-only work, translate Hindi, or
enable a cloud processor. Model downloads and redistribution are separate license decisions.
