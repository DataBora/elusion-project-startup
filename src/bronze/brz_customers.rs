use elusion::prelude::*;

pub const DEPS: &[&str] = &["raw_customers"];

pub async fn model(ctx: NodeRegistry) -> ElusionResult<CustomDataFrame> {
    ctx.ref_source("raw_customers")?
        .select(["customerkey", "firstname", "lastname", "emailaddress"])
        .filter("customerkey IS NOT NULL")
        .elusion("brz_customers")
        .await
}