//! Context-aware technical and shortcut formatting for dictation.
//!
//! Converts spoken technical patterns into canonical developer syntax:
//! - Keyboard shortcuts (e.g. "left alt space" -> "Left Alt + Space", "ctrl c" -> "Ctrl + C")
//! - Casing commands (e.g. "camel case user id" -> "userId", "snake case api key" -> "api_key")
//! - CLI flags and arguments (e.g. "double dash rebase" -> "--rebase", "minus m" -> "-m")
//! - Technical symbols and operators (e.g. "arrow function" -> "=>", "double equals" -> "==")
//! - Web domains & file extensions (e.g. "google dot com" -> "google.com", "schema dot json" -> "schema.json")
//! - Technical terminology capitalization (e.g. "github" -> "GitHub", "oauth" -> "OAuth")
//! - Spoken punctuation & layout (e.g. "new line" -> "\n", "open paren" -> "(")

use once_cell::sync::Lazy;
use regex::Regex;

static SHORTCUT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b(?:(left|right)\s+)?(ctrl|control|alt|shift|cmd|command|option|opt|super|win|windows|meta)(?:\s*(?:\+|\bplus\b)\s*|\s+)(?:(?:(left|right)\s+)?(ctrl|control|alt|shift|cmd|command|option|opt|super|win|windows|meta)(?:\s*(?:\+|\bplus\b)\s*|\s+))?(?:(?:(left|right)\s+)?(ctrl|control|alt|shift|cmd|command|option|opt|super|win|windows|meta)(?:\s*(?:\+|\bplus\b)\s*|\s+))?(space|enter|return|tab|backspace|delete|del|escape|esc|insert|home|end|page\s*up|page\s*down|up\s*arrow|down\s*arrow|left\s*arrow|right\s*arrow|arrow\s*up|arrow\s*down|arrow\s*left|arrow\s*right|f\s*(?:1[0-2]|[1-9])|[a-z0-9])\b",
    )
    .unwrap()
});

static CASING_CMD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b(camel\s*case|snake\s*case|kebab\s*case|dash\s*case|constant\s*case|upper\s*snake\s*case|screaming\s*snake\s*case|pascal\s*case|dot\s*case)\b",
    )
    .unwrap()
});

static CLI_DOUBLE_DASH_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)\b(?:double\s*dash|dash\s*dash)\s+([a-zA-Z0-9_\-]+)\b").unwrap());

static CLI_SINGLE_DASH_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)\b(?:minus|dash)\s+([a-zA-Z]{1,4})\b").unwrap());

static WEB_AND_EXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b([a-zA-Z0-9_-]+)\s+(?:dot|\.)\s*(com|org|net|io|dev|ai|app|ts|js|rs|json|md|env|toml|yaml|yml|py|html|css|sql)\b",
    )
    .unwrap()
});

static SPOKEN_NEWLINE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)\b(?:new\s*line|newline)\b").unwrap());

static MULTI_NEWLINE_SPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ \t]*\n[ \t]*").unwrap());

