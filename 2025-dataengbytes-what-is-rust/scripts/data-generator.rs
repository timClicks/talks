use chrono::{Datelike, NaiveDate, NaiveTime, Weekday};
use std::io;

use rand::prelude::*;

use clap::Parser;

// Define command line arguments
#[derive(Parser)]
#[command(
    author,
    version,
    about = "Generate synthetic data warehouse sales data with controlled error rates"
)]
struct Args {
    /// Number of products to generate
    #[arg(long, default_value_t = 100)]
    products: usize,

    /// Number of sites to generate
    #[arg(long, default_value_t = 20)]
    sites: usize,

    /// Number of customers to generate
    #[arg(long, default_value_t = 500)]
    customers: usize,

    /// Number of sales to generate
    #[arg(long, default_value_t = 10000)]
    sales: usize,

    /// Error rate for products (0.0-1.0)
    #[arg(long, default_value_t = 0.02)]
    product_errors: f64,

    /// Error rate for sites (0.0-1.0)
    #[arg(long, default_value_t = 0.03)]
    site_errors: f64,

    /// Error rate for customers (0.0-1.0)
    #[arg(long, default_value_t = 0.02)]
    customer_errors: f64,

    /// Error rate for sales (0.0-1.0)
    #[arg(long, default_value_t = 0.10)]
    sales_errors: f64,

    /// Random seed for reproducibility
    #[arg(long, default_value_t = 42)]
    seed: u64,

    /// Output directory for CSV files
    #[arg(short, long, default_value = "data")]
    output_dir: String,
}

// Data structures
struct Product {
    name: String,
    sku: String,
    category: String,
}

struct Site {
    id: u32,
    name: String,
    region: String,
}

struct Customer {
    id: u32,
    discount: f64,
}

struct Sale {
    id: u32,
    product_sku: String,
    currency: String,
    price: f64,
    sale_at: String,
    site_id: u32,
    customer_id: u32,
    date: String,
    time: String,
    is_weekend: bool,
    is_holiday: bool,
}

// Data generator
struct DataGenerator {
    product_error_rate: f64,
    site_error_rate: f64,
    customer_error_rate: f64,
    sales_error_rate: f64,
    rng: StdRng,
}

impl DataGenerator {
    // Initialize generator with error rates and optional seed
    fn new(
        product_error_rate: f64,
        site_error_rate: f64,
        customer_error_rate: f64,
        sales_error_rate: f64,
        seed: Option<u64>,
    ) -> Self {
        let rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_os_rng(),
        };

