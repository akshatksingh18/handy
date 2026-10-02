# Voice Workspace dictation changes

These changes are maintained in Akshat's private Handy fork. The integrated notes/meeting
application is still planned; no restricted Granola/Natively code is used here.

## Implemented behavior

The active Voice Workspace profile is **English only**, with Parakeet Unified EN Q8/NVIDIA as
the test candidate. English is selected explicitly; Roman Hinglish, translation and AI cleanup
are off. The user's updated Turbo live test still took 3–4 seconds and Hindi was delivered in
English, so the earlier Hindi/Hinglish requirement is superseded. The converter below remains
dormant/recoverable; its synthetic script tests were not a successful acoustic/meaning test.

- General Settings → Voice Workspace exposes **Hindi in Roman Hinglish** and **Warm model on
  startup**. Both default off for existing upstream users; the isolated Voice Workspace profile
  enables warm-up only. No new inference dependency or server is required.
- Hindi script is converted locally before optional cleanup and validated/converted again after
  cleanup. Already-English text is copied verbatim. This is not English translation or an ASR
  correction model. Common chat spellings and exact technical loanwords have a small explicit
  dictionary; unknown words use conservative letter transliteration and can sound unnatural.
- Translation to English cannot be enabled with Roman Hinglish. Unsupported Hindi-script symbols
  produce an error instead of a silent Devanagari paste. The original recording/raw recognition
  remains local for retry/recovery under the existing retention policy.
- History displays/copies delivery text by default. When Roman mode is explicitly enabled,
  old/error Hindi entries require retry; an
  explicit **Show raw recognition (recovery only)** reveals the preserved ASR instead.
- Startup preload loads the selected downloaded speech model asynchronously and primes native
  batch or streaming inference according to its runtime capabilities
  with discarded synthetic silence before reporting loading complete. It does not open
  the microphone, deliver/store the synthetic result or fetch models. Model Unload → Never avoids repeated idle model loads; it
  consumes RAM/VRAM and is appropriate only for this measured test profile. Startup itself takes
  longer; wait for loading to finish. Warm-up failures are logged and can leave first-run latency.
- Release INFO logs contain `dictation_latency` with capture, recognition, output, paste and
  stop-to-paste milliseconds, never transcript content in that new metric. Keep debug mode off.
- Headless `--stats-only --json` omits transcript text; `--list-devices --json` exposes stable GPU
  selectors. File transcription also honors Roman Hinglish. No mic, paste, history entry or LLM
  call is made by that headless benchmark path.
- `src-tauri/examples/asr_latency.rs` is a standalone, content-free native diagnostic: explicit
  device name/index, batch or real-time-paced streaming, optional local gold-text scoring. It uses
  cached native dependencies, prints actual bound device and aggregate timings/errors, never raw
  recognized text. It does not benchmark the complete GUI, mic, VAD, filters, clipboard or paste.

## Verification and limitations

Eight native converter tests cover mixed text, H01–H07 synthetic fixtures, English identifiers,
negation/numbers, conjunct/nukta/digits, exact loanwords, fail-closed symbols and idempotence.
Six history delivery regression checks, TypeScript, frontend build and changed-file ESLint pass.
The updated Tauri Windows release build, real NVIDIA/Turbo inference and script delivery checks
pass, as do timing-only output suppression and GUI startup/priming/settings persistence.
See setup/performance for the deployed executable hash and individual timings.
The English optimization also passes the TypeScript/frontend/native Tauri release build, local
ESLint, standalone scoring regression and retained eight converter regressions. The new runtime
loads/primes streaming-capable Parakeet on the persisted NVIDIA selector and passes timing-only
headless inference. First-run and vocabulary accuracy limits remain recorded in performance.md.
The converter microbenchmark averages about 5 microseconds on this laptop for the short synthetic
fixture; this is not a recognition or insertion measurement.

The original CPU profile failed Akshat's interactive acceptance: text appeared in Notepad, but
Hindi used Devanagari, a technical acronym was misheard, and release-to-text exceeded 10 seconds.
Aggregate device/model measurements and current activation decisions belong in
`../voice-workspace/performance.md` and `../voice-workspace/setup.md`.
The updated Turbo live test failed latency/Hindi acceptance. Akshat accepts the selected
Parakeet profile's live English speed, but has not tested technical words. This is qualitative
user acceptance, not a measured new latency/p95 or a completed insertion/reliability suite.
Nothing here claims guaranteed instant dictation, maximum English accuracy, Hindi readiness,
auto-learning, meeting integration,
cross-platform device verification or signed packaging.

## Spoken correction boundary

The active profile has AI cleanup off and Custom Words empty. Advanced Settings → Custom Words
supports manual vocabulary; this Parakeet path applies fuzzy post-ASR matching, not acoustic
prompting or automatic learning. Optional LLM post-processing exists, but its default prompt
does not explicitly resolve spoken backtracking and it is not configured for this profile.

There is no dedicated rule that turns "Let's meet at 7 pm. Sorry, let's meet at 6 pm." into one
corrected sentence. The current pipeline finalizes ASR, handles output/history, then pastes once;
streaming text does not continuously overwrite the external editor. The proposed first slice is
same-recording correction before paste, with raw recovery, conservative matching and latency tests.
Editing a previous insertion is a separate target-validated mode, not blind deletion or monitoring
of unrelated typing. Requirements/design/fixtures belong in the sibling Voice Workspace documents.

## Maintenance

Converter source: `src-tauri/src/audio_toolkit/hinglish.rs`. Keep fixtures synthetic; add spelling
preferences only as explicit exact entries, not fuzzy substitutions that could change meaning.
The dormant delivery stage is independent of cleanup. Do not re-enable it for English-only work.
If later explicitly enabled, do not silently replace a failed conversion with
raw script, translate Hindi, infer missing spoken words, or enable a cloud processor.
Model downloads and model redistribution are separate authorization/license decisions.
