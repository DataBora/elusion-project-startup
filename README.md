# 🦎 Elusion Project — Starter Template

<div align="center">

[![Crates.io version](https://img.shields.io/crates/v/elusion?style=for-the-badge&color=brightgreen&logo=rust)](https://crates.io/crates/elusion)
[![docs.rs](https://img.shields.io/docsrs/elusion?style=for-the-badge&color=blue&logo=docs.rs)](https://docs.rs/elusion)
[![GitHub license](https://img.shields.io/github/license/DataBora/elusion?style=for-the-badge&color=green&logo=github)](https://github.com/DataBora/elusion/blob/main/LICENSE)

</div>

---

> *Welcome to Elusion Project — your starting point for building production-grade data pipelines in pure Rust.*

This repository is a ready-to-use starter template for **Elusion Project**, the Medallion Architecture Pipeline Framework built into the [Elusion](https://crates.io/crates/elusion) library.

Clone it, configure your sources, write your models, and run. That's it.

---

## 🏗️ What's Inside
```
elusion-project-startup/
├── Cargo.toml                  # elusion dependency already configured
├── elusion.toml                # materialization + output paths
├── connections.toml            # source declarations
├── .env.example                # secrets template — copy to .env
└── src/
    ├── main.rs                 # wiring — register your models here
    ├── bronze/
    │   ├── mod.rs
    │   └── brz_example.rs      # example bronze model
    ├── silver/
    │   ├── mod.rs
    │   └── slv_example.rs      # example silver model
    └── gold/
        ├── mod.rs
        └── fct_example.rs      # example gold model (SQL)
```

---

## 🚀 Getting Started

**1. Clone the repo:**
```bash
git clone https://github.com/DataBora/elusion-project-startup
cd elusion-project-startup
```

**2. Copy the secrets template:**
```bash
cp .env.example .env
```

**3. Edit `.env` with your credentials (for Fabric sources):**
```
TENANT_ID=your-tenant-id
CLIENT_ID=your-client-id
CLIENT_SECRET=your-client-secret
```

**4. Configure your sources in `connections.toml`:**
```toml
[sources.raw_sales]
type = "csv"
path = "C:\\Data\\SalesData.csv"

# Fabric source example:
# [sources.raw_fabric]
# type = "fabric"
# abfss_path = "abfss://container@account.dfs.core.windows.net"
# file_path = "bronze/sales.parquet"
# tenant_id = "TENANT_ID"
# client_id = "CLIENT_ID"
# client_secret = "CLIENT_SECRET"
```

**5. Configure output paths in `elusion.toml`:**
```toml
[project]
name = "my_pipeline"
version = "1.0"

[materialization]
bronze = "parquet"
silver = "parquet"
gold = "delta"

[output]
destination = "local"

[output.local]
bronze_path = "C:\\Data\\output\\bronze"
silver_path = "C:\\Data\\output\\silver"
gold_path = "C:\\Data\\output\\gold"
```

**6. Write your models and wire them in `main.rs`, then run:**
```bash
cargo run
```

---

## 📐 Model Structure

Each model is a separate file that declares its dependencies and transformation logic.

**Bronze model — `src/bronze/brz_sales.rs`:**
```rust
use elusion::prelude::*;

pub const DEPS: &[&str] = &["raw_sales"];

pub async fn model(ctx: NodeRegistry) -> ElusionResult<CustomDataFrame> {
    ctx.ref_source("raw_sales")?
        .select(["customerkey", "productkey", "orderquantity", "orderdate"])
        .filter("orderquantity > 0")
        .filter("customerkey IS NOT NULL")
        .elusion("brz_sales")
        .await
}
```

**Gold model — `src/gold/fct_summary.rs` (Raw SQL):**
```rust
pub const DEPS: &[&str] = &["slv_enriched"];

pub const SQL: &str = r#"
    SELECT
        customerkey,
        productname,
        SUM(orderquantity) AS total_quantity,
        COUNT(*) AS order_count
    FROM slv_enriched
    GROUP BY customerkey, productname
    ORDER BY total_quantity DESC
"#;
```

**`src/main.rs` — wiring only:**
```rust
use elusion::prelude::*;

mod bronze;
mod silver;
mod gold;

#[tokio::main]
async fn main() -> ElusionResult<()> {
    ElusionProject::from_config("elusion.toml", "connections.toml")
        .await?
        .source("raw_sales")
        .bronze_slice("brz_sales", bronze::brz_sales::DEPS, bronze::brz_sales::model)
        .silver_slice("slv_enriched", silver::slv_enriched::DEPS, silver::slv_enriched::model)
        .gold_sql_slice("fct_summary", gold::fct_summary::DEPS, gold::fct_summary::SQL)
        .run()
        .await
}
```

---

## ⚡ What Happens When You Run
```
🚀 Elusion Project - Loading Configuration...
✅ Project config loaded: my_pipeline v1.0
✅ Source 'raw_sales' validated

🗺️  Execution Plan:
  Level 1 [Source]      raw_sales
  Level 2 [Bronze]      brz_sales      → Parquet
  Level 3 [Silver]      slv_enriched   → Parquet
  Level 4 [Gold]        fct_summary    → Delta

⚡ Sources load in parallel
⚡ Bronze models clean in parallel
▶️  Silver enrichment runs after bronze
▶️  Gold aggregation runs last

🎉 Project completed successfully!
```

---

## 📚 Learn More

- 📦 [Elusion on crates.io](https://crates.io/crates/elusion)
- 📖 [Documentation](https://docs.rs/elusion)
- 🐙 [Elusion GitHub](https://github.com/DataBora/elusion)
- 🎓 [Udemy Course](https://www.udemy.com/course/rust-data-engineering-analytics-elusion/)
- 🎸 [Elusion Anthem](https://www.youtube.com/watch?v=sk8mVplzVI4)

---

<div align="center">
Built with 🦀 Rust + Apache DataFusion
</div>