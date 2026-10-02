//! Offline, deterministic Hindi-script conversion. This is not translation or
//! an ASR correction model: unknown words use conservative transliteration.
//! English spans are copied verbatim; unsupported script symbols fail closed.

pub fn romanize_hindi(text: &str) -> Result<String, String> {
    let mut output = String::with_capacity(text.len());
    let mut word = String::new();
    let flush = |word: &mut String, output: &mut String| -> Result<(), String> {
        if word.is_empty() {
            return Ok(());
        }
        let mut roman = romanize_word(word)?;
        let sentence_start = output.trim_end().is_empty()
            || output.trim_end().ends_with(['.', '!', '?', '\n']);
        if sentence_start && roman.starts_with(|c: char| c.is_ascii_lowercase()) {
            roman[..1].make_ascii_uppercase();
        }
        output.push_str(&roman);
        word.clear();
        Ok(())
    };
    for c in text.chars() {
        if ('\u{0900}'..='\u{097f}').contains(&c) && !matches!(c, '।' | '॥') {
            word.push(c);
        } else if matches!(c, '\u{200c}' | '\u{200d}') && !word.is_empty() {
            // Shaping-only marks inside a Hindi conjunct.
        } else {
            flush(&mut word, &mut output)?;
            output.push(if matches!(c, '।' | '॥') { '.' } else { c });
        }
    }
    flush(&mut word, &mut output)?;
    Ok(output)
}

fn romanize_word(word: &str) -> Result<String, String> {
    // Common chat spellings and unambiguous technical loanwords, not fuzzy
    // substitutions. In particular, never guess API from an unrelated word.
    let common = match word {
        "आज" => Some("aaj"), "कल" => Some("kal"),
        "की" => Some("ki"), "कि" => Some("ki"), "के" => Some("ke"),
        "को" => Some("ko"), "का" => Some("ka"), "कर" => Some("kar"),
        "करना" => Some("karna"), "करो" => Some("karo"), "करेंगे" => Some("karenge"),
        "मत" => Some("mat"), "नहीं" | "नही" => Some("nahi"),
        "पहले" => Some("pehle"), "बाद" => Some("baad"), "पर" => Some("par"),
        "लो" => Some("lo"), "तो" => Some("to"), "और" => Some("aur"),
        "है" => Some("hai"), "हैं" => Some("hain"), "हूँ" | "हूं" => Some("hoon"),
        "था" => Some("tha"), "थी" => Some("thi"), "हो" => Some("ho"),
        "में" => Some("mein"), "मैं" => Some("main"), "मुझे" => Some("mujhe"),
        "मेरा" => Some("mera"), "मेरी" => Some("meri"), "हम" => Some("hum"),
        "हमारा" => Some("hamara"), "आप" => Some("aap"), "तुम" => Some("tum"),
        "यह" | "ये" | "यहां" => Some(if word == "यहां" { "yahan" } else { "yeh" }),
        "वह" => Some("woh"), "इसका" => Some("iska"), "इसको" => Some("isko"),
        "रखना" => Some("rakhna"), "रखो" => Some("rakho"), "दो" => Some("do"),
        "देना" => Some("dena"), "भेजना" => Some("bhejna"), "बजे" => Some("baje"),
        "तीन" => Some("teen"), "चार" => Some("chaar"), "पांच" | "पाँच" => Some("paanch"),
        "पंद्रह" | "पन्द्रह" => Some("pandrah"), "हजार" | "हज़ार" => Some("hazaar"),
        "रुपये" => Some("rupaye"), "हिंदी" | "हिन्दी" => Some("Hindi"),
        "अनुवाद" => Some("anuvaad"), "मैंने" => Some("maine"), "बोला" => Some("bola"),
        "राहुल" => Some("Rahul"), "मीटिंग" => Some("meeting"),
        "डिप्लॉयमेंट" | "डिप्लायमेंट" | "डेप्लॉयमेंट" => Some("deployment"),
        "टेस्ट" => Some("test"), "स्टेजिंग" => Some("staging"),
        "एपीआई" | "एपिआई" | "एपीआइ" => Some("API"),
        "एजेंडा" | "अजेंडा" => Some("agenda"), "नोट" => Some("note"),
        "इंग्लिश" => Some("English"), "अंग्रेजी" | "अंग्रेज़ी" => Some("angrezi"),
        "बजट" => Some("budget"), "रीशेड्यूल" => Some("reschedule"),
        "रिवाइज्ड" | "रिवाइज़्ड" => Some("revised"),
        _ => None,
    };
    if let Some(roman) = common {
        return Ok(roman.to_string());
    }
    let chars: Vec<char> = word.chars().collect();
    let mut output = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if let Some(onset) = consonant(c) {
            let mut onset = onset;
            if chars.get(i + 1) == Some(&'़') {
                onset = match c {
                    'क' => "q", 'ख' => "kh", 'ग' => "gh", 'ज' => "z",
                    'ड' => "r", 'ढ' => "rh", 'फ' => "f", 'य' => "y",
                    _ => return Err("Roman-Hinglish: unsupported nukta combination; nothing pasted".into()),
                };
                i += 1;
            }
            output.push_str(onset);
            match chars.get(i + 1).copied() {
                Some('्') => i += 1,
                Some(next) if matra(next).is_some() => {
                    output.push_str(matra(next).unwrap());
                    i += 1;
                }
                // Conservative schwa deletion only at the end of a word.
                None => {}
                _ => output.push('a'),
            }
        } else if let Some(vowel) = independent_vowel(c) {
            output.push_str(vowel);
        } else {
            match c {
                'ँ' | 'ं' => output.push('n'),
                'ः' => output.push('h'),
                '०'..='९' => output.push(char::from_u32('0' as u32 + c as u32 - '०' as u32).unwrap()),
                _ => return Err(format!("Roman-Hinglish: unsupported symbol U+{:04X}; nothing pasted", c as u32)),
            }
        }
        i += 1;
    }
    Ok(output)
}