static PAREN_ATTACH: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\b[a-zA-Z0-9_]+)\s+\(").unwrap());

static CODE_SYMBOLS: Lazy<Vec<(Regex, &'static str)>> = Lazy::new(|| {
    vec![
        (
            Regex::new(r"(?i)\b(?:arrow\s+function|fat\s+arrow)\b").unwrap(),
            "=>",
        ),
        (
            Regex::new(r"(?i)\b(?:skinny\s+arrow|thin\s+arrow)\b").unwrap(),
            "->",
        ),
        (
            Regex::new(r"(?i)\b(?:triple\s+equals|equals\s+equals\s+equals)\b").unwrap(),
            "===",
        ),
        (
            Regex::new(r"(?i)\b(?:not\s+equals\s+equals|bang\s+double\s+equals)\b").unwrap(),
            "!==",
        ),
        (
            Regex::new(r"(?i)\b(?:double\s+equals|equals\s+equals)\b").unwrap(),
            "==",
        ),
        (
            Regex::new(r"(?i)\b(?:not\s+equals|bang\s+equals)\b").unwrap(),
            "!=",
        ),
        (
            Regex::new(r"(?i)\b(?:greater\s+than\s+equals|greater\s+than\s+or\s+equal\s+to)\b")
                .unwrap(),
            ">=",
        ),
        (
            Regex::new(r"(?i)\b(?:less\s+than\s+equals|less\s+than\s+or\s+equal\s+to)\b").unwrap(),
            "<=",
        ),
        (Regex::new(r"(?i)\bplus\s+equals\b").unwrap(), "+="),
        (Regex::new(r"(?i)\bminus\s+equals\b").unwrap(), "-="),
        (
            Regex::new(r"(?i)\s*(?:double\s+colon|colon\s+colon)\s*").unwrap(),
            "::",
        ),
        (
            Regex::new(r"(?i)\b(?:double\s+ampersand|logical\s+and)\b").unwrap(),
            "&&",
        ),
        (
            Regex::new(r"(?i)\b(?:double\s+pipe|logical\s+or)\b").unwrap(),
            "||",
        ),
        (
            Regex::new(r"(?i)\b(?:double\s+question\s+mark|nullish\s+coalescing)\b").unwrap(),
            "??",
        ),
        (
            Regex::new(r"(?i)\b(?:question\s+dot|optional\s+chaining)\b").unwrap(),
            "?.",
        ),
    ]
});

static SPOKEN_PARENS: Lazy<Vec<(Regex, &'static str)>> = Lazy::new(|| {
    vec![
        (Regex::new(r"(?i)\bopen\s+paren(?:thesis)?\b").unwrap(), "("),
        (
            Regex::new(r"(?i)\bclose\s+paren(?:thesis)?\b").unwrap(),
            ")",
        ),
        (Regex::new(r"(?i)\bopen\s+bracket\b").unwrap(), "["),
        (Regex::new(r"(?i)\bclose\s+bracket\b").unwrap(), "]"),
        (
            Regex::new(r"(?i)\bopen\s+(?:curly\s+bracket|brace)\b").unwrap(),
            "{",
        ),
        (
            Regex::new(r"(?i)\bclose\s+(?:curly\s+bracket|brace)\b").unwrap(),
            "}",
        ),
    ]
});

static CLEAN_PAREN_SPACES: Lazy<Vec<(Regex, &'static str)>> = Lazy::new(|| {
    vec![
        (Regex::new(r"\(\s+").unwrap(), "("),
        (Regex::new(r"\s+\)").unwrap(), ")"),
        (Regex::new(r"\[\s+").unwrap(), "["),
        (Regex::new(r"\s+\]").unwrap(), "]"),
        (Regex::new(r"\{\s+").unwrap(), "{"),
        (Regex::new(r"\s+\}").unwrap(), "}"),
    ]
});

