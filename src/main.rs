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
            .silver_slice("slv_sales_enriched", silver::slv_sales_enriched::DEPS, silver::slv_sales_enriched::model)
            .gold_sql_slice("fct_sales_summary", gold::fct_sales_summary::DEPS, gold::fct_sales_summary::SQL)
            .run()
            .await?;

        Ok(())

    }).await?;

    scheduler.shutdown().await?;

    Ok(())
}