        Self {
            product_error_rate,
            site_error_rate,
            customer_error_rate,
            sales_error_rate,
            rng,
        }
    }

    // Helper to decide if a value should be corrupted
    fn should_corrupt(&mut self, error_rate: f64) -> bool {
        self.rng.random::<f64>() < error_rate
    }

    // Helper to create a corrupted string value
    fn corrupt_string(&mut self) -> String {
        let corruptions = ["NULL", "n/a", "", "undefined"];
        corruptions.choose(&mut self.rng).unwrap().to_string()
    }

    // Helper to create a corrupted date
    fn corrupt_date(&mut self) -> String {
        "99-99-9999".to_string()
    }

    // Helper to create a corrupted time
    fn corrupt_time(&mut self) -> String {
        "99:99:99".to_string()
    }

    // Generate product data
    fn generate_products(&mut self, count: usize) -> Vec<Product> {
        let categories = ["Electronics", "Clothing", "Home", "Kitchen", "Food"];
        let product_types = [
            (
                "Electronics",
                &["TV", "Laptop", "Phone", "Tablet", "Camera", "Headphones"][..],
            ),
            (
                "Clothing",
                &["Shirt", "Pants", "Dress", "Jacket", "Socks", "Hat"][..],
            ),
            (
                "Home",
                &["Chair", "Table", "Sofa", "Lamp", "Rug", "Curtains"][..],
            ),
            (
                "Kitchen",
                &["Blender", "Toaster", "Kettle", "Knife", "Pot", "Pan"][..],
            ),
            (
                "Food",
                &["Bread", "Milk", "Eggs", "Cheese", "Meat", "Fruit"][..],
            ),
        ];
        let adjectives = [
            "Premium",
            "Deluxe",
            "Standard",
            "Basic",
            "Professional",
            "Eco",
            "Smart",
        ];

        let mut products = Vec::with_capacity(count);

        for i in 1..=count {
            let category = *categories.choose(&mut self.rng).unwrap();

            if self.should_corrupt(self.product_error_rate) {
                // Create corrupt product
                products.push(Product {
                    name: self.corrupt_string(),
                    sku: format!(
                        "{}-{}",
                        category.chars().take(3).collect::<String>().to_uppercase(),
                        i
                    ),
                    category: if self.should_corrupt(0.5) {
                        self.corrupt_string()
                    } else {
                        category.to_string()
                    },
                });
            } else {
                // Create valid product
                let product_type = product_types
                    .iter()
                    .find(|(cat, _)| *cat == category)
                    .map(|(_, types)| *types.choose(&mut self.rng).unwrap())
                    .unwrap_or("Item");

                let adjective = *adjectives.choose(&mut self.rng).unwrap();

                products.push(Product {
                    name: format!("{} {}", adjective, product_type),
                    sku: format!(
                        "{}-{}",
                        category.chars().take(3).collect::<String>().to_uppercase(),
                        i
                    ),
                    category: category.to_string(),
                });
            }
        }

        products
    }

    // Generate site data
    fn generate_sites(&mut self, count: usize) -> Vec<Site> {
        let regions = ["North", "South", "East", "West", "Central"];
        let site_types = ["Store", "Outlet", "Warehouse", "Market"];
        let locations = [
            "Auckland",
            "Wellington",
            "Christchurch",
            "Hamilton",
            "Tauranga",
            "Dunedin",
            "Palmerston North",
            "Napier",
            "New Plymouth",
            "Whangarei",
        ];

        let mut sites = Vec::with_capacity(count);

        for i in 1..=count {
            if self.should_corrupt(self.site_error_rate) {
                // Create corrupt site
                sites.push(Site {
                    id: i as u32,
                    name: self.corrupt_string(),
                    region: if self.should_corrupt(0.5) {
                        self.corrupt_string()
                    } else {
                        regions.choose(&mut self.rng).unwrap().to_string()
                    },
                });
            } else {
                // Create valid site
                let site_type = *site_types.choose(&mut self.rng).unwrap();
                let location = *locations.choose(&mut self.rng).unwrap();

                sites.push(Site {
                    id: i as u32,
                    name: format!("{} {}", site_type, location),
                    region: regions.choose(&mut self.rng).unwrap().to_string(),
                });
            }
        }

        sites
    }

    // Generate customer data
    fn generate_customers(&mut self, count: usize) -> Vec<Customer> {
        // Most customers have no discount, some have small discounts, few have large discounts
        let discount_distribution = [
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // 80% chance of no discount
            0.05, 0.05, // 10% chance of 5% discount
            0.1, 0.15, 0.2, // 10% chance of larger discount
        ];

        let mut customers = Vec::with_capacity(count);

        for i in 1..=count {
            if self.should_corrupt(self.customer_error_rate) {
                // Create corrupt customer
                customers.push(Customer {
                    id: i as u32,
                    discount: -1.0, // Invalid discount
                });
            } else {
                // Create valid customer
                let discount = *discount_distribution.choose(&mut self.rng).unwrap();

                customers.push(Customer {
                    id: i as u32,
                    discount,
                });
            }
        }

        customers
    }

    // Generate realistic price based on product category
    fn generate_price(&mut self, category: &str, discount: f64) -> f64 {
        // Base prices vary by category
        let base_price = match category {
            "Electronics" => self.rng.random_range(50.0..2000.0),
            "Clothing" => self.rng.random_range(10.0..200.0),
            "Home" => self.rng.random_range(30.0..500.0),
            "Kitchen" => self.rng.random_range(15.0..300.0),
            "Food" => self.rng.random_range(2.0..50.0),
            _ => self.rng.random_range(5.0..100.0),
        };

        // Apply the discount
        let discounted_price = base_price * (1.0 - discount);

        // Make prices look realistic by adjusting cents
        // 60% end in .99, 20% end in .95, 20% end in .00
        let cents_pattern = self.rng.random_range(0..100);
        let price = (discounted_price.floor() as i32) as f64;

        if cents_pattern < 60 {
            price + 0.99
        } else if cents_pattern < 80 {
            price + 0.95
        } else {
            price
        }
    }

    // Generate sales data
    fn generate_sales(
        &mut self,
        count: usize,
        products: &[Product],
        sites: &[Site],
        customers: &[Customer],
    ) -> Vec<Sale> {
        let currencies = ["NZD", "AUD", "USD", "EUR", "GBP"];
        let holidays = [
            "01-01", // New Year's
            "06-02", // Waitangi Day
            "25-04", // ANZAC Day
            "25-12", // Christmas
            "26-12", // Boxing Day
        ];

        let mut sales = Vec::with_capacity(count);

        for i in 1..=count {
            let product_idx = self.rng.random_range(0..products.len());
            let site_idx = self.rng.random_range(0..sites.len());
            let customer_idx = self.rng.random_range(0..customers.len());

            let product = &products[product_idx];
            let site = &sites[site_idx];
            let customer = &customers[customer_idx];

            // Generate random date between 2020-01-01 and 2024-12-31
            let days_since_epoch = self.rng.random_range(18262..20454); // 2020-01-01 to 2024-12-31
            let date = NaiveDate::from_ymd_opt(1970, 1, 1)
                .unwrap()
                .checked_add_days(chrono::Days::new(days_since_epoch))
                .unwrap();

            // Generate random time
            let hour = self.rng.random_range(8..22); // Business hours
            let minute = self.rng.random_range(0..60);
            let second = self.rng.random_range(0..60);
            let time = NaiveTime::from_hms_opt(hour, minute, second).unwrap();

            // Check if it's a weekend
            let is_weekend = matches!(date.weekday(), Weekday::Sat | Weekday::Sun);

            // Check if it's a holiday
            let month_day = format!("{:02}-{:02}", date.month(), date.day());
            let is_holiday = holidays.contains(&&month_day[..]);

            // Format date and time as strings
            let date_str = date.format("%Y-%m-%d").to_string();
            let time_str = time.format("%H:%M:%S").to_string();

            if self.should_corrupt(self.sales_error_rate) {
                // Create corrupt sale record
                let corruption_type = self.rng.random_range(0..5);

                match corruption_type {
                    0 => {
                        // Corrupt product SKU
                        sales.push(Sale {
                            id: i as u32,
                            product_sku: self.corrupt_string(),
                            currency: currencies.choose(&mut self.rng).unwrap().to_string(),
                            price: self.generate_price(&product.category, customer.discount),
                            sale_at: format!("{} {}", date_str, time_str),
                            site_id: site.id,
                            customer_id: customer.id,
                            date: date_str,
                            time: time_str,
                            is_weekend,
                            is_holiday,
                        });
                    }
                    1 => {
                        // Corrupt date
                        sales.push(Sale {
                            id: i as u32,
                            product_sku: product.sku.clone(),
                            currency: currencies.choose(&mut self.rng).unwrap().to_string(),
                            price: self.generate_price(&product.category, customer.discount),
                            sale_at: format!("{} {}", self.corrupt_date(), time_str),
                            site_id: site.id,
                            customer_id: customer.id,
                            date: self.corrupt_date(),
                            time: time_str,
                            is_weekend,
                            is_holiday,
                        });
                    }
                    2 => {
                        // Corrupt time
                        sales.push(Sale {
                            id: i as u32,
                            product_sku: product.sku.clone(),
                            currency: currencies.choose(&mut self.rng).unwrap().to_string(),
                            price: self.generate_price(&product.category, customer.discount),
                            sale_at: format!("{} {}", date_str, self.corrupt_time()),
                            site_id: site.id,
                            customer_id: customer.id,
                            date: date_str,
                            time: self.corrupt_time(),
                            is_weekend,
                            is_holiday,
                        });
                    }
                    3 => {
                        // Corrupt price (negative or unreasonably high)
                        let corrupted_price = if self.rng.random_bool(0.5) {
                            -1.0 * self.rng.random_range(1.0..1000.0)
                        } else {
                            self.rng.random_range(100000.0..1000000.0) // Unreasonably high price
                        };

                        sales.push(Sale {
                            id: i as u32,
                            product_sku: product.sku.clone(),
                            currency: currencies.choose(&mut self.rng).unwrap().to_string(),
                            price: corrupted_price,
                            sale_at: format!("{} {}", date_str, time_str),
                            site_id: site.id,
                            customer_id: customer.id,
                            date: date_str,
                            time: time_str,
                            is_weekend,
                            is_holiday,
                        });
                    }
                    _ => {
                        // Corrupt site or customer ID
                        sales.push(Sale {
                            id: i as u32,
                            product_sku: product.sku.clone(),
                            currency: currencies.choose(&mut self.rng).unwrap().to_string(),
                            price: self.generate_price(&product.category, customer.discount),
                            sale_at: format!("{} {}", date_str, time_str),
                            site_id: if self.rng.random_bool(0.5) { 0 } else { 9999 }, // Invalid site ID
                            customer_id: if self.rng.random_bool(0.5) { 0 } else { 9999 }, // Invalid customer ID
                            date: date_str,
                            time: time_str,
                            is_weekend,
                            is_holiday,
                        });
                    }
                }
            } else {
                // Create valid sale record
                let currency = currencies.choose(&mut self.rng).unwrap();
                let price = self.generate_price(&product.category, customer.discount);

                sales.push(Sale {
                    id: i as u32,
                    product_sku: product.sku.clone(),
                    currency: currency.to_string(),
                    price,
                    sale_at: format!("{} {}", date_str, time_str),
                    site_id: site.id,
                    customer_id: customer.id,
                    date: date_str,
                    time: time_str,
                    is_weekend,
                    is_holiday,
                });
            }
        }

        sales
    }
}

