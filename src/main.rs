mod fetch;
mod parse;
use fetch::{load_tle_cache, Cache};


#[tokio::main]
async fn main() -> Result<(), fetch::Error> {
    println!("Initalising Cache.");
    let mut cache: Cache = load_tle_cache(Some("./output/cache.json".to_string())).await.expect("Failed to load cache.");
    println!("Cache initalised with length.");

    match cache.get_tle(25544).await {
        Ok(tle) => println!("{}", tle),
        Err(e) => println!("Error occurred whilst getting TLE from cache: {}", e),
    }

    match cache.get_tle(99999).await {
        Ok(tle) => println!("{}", tle),
        Err(e) => println!("Error occurred whilst getting TLE from cache: {}", e),
    }

    cache.to_file("./output/cache.json".to_string());

    Ok(())
}
