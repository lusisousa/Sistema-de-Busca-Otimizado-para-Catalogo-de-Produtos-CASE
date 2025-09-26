use std::collections::HashMap;
use parking_lot::RwLock;
use crate::model::Product;
use crate::tokenizer::tokenize;

/// Posting: (product_id, term_freq)
#[derive(Debug, Clone)]
pub struct Posting {
    pub product_id: u32,
    pub freq: u32,
}

#[derive(Debug, Default)]
pub struct InvertedIndex {
    // token -> list of postings
    index: RwLock<HashMap<String, Vec<Posting>>>,
    // product_id -> product
    pub products: RwLock<HashMap<u32, Product>>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            index: RwLock::new(HashMap::new()),
            products: RwLock::new(HashMap::new()),
        }
    }

    /// Add or update a product in the index.
    pub fn add_product(&self, product: Product) {
        let id = product.id;
        // store product
        self.products.write().insert(id, product.clone());

        // index name, brand, category, description
        let mut terms = Vec::new();
        terms.extend(tokenize(&product.name));
        if let Some(ref b) = product.brand { terms.extend(tokenize(b)); }
        if let Some(ref c) = product.category { terms.extend(tokenize(c)); }
        if let Some(ref d) = product.description { terms.extend(tokenize(d)); }

        // count term frequencies
        let mut tf: HashMap<String, u32> = HashMap::new();
        for t in terms {
            *tf.entry(t).or_insert(0) += 1;
        }

        // update postings
        let mut idx = self.index.write();
        for (term, freq) in tf {
            let postings = idx.entry(term).or_insert_with(Vec::new);
            // simple approach: append or update existing posting
            if let Some(p) = postings.iter_mut().find(|p| p.product_id == id) {
                p.freq = freq;
            } else {
                postings.push(Posting { product_id: id, freq });
            }
        }
    }

    /// Remove product (basic)
    pub fn remove_product(&self, product_id: u32) {
        self.products.write().remove(&product_id);
        let mut idx = self.index.write();
        let mut to_remove = Vec::new();
        for (term, postings) in idx.iter_mut() {
            postings.retain(|p| p.product_id != product_id);
            if postings.is_empty() {
                to_remove.push(term.clone());
            }
        }
        for t in to_remove {
            idx.remove(&t);
        }
    }

    /// Expose a read handle to index for search (cloned snapshot)
    pub fn get_index_snapshot(&self) -> HashMap<String, Vec<Posting>> {
        self.index.read().clone()
    }

    pub fn get_product(&self, product_id: &u32) -> Option<Product> {
        self.products.read().get(product_id).cloned()
    }
}