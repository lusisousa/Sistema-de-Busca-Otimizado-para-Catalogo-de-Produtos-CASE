use crate::index::{InvertedIndex, Posting};
use crate::tokenizer::tokenize;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SearchResult {
    pub product_id: u32,
    pub score: f64,
}

pub struct Searcher<'a> {
    pub index: &'a InvertedIndex,
}

impl<'a> Searcher<'a> {
    pub fn new(index: &'a InvertedIndex) -> Self {
        Self { index }
    }

    pub fn search(&self, query: &str, top_k: usize) -> Vec<SearchResult> {
        let tokens = tokenize(query);
        if tokens.is_empty() {
            return vec![];
        }

        let idx_snapshot = self.index.get_index_snapshot();

        let mut scores: HashMap<u32, f64> = HashMap::new();

        for token in tokens {
            if let Some(postings) = idx_snapshot.get(&token) {
                for p in postings {
                    let boost = 1.0;
                    let contribution = (p.freq as f64) * boost;
                    *scores.entry(p.product_id).or_insert(0.0) += contribution;
                }
            }
        }

        let mut results: Vec<SearchResult> = scores.into_iter()
            .map(|(product_id, score)| SearchResult { product_id, score })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(top_k);
        results
    }

    pub fn search_products(&self, query: &str, top_k: usize) -> Vec<(crate::model::Product, f64)> {
        let results = self.search(query, top_k);
        results.into_iter()
            .filter_map(|r| {
                self.index.get_product(&r.product_id).map(|p| (p, r.score))
            })
            .collect()
    }
}