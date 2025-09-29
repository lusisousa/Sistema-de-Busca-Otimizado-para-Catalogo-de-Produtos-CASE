use megastore_search::index::InvertedIndex;
use megastore_search::model::Product;
use megastore_search::search::Searcher;

#[test]
fn basic_index_and_search() {
    let idx = InvertedIndex::new();
    idx.add_product(Product::new(1, "Smartphone SuperX 64GB", Some("ZenTech"), Some("Eletrônicos"), Some("Ótima câmera")));
    idx.add_product(Product::new(2, "Fone de Ouvido XPro", Some("SoundInc"), Some("Eletrônicos"), Some("Cancelamento de ruído")));
    idx.add_product(Product::new(3, "Camiseta Esportiva", Some("Sporty"), Some("Vestuário"), Some("Secagem rápida")));

    let searcher = Searcher::new(&idx);
    let res = searcher.search_products("smartphone", 5);

    assert!(!res.is_empty());
    assert_eq!(res[0].0.id, 1);
}

#[test]
fn search_filters_by_tokens() {
    let idx = InvertedIndex::new();
    idx.add_product(Product::new(10, "Camisa Preta", Some("MarcaA"), Some("Vestuário"), None));
    idx.add_product(Product::new(11, "Camisa Branca", Some("MarcaB"), Some("Vestuário"), None));

    let searcher = Searcher::new(&idx);
    let res = searcher.search_products("branca", 10);
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].0.id, 11);
}