# Voice Workspace dictation changes

These changes are maintained in Akshat's private Handy fork. The integrated notes/meeting
application is still planned; no restricted Granola/Natively code is used here.

## Implemented behavior

- General Settings → Voice Workspace exposes **Hindi in Roman Hinglish** and **Warm model on
  startup**. Both default off for existing upstream users; the isolated Voice Workspace profile
  explicitly enables them. No new inference dependency or server is required.
- Hindi script is converted locally before optional cleanup and validated/converted again after
  cleanup. Already-English text is copied verbatim. This is not English translation or an ASR
  correction model. Common chat spellings and exact technical loanwords have a small explicit
  dictionary; unknown words use conservative letter transliteration and can sound unnatural.
- Translation to English cannot be enabled with Roman Hinglish. Unsupported Hindi-script symbols
  produce an error instead of a silent Devanagari paste. The original recording/raw recognition
  remains local for retry/recovery under the existing retention policy.
- History displays/copies delivery text by default. Old/error Hindi entries require retry; an
  explicit **Show raw recognition (recovery only)** reveals the preserved ASR instead.
- Startup preload loads the selected downloaded speech model asynchronously and primes Whisper
  with discarded synthetic silence before reporting loading complete. It does not open
  the microphone, deliver/store the synthetic result or fetch models. Model Unload → Never avoids repeated idle model loads; it
  consumes RAM/VRAM and is appropriate only for this measured test profile. Startup itself takes
  longer; wait for loading to finish. Warm-up failures are logged and can leave first-run latency.
- Release INFO logs contain `dictation_latency` with capture, recognition, output, paste and
  stop-to-paste milliseconds, never transcript content in that new metric. Keep debug mode off.
- Headless `--stats-only --json` omits transcript text; `--list-devices --json` exposes stable GPU
  selectors. File transcription also honors Roman Hinglish. No mic, paste, history entry or LLM
  call is made by that headless benchmark path.

## Verification and limitations

Eight native converter tests cover mixed text, H01–H07 synthetic fixtures, English identifiers,
negation/numbers, conjunct/nukta/digits, exact loanwords, fail-closed symbols and idempotence.
Six history delivery regression checks, TypeScript, frontend build and changed-file ESLint pass.
The updated Tauri Windows release build, real NVIDIA/Turbo inference and script delivery checks
pass, as do timing-only output suppression and GUI startup/priming/settings persistence.
See setup/performance for the deployed executable hash and individual timings.
The converter microbenchmark averages about 5 microseconds on this laptop for the short synthetic
fixture; this is not a recognition or insertion measurement.

The original CPU profile failed Akshat's interactive acceptance: text appeared in Notepad, but
Hindi used Devanagari, a technical acronym was misheard, and release-to-text exceeded 10 seconds.
Aggregate device/model measurements and current activation decisions belong in
`../voice-workspace/performance.md` and `../voice-workspace/setup.md`.
The updated runtime still requires a new live microphone/paste test. Nothing here claims instant
dictation, complete Hindi spelling/recognition accuracy, auto-learning, meeting integration,
cross-platform device verification or signed packaging.

## Maintenance

Converter source: `src-tauri/src/audio_toolkit/hinglish.rs`. Keep fixtures synthetic; add spelling
preferences only as explicit exact entries, not fuzzy substitutions that could change meaning.
The delivery stage is independent of cleanup. Do not silently replace a failed conversion with
raw script, translate Hindi, infer missing spoken words, or enable a cloud processor.
Model downloads and model redistribution are separate authorization/license decisions.