static TECHNICAL_TERMS: Lazy<Vec<(Regex, &'static str)>> = Lazy::new(|| {
    let terms: &[(&str, &str)] = &[
        (r"(?i)\bgithub\b", "GitHub"),
        (r"(?i)\bgitlab\b", "GitLab"),
        (r"(?i)\bjavascript\b", "JavaScript"),
        (r"(?i)\btypescript\b", "TypeScript"),
        (r"(?i)\bnode\s*js\b", "Node.js"),
        (r"(?i)\breact\s*js\b", "React"),
        (r"(?i)\bnext\s*js\b", "Next.js"),
        (r"(?i)\bvue\s*js\b", "Vue.js"),
        (r"(?i)\bpostgresql\b", "PostgreSQL"),
        (r"(?i)\bpostgres\b", "PostgreSQL"),
        (r"(?i)\bmongodb\b", "MongoDB"),
        (r"(?i)\bmongo\s*db\b", "MongoDB"),
        (r"(?i)\bgraphql\b", "GraphQL"),
        (r"(?i)\brest\s+apis\b", "REST APIs"),
        (r"(?i)\brest\s+api\b", "REST API"),
        (r"(?i)\boauth\b", "OAuth"),
        (r"(?i)\bjwts\b", "JWTs"),
        (r"(?i)\bjwt\b", "JWT"),
        (r"(?i)\bhtml\b", "HTML"),
        (r"(?i)\bcss\b", "CSS"),
        (r"(?i)\bsql\b", "SQL"),
        (r"(?i)\bnosql\b", "NoSQL"),
        (r"(?i)\bjson\b", "JSON"),
        (r"(?i)\byaml\b", "YAML"),
        (r"(?i)\btoml\b", "TOML"),
        (r"(?i)\buuids\b", "UUIDs"),
        (r"(?i)\buuid\b", "UUID"),
        (r"(?i)\bguids\b", "GUIDs"),
        (r"(?i)\bguid\b", "GUID"),
        (r"(?i)\bhttps\b", "HTTPS"),
        (r"(?i)\bhttp\b", "HTTP"),
        (r"(?i)\bgrpc\b", "gRPC"),
        (r"(?i)\bsdks\b", "SDKs"),
        (r"(?i)\bsdk\b", "SDK"),
        (r"(?i)\bapis\b", "APIs"),
        (r"(?i)\bapi\b", "API"),
        (r"(?i)\bclis\b", "CLIs"),
        (r"(?i)\bcli\b", "CLI"),
        (r"(?i)\bguis\b", "GUIs"),
        (r"(?i)\bgui\b", "GUI"),
        (r"(?i)\burls\b", "URLs"),
        (r"(?i)\burl\b", "URL"),
        (r"(?i)\buris\b", "URIs"),
        (r"(?i)\buri\b", "URI"),
        (r"(?i)\barm64\b", "ARM64"),
        (r"(?i)\bx86_64\b", "x86_64"),
        (r"(?i)\bx64\b", "x64"),
        (r"(?i)\bcpus\b", "CPUs"),
        (r"(?i)\bcpu\b", "CPU"),
        (r"(?i)\bgpus\b", "GPUs"),
        (r"(?i)\bgpu\b", "GPU"),
        (r"(?i)\bnpus\b", "NPUs"),
        (r"(?i)\bnpu\b", "NPU"),
        (r"(?i)\bram\b", "RAM"),
        (r"(?i)\bdockerfiles\b", "Dockerfiles"),
        (r"(?i)\bdockerfile\b", "Dockerfile"),
        (r"(?i)\bdevops\b", "DevOps"),
        (r"(?i)\bci\s*\/\s*cd\b", "CI/CD"),
        (r"(?i)\bci\s*cd\b", "CI/CD"),
        (r"(?i)\bprs\b", "PRs"),
        (r"(?i)\bpr\b", "PR"),
    ];

    terms
        .iter()
        .map(|(pat, rep)| (Regex::new(pat).unwrap(), *rep))
        .collect()
});

fn canonical_mod(side: Option<&str>, mod_key: &str) -> String {
    let lower = mod_key.to_ascii_lowercase();
    let base = match lower.as_str() {
        "ctrl" | "control" => "Ctrl",
        "alt" => "Alt",
        "shift" => "Shift",
        "cmd" | "command" => "Cmd",
        "opt" | "option" => "Option",
        "win" | "windows" | "super" => "Win",
        "meta" => "Meta",
        _ => return mod_key.to_string(),
    };

    if let Some(s) = side {
        format!("{} {}", capitalize(s), base)
    } else {
        base.to_string()
    }
}

fn canonical_key(key: &str) -> String {
    let k = key
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(|c| c.to_lowercase())
        .collect::<String>();
    match k.as_str() {
        "space" => "Space".to_string(),
        "enter" | "return" => "Enter".to_string(),
        "tab" => "Tab".to_string(),
        "backspace" => "Backspace".to_string(),
        "delete" | "del" => "Delete".to_string(),
        "escape" | "esc" => "Esc".to_string(),
        "insert" => "Insert".to_string(),
        "home" => "Home".to_string(),
        "end" => "End".to_string(),
        "pageup" => "Page Up".to_string(),
        "pagedown" => "Page Down".to_string(),
        "uparrow" | "arrowup" => "Up Arrow".to_string(),
        "downarrow" | "arrowdown" => "Down Arrow".to_string(),
        "leftarrow" | "arrowleft" => "Left Arrow".to_string(),
        "rightarrow" | "arrowright" => "Right Arrow".to_string(),
        other
            if other.starts_with('f')
                && other.len() <= 3
                && other[1..].chars().all(|c| c.is_ascii_digit()) =>
        {
            format!("F{}", &other[1..])
        }
        other if other.len() == 1 && other.chars().all(|c| c.is_ascii_alphabetic()) => {
            other.to_ascii_uppercase()
        }
        _ => key.to_string(),
    }
}

