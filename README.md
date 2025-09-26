# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📌 Descrição do Projeto
Este projeto implementa um **sistema de busca otimizado** para o catálogo de produtos da **MegaStore**, um e-commerce fictício com milhões de itens.  
O objetivo é permitir **buscas rápidas, precisas e escaláveis**, garantindo que os clientes encontrem os produtos desejados de forma eficiente.  

O sistema foi desenvolvido em **Rust**, utilizando um **índice invertido** para indexação e recuperação de produtos por diferentes critérios de busca (nome, marca, categoria e descrição).

---

## ⚙️ Tecnologias Utilizadas
- **Linguagem**: [Rust](https://www.rust-lang.org/) (Edition 2021)  
- **Gerenciador de pacotes**: Cargo  
- **Crates principais**:
  - [`regex`](https://crates.io/crates/regex) → limpeza de texto na tokenização  
  - [`lazy_static`](https://crates.io/crates/lazy_static) → inicialização estática de regex, stopwords e stemmer  
  - [`rust-stemmers`](https://crates.io/crates/rust-stemmers) → stemming para português  
  - [`serde`](https://crates.io/crates/serde) + [`serde_json`](https://crates.io/crates/serde_json) → serialização e desserialização  
  - [`parking_lot`](https://crates.io/crates/parking_lot) → locks eficientes para concorrência  
- **Ferramentas de teste**: `cargo test` (unitários e de integração)

---

## 🚀 Como Executar o Sistema de Busca

### 1. Clonar o repositório
```bash
git clone https://github.com/seu-usuario/megastore-search.git
cd megastore-search
```

### 2. Compilar o projeto
```bash
cargo build
```

### 3. Executar o sistema (exemplo CLI em `main.rs`)
```bash
cargo run
```

Saída esperada (exemplo):
```
Buscando por: 'smartphone 4k'
id=1 score=2.00 name=Smartphone SuperX 64GB
id=3 score=1.00 name=Smart TV 55"
```

---

## 🧪 Como Executar os Testes
O projeto contém testes unitários e de integração no diretório `tests/`.

### Rodar todos os testes
```bash
cargo test
```

### Rodar apenas um teste específico
```bash
cargo test basic_index_and_search
```

---

## 📖 Exemplos de Uso

### Inserção de produtos no índice
```rust
let idx = InvertedIndex::new();

idx.add_product(Product::new(
    1, "Smartphone SuperX 64GB", Some("ZenTech"),
    Some("Eletrônicos"), Some("Smartphone com câmera dupla")
));
idx.add_product(Product::new(
    2, "Camiseta Polo Masculina", Some("ClothBrand"),
    Some("Vestuário"), Some("100% algodão")
));
```

### Consultas de busca
```rust
let searcher = Searcher::new(&idx);

// Consulta simples
let results = searcher.search_products("smartphone", 10);

// Consulta com múltiplos termos
let results = searcher.search_products("smartphone 4k", 10);
```

Saída típica:
```
id=1 score=2.00 name=Smartphone SuperX 64GB
id=3 score=1.00 name=Smart TV 55"
```

---

## 🏗️ Arquitetura do Sistema
O projeto está organizado nos seguintes módulos:

- **`model.rs`** → definição da estrutura `Product`  
- **`tokenizer.rs`** → tokenização com *stopwords* e *stemming*  
- **`index.rs`** → implementação do índice invertido (armazenamento e indexação de produtos)  
- **`search.rs`** → algoritmos de busca e ranqueamento simples  
- **`main.rs`** → exemplo de uso via CLI  
- **`tests/`** → testes unitários e de integração  

Fluxo resumido:
1. Produtos são adicionados ao índice.  
2. Cada campo de texto é tokenizado, normalizado e armazenado em um **índice invertido** (`HashMap<String, Vec<Posting>>`).  
3. Consultas são tokenizadas da mesma forma e comparadas contra o índice.  
4. Resultados são ranqueados por frequência de termos (TF).  

---

## 🧩 Algoritmos e Estruturas de Dados Utilizados
- **Índice invertido** → estrutura baseada em `HashMap<String, Vec<Posting>>`, onde cada termo aponta para uma lista de produtos que o contêm.  
- **HashMap** → acesso O(1) médio para tokens e produtos.  
- **Posting List** → lista de pares `(product_id, term_frequency)`.  
- **Tokenização** → normalização com regex, remoção de stopwords e stemming em português.  
- **Busca** → soma de frequências como score básico, ordenação por relevância.  

---

## 📊 Considerações sobre Desempenho e Escalabilidade
- **Desempenho atual**:
  - Inserção de produto: O(k), onde k = número de tokens distintos do produto.  
  - Consulta: O(m + n log n), onde m = tamanho total das posting lists dos termos consultados, n = número de matches.  
- **Escalabilidade**:
  - Para milhões de produtos, o índice em memória pode ser otimizado via compressão de posting lists.  
  - Suporte a persistência em disco (ex.: `sled`, `tantivy`).  
  - Sharding do índice para consultas distribuídas.  
- **Melhorias futuras**:
  - Ranking por **BM25** em vez de apenas TF.  
  - Suporte a **autocomplete** e **busca aproximada (fuzzy)**.  
  - Cache de consultas mais frequentes.  

---

## 🤝 Contribuições
Contribuições são bem-vindas!  
Para contribuir:
1. Faça um fork do projeto.  
2. Crie uma branch com sua feature/bugfix:  
   ```bash
   git checkout -b minha-feature
   ```
3. Faça commit das alterações:  
   ```bash
   git commit -m "Adiciona nova feature"
   ```
4. Envie um pull request.
