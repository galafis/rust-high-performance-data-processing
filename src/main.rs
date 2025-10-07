use datafusion::prelude::*;
use datafusion::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new SessionContext
    let ctx = SessionContext::new();

    // Create dummy data
    let _ = std::fs::create_dir_all("data");
    let mut wtr = csv::Writer::from_path("data/sales.csv")?;
    wtr.write_record(&["product_category", "sales"])?;
    wtr.write_record(&["Electronics", "1500.50"])?;
    wtr.write_record(&["Books", "300.20"])?;
    wtr.write_record(&["Clothing", "800.75"])?;
    wtr.write_record(&["Electronics", "2500.00"])?;
    wtr.write_record(&["Home Goods", "1200.90"])?;
    wtr.write_record(&["Books", "150.00"])?;
    wtr.flush()?;

    // Register a CSV file as a table
    ctx.register_csv("sales", "data/sales.csv", CsvReadOptions::new()).await?;

    // Define a SQL query
    let sql = "SELECT product_category, SUM(sales) as total_sales FROM sales GROUP BY product_category ORDER BY total_sales DESC LIMIT 5";

    // Execute the query
    let df = ctx.sql(sql).await?;

    // Print the results
    df.show().await?;

    Ok(())
}
