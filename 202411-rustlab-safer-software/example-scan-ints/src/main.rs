use std::io::prelude::*;
use std::io::BufWriter;
use std::panic::catch_unwind;
use std::thread;
use std::time::{Duration, Instant};
use std::fmt::Display;
use std::ops::RangeInclusive;

fn check_multiplication<T>(a: T, b: T) -> bool 
where T: std::ops::Mul + Copy + num::CheckedMul + std::panic::RefUnwindSafe {
    catch_unwind(|| {
        let _ans = a.checked_mul(&b);
        true
    }).unwrap_or(false)
}

fn run_checks<T>(filename: &str, lower: T, upper: T) -> std::io::Result<()>
where
    T: Copy + Ord + Display  + std::ops::Mul + num::CheckedMul + std::panic::RefUnwindSafe + 'static,
    T: std::convert::TryFrom<i32>,
    i64: From<T>,
{
    let f = std::fs::File::create(filename)?;
    let mut writer = BufWriter::with_capacity(1024 * 1024, f);
    let start_time = Instant::now();
    let mut checks = 0u64;
    let mut last_update = Instant::now();

    let range_start: i64 = lower.into();
    let range_end: i64 = upper.into();
    let range_size = (range_end - range_start + 1) as u64;
    let total_ops = range_size.saturating_mul(range_size);

    let mut results = Vec::with_capacity(1024 * 1024);
    let mut output = String::with_capacity(1024 * 1024);

    for a in lower..=upper {
        'b_loop: for b in lower..=upper {
            if b <=  a {
                continue;
            }

            let result = thread::scope(|_| {
                check_multiplication(a, b)
            });

            match result {
                true => {
                    results.push((a, b, 1));
                    if a != b {
                        results.push((b, a, 1));
                        checks += 2;
                    } else {
                        checks += 1;
                    }
                },
                _ => {
                    results.push((a, b, 0));
                    if a != b {
                        results.push((b, a, 0));
                        checks += 2;
                    } else {
                        checks += 1;
                    }
                    
                    if b < *range.end() {
                        for fail_b in range.clone().filter(|&x| x > b) {
                            results.push((a, fail_b, 0));
                            results.push((fail_b, a, 0));
                        }

                        let b: i64 = b.into();
                        checks += 2 * (range_end - b) as u64;
                        break 'b_loop;
                    }
                },
            }
            
            if results.len() > 1024 * 1024 {
                for (x, y, success) in results.drain(..) {
                    output.push_str(&format!("{x}\t{y}\t{success}\n"));
                }
                writer.write_all(output.as_bytes())?;
                output.clear();
            }
            
            if last_update.elapsed() >= Duration::from_secs(1) {
                let elapsed = start_time.elapsed().as_secs_f64();
                let checks_per_sec = checks as f64 / elapsed;
                let progress = (checks as f64 / total_ops as f64) * 100.0;
                
                print!("\r\x1B[K");
                print!(
                    "Progress: {:.2}% | Total Entries: {} | Speed: {:.2}M entries/sec | Elapsed: {:.1}s",
                    progress,
                    checks,
                    checks_per_sec / 1_000_000.0,
                    elapsed
                );
                std::io::stdout().flush()?;
                
                last_update = Instant::now();
            }
        }
    }
    
    if !results.is_empty() {
        for (x, y, success) in results.drain(..) {
            output.push_str(&format!("{x}\t{y}\t{success}\n"));
        }
        writer.write_all(output.as_bytes())?;
    }
    
    writer.flush()?;
    println!();
    Ok(())
}

fn main() -> std::io::Result<()> {
    // Run checks for each integer type
    println!("Testing i8...");
    run_checks("can-multiply-i8.tsv", i8::MIN, i8::MAX)?;

    println!("Testing u8...");
    run_checks("can-multiply-u8.tsv", u8::MIN, u8::MAX)?;
    
    println!("Testing i16...");
    run_checks("can-multiply-i16.tsv", i16::MIN, i16::MAX)?;
    
    println!("Testing u16...");
    run_checks("can-multiply-u16.tsv", u16::MIN, u16::MAX)?;
    
    Ok(())
}