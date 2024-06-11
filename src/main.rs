use chrono::{Days, Duration, NaiveDate, NaiveDateTime, NaiveTime};
use error_chain::error_chain;
use std::fs::{self, write};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub struct TLE {
    pub name: String,
    pub satellite_number: u32,
    pub classification: char,
    pub international_designator: String,
    pub epoch: i64,
    pub first_derivative_mean_motion: f64,
    pub second_derivative_mean_motion: f64,
    pub drag_term: f64,
    pub ephemeris_type: u32,
    pub element_number: u32,
    pub inclination: f64,
    pub right_ascension: f64,
    pub eccentricity: f64,
    pub argument_of_perigee: f64,
    pub mean_anomaly: f64,
    pub mean_motion: f64,
    pub revolution_number: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = fetch_tle("weather".to_string()).await?;
    let lines = split_tle(res.to_string());

    println!("{:?}", lines[0]);
    parse_tle(&lines[0]);
    if fs::metadata("../test.txt").is_ok() {
        write("../test.txt", lines.join("\n")).expect("");
    } else {
        fs::File::create("../test.txt")?;
        write("../test.txt", lines.join("\n")).expect("");
    }
    Ok(())
}

async fn fetch_tle(group: String) -> Result<String> {
    let res = reqwest::get(&format!(
        "https://celestrak.org/NORAD/elements/gp.php?GROUP={}&FORMAT=tle",
        group
    ))
    .await?;

    let body = res.text().await?;
    Ok(body)
}

fn split_tle(tles: String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut grouped = String::new();

    for (index, line) in tles.lines().enumerate() {
        grouped.push_str(line);
        grouped.push_str("\n");
        if (index + 1) % 3 == 0 {
            lines.push(grouped.clone());
            grouped.clear();
        }
    }

    return lines;
}

fn parse_tle(tle: &String) {
    let mut lines = tle.lines();

    let name = lines.next().expect("Expected TLE Name Line").to_string();
    let line1 = lines.next().expect("Expected TLE Line 1");
    let line2 = lines.next().expect("Expected TLE Line 2");

    let mut epoch: u32 = 0;

    println!("{}", get_epoch(line1[18..=32].trim().to_string()));
    println!("{}", (line1[18..=32].trim().to_string()));

    /*     let mut parsed_tle: TLE = TLE {
        name: name,
        satellite_number: line1[2..7].trim().parse::<u32>().expect("Could not parse Sat Num."),
        classification: line1[7..8].chars().next().expect("Could not parse classification."),
        international_designator: line1[9..17].trim().to_string(),
        epoch: get_epoch(line1[18..=32].trim().to_string())
    } */
}

fn get_epoch(epoch: String) -> i64 {
    let mut epoch_year: i32 = epoch[0..2]
        .to_string()
        .parse::<i32>()
        .expect("Could not parse epoch year.");
    if epoch_year < 57 {
        epoch_year = 2000 + epoch_year
    } else {
        epoch_year = 1900 + epoch_year;
    }

    println!("Year: {}", epoch_year);

    let decimal_day: String = epoch[2..].to_string();
    let full_day: Vec<&str> = decimal_day.split_terminator(".").collect();

    let mut day_fraction: f64 = (".".to_owned() + full_day[1])
        .parse::<f64>()
        .expect("Could not parse day fraction.");

    let hours = (day_fraction * 24.0).floor();
    println!("Hours: {}", hours);
    day_fraction -= hours / 24.0;
    let minutes = (day_fraction * 24.0 * 60.0).floor();
    println!("Minutes: {}", minutes);
    day_fraction -= minutes / (24.0 * 60.0);
    let seconds = (day_fraction * 24.0 * 60.0 * 60.0).floor();
    println!("Seconds: {}", seconds);
    day_fraction -= seconds / (24.0 * 60.0 * 60.0);
    let milliseconds = ((day_fraction * 24.0 * 60.0 * 60.0 * 1000.0) + 0.5).floor();
    println!("Milliseconds: {}", milliseconds);

    let date = NaiveDate::from_ymd_opt(epoch_year, 1, 1).unwrap();
    let time = NaiveTime::from_hms_milli_opt(
        hours as u32,
        minutes as u32,
        seconds as u32,
        milliseconds as u32,
    )
    .unwrap();
    let mut date_time = NaiveDateTime::new(date, time);

    let days = full_day[0]
        .to_string()
        .parse::<i64>()
        .expect("Could not parse full days.");
    println!("Days: {:?}", days);
    date_time += Duration::days(days - 1);

    println!("Date: {}", date_time.and_utc().to_rfc3339());
    return date_time.and_utc().timestamp();
}

