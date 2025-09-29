use megastore_search::index::InvertedIndex;
use megastore_search::model::Product;
use megastore_search::search::Searcher;

fn main() {
    let idx = InvertedIndex::new();

    idx.add_product(Product::new(
        1,
        "Smartphone SuperX 64GB",
        Some("ZenTech"),
        Some("Eletrônicos"),
        Some("Smartphone com câmera dupla"),
    ));
    idx.add_product(Product::new(
        2,
        "Camiseta Polo Masculina",
        Some("ClothBrand"),
        Some("Vestuário"),
        Some("100% algodão"),
    ));
    idx.add_product(Product::new(
        3,
        "Smart TV 55\"",
        Some("ViewTech"),
        Some("Eletrônicos"),
        Some("4K UHD Smart TV"),
    ));

    let searcher = Searcher::new(&idx);

    let q = "smartphone 4k";
    println!("Buscando por: '{}'", q);

    let results = searcher.search_products(q, 10);
    for (prod, score) in results {
        println!("id={} score={:.2} name={}", prod.id, score, prod.name);
    }
}