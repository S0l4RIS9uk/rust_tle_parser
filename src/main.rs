use std::fs::{self, write};
mod fetch;
mod parse;
use parse::{TLE, parse_tle, split_tle};
use fetch::fetch_tle;


#[tokio::main]
async fn main() -> Result<(), fetch::Error> {
    let res = fetch_tle("GROUP=weather".to_string()).await?;
    let lines = split_tle(res.to_string());
    println!("{}", lines[0].to_string());
    println!("{}", parse_tle(&lines[0]).to_string());
    let parsed = lines.iter().map(parse_tle);
    if !fs::metadata("./output").is_ok() {
        fs::create_dir("./output").expect("Could not create output dir.");
        write(
            "./output/test.txt",
            parsed
                .clone()
                .map(|tle: TLE| tle.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
        )?;
        write(
            "./output/test.json",
            serde_json::to_string_pretty(&parsed.collect::<Vec<TLE>>())
                .expect("Failed to serialise TLE's"),
        )?;
    } else {
        write(
            "./output/test.txt",
            parsed
                .clone()
                .map(|tle: TLE| tle.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
        )?;
        write(
            "./output/test.json",
            serde_json::to_string_pretty(&parsed.collect::<Vec<TLE>>())
                .expect("Failed to serialise TLE's"),
        )?;
    }

    Ok(())
}
