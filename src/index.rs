use std::collections::HashMap;
use parking_lot::RwLock;
use crate::model::Product;
use crate::tokenizer::tokenize;

#[derive(Debug, Clone)]
pub struct Posting {
    pub product_id: u32,
    pub freq: u32,
}

#[derive(Debug, Default)]
pub struct InvertedIndex {
    index: RwLock<HashMap<String, Vec<Posting>>>,
    pub products: RwLock<HashMap<u32, Product>>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            index: RwLock::new(HashMap::new()),
            products: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_product(&self, product: Product) {
        let id = product.id;
        // store product
        self.products.write().insert(id, product.clone());

        let mut terms = Vec::new();
        terms.extend(tokenize(&product.name));
        if let Some(ref b) = product.brand { terms.extend(tokenize(b)); }
        if let Some(ref c) = product.category { terms.extend(tokenize(c)); }
        if let Some(ref d) = product.description { terms.extend(tokenize(d)); }

        let mut tf: HashMap<String, u32> = HashMap::new();
        for t in terms {
            *tf.entry(t).or_insert(0) += 1;
        }

        let mut idx = self.index.write();
        for (term, freq) in tf {
            let postings = idx.entry(term).or_insert_with(Vec::new);
            if let Some(p) = postings.iter_mut().find(|p| p.product_id == id) {
                p.freq = freq;
            } else {
                postings.push(Posting { product_id: id, freq });
            }
        }
    }

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

    pub fn get_index_snapshot(&self) -> HashMap<String, Vec<Posting>> {
        self.index.read().clone()
    }

    pub fn get_product(&self, product_id: &u32) -> Option<Product> {
        self.products.read().get(product_id).cloned()
    }
}