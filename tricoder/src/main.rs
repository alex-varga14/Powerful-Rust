// Rare to directly use the thread API from the std lib
// instead, we will use rayon, a data-parallelism lib for Rust
use rayon::prelude::*;
// can also use ThreadPool: use threadpool::ThreadPool;
// use for very specific requirements
use reqwest::{blocking::Client, redirect};
use std::{env, time::Duration};

mod error;
pub use error::Error;
mod model;
mod ports;
mod subdomains;
use model::Subdomain;
mod common_ports;

// IN RUST, thanks to its ownership system, the compiler gurantees our programs to be data race free

fn main() -> Result<(), anyhow::Error>{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].as_str();

    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    // leverage a custom thread pool to improve speed
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();
    
    // pool.install needed for custom threadpool
    pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()        //replace into_iter() -> into_par_iter for parallelism
            .map(ports::scan_ports)
            .collect();
        
        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("      {}", port.port);
            }

            println!("");
        }
    });

    Ok(())
}
