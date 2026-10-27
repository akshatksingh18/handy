//! Early phrase recognition for the full-recording mode: each finished phrase
//! is decoded at a pause while the key is still held, so release only
//! decodes the audio after the last pause.
//!
//! The recorder passes only frames its voice detector keeps, so a pause shows
//! up as frames that stop arriving. The frames fed here are exactly the
//! recording's samples; `finish` checks that before trusting the result.

use std::time::{Duration, Instant};

use log::info;
use transcribe_cpp::{RunOptions, Transcript};

use super::TranscriptionManager;
use crate::engine_supervisor::{CancelMark, EngineError};

const RATE: usize = 16_000;
/// No frame for this long (on top of the voice detector's hangover) is a pause.
const PAUSE: Duration = Duration::from_millis(200);
/// How often the stream worker checks for a pause while no frames arrive.
pub(super) const POLL: Duration = Duration::from_millis(50);
/// A phrase is decoded early only once it holds this much audio, so short
/// hesitations do not split sentences into fragments.
const MIN_PHRASE_SAMPLES: usize = 3 * RATE;
/// A very short final phrase is padded with silence to this length, as a
/// short recording is.
const MIN_TAIL_SAMPLES: usize = RATE * 5 / 4;

pub(super) struct PhraseDecode<'a> {
    tm: &'a TranscriptionManager,
    /// Every phrase's run stops with one cancel.
    mark: CancelMark,
    run_options: RunOptions,
    model_id: String,
    audio: Vec<f32>,
    /// Samples already decoded into `transcripts`.
    decoded: usize,
    transcripts: Vec<Transcript>,
    early_decode: Duration,
    last_feed: Instant,
    /// The first run that failed or was cancelled; nothing more is decoded.
    failure: Option<EngineError>,
}

impl<'a> PhraseDecode<'a> {
    pub(super) fn new(
        tm: &'a TranscriptionManager,
        mark: CancelMark,
        run_options: RunOptions,
        model_id: &str,
    ) -> Self {
        Self {
            tm,
            mark,
            run_options,
            model_id: model_id.to_string(),
            audio: Vec::new(),
            decoded: 0,
            transcripts: Vec::new(),
            early_decode: Duration::ZERO,
            last_feed: Instant::now(),
            failure: None,
        }
    }

    pub(super) fn push(&mut self, pcm: Vec<f32>) {
        self.audio.extend_from_slice(&pcm);
        self.last_feed = Instant::now();
    }

    /// Called when no frame arrived for a poll interval: decodes the phrase
    /// so far if the speaker paused after enough audio.
    pub(super) fn tick(&mut self) {
        if self.failure.is_some()
            || self.audio.len() - self.decoded < MIN_PHRASE_SAMPLES
            || self.last_feed.elapsed() < PAUSE
        {
            return;
        }
        let started = Instant::now();
        let phrase = self.audio[self.decoded..].to_vec();
        match self.run(phrase) {
            Ok(transcript) => {
                self.transcripts.push(transcript);
                self.decoded = self.audio.len();
                self.early_decode += started.elapsed();
            }
            Err(e) => {
                info!("Early phrase decoding stopped: {}", e);
                self.failure = Some(e);
            }
        }
    }

    /// Decodes the last phrase. Every phrase's transcript in order; `Ok(None)`
    /// when the result can't be trusted and the recording should be decoded
    /// whole instead; `Err(Cancelled)` when the user cancelled.
    pub(super) fn finish(
        mut self,
        recorded_samples: usize,
    ) -> Result<Option<Vec<Transcript>>, EngineError> {
        match self.failure.take() {
            Some(EngineError::Cancelled) => return Err(EngineError::Cancelled),
            Some(_) => return Ok(None),
            None => {}
        }
        if self.audio.len() != recorded_samples {
            info!(
                "Early phrases saw {} of the recording's {} samples; transcribing it whole",
                self.audio.len(),
                recorded_samples
            );
            return Ok(None);
        }
        let started = Instant::now();
        let mut tail = self.audio.split_off(self.decoded);
        let tail_samples = tail.len();
        if !tail.is_empty() {
            if tail.len() < MIN_TAIL_SAMPLES {
                tail.resize(MIN_TAIL_SAMPLES, 0.0);
            }
            match self.run(tail) {
                Ok(transcript) => self.transcripts.push(transcript),
                Err(EngineError::Cancelled) => return Err(EngineError::Cancelled),
                Err(e) => {
                    info!(
                        "Early phrases' last phrase failed ({}); transcribing whole",
                        e
                    );
                    return Ok(None);
                }
            }
        }
        info!(
            "early_chunks count={} early_audio_ms={} early_decode_ms={} tail_audio_ms={} tail_decode_ms={}",
            self.transcripts.len().saturating_sub(usize::from(tail_samples > 0)),
            self.decoded * 1000 / RATE,
            self.early_decode.as_millis(),
            tail_samples * 1000 / RATE,
            started.elapsed().as_millis()
        );
        Ok(Some(self.transcripts))
    }

    fn run(&self, pcm: Vec<f32>) -> Result<Transcript, EngineError> {
        self.tm
            .run_batch(self.mark, pcm, &self.run_options, &self.model_id)
    }
}
