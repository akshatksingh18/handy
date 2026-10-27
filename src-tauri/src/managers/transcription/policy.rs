//! How audio is decoded: the one place the model's traits and the settings
//! choose between streaming, long-form chunking and a single run.

use crate::engine_supervisor::LoadedInfo;
use crate::settings::{AppSettings, ModelUnloadTimeout};

/// What a model can do that decides how its audio is decoded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModelTraits {
    /// Decodes audio incrementally as it is recorded.
    pub streaming: bool,
    /// Keeps its accuracy on audio longer than its native window in one run
    /// (whisper today).
    pub long_form: bool,
}

impl ModelTraits {
    /// The loaded model's own traits: the authority once a model is loaded.
    pub fn of(info: &LoadedInfo) -> Self {
        Self {
            streaming: info.capabilities.supports_streaming,
            long_form: info.supports_long_form,
        }
    }
}

/// How a recording is decoded while it is still going.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiveDecode {
    /// The model decodes the audio as it arrives.
    Stream,
    /// Long audio is cut at pauses and each chunk decoded once its cut is
    /// certain.
    Chunked,
    /// Full-recording mode with early chunks: each finished phrase is decoded
    /// at a pause while the key is still held.
    Phrases,
    /// Nothing until the recording stops; then it is decoded in batch.
    None,
}

/// How a whole clip is decoded: a recording without a live result, a
/// re-transcribed history entry, or a file from the CLI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatchDecode {
    /// Cut at pauses into chunks of about 30 s, each decoded in turn; a clip
    /// of 40 s or less is one chunk.
    Chunked,
    /// One run.
    Whole,
}

/// Streaming wins: a streaming model handles long audio itself. Otherwise
/// long audio is chunked unless the model keeps its accuracy on it.
pub fn live_decode(model: ModelTraits, long_form_chunking: bool) -> LiveDecode {
    if model.streaming {
        LiveDecode::Stream
    } else if long_form_chunking && !model.long_form {
        LiveDecode::Chunked
    } else {
        LiveDecode::None
    }
}

/// An explicit full-recording preference suppresses both live streams and
/// live chunks; it only decodes finished phrases early when `early_phrases`.
pub fn recording_decode(
    model: ModelTraits,
    long_form_chunking: bool,
    prefer_batch: bool,
    early_phrases: bool,
) -> LiveDecode {
    match (prefer_batch, early_phrases) {
        (true, true) => LiveDecode::Phrases,
        (true, false) => LiveDecode::None,
        (false, _) => live_decode(model, long_form_chunking),
    }
}

/// Early phrase recognition needs the full-recording mode, the voice detector
/// (a pause is a gap in the frames it passes) and a model that stays loaded
/// between phrases.
pub fn early_phrases_enabled(settings: &AppSettings) -> bool {
    settings.prefer_batch_transcription
        && settings.early_chunk_recognition
        && settings.vad_enabled
        && settings.model_unload_timeout != ModelUnloadTimeout::Immediately
}

/// Keep the opt-in full-recording path whole even when upstream chunking is enabled.
pub fn recording_batch_decode(
    model: ModelTraits,
    long_form_chunking: bool,
    prefer_batch: bool,
) -> BatchDecode {
    if prefer_batch {
        BatchDecode::Whole
    } else {
        batch_decode(model, long_form_chunking)
    }
}

/// As live, except that a streaming model takes a whole clip in one run, as
/// it did before long-form chunking existed.
pub fn batch_decode(model: ModelTraits, long_form_chunking: bool) -> BatchDecode {
    if long_form_chunking && !model.long_form && !model.streaming {
        BatchDecode::Chunked
    } else {
        BatchDecode::Whole
    }
}

#[cfg(test)]
mod voice_workspace_tests {
    use super::*;

    #[test]
    fn full_recording_does_not_stream_or_split_the_clip() {
        for streaming in [false, true] {
            for long_form in [false, true] {
                for chunking in [false, true] {
                    let model = ModelTraits {
                        streaming,
                        long_form,
                    };
                    assert_eq!(
                        recording_decode(model, chunking, true, false),
                        LiveDecode::None
                    );
                    assert_eq!(
                        recording_decode(model, chunking, true, true),
                        LiveDecode::Phrases
                    );
                    assert_eq!(
                        recording_batch_decode(model, chunking, true),
                        BatchDecode::Whole
                    );
                    for early in [false, true] {
                        assert_eq!(
                            recording_decode(model, chunking, false, early),
                            live_decode(model, chunking)
                        );
                    }
                    assert_eq!(
                        recording_batch_decode(model, chunking, false),
                        batch_decode(model, chunking)
                    );
                }
            }
        }
    }

    #[test]
    fn early_phrases_need_full_recording_vad_and_a_resident_model() {
        let mut settings = crate::settings::get_default_settings();
        settings.prefer_batch_transcription = true;
        settings.early_chunk_recognition = true;
        settings.vad_enabled = true;
        settings.model_unload_timeout = ModelUnloadTimeout::Never;
        assert!(early_phrases_enabled(&settings));

        let mut off = settings.clone();
        off.early_chunk_recognition = false;
        assert!(!early_phrases_enabled(&off));
        let mut streaming = settings.clone();
        streaming.prefer_batch_transcription = false;
        assert!(!early_phrases_enabled(&streaming));
        let mut no_vad = settings.clone();
        no_vad.vad_enabled = false;
        assert!(!early_phrases_enabled(&no_vad));
        let mut unloads = settings;
        unloads.model_unload_timeout = ModelUnloadTimeout::Immediately;
        assert!(!early_phrases_enabled(&unloads));
    }
}
