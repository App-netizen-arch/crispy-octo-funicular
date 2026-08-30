//! Party detection: capitalized proper names and role words ("my landlord").

pub fn detect_party(text: &str) -> Option<String> {
    // role words after possessives
    const ROLES: &[&str] = &[
        "landlord", "sister", "brother", "mum", "mom", "dad", "boss", "dentist",
        "doctor", "vet", "garage", "library", "sitter", "plumber", "electrician",
        "agent", "professor", "teacher", "cousin", "neighbour", "neighbor",
        "flatmate", "roommate", "accountant", "lawyer",
    ];
    let lower = text.to_lowercase();
    for role in ROLES {
        for marker in ["my ", "the ", "our "] {
            if let Some(pos) = lower.find(&format!("{marker}{role}")) {
                let mut end = pos + marker.len() + role.len();
                let bytes = text.as_bytes();
                // extend over a following name if present ("my landlord John")
                while end < bytes.len() && bytes[end].is_ascii_alphabetic() {
                    end += 1;
                }
                return Some(text[pos..end].trim().to_string());
            }
        }
    }

    // organizations / institutions worth tracking as parties
    const ORGS: &[&str] = &[
        "ups", "fedex", "dhl", "royal mail", "hmrc", "irs", "amazon", "ebay",
        "paypal", "bank", "gym", "university", "council",
    ];
    for org in ORGS {
        if lower.contains(org) {
            return Some(title_case(org));
        }
    }

    // capitalized tokens that are not sentence-initial and not common words
    let mut candidates: Vec<String> = Vec::new();
    let mut prev_ended_sentence = false;
    for (i, tok) in text.split_whitespace().enumerate() {
        let clean = tok.trim_matches(|c: char| !c.is_ascii_alphanumeric());
        let is_cap = clean.chars().next().is_some_and(|c| c.is_uppercase())
            && clean.len() > 1
            && !STOP_CAPS.contains(&clean.to_lowercase().as_str());
        let sentence_initial =
            i == 0 || prev_ended_sentence;
        prev_ended_sentence = tok.ends_with('.') || tok.ends_with('!') || tok.ends_with('?');
        if is_cap && !sentence_initial {
            candidates.push(clean.to_string());
        }
    }

    candidates.into_iter().next()
}

fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

const STOP_CAPS: &[&str] = &[
    "i", "i'll", "i'm", "i'd", "i've", "it's", "monday", "tuesday", "wednesday",
    "thursday", "friday", "saturday", "sunday", "note", "ok", "so", "the", "need",
    "must", "told", "promised", "still", "also", "and", "but", "when", "she", "he",
    "they", "this", "that", "reminder",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roles() {
        assert_eq!(detect_party("told the landlord I'd pay"), Some("the landlord".into()));
        assert_eq!(detect_party("owe my sister a call"), Some("my sister".into()));
    }

    #[test]
    fn names() {
        assert_eq!(detect_party("I owe Dave 15 quid"), Some("Dave".into()));
        assert_eq!(
            detect_party("send Ingrid the signed form tomorrow"),
            Some("Ingrid".into())
        );
    }

    #[test]
    fn none_for_pronouns_only() {
        assert_eq!(detect_party("I told him I'd get it back to him"), None);
    }
}
