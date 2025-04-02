use anyhow::{Context, Result};
use polars::prelude::*;
use std::collections::HashMap;
use std::path::Path;

fn main() -> Result<()> {
    // Read sales data
    println!("Reading sales data...");
    let sales_path = Path::new("data/sales.csv");
    let sales_df = CsvReader::from_path(sales_path)
        .context("Failed to read sales.csv")?
        .has_header(true)
        .finish()
        .context("Failed to parse sales.csv")?;

    // Read customers data
    println!("Reading customers data...");
    let customers_path = Path::new("data/customers.csv");
    let _customers_df = CsvReader::from_path(customers_path)
        .context("Failed to read customers.csv")?
        .has_header(true)
        .finish()
        .context("Failed to parse customers.csv")?;

    // Create lazy frame for data processing
    let df_lazy = sales_df.lazy();

    // Filter out sales with invalid dates and parse dates
    println!("Processing dates...");
    let df_lazy = df_lazy
        // Filter rows starting with a digit (to keep valid date strings)
        .filter(col("sale_at").str().starts_with(lit("2")))
        // Convert date strings to timestamps
        .with_column(
            col("sale_at").alias("sale_date_str")
        )
        // Remove rows with null dates 
        .filter(col("sale_date_str").is_not_null());

    // Calculate first and last sale dates per customer
    println!("Calculating customer lifetimes...");
    let customer_sales = df_lazy
        .select([
            col("customer_id"),
            col("sale_date_str"),
            col("price"),
        ])
        .collect()
        .context("Failed to prepare customer sales data")?;

    // Manually extract and process customer data
    let mut customer_first_sales: HashMap<i64, String> = HashMap::new();
    let mut customer_last_sales: HashMap<i64, String> = HashMap::new();
    
    // Extract customer id and sale date columns
    let customer_id_series = customer_sales.column("customer_id")?.i64()?;
    let sale_date_series = customer_sales.column("sale_date_str")?.str()?;

    // Calculate first and last sale dates for each customer
    for i in 0..customer_id_series.len() {
        if let (Some(customer_id), Some(sale_date)) = (customer_id_series.get(i), sale_date_series.get(i)) {
            let customer_id = customer_id;
            let date_str = sale_date.to_string();
            
            // Update first sale date (if earlier than current)
            if let Some(current_first) = customer_first_sales.get(&customer_id) {
                if date_str < *current_first {
                    customer_first_sales.insert(customer_id, date_str.clone());
                }
            } else {
                customer_first_sales.insert(customer_id, date_str.clone());
            }
            
            // Update last sale date (if later than current)
            if let Some(current_last) = customer_last_sales.get(&customer_id) {
                if date_str > *current_last {
                    customer_last_sales.insert(customer_id, date_str);
                }
            } else {
                customer_last_sales.insert(customer_id, date_str);
            }
        }
    }

    // Process customer sales data to determine cohorts
    let mut customer_cohorts: HashMap<i64, String> = HashMap::new();
    
    // Simple function to calculate days between dates (approximation using string comparison)
    for (customer_id, first_sale) in &customer_first_sales {
        if let Some(last_sale) = customer_last_sales.get(customer_id) {
            // Simple estimate of lifetime by comparing year-month components
            let lifetime_days = if first_sale == last_sale {
                0 // Same day
            } else {
                // Extract year and month for simple comparison
                let first_parts: Vec<&str> = first_sale.split('-').collect();
                let last_parts: Vec<&str> = last_sale.split('-').collect();
                
                if first_parts.len() >= 2 && last_parts.len() >= 2 {
                    let first_year: i32 = first_parts[0].parse().unwrap_or(0);
                    let first_month: i32 = first_parts[1].parse().unwrap_or(0);
                    
                    let last_year: i32 = last_parts[0].parse().unwrap_or(0);
                    let last_month: i32 = last_parts[1].parse().unwrap_or(0);
                    
                    // Approximate days based on months difference
                    let months_diff = (last_year - first_year) * 12 + (last_month - first_month);
                    months_diff * 30 // Approximate days
                } else {
                    0
                }
            };
            
            // Assign cohort based on lifetime days
            let cohort = if lifetime_days < 30 {
                "0-30 days"
            } else if lifetime_days < 90 {
                "30-90 days"
            } else if lifetime_days < 180 {
                "90-180 days"
            } else if lifetime_days < 365 {
                "180-365 days"
            } else {
                "365+ days"
            };
            
            customer_cohorts.insert(*customer_id, cohort.to_string());
        }
    }
    
    // Build Series for cohort analysis
    let mut customer_ids = Vec::new();
    let mut cohorts = Vec::new();
    
    for (customer_id, cohort) in &customer_cohorts {
        customer_ids.push(*customer_id);
        cohorts.push(cohort.clone());
    }
    
    let customer_id_series = Series::new("customer_id", customer_ids);
    let cohort_series = Series::new("cohort", cohorts);
    
    let cohort_df = DataFrame::new(vec![customer_id_series, cohort_series])
        .context("Failed to create cohort dataframe")?;
    
    // Join with sales data
    println!("Joining with sales data...");
    let joined_df = customer_sales
        .lazy()
        .join(
            cohort_df.lazy(),
            [col("customer_id")],
            [col("customer_id")],
            JoinType::Inner.into(),
        )
        .collect()
        .context("Failed to join sales with cohorts")?;
    
    // Calculate average price by cohort
    println!("Calculating average prices by cohort...");
    let avg_price_by_cohort = joined_df
        .lazy()
        .group_by([col("cohort")])
        .agg([
            col("price").mean().alias("avg_sale_price"),
            col("price").count().alias("num_sales"),
        ])
        .collect()
        .context("Failed to calculate average price by cohort")?;
    
    // Extract data for custom sorting
    let cohorts = avg_price_by_cohort
        .column("cohort")?
        .str()?
        .into_iter()
        .collect::<Vec<_>>();
    
    let avg_prices = avg_price_by_cohort
        .column("avg_sale_price")?
        .f64()?
        .into_iter()
        .collect::<Vec<_>>();
    
    let num_sales = avg_price_by_cohort
        .column("num_sales")?
        .u32()?
        .into_iter()
        .collect::<Vec<_>>();
    
    // Prepare results for sorting
    let mut results = Vec::new();
    for i in 0..cohorts.len() {
        if let (Some(cohort), Some(avg_price), Some(count)) = (cohorts[i], avg_prices[i], num_sales[i]) {
            results.push((cohort.to_string(), avg_price, count));
        }
    }
    
    // Sort by cohort order
    let cohort_order: HashMap<&str, usize> = [
        ("0-30 days", 0),
        ("30-90 days", 1),
        ("90-180 days", 2),
        ("180-365 days", 3),
        ("365+ days", 4),
    ]
    .iter()
    .cloned()
    .collect();
    
    results.sort_by_key(|(cohort, _, _)| cohort_order.get(cohort.as_str()).unwrap_or(&999));
    
    // Print results
    println!("\nAverage Sale Price by Customer Lifetime Cohort:");
    println!("-----------------------------------------------");
    println!("{:<15} {:<20} {:<10}", "Cohort", "Avg Sale Price", "Num Sales");
    println!("-----------------------------------------------");
    for (cohort, avg_price, num_sales) in results {
        println!("{:<15} {:<20.2} {:<10}", cohort, avg_price, num_sales);
    }
    
    Ok(())
}