pub fn format_shortcuts(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;

    for caps in SHORTCUT_REGEX.captures_iter(text) {
        let entire = caps.get(0).unwrap();
        let start = entire.start();
        let end = entire.end();

        let side1 = caps.get(1).map(|m| m.as_str());
        let mod1 = caps.get(2).map(|m| m.as_str()).unwrap();
        let side2 = caps.get(3).map(|m| m.as_str());
        let mod2 = caps.get(4).map(|m| m.as_str());
        let side3 = caps.get(5).map(|m| m.as_str());
        let mod3 = caps.get(6).map(|m| m.as_str());
        let key = caps.get(7).map(|m| m.as_str()).unwrap();

        // Edge case protection for verb "control/command/shift" + "a" / "i" followed by words
        if mod2.is_none()
            && mod3.is_none()
            && (key.eq_ignore_ascii_case("a") || key.eq_ignore_ascii_case("i"))
        {
            if (mod1.eq_ignore_ascii_case("control")
                || mod1.eq_ignore_ascii_case("command")
                || mod1.eq_ignore_ascii_case("shift"))
                && side1.is_none()
            {
                let pre = text[..start].trim_end();
                if let Some(last_word) = pre.split_whitespace().last() {
                    let lw = last_word.to_ascii_lowercase();
                    if matches!(
                        lw.as_str(),
                        "to" | "can"
                            | "could"
                            | "will"
                            | "would"
                            | "must"
                            | "might"
                            | "shall"
                            | "should"
                    ) {
                        // Keep prose
                        result.push_str(&text[last_end..end]);
                        last_end = end;
                        continue;
                    }
                }
            }
        }

        result.push_str(&text[last_end..start]);

        let mut parts = Vec::with_capacity(4);
        parts.push(canonical_mod(side1, mod1));
        if let Some(m2) = mod2 {
            parts.push(canonical_mod(side2, m2));
        }
        if let Some(m3) = mod3 {
            parts.push(canonical_mod(side3, m3));
        }
        parts.push(canonical_key(key));

        result.push_str(&parts.join(" + "));
        last_end = end;
    }

    result.push_str(&text[last_end..]);
    result
}

const CASING_STOP_WORDS: &[&str] = &[
    "equals", "equal", "===", "==", "=", "open", "close", "here", "there", "to", "with", "from",
    "for", "in", "on", "at", "by", "as", "is", "was", "are", "were", "null", "true", "false",
    "and", "or", "then",
];

const PREPOSITION_ID_PARTS: &[&str] = &[
    "id", "string", "file", "json", "user", "client", "event", "data", "token", "name",
];

