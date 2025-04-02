#!/usr/bin/env python3

import polars as pl
from datetime import datetime

# Read in the CSV files from data/
sales_df = pl.read_csv("data/sales.csv")
customers_df = pl.read_csv("data/customers.csv")

# Clean sales data - filter out invalid dates
sales_df = sales_df.filter(pl.col("sale_at").str.contains(r'^\d{4}-\d{2}-\d{2}'))

# Convert sale_at to datetime
sales_df = sales_df.with_columns(
    pl.col("sale_at").str.strptime(pl.Datetime, "%Y-%m-%d %H:%M:%S")
)

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
    pl.when(pl.col("lifetime_days") < 30).then("0-30 days")
    .when(pl.col("lifetime_days") < 90).then("30-90 days")
    .when(pl.col("lifetime_days") < 180).then("90-180 days")
    .when(pl.col("lifetime_days") < 365).then("180-365 days")
    .otherwise("365+ days")
    .alias("cohort")
)

# Merge sales data with cohort information
merged_df = sales_df.join(customer_lifetime.select(["customer_id", "cohort"]), on="customer_id")

# Calculate average sale price by cohort
avg_price_by_cohort = merged_df.group_by("cohort").agg(
    pl.col("price").mean().alias("avg_sale_price"),
    pl.count().alias("num_sales")
)

# Sort cohorts in a logical order
cohort_order = ["0-30 days", "30-90 days", "90-180 days", "180-365 days", "365+ days"]
avg_price_by_cohort = avg_price_by_cohort.with_columns(
    pl.col("cohort").cast(pl.Categorical).cat.set_ordering("lexical", ordering=cohort_order)
).sort("cohort")

print("Average Sale Price by Customer Lifetime Cohort:")
print(avg_price_by_cohort)