// Function to write products to CSV
fn write_products_csv(products: &[Product], output_dir: &str) -> io::Result<()> {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    let path = Path::new(output_dir).join("products.csv");
    let mut file = File::create(path)?;

    // Write header
    writeln!(file, "Product Name,SKU,Category")?;

    // Write data
    for product in products {
        writeln!(
            file,
            "{},{},{}",
            product.name, product.sku, product.category
        )?;
    }

    Ok(())
}

// Function to write sites to CSV
fn write_sites_csv(sites: &[Site], output_dir: &str) -> io::Result<()> {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    let path = Path::new(output_dir).join("sites.csv");
    let mut file = File::create(path)?;

    // Write header
    writeln!(file, "SiteId,SiteName,Region")?;

    // Write data
    for site in sites {
        writeln!(file, "{},{},{}", site.id, site.name, site.region)?;
    }

    Ok(())
}

// Function to write customers to CSV
fn write_customers_csv(customers: &[Customer], output_dir: &str) -> io::Result<()> {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    let path = Path::new(output_dir).join("customers.csv");
    let mut file = File::create(path)?;

    // Write header
    writeln!(file, "CustomerID,discount")?;

    // Write data
    for customer in customers {
        writeln!(file, "{},{:.2}", customer.id, customer.discount)?;
    }

    Ok(())
}