fn consonant(c: char) -> Option<&'static str> {
    Some(match c {
        'क' => "k", 'ख' => "kh", 'ग' => "g", 'घ' => "gh", 'ङ' => "ng",
        'च' => "ch", 'छ' => "chh", 'ज' => "j", 'झ' => "jh", 'ञ' => "ny",
        'ट' | 'त' => "t", 'ठ' | 'थ' => "th", 'ड' | 'द' => "d", 'ढ' | 'ध' => "dh",
        'ण' | 'न' => "n", 'प' => "p", 'फ' => "ph", 'ब' => "b", 'भ' => "bh",
        'म' => "m", 'य' => "y", 'र' => "r", 'ल' | 'ळ' => "l", 'व' => "v",
        'श' | 'ष' => "sh", 'स' => "s", 'ह' => "h", 'क़' => "q", 'ख़' => "kh",
        'ग़' => "gh", 'ज़' => "z", 'ड़' => "r", 'ढ़' => "rh", 'फ़' => "f", 'य़' => "y",
        _ => return None,
    })
}

fn matra(c: char) -> Option<&'static str> {
    Some(match c {
        'ा' => "aa", 'ि' => "i", 'ी' => "ee", 'ु' => "u", 'ू' => "oo",
        'ृ' => "ri", 'ॄ' => "ree", 'े' => "e", 'ै' => "ai", 'ो' => "o", 'ौ' => "au",
        'ॅ' => "a", 'ॉ' => "o", 'ॆ' => "e", 'ॊ' => "o", _ => return None,
    })
}

fn independent_vowel(c: char) -> Option<&'static str> {
    Some(match c {
        'अ' => "a", 'आ' => "aa", 'इ' => "i", 'ई' => "ee", 'उ' => "u", 'ऊ' => "oo",
        'ऋ' => "ri", 'ॠ' => "ree", 'ए' => "e", 'ऐ' => "ai", 'ओ' => "o", 'औ' => "au",
        'ऍ' | 'ऎ' => "e", 'ऑ' | 'ऒ' => "o", _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_speech_keeps_english_and_negation() {
        assert_eq!(romanize_hindi("Please move the meeting to tomorrow, and आज deployment मत करना, पहले API test कर लो।").unwrap(),
            "Please move the meeting to tomorrow, and aaj deployment mat karna, pehle API test kar lo.");
    }

    #[test]
    fn fixtures_use_chat_spelling_without_translation() {
        for (input, expected) in [
            ("कल की meeting तीन बजे रखो, चार बजे नहीं।", "Kal ki meeting teen baje rakho, chaar baje nahi."),
            ("मुझे यह note हिंदी में रखना है, इसका English में अनुवाद मत करना।", "Mujhe yeh note Hindi mein rakhna hai, iska English mein anuvaad mat karna."),
            ("आज deployment मत करना, पहले staging पर API test कर लो।", "Aaj deployment mat karna, pehle staging par API test kar lo."),
            ("कल की meeting reschedule कर दो, और Rahul को revised agenda भेजना है।", "Kal ki meeting reschedule kar do, aur Rahul ko revised agenda bhejna hai."),
            ("Budget पंद्रह हजार rupees है, fifty thousand नहीं।", "Budget pandrah hazaar rupees hai, fifty thousand nahi."),
            ("मैंने बोला PostgreSQL, post office नहीं।", "Maine bola PostgreSQL, post office nahi."),
            ("कल API टेस्ट करना, deployment नहीं।", "Kal API test karna, deployment nahi."),
        ] {
            assert_eq!(romanize_hindi(input).unwrap(), expected);
        }
    }

    #[test]
    fn english_identifiers_and_whitespace_are_verbatim() {
        let input = "  API apiKey PostgreSQL ₹15,000 https://example.test/a?b=1\nDo NOT deploy 😀  ";
        assert_eq!(romanize_hindi(input).unwrap(), input);
    }

    #[test]
    fn fallback_handles_matras_conjuncts_nukta_and_digits() {
        assert_eq!(romanize_hindi("नमस्ते दुनिया क़िला १२३").unwrap(), "Namaste duniyaa qilaa 123");
        assert_eq!(romanize_hindi("शक्\u{200d}ति").unwrap(), "Shakti");
        assert_eq!(romanize_hindi("फ़ाइल फ़ाइल").unwrap(), "Faail faail");
    }

    #[test]
    fn exact_loanwords_do_not_guess_misheard_api() {
        assert_eq!(romanize_hindi("एपीआई टेस्ट, आपये टेस्ट").unwrap(), "API test, aapaye test");
    }

    #[test]
    fn unsupported_marks_fail_closed() {
        assert!(romanize_hindi("अ॑").is_err());
        assert!(romanize_hindi("़").is_err());
        assert_eq!(romanize_hindi("").unwrap(), "");
    }

    #[test]
    fn conversion_is_idempotent() {
        let converted = romanize_hindi("आज API टेस्ट करना, नहीं तो कल।").unwrap();
        assert_eq!(romanize_hindi(&converted).unwrap(), converted);
        assert!(!converted.chars().any(|c| ('\u{0900}'..='\u{097f}').contains(&c)));
    }

    #[test]
    fn conversion_timing_smoke() {
        let start = std::time::Instant::now();
        for _ in 0..10_000 {
            let result = romanize_hindi("आज deployment मत करना, पहले staging पर API test कर लो।").unwrap();
            std::hint::black_box(result);
        }
        println!("Roman conversion average: {} microseconds (10,000 synthetic iterations)", start.elapsed().as_micros() / 10_000);
    }
}