pub fn format_casing_commands(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;

    for caps in CASING_CMD_REGEX.captures_iter(text) {
        let entire = caps.get(0).unwrap();
        let cmd_start = entire.start();
        let cmd_end = entire.end();
        if cmd_start < last_end {
            continue;
        }

        let cmd_raw = caps.get(1).unwrap().as_str();
        let cmd = cmd_raw
            .chars()
            .filter(|c| !c.is_whitespace())
            .flat_map(|c| c.to_lowercase())
            .collect::<String>();

        result.push_str(&text[last_end..cmd_start]);

        let rem = &text[cmd_end..];
        let tokens: Vec<&str> = rem.split_whitespace().collect();
        let mut words: Vec<&str> = Vec::new();

        for (i, &t) in tokens.iter().enumerate() {
            let clean_t = t.trim_end_matches(|c: char| c.is_ascii_punctuation());
            let lower_clean = clean_t.to_ascii_lowercase();

            if i > 0 && CASING_STOP_WORDS.contains(&lower_clean.as_str()) {
                let is_preposition = matches!(
                    lower_clean.as_str(),
                    "by" | "to" | "for" | "with" | "from" | "in"
                );
                if is_preposition && i + 1 < tokens.len() {
                    let next_clean = tokens[i + 1]
                        .trim_end_matches(|c: char| c.is_ascii_punctuation())
                        .to_ascii_lowercase();
                    if PREPOSITION_ID_PARTS.contains(&next_clean.as_str()) {
                        words.push(clean_t);
                        continue;
                    }
                }
                break;
            }

            if clean_t.is_empty() || words.len() >= 6 {
                break;
            }
            words.push(clean_t);
            if clean_t.len() < t.len() {
                break;
            }
        }

        if words.is_empty() {
            result.push_str(&text[cmd_start..cmd_end]);
            last_end = cmd_end;
            continue;
        }

        let transformed = match cmd.as_str() {
            "camelcase" => {
                let first = words[0].to_ascii_lowercase();
                let rest: String = words[1..].iter().map(|w| capitalize(w)).collect();
                format!("{}{}", first, rest)
            }
            "snakecase" => words
                .iter()
                .map(|w| w.to_ascii_lowercase())
                .collect::<Vec<_>>()
                .join("_"),
            "kebabcase" | "dashcase" => words
                .iter()
                .map(|w| w.to_ascii_lowercase())
                .collect::<Vec<_>>()
                .join("-"),
            "constantcase" | "uppersnakecase" | "screamingsnakecase" => words
                .iter()
                .map(|w| w.to_ascii_uppercase())
                .collect::<Vec<_>>()
                .join("_"),
            "pascalcase" => words.iter().map(|w| capitalize(w)).collect::<String>(),
            "dotcase" => words
                .iter()
                .map(|w| w.to_ascii_lowercase())
                .collect::<Vec<_>>()
                .join("."),
            _ => {
                result.push_str(&text[cmd_start..cmd_end]);
                last_end = cmd_end;
                continue;
            }
        };

        result.push_str(&transformed);
        let last_word = words.last().unwrap();
        if let Some(pos) = rem.find(last_word) {
            last_end = cmd_end + pos + last_word.len();
        } else {
            last_end = cmd_end;
        }
    }

    result.push_str(&text[last_end..]);
    result
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            first.to_uppercase().collect::<String>() + &chars.as_str().to_ascii_lowercase()
        }
    }
}

const ENGLISH_STOP_WORDS: &[&str] = &[
    "of", "in", "to", "at", "on", "by", "for", "the", "a", "an", "is", "it", "as", "and", "or",
    "salt", "code",
];

pub fn format_cli_flags(text: &str) -> String {
    let text = CLI_DOUBLE_DASH_REGEX.replace_all(text, "--$1");

    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;

    for caps in CLI_SINGLE_DASH_REGEX.captures_iter(&text) {
        let entire = caps.get(0).unwrap();
        let start = entire.start();
        let end = entire.end();
        let flag = caps.get(1).unwrap().as_str();

        if ENGLISH_STOP_WORDS.contains(&flag.to_ascii_lowercase().as_str()) {
            continue;
        }

        let pre = text[..start].trim_end();
        if let Some(last_ch) = pre.chars().last() {
            if last_ch.is_ascii_digit() {
                continue;
            }
        }

        let post = text[end..].trim_start();
        if let Some(next_word) = post.split_whitespace().next() {
            let nw = next_word.to_ascii_lowercase();
            if matches!(nw.as_str(), "of" | "and" | "or") {
                continue;
            }
        }

        result.push_str(&text[last_end..start]);
        result.push('-');
        result.push_str(flag);
        last_end = end;
    }

    result.push_str(&text[last_end..]);
    result
}

pub fn format_symbols(text: &str) -> String {
    let mut formatted = text.to_string();
    for (pattern, replacement) in CODE_SYMBOLS.iter() {
        formatted = pattern.replace_all(&formatted, *replacement).to_string();
    }
    formatted
}

pub fn format_web_and_ext(text: &str) -> String {
    WEB_AND_EXT_REGEX.replace_all(text, "$1.$2").to_string()
}

