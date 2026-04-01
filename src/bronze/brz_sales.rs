use elusion::prelude::*;

pub const DEPS: &[&str] = &["raw_sales"];

pub async fn model(ctx: NodeRegistry) -> ElusionResult<CustomDataFrame> {
    ctx.ref_source("raw_sales")?
        .select(["customerkey", "productkey", "orderquantity", "orderdate"])
        .filter("orderquantity > 0")
        .filter("customerkey IS NOT NULL")
        .filter("productkey IS NOT NULL")
        .elusion("brz_sales")
        .await
}