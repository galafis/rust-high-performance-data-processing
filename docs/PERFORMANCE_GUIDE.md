# Guia de Performance - Processamento de Dados com Rust

## 📊 Visão Geral

Este guia apresenta técnicas e melhores práticas para otimizar o processamento de dados usando Rust. O objetivo é demonstrar como aproveitar as características únicas de Rust para construir soluções de alto desempenho.

## 🚀 Técnicas de Otimização

### 1. Processamento em Memória

Rust oferece controle preciso sobre a memória, permitindo otimizações que seriam difíceis ou impossíveis em linguagens com garbage collection.

**Vantagens:**
- Zero-cost abstractions
- Ausência de overhead de GC
- Controle fino sobre alocações

**Exemplo:**
```rust
let records: Vec<DataRecord> = (0..1_000_000)
    .map(|i| DataRecord { id: i, value: (i as f64) * 1.5 })
    .collect();
```

### 2. Paralelização com Rayon (Melhoria Futura)

Para operações que podem ser paralelizadas, considere adicionar a biblioteca Rayon como dependência:

```rust
// Exemplo: requer adicionar rayon = "1.10" ao Cargo.toml
use rayon::prelude::*;

let sum: f64 = records.par_iter()
    .map(|r| r.value)
    .sum();
```

### 3. Processamento de CSV Eficiente

O módulo `csv_processing` demonstra o uso da biblioteca `csv` com desserialização automática usando Serde:

```rust
let mut rdr = ReaderBuilder::new()
    .has_headers(true)
    .from_path(file_path)?;

for result in rdr.deserialize() {
    let passenger: TitanicPassenger = result?;
    // Processar dados...
}
```

**Dicas de Performance:**
- Use tipos apropriados (evite strings quando números são suficientes)
- Utilize `Option<T>` para campos opcionais
- Considere usar buffers maiores com `.buffer_capacity()`

## 📈 Benchmarks

Os benchmarks incluídos no projeto demonstram a performance em diferentes cenários:

### Resultados Típicos

| Operação | Tamanho do Dataset | Tempo Médio |
|----------|-------------------|-------------|
| Processamento em Memória | 100K registros | ~1-2 ms |
| Processamento em Memória | 1M registros | ~10-30 ms |
| Análise CSV Titanic | 891 registros | ~1-2 ms |

### Executando Benchmarks

```bash
cargo bench
```

Os resultados serão salvos em `target/criterion/` com gráficos HTML detalhados.

## 🎯 Melhores Práticas

### 1. Use Iteradores ao Invés de Loops

Iteradores em Rust são otimizados pelo compilador e geralmente resultam em código mais eficiente:

```rust
// Bom
let avg = records.iter().map(|r| r.value).sum::<f64>() / records.len() as f64;

// Evitar (menos idiomático)
let mut sum = 0.0;
for record in &records {
    sum += record.value;
}
```

### 2. Minimize Alocações

Reutilize buffers quando possível:

```rust
let mut buffer = String::new();
// Reutilizar buffer em loop
```

### 3. Use Tipos Adequados

- `u32` ao invés de `i64` quando valores são sempre positivos
- `f32` ao invés de `f64` quando precisão extra não é necessária
- `&str` ao invés de `String` para referências

### 4. Aproveite o Sistema de Tipos

O compilador Rust elimina muitas verificações em tempo de execução:

```rust
// O compilador garante que isso é seguro
let slice = &records[0..100];
```

## 🔍 Análise de Performance

### Ferramentas Recomendadas

1. **Criterion** - Benchmarking estatisticamente rigoroso
2. **cargo-flamegraph** - Visualização de perfil de execução
3. **perf** (Linux) - Análise de performance a nível de CPU

### Usando cargo-flamegraph

```bash
cargo install flamegraph
cargo flamegraph
```

## 📚 Recursos Adicionais

- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Documentação do Criterion](https://bheisler.github.io/criterion.rs/book/)

## 🎓 Exemplos Práticos

### Processamento Paralelo de Múltiplos Arquivos (Melhoria Futura)

Com a biblioteca Rayon adicionada como dependência:

```rust
// Exemplo: requer adicionar rayon = "1.10" ao Cargo.toml
use rayon::prelude::*;

let files = vec!["data1.csv", "data2.csv", "data3.csv"];
let results: Vec<_> = files.par_iter()
    .map(|file| process_titanic_csv(file))
    .collect();
```

### Streaming de Dados Grandes

Para arquivos muito grandes que não cabem na memória:

```rust
let mut rdr = csv::Reader::from_path(path)?;
for result in rdr.deserialize() {
    let record: DataRecord = result?;
    // Processar um registro por vez
}
```

## 🔬 Medindo Performance

Sempre meça antes de otimizar:

```rust
use std::time::Instant;

let start = Instant::now();
// Código a medir
let duration = start.elapsed();
println!("Tempo: {:?}", duration);
```

## 💡 Conclusão

Rust oferece um equilíbrio único entre performance, segurança e ergonomia. Ao seguir estas práticas, você pode construir sistemas de processamento de dados que são:

- **Rápidos**: Performance comparável a C/C++
- **Seguros**: Sem data races ou memory leaks
- **Sustentáveis**: Código claro e fácil de manter

---

**Nota**: Este guia é um documento vivo e será atualizado com novas técnicas e exemplos conforme o projeto evolui.
