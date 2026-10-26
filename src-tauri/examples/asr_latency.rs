//! Local, content-free ASR benchmark. No microphone, network, history or paste.
//! Usage: asr_latency MODEL WAV DEVICE_INDEX_OR_NAME RUNS batch|stream [GOLD_TEXT_FILE] [VOCABULARY_FILE]
//! Streaming feeds 100 ms frames at real-time pace; release_ms includes backlog.
use std::{
    error::Error,
    time::{Duration, Instant},
};
use transcribe_cpp::{
    devices, init_backends_default, Model, ModelOptions, RunOptions, StreamOptions,
};

#[path = "../src/audio_toolkit/text.rs"]
mod text_filters;

fn critical_tokens_retained(reference: &[String], text: &str) -> bool {
    let actual = tokens(text);
    [
        "not",
        "never",
        "api",
        "postgresql",
        "kubernetes",
        "thursday",
        "friday",
        "deploy",
        "staging",
    ]
    .iter()
    .filter(|word| reference.iter().any(|token| token == **word))
    .all(|word| {
        actual
            .iter()
            .filter(|token| token.as_str() == *word)
            .count()
            == reference
                .iter()
                .filter(|token| token.as_str() == *word)
                .count()
    })
}

fn tokens(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_owned)
        .collect()
}

