use regex::{Regex, RegexBuilder};

const RE_PREFIX: &str = r"^(?:https?://)?(?:";
const RE_SUFFIX: &str = r")";

pub fn url_patterns_to_regex(url_patterns: &[String]) -> Option<Regex> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_escape_url_pattern() {
        const TEST_CASES: &[(&str, &str)] = &[
            ("example.com", r"example\.com(?:/|$)"),
            ("*.example.com", r"(?:.+\.)?example\.com(?:/|$)"),
        ];

        for (url_pattern, expected) in TEST_CASES {
            let result = escape_url_pattern(url_pattern);
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn test_url_pattern_to_regex() {
        let test_cases: &[(&[_], &[_], &[_])] = &[
            (
                &["example.com".to_owned()],
                &[
                    "example.com/",
                    "example.com/path",
                    "http://example.com",
                    "http://example.com/",
                    "https://example.com",
                    "https://example.com/",
                    "EXAMPLE.COM",
                    "HTTP://EXAMPLE.COM",
                    "HTTP://EXAMPLE.COM/",
                    "HTTPS://EXAMPLE.COM",
                    "HTTPS://EXAMPLE.COM/",
                ],
                &[
                    "www.example.com",
                    "example.com.evil.net",
                    "example.net",
                    "ftp://example.com",
                    "badexample.com",
                ],
            ),
            (
                &["*.example.com".to_owned()],
                &[
                    "example.com",
                    "example.com/path",
                    "www.example.com",
                    "www2.example.com/path",
                    "http://example.com",
                    "http://example.com/",
                    "https://example.com",
                    "https://example.com/",
                ],
                &["example.com.evil.net", "example.net", "badexample.com"],
            ),
            (
                &["example.com/path".to_owned()],
                &["example.com/path", "example.com/path/subpath"],
                &["example.com/path-to-evil-subdir"],
            ),
            (
                &["example.com".to_owned(), "example2.com".to_owned()],
                &["example.com", "example2.com"],
                &["example.com.example2.com", "example2.com.example.com"],
            ),
        ];

        for (url_patterns, expected_matches, expected_mismatches) in test_cases {
            let re = url_patterns_to_regex(*url_patterns).unwrap();

            for url in *expected_matches {
                assert!(re.is_match(url), "{url_patterns:?} matches {url}");
            }

            for url in *expected_mismatches {
                assert!(!re.is_match(url), "{url_patterns:?} does not match {url}");
            }
        }
    }
}
