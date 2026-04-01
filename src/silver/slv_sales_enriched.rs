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
            "brz_products.productcost",
            "brz_products.productprice",
            "brz_sales.orderquantity",
        ])
        .elusion("slv_sales_enriched")
        .await
}