pub fn format_spoken_parens(text: &str) -> String {
    let mut formatted = text.to_string();
    for (pattern, replacement) in SPOKEN_PARENS.iter() {
        formatted = pattern.replace_all(&formatted, *replacement).to_string();
    }
    for (pattern, replacement) in CLEAN_PAREN_SPACES.iter() {
        formatted = pattern.replace_all(&formatted, *replacement).to_string();
    }
    formatted = PAREN_ATTACH.replace_all(&formatted, "$1(").to_string();
    formatted
}

pub fn format_layout(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;

    for caps in SPOKEN_NEWLINE.captures_iter(text) {
        let entire = caps.get(0).unwrap();
        let start = entire.start();
        let end = entire.end();

        let post = text[end..].trim_start();
        if let Some(next_word) = post.split_whitespace().next() {
            if next_word.eq_ignore_ascii_case("of") {
                continue;
            }
        }

        result.push_str(&text[last_end..start]);
        result.push('\n');
        last_end = end;
    }

    result.push_str(&text[last_end..]);
    MULTI_NEWLINE_SPACE.replace_all(&result, "\n").to_string()
}

pub fn format_technical_terms(text: &str) -> String {
    let mut current = text.to_string();

    for (pattern, replacement) in TECHNICAL_TERMS.iter() {
        let mut result = String::with_capacity(current.len());
        let mut last_end = 0;

        for caps in pattern.captures_iter(&current) {
            let entire = caps.get(0).unwrap();
            let start = entire.start();
            let end = entire.end();

            if start > 0 && current[..start].ends_with('.') {
                continue;
            }

            result.push_str(&current[last_end..start]);
            result.push_str(replacement);
            last_end = end;
        }

        result.push_str(&current[last_end..]);
        current = result;
    }

    current
}

