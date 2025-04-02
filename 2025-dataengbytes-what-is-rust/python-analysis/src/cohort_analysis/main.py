import polars as pl
from pathlib import Path

def main():
    # Find the data directory relative to where the script is run
    project_root = Path(__file__).parents[3]  # Go up 3 directories from script to project root
    data_dir = project_root / "data"
    
    # Read in the CSV files from data/
    sales_df = pl.read_csv(data_dir / "sales.csv")
    customers_df = pl.read_csv(data_dir / "customers.csv")

    # Clean sales data - filter out invalid dates
    sales_df = sales_df.filter(pl.col("sale_at").str.contains(r'^\d{4}-\d{2}-\d{2}'))

    # Convert sale_at to datetime - handle potentially invalid formats
    sales_df = sales_df.with_columns(
        pl.col("sale_at").str.strptime(pl.Datetime, "%Y-%m-%d %H:%M:%S", strict=False)
    )
    
    # Filter out null datetimes
    sales_df = sales_df.filter(~pl.col("sale_at").is_null())

    # Calculate first and last sale dates per customer
    customer_sales_dates = sales_df.group_by("customer_id").agg(
        pl.col("sale_at").min().alias("first_sale"),
        pl.col("sale_at").max().alias("last_sale")
    )

    # Calculate customer lifetime in days
    customer_lifetime = customer_sales_dates.with_columns(
        ((pl.col("last_sale") - pl.col("first_sale")).dt.total_days()).alias("lifetime_days")
    )

    # Create cohorts based on lifetime days
    customer_lifetime = customer_lifetime.with_columns(
        pl.when(pl.col("lifetime_days") < 30).then(pl.lit("0-30 days"))
        .when(pl.col("lifetime_days") < 90).then(pl.lit("30-90 days"))
        .when(pl.col("lifetime_days") < 180).then(pl.lit("90-180 days"))
        .when(pl.col("lifetime_days") < 365).then(pl.lit("180-365 days"))
        .otherwise(pl.lit("365+ days"))
        .alias("cohort")
    )

    # Merge sales data with cohort information
    merged_df = sales_df.join(customer_lifetime.select(["customer_id", "cohort"]), on="customer_id")

    # Calculate average sale price by cohort
    avg_price_by_cohort = merged_df.group_by("cohort").agg(
        pl.col("price").mean().alias("avg_sale_price"),
        pl.count().alias("num_sales")
    )

    # Create a simple custom sorting function
    def cohort_sort_key(cohort):
        order = {
            "0-30 days": 0,
            "30-90 days": 1,
            "90-180 days": 2,
            "180-365 days": 3,
            "365+ days": 4
        }
        return order.get(cohort, 999)  # Default high value for unknown cohorts
    
    # Convert to pandas for simple sorting and back to polars
    avg_price_by_cohort = avg_price_by_cohort.to_pandas()
    avg_price_by_cohort = avg_price_by_cohort.sort_values(by="cohort", key=lambda x: x.map(cohort_sort_key))
    avg_price_by_cohort = pl.from_pandas(avg_price_by_cohort)

    print("Average Sale Price by Customer Lifetime Cohort:")
    print(avg_price_by_cohort)
    
    return avg_price_by_cohort

if __name__ == "__main__":
    main()