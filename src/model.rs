use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub brand: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
}

impl Product {
    pub fn new(id: u32, name: &str, brand: Option<&str>, category: Option<&str>, description: Option<&str>) -> Self {
        Self {
            id,
            name: name.to_string(),
            brand: brand.map(|s| s.to_string()),
            category: category.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
        }
    }
}