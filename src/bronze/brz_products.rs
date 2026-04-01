use elusion::prelude::*;

pub const DEPS: &[&str] = &["raw_products"];

pub async fn model(ctx: NodeRegistry) -> ElusionResult<CustomDataFrame> {
    ctx.ref_source("raw_products")?
        .select(["productkey", "productname", "productcost", "productprice"])
        .filter("productkey IS NOT NULL")
        .elusion("brz_products")
        .await
}