use polars::prelude::*;

#[allow(dead_code)]
fn create_queries(_lf: LazyFrame) -> Vec<LazyFrame> {
    // Perform queries on the LazyFrame
    todo!()
}

pub fn collect_csv(csv_path: PlRefPath) -> PolarsResult<()> {
    // Create LazyFrame from a CSV
    let lf: LazyFrame = LazyCsvReader::new(csv_path)
        .with_has_header(true)
        .finish()?;

    let query = lf
        .filter(col("age").gt(lit(30)))
        .select([col("sleep_hours"), col("age")]);

    let df = query.collect()?;
    println!("{df:?}");

    Ok(())
}
