use crate::index::{InvertedIndex, Posting};
use crate::tokenizer::tokenize;
use std::collections::HashMap;

/// Search result with score
#[derive(Debug)]
pub struct SearchResult {
    pub product_id: u32,
    pub score: f64,
}

/// Simple searcher: tokenizes query, fetches posting lists and scores by aggregated TF.
/// Field boosting: matches from product name -> higher weight.
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
                    // score contribution = freq (could be tuned) * boost
                    // heuristic: if token appears in product name more often it likely has higher freq since name was indexed first
                    let boost = 1.0; // placeholder - could check fields separately
                    let contribution = (p.freq as f64) * boost;
                    *scores.entry(p.product_id).or_insert(0.0) += contribution;
                }
            }
        }

        // convert to vec and sort descending by score
        let mut results: Vec<SearchResult> = scores.into_iter()
            .map(|(product_id, score)| SearchResult { product_id, score })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(top_k);
        results
    }

    /// Convenience: return Products for results
    pub fn search_products(&self, query: &str, top_k: usize) -> Vec<(crate::model::Product, f64)> {
        let results = self.search(query, top_k);
        results.into_iter()
            .filter_map(|r| {
                self.index.get_product(&r.product_id).map(|p| (p, r.score))
            })
            .collect()
    }
}