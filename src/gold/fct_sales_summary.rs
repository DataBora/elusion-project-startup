pub const DEPS: &[&str] = &["slv_sales_enriched"];

pub const SQL: &str = r#"
    SELECT
        customerkey,
        firstname,
        lastname,
        productname,
        SUM(orderquantity) AS total_quantity,
        AVG(orderquantity) AS avg_quantity,
        COUNT(*) AS order_count,
        SUM(productprice) AS total_price,
        AVG(productcost) AS avg_cost
    FROM slv_sales_enriched
    GROUP BY
        customerkey,
        firstname,
        lastname,
        productname
    HAVING
        SUM(orderquantity) > 10
        AND AVG(orderquantity) < 100
    ORDER BY
        total_quantity ASC,
        productname DESC
"#;