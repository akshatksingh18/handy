//! How audio is decoded: the one place the model's traits and the settings
//! choose between streaming, long-form chunking and a single run.

use crate::engine_supervisor::LoadedInfo;

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

/// An explicit full-recording preference suppresses both live streams and live chunks.
pub fn recording_decode(
    model: ModelTraits,
    long_form_chunking: bool,
    prefer_batch: bool,
) -> LiveDecode {
    if prefer_batch {
        LiveDecode::None
    } else {
        live_decode(model, long_form_chunking)
    }
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
                    assert_eq!(recording_decode(model, chunking, true), LiveDecode::None);
                    assert_eq!(
                        recording_batch_decode(model, chunking, true),
                        BatchDecode::Whole
                    );
                    assert_eq!(
                        recording_decode(model, chunking, false),
                        live_decode(model, chunking)
                    );
                    assert_eq!(
                        recording_batch_decode(model, chunking, false),
                        batch_decode(model, chunking)
                    );
                }
            }
        }
    }
}