// Function to write sales to CSV
fn write_sales_csv(sales: &[Sale], output_dir: &str) -> io::Result<()> {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    let path = Path::new(output_dir).join("sales.csv");
    let mut file = File::create(path)?;

    // Write header
    writeln!(file, "sale_id,product_sku,currency,price,sale_at,site_id,customer_id,date,time,is_weekend,is_holiday")?;

    // Write data
    for sale in sales {
        writeln!(
            file,
            "{},{},{},{:.2},{},{},{},{},{},{},{}",
            sale.id,
            sale.product_sku,
            sale.currency,
            sale.price,
            sale.sale_at,
            sale.site_id,
            sale.customer_id,
            sale.date,
            sale.time,
            sale.is_weekend,
            sale.is_holiday
        )?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    println!("Generating synthetic sales data warehouse with controlled error rates");
    println!(
        "Products: {} (Error rate: {:.1}%)",
        args.products,
        args.product_errors * 100.0
    );
    println!(
        "Sites: {} (Error rate: {:.1}%)",
        args.sites,
        args.site_errors * 100.0
    );
    println!(
        "Customers: {} (Error rate: {:.1}%)",
        args.customers,
        args.customer_errors * 100.0
    );
    println!(
        "Sales: {} (Error rate: {:.1}%)",
        args.sales,
        args.sales_errors * 100.0
    );
    println!("Output directory: {}", args.output_dir);

    // Initialize the data generator
    let mut generator = DataGenerator::new(
        args.product_errors,
        args.site_errors,
        args.customer_errors,
        args.sales_errors,
        Some(args.seed),
    );

    // Generate data
    println!("Generating products...");
    let products = generator.generate_products(args.products);

    println!("Generating sites...");
    let sites = generator.generate_sites(args.sites);

    println!("Generating customers...");
    let customers = generator.generate_customers(args.customers);

    println!("Generating sales...");
    let sales = generator.generate_sales(args.sales, &products, &sites, &customers);

    // Write data to CSV files
    println!("Writing data to CSV files...");
    write_products_csv(&products, &args.output_dir)?;
    write_sites_csv(&sites, &args.output_dir)?;
    write_customers_csv(&customers, &args.output_dir)?;
    write_sales_csv(&sales, &args.output_dir)?;

    println!("Done!");

    Ok(())
}
