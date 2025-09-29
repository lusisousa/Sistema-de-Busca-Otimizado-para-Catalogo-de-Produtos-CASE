use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashSet;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"[^\w\s]").unwrap();
    static ref STOPWORDS: HashSet<&'static str> = {
        // Stopwords básicas em português (pode expandir)
        let words = [
            "a","o","as","os","um","uma","de","do","da","dos","das",
            "em","para","por","com","sem","e","ou","que","se","no","na","nos","nas"
        ];
        words.iter().cloned().collect()
    };
    static ref STEMMER: Stemmer = Stemmer::create(Algorithm::Portuguese);
}

pub fn tokenize(text: &str) -> Vec<String> {
    let lowered = text.to_lowercase();
    let cleaned = RE.replace_all(&lowered, " ");

    cleaned
        .split_whitespace()
        .filter_map(|token| {
            if STOPWORDS.contains(token) {
                return None; 
            }
            let stem = STEMMER.stem(token).to_string();
            Some(stem)
        })
        .collect()
}