fn word_errors(reference: &[String], hypothesis: &[String]) -> usize {
    let mut row: Vec<usize> = (0..=hypothesis.len()).collect();
    for (i, expected) in reference.iter().enumerate() {
        let mut diagonal = row[0];
        row[0] = i + 1;
        for (j, actual) in hypothesis.iter().enumerate() {
            let above = row[j + 1];
            row[j + 1] = (above + 1)
                .min(row[j] + 1)
                .min(diagonal + usize::from(expected != actual));
            diagonal = above;
        }
    }
    row[hypothesis.len()]
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !(5..=7).contains(&args.len()) || !matches!(args[4].as_str(), "batch" | "stream") {
        return Err("Expected MODEL WAV DEVICE_INDEX_OR_NAME RUNS batch|stream [GOLD_TEXT_FILE] [VOCABULARY_FILE]".into());
    }
    let mut reader = hound::WavReader::open(&args[1])?;
    let spec = reader.spec();
    if spec.sample_rate != 16000
        || spec.channels != 1
        || spec.bits_per_sample != 16
        || spec.sample_format != hound::SampleFormat::Int
    {
        return Err("Expected 16 kHz mono 16-bit PCM WAV".into());
    }
    let pcm: Vec<f32> = reader
        .samples::<i16>()
        .map(|sample| sample.map(|s| s as f32 / 32768.0))
        .collect::<Result<_, _>>()?;
    let gold = args
        .get(5)
        .map(std::fs::read_to_string)
        .transpose()?
        .map(|text| tokens(&text));
    let vocabulary: Vec<String> = args
        .get(6)
        .map(std::fs::read_to_string)
        .transpose()?
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|word| !word.is_empty())
        .map(str::to_owned)
        .collect();
    #[cfg(target_os = "windows")]
    if std::env::var_os("VK_LOADER_LAYERS_DISABLE").is_none() {
        std::env::set_var("VK_LOADER_LAYERS_DISABLE", "~implicit~");
    }
    init_backends_default()?;
    let device_index: Option<usize> = args[2].parse().ok();
    let device = devices()
        .into_iter()
        .find(|device| {
            device_index.is_some_and(|index| device.index == Some(index))
                || device.description == args[2]
                || device.kind == args[2]
        })
        .ok_or("Unknown device index/name")?;
    let requested_device = device.description.clone();
    let started = Instant::now();
    let model = Model::load_with(
        &args[0],
        &ModelOptions {
            device: Some(device),
            ..Default::default()
        },
    )?;
    let load_ms = started.elapsed().as_millis();
    let backend = model.backend();
    let bound_device = model.device()?.description;
    let supports_streaming = model.capabilities()?.supports_streaming;
    if args[4] == "stream" && !supports_streaming {
        return Err("Model cannot stream".into());
    }
    let mut session = model.session()?;
    let opts = RunOptions {
        language: Some("en".into()),
        ..Default::default()
    };
    // Warm the same path under test, with discarded silence rather than private speech.
    let warm_started = Instant::now();
    if args[4] == "stream" {
        let mut stream = session.stream(&opts, &StreamOptions::default())?;
        stream.feed(&vec![0.0; 16000])?;
        stream.finalize()?;
    } else {
        session.run(&vec![0.0; 16000], &opts)?;
    }
    let prime_ms = warm_started.elapsed().as_millis();
    let runs: usize = args[3].parse()?;
    if !(1..=20).contains(&runs) {
        return Err("RUNS must be 1 through 20".into());
    }
    let audio_duration = Duration::from_secs_f64(pcm.len() as f64 / 16000.0);
    let mut reports = Vec::new();
    for _ in 0..runs {
        let started = Instant::now();
        let (text, compute_ms, release_ms, first_partial_ms) = if args[4] == "stream" {
            let mut stream = session.stream(&opts, &StreamOptions::default())?;
            let mut compute = Duration::ZERO;
            let mut first_partial = None;
            let mut received = 0;
            for frame in pcm.chunks(1600) {
                received += frame.len();
                let due = Duration::from_secs_f64(received as f64 / 16000.0);
                std::thread::sleep(due.saturating_sub(started.elapsed()));
                let feed_started = Instant::now();
                let update = stream.feed(frame)?;
                compute += feed_started.elapsed();
                if first_partial.is_none()
                    && (update.committed_changed || update.tentative_changed)
                    && !stream.text().full.trim().is_empty()
                {
                    first_partial = Some(started.elapsed().as_millis());
                }
            }
            let finalize_started = Instant::now();
            stream.finalize()?;
            compute += finalize_started.elapsed();
            (
                stream.text().full,
                compute.as_millis(),
                started.elapsed().saturating_sub(audio_duration).as_millis(),
                first_partial,
            )
        } else {
            let text = session.run(&pcm, &opts)?.text;
            (
                text,
                started.elapsed().as_millis(),
                started.elapsed().as_millis(),
                None,
            )
        };
        let errors = gold
            .as_ref()
            .map(|reference| word_errors(reference, &tokens(&text)));
        let critical = gold
            .as_ref()
            .map(|reference| critical_tokens_retained(reference, &text));
        let vocabulary_started = Instant::now();
        let corrected = text_filters::apply_custom_words(&text, &vocabulary, 0.18);
        let vocabulary_us = vocabulary_started.elapsed().as_micros();
        let vocabulary_errors = gold
            .as_ref()
            .map(|reference| word_errors(reference, &tokens(&corrected)));
        let vocabulary_critical = gold
            .as_ref()
            .map(|reference| critical_tokens_retained(reference, &corrected));
        reports.push(
            serde_json::json!({"compute_ms": compute_ms, "release_ms": release_ms,
            "first_partial_ms": first_partial_ms, "word_errors": errors,
            "reference_words": gold.as_ref().map(Vec::len), "critical_tokens_retained": critical,
            "nonempty": !text.trim().is_empty(), "commas": text.matches(',').count(),
            "ends_sentence": text.trim_end().ends_with(['.', '?', '!']),
            "vocabulary_us": vocabulary_us, "vocabulary_changed": corrected != text,
            "vocabulary_word_errors": vocabulary_errors,
            "vocabulary_critical_tokens_retained": vocabulary_critical}),
        );
    }
    // Intentionally never print recognized text or paths to private audio/gold.
    println!(
        "{}",
        serde_json::json!({"backend": backend, "requested_device": requested_device,
        "bound_device": bound_device, "supports_streaming": supports_streaming,
        "mode": args[4], "audio_secs": audio_duration.as_secs_f64(), "load_ms": load_ms,
        "prime_ms": prime_ms, "runs": reports})
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scoring_preserves_negation_and_ignores_case_punctuation() {
        let gold = tokens("Do NOT deploy the API.");
        assert_eq!(word_errors(&gold, &tokens("do not deploy the api")), 0);
        assert_eq!(word_errors(&gold, &tokens("do deploy the API")), 1);
        assert_eq!(
            word_errors(&tokens("one two"), &tokens("one three four")),
            2
        );
    }

    #[test]
    fn technical_vocabulary_repairs_split_term_without_rewriting_the_sentence() {
        let raw = "We should review a post-Gree SQL Kubernetes and the REST API before release";
        let vocabulary = vec!["PostgreSQL".into(), "Kubernetes".into(), "REST API".into()];
        let corrected = text_filters::apply_custom_words(raw, &vocabulary, 0.18);
        assert!(critical_tokens_retained(
            &tokens("PostgreSQL Kubernetes REST API"),
            &corrected
        ));
        assert!(corrected.contains("review a "));
        assert!(!corrected.contains(','));
    }
}
