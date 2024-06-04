use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = fetch_tle("weather".to_string()).await?;
    let lines = split_tle(res.to_string());
    
    println!("{:?}", lines[0]);
    Ok(())
}

async fn fetch_tle(group: String) -> Result<String> {
    let res = reqwest::get(&format!("https://celestrak.org/NORAD/elements/gp.php?GROUP={}&FORMAT=tle", group)).await?;

    let body = res.text().await?;
    Ok(body)
}

fn split_tle(tles: String) -> Vec<String> {
    let mut lines = Vec::new();
    let mut grouped = String::new();

    for (index, line) in tles.lines().enumerate() {
        grouped.push_str(line);
        grouped.push_str("\n");
        if (index + 1 ) % 3 == 0 {
            lines.push(grouped.clone());
            grouped.clear();
        }
    }

    return lines;
}