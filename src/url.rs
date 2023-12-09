use regex::{Regex, RegexBuilder};

const RE_PREFIX: &str = r"^(?:https?://)?(?:";
const RE_SUFFIX: &str = r")";

pub fn url_pattern_to_regex(url_patterns: &[String]) -> Option<Regex> {
    if url_patterns.is_empty() {
        None
    } else {
        let re_body = url_patterns
            .iter()
            .map(|pat| escape_url_pattern(pat))
            .collect::<Vec<_>>()
            .join("|");

        let mut pattern = String::with_capacity(RE_PREFIX.len() + re_body.len() + RE_SUFFIX.len());
        pattern.push_str(RE_PREFIX);
        pattern.push_str(&re_body);
        pattern.push_str(RE_SUFFIX);

        Some(
            RegexBuilder::new(&pattern)
                .case_insensitive(true)
                .build()
                .expect("invalid regex"),
        )
    }
}

const WILDCARD: &str = "*.";
const WILDCARD_RE: &str = r"(?:.+\.)?";
const URL_PATTERN_SUFFIX: &str = r"(?:/|$)";

fn escape_url_pattern(url_pattern: &str) -> String {
    let mut pattern = if url_pattern.starts_with(WILDCARD) {
        let rest = url_pattern.split_at(WILDCARD.len()).1;

        let mut escaped = String::new();
        escaped.push_str(WILDCARD_RE);
        escaped.push_str(&regex::escape(rest));
        escaped
    } else {
        regex::escape(url_pattern)
    };

    pattern.push_str(URL_PATTERN_SUFFIX);

    pattern
}
