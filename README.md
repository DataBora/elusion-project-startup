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
├── Dockerfile                  # Docker build configuration
├── docker-compose.yml          # Docker Compose with pipeline scheduler
├── elusion.toml                # materialization + output paths
├── connections.toml            # source declarations
├── .env.example                # secrets template — copy to .env
├── files/                      # put your source data files here
└── src/
    ├── main.rs                 # wiring — register your models here
    ├── bronze/
    │   ├── mod.rs
    │   └── brz_sales.rs
    ├── silver/
    │   ├── mod.rs
    │   └── slv_sales_enriched.rs
    └── gold/
        ├── mod.rs
        └── fct_sales_summary.rs
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

**4. Put your data files in the `files/` folder:**
```
files/
├── SalesData2022.csv
├── Products.csv
└── Customers.csv
```

**5. Configure your sources in `connections.toml`:**
```toml
[sources.raw_sales]
type = "csv"
path = "files/SalesData2022.csv"

[sources.raw_products]
type = "csv"
path = "files/Products.csv"

[sources.raw_customers]
type = "csv"
path = "files/Customers.csv"

# Fabric source example:
# [sources.raw_fabric]
# type = "fabric"
# abfss_path = "abfss://container@account.dfs.core.windows.net"
# file_path = "bronze/sales.parquet"
# tenant_id = "TENANT_ID"
# client_id = "CLIENT_ID"
# client_secret = "CLIENT_SECRET"
```

**6. Configure output paths in `elusion.toml`:**
```toml
[project]
name = "my_pipeline"
version = "1.0"

[materialization]
bronze = "parquet"
silver = "parquet"
gold = "parquet"

[output]
destination = "local"

[output.local]
bronze_path = "output/bronze"
silver_path = "output/silver"
gold_path = "output/gold"
```

**7. Write your models and wire them in `main.rs`, then run:**
```bash
cargo run
```

---

## 🐳 Running with Docker

The template includes a ready-to-use Docker setup with the pipeline scheduler built in.

**Build and run:**
```bash
docker-compose up --build
```

**Run in background:**
```bash
docker-compose up -d --build
```

**View logs:**
```bash
docker-compose logs -f
```

**Stop:**
```bash
docker-compose down
```

Output Parquet/Delta files will appear in your local `output/` folder via volume mount. The pipeline runs on the schedule defined in `main.rs` — default is every 1 minute.

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

**Silver model — `src/silver/slv_sales_enriched.rs`:**
```rust
use elusion::prelude::*;

pub const DEPS: &[&str] = &["brz_sales", "brz_customers", "brz_products"];

pub async fn model(ctx: NodeRegistry) -> ElusionResult<CustomDataFrame> {
    let sales = ctx.ref_bronze("brz_sales")?;
    let customers = ctx.ref_bronze("brz_customers")?;
    let products = ctx.ref_bronze("brz_products")?;

    sales
        .join_many([
            (customers, ["brz_sales.customerkey = brz_customers.customerkey"], "RIGHT"),
            (products, ["brz_sales.productkey = brz_products.productkey"], "LEFT OUTER"),
        ])
        .select([
            "brz_customers.customerkey",
            "brz_customers.firstname",
            "brz_customers.lastname",
            "brz_products.productname",
            "brz_sales.orderquantity",
        ])
        .elusion("slv_sales_enriched")
        .await
}
```

**Gold model — `src/gold/fct_sales_summary.rs` (Raw SQL):**
```rust
pub const DEPS: &[&str] = &["slv_sales_enriched"];

pub const SQL: &str = r#"
    SELECT
        customerkey,
        firstname,
        lastname,
        productname,
        SUM(orderquantity) AS total_quantity,
        AVG(orderquantity) AS avg_quantity,
        COUNT(*) AS order_count
    FROM slv_sales_enriched
    GROUP BY customerkey, firstname, lastname, productname
    HAVING SUM(orderquantity) > 10
    ORDER BY total_quantity ASC
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
    let scheduler = PipelineScheduler::new("1min", || async {
        ElusionProject::from_config("elusion.toml", "connections.toml")
            .await?
            .source("raw_sales")
            .source("raw_products")
            .source("raw_customers")
            .bronze_slice("brz_sales", bronze::brz_sales::DEPS, bronze::brz_sales::model)
            .bronze_slice("brz_customers", bronze::brz_customers::DEPS, bronze::brz_customers::model)
            .bronze_slice("brz_products", bronze::brz_products::DEPS, bronze::brz_products::model)
            .silver_slice("slv_sales_enriched",
                silver::slv_sales_enriched::DEPS,
                silver::slv_sales_enriched::model)
            .gold_sql_slice("fct_sales_summary",
                gold::fct_sales_summary::DEPS,
                gold::fct_sales_summary::SQL)
            .run()
            .await?;
        Ok(())
    }).await?;

    scheduler.shutdown().await?;
    Ok(())
}
```

---

## ⚡ What Happens When You Run
```
🚀 Elusion Project - Loading Configuration...
✅ Project config loaded: my_pipeline v1.0
✅ Source 'raw_sales' validated: files/SalesData2022.csv
✅ Source 'raw_customers' validated: files/Customers.csv
✅ Source 'raw_products' validated: files/Products.csv
✅ Configuration loaded successfully

🗺️  Execution Plan:
  Level 1 [Source]  raw_sales
  Level 1 [Source]  raw_customers
  Level 1 [Source]  raw_products
  Level 2 [Bronze]  brz_sales      → Parquet
  Level 2 [Bronze]  brz_customers  → Parquet
  Level 2 [Bronze]  brz_products   → Parquet
  Level 3 [Silver]  slv_sales_enriched → Parquet
  Level 4 [Gold]    fct_sales_summary  → Parquet

⚡ Running 3 nodes in parallel (Level 1)
⚡ Running 3 nodes in parallel (Level 2)
▶️  Running [Silver] slv_sales_enriched
▶️  Running [Gold]   fct_sales_summary

🎉 Project completed successfully!
======================================================================
Model                          Layer      Rows    Time
----------------------------------------------------------------------
raw_sales                      Source     29481   352ms
raw_customers                  Source     18151   368ms
raw_products                   Source     293     381ms
brz_sales                      Bronze     29481   4ms
brz_customers                  Bronze     18147   4ms
brz_products                   Bronze     293     3ms
slv_sales_enriched             Silver     37126   26ms
fct_sales_summary              Gold       5       65ms
======================================================================
Next job execution: 2026-04-05T09:40:00Z UTC Time
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