/// Applies all technical and shortcut normalizations to transcription text.
pub fn apply_technical_formatting(text: &str) -> String {
    let text = format_layout(text);
    let text = format_casing_commands(&text);
    let text = format_spoken_parens(&text);
    let text = format_shortcuts(&text);
    let text = format_cli_flags(&text);
    let text = format_symbols(&text);
    let text = format_web_and_ext(&text);
    let text = format_technical_terms(&text);
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_exact_prompt() {
        let input = "Yes I guess, continue with left alt space using this voice workspace only.";
        let expected =
            "Yes I guess, continue with Left Alt + Space using this voice workspace only.";
        assert_eq!(apply_technical_formatting(input), expected);
    }

    #[test]
    fn test_shortcuts_variations() {
        assert_eq!(
            apply_technical_formatting("continue with left alt + space"),
            "continue with Left Alt + Space"
        );
        assert_eq!(
            apply_technical_formatting("continue with left alt plus space"),
            "continue with Left Alt + Space"
        );
        assert_eq!(
            apply_technical_formatting("press right alt space"),
            "press Right Alt + Space"
        );
        assert_eq!(
            apply_technical_formatting("hit alt tab to switch windows"),
            "hit Alt + Tab to switch windows"
        );
        assert_eq!(
            apply_technical_formatting("Press ctrl c then ctrl v."),
            "Press Ctrl + C then Ctrl + V."
        );
        assert_eq!(
            apply_technical_formatting("Use control shift p to open command palette."),
            "Use Ctrl + Shift + P to open command palette."
        );
        assert_eq!(
            apply_technical_formatting("Hit alt f4 to exit."),
            "Hit Alt + F4 to exit."
        );
        assert_eq!(
            apply_technical_formatting("Win r opens the run prompt."),
            "Win + R opens the run prompt."
        );
        assert_eq!(
            apply_technical_formatting("Command option esc opens task manager."),
            "Cmd + Option + Esc opens task manager."
        );
        assert_eq!(
            apply_technical_formatting("Shift tab moves backward."),
            "Shift + Tab moves backward."
        );
    }

    #[test]
    fn test_casing_commands() {
        assert_eq!(
            apply_technical_formatting("Let us use camel case user id here."),
            "Let us use userId here."
        );
        assert_eq!(
            apply_technical_formatting(
                "Function camel case get user details open paren close paren"
            ),
            "Function getUserDetails()"
        );
        assert_eq!(
            apply_technical_formatting("Set snake case auth token to null."),
            "Set auth_token to null."
        );
        assert_eq!(
            apply_technical_formatting("Use kebab case primary button style."),
            "Use primary-button-style."
        );
        assert_eq!(
            apply_technical_formatting("Define constant case max retry count equals five."),
            "Define MAX_RETRY_COUNT equals five."
        );
        assert_eq!(
            apply_technical_formatting("Class pascal case app notification manager."),
            "Class AppNotificationManager."
        );
        assert_eq!(
            apply_technical_formatting("Package dot case com example test."),
            "Package com.example.test."
        );
    }

    #[test]
    fn test_cli_flags() {
        assert_eq!(
            apply_technical_formatting("Run cargo test double dash nocapture."),
            "Run cargo test --nocapture."
        );
        assert_eq!(
            apply_technical_formatting("Git commit minus m in backticks initial commit."),
            "Git commit -m in backticks initial commit."
        );
        assert_eq!(
            apply_technical_formatting("Execute ls minus la in the terminal."),
            "Execute ls -la in the terminal."
        );
        assert_eq!(
            apply_technical_formatting("Git pull double dash rebase."),
            "Git pull --rebase."
        );
    }

    #[test]
    fn test_code_symbols() {
        assert_eq!(
            apply_technical_formatting("Const add equals x fat arrow x plus 1."),
            "Const add equals x => x plus 1."
        );
        assert_eq!(
            apply_technical_formatting("If x double equals y double ampersand is valid"),
            "If x == y && is valid"
        );
        assert_eq!(
            apply_technical_formatting("If token not equals null"),
            "If token != null"
        );
        assert_eq!(
            apply_technical_formatting("Value double question mark default value"),
            "Value ?? default value"
        );
        assert_eq!(
            apply_technical_formatting("Count plus equals 1"),
            "Count += 1"
        );
        assert_eq!(
            apply_technical_formatting("Use std double colon fs"),
            "Use std::fs"
        );
    }

    #[test]
    fn test_web_and_extensions() {
        assert_eq!(
            apply_technical_formatting("Go to google dot com."),
            "Go to google.com."
        );
        assert_eq!(
            apply_technical_formatting("Open schema dot json and config dot toml."),
            "Open schema.json and config.toml."
        );
        assert_eq!(
            apply_technical_formatting("Edit index dot ts or app dot py."),
            "Edit index.ts or app.py."
        );
    }

    #[test]
    fn test_technical_terms() {
        assert_eq!(
            apply_technical_formatting(
                "We should review postgresql, kubernetes and the rest api before release."
            ),
            "We should review PostgreSQL, kubernetes and the REST API before release."
        );
        assert_eq!(
            apply_technical_formatting("Akshat pushed the code to github for review."),
            "Akshat pushed the code to GitHub for review."
        );
        assert_eq!(
            apply_technical_formatting("Use oauth with a jwt on node js and redis."),
            "Use OAuth with a JWT on Node.js and redis."
        );
        assert_eq!(
            apply_technical_formatting("Running on arm64 cpu and checking cicd."),
            "Running on ARM64 CPU and checking CI/CD."
        );
    }

    #[test]
    fn test_layout() {
        assert_eq!(
            apply_technical_formatting("First line period new line second line"),
            "First line period\nsecond line"
        );
    }

    #[test]
    fn test_prose_false_positive_protections() {
        assert_eq!(
            apply_technical_formatting("We need to control a few variables."),
            "We need to control a few variables."
        );
        assert_eq!(
            apply_technical_formatting("A new line of products was released."),
            "A new line of products was released."
        );
        assert_eq!(
            apply_technical_formatting("There was a shift in our strategy."),
            "There was a shift in our strategy."
        );
        assert_eq!(
            apply_technical_formatting("Add a dash of salt."),
            "Add a dash of salt."
        );
        assert_eq!(
            apply_technical_formatting("10 minus 5 is 5."),
            "10 minus 5 is 5."
        );
    }
}
