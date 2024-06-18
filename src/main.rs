use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use error_chain::error_chain;
use std::{fmt::{Display, Formatter}, fs::{self, write}};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

/*
Potential implementation of Query struct and strict query checking.

enum QueryType {
    CategoryNumber,
    InternationalDesignator,
    Group,
    Name,
    Special,
}

impl QueryType {
    fn as_str(&self) -> &'static str {
        match self {
            QueryType::CategoryNumber => "CATNR",
            QueryType::InternationalDesignator => "INTDES",
            QueryType::Group => "GROUP",
            QueryType::Name => "NAME",
            QueryType::Special => "SPECIAL",
        }
    }
}

struct CelestrakQuery {
    query: QueryType,
    value: String,
}
*/

pub struct TLE {
    pub name: String,
    pub satellite_number: u32,
    pub classification: char,
    pub international_designator: String,
    pub epoch: i64,
    pub date_time: String,
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

impl Display for TLE {

    fn fmt(&self, formatter: &mut Formatter<'_>) -> ::std::fmt::Result { 
        write!(
            formatter, 
            "{}\nSatellite #: {}\nClassification: {}\nInternational Designator: {}\nElement #: {}\nEpoch: {}\nEpoch (ISO8601) {}\nMean Motion: {}\nFirst Derivative Mean Motion: {}\nSecond Derivative Mean Motion: {}\nDrag Term: {}\nInclination: {}\nRight Angle of Ascencion: {}\nEccentricity: {}\nArgument of Perigee: {}\nMean Anomaly: {}\nRevolution #: {}", 
            self.name, 
            self.satellite_number, 
            self.classification,
            self.international_designator,
            self.element_number,
            self.epoch, 
            self.date_time,
            self.mean_motion, 
            self.first_derivative_mean_motion,
            self.second_derivative_mean_motion, 
            self.drag_term, 
            self.inclination, 
            self.right_ascension, 
            self.eccentricity, 
            self.argument_of_perigee, 
            self.mean_anomaly, 
            self.revolution_number
        )

    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = fetch_tle("GROUP=weather".to_string()).await?;
    let lines = split_tle(res.to_string());
    println!("{}", lines[0].to_string());
    println!("{}", parse_tle(&lines[0]).to_string());
/*     let parsed = lines.iter().map(parse_tle);
    if fs::metadata("./test.txt").is_ok() {
        write("./test.txt", parsed.map(|tle: TLE| tle.to_string()).collect::<Vec<String>>().join("\n\n"))?;
    } else {
        write("./test.txt", parsed.map(|tle: TLE| tle.to_string()).collect::<Vec<String>>().join("\n\n"))?;
    } */
    Ok(())
}

async fn fetch_tle(query: String) -> Result<String> {
    let res = reqwest::get(&format!(
        "https://celestrak.org/NORAD/elements/gp.php?{}&FORMAT=tle",
        query
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

fn parse_tle(tle: &String) -> TLE {
    let mut lines = tle.lines();

    let name = lines.next().expect("Expected TLE Name Line").to_string();
    let line1 = lines.next().expect("Expected TLE Line 1");
    let line2 = lines.next().expect("Expected TLE Line 2");

    let epoch: DateTime<Utc> = get_epoch_from_tle(line1[18..=32].trim().to_string());

    let parsed_tle: TLE = TLE {
        name: name,
        satellite_number: line1[2..7]
            .trim()
            .parse::<u32>()
            .expect("Could not parse satellite_number."),
        classification: line1[7..8].trim()
            .chars()
            .next()
            .expect("Could not parse classification."),
        international_designator: line1[9..17].trim().to_string(),
        epoch: epoch.timestamp(),
        date_time: epoch.to_rfc3339(),
        first_derivative_mean_motion: line1[34..43]
            .trim()
            .parse::<f64>()
            .expect("Could not parse first_derivative_mean_motion."),
        second_derivative_mean_motion: parse_decimal_point_assumed(line1[45..52].trim().to_string()),
        drag_term: parse_decimal_point_assumed(line1[54..61].trim().to_string()),
        ephemeris_type: line1[62..63].trim()
            .parse::<u32>()
            .expect("Could not parse ephemeris_type."),
        element_number: line1[65..68].trim()
            .parse::<u32>()
            .expect("Could not parse element_number."),
        inclination: line2[9..16].trim()
            .parse::<f64>()
            .expect("Could not parse inclination"),
        right_ascension: line2[17..25].trim()
            .parse::<f64>()
            .expect("Could not parse right_ascencion."),
        eccentricity: parse_decimal_point_assumed(line2[27..33].trim().to_string()),
        argument_of_perigee: line2[35..42].trim()
            .parse::<f64>()
            .expect("Could not parse argument_of_perigee."),
        mean_anomaly: line2[44..51].trim()
            .parse::<f64>()
            .expect("Could not parse mean_anomaly."),
        mean_motion: line2[52..63].trim()
            .parse::<f64>()
            .expect("Could not parse mean_motion."),
        revolution_number: line2[64..68].trim()
            .parse::<u32>()
            .expect("Could not parse revolution_number."),
    };
    return parsed_tle;
}

fn get_epoch_from_tle(tle_epoch: String) -> DateTime<Utc> {
    // get year from first 2 chars
    let mut epoch_year: i32 = tle_epoch[0..2]
        .to_string()
        .parse::<i32>()
        .expect("Could not parse epoch year.");
    if epoch_year < 57 {
        epoch_year = 2000 + epoch_year
    } else {
        epoch_year = 1900 + epoch_year;
    }
    // get decimal day from remainder of string
    let decimal_day: String = tle_epoch[2..].to_string();

    // get the full num of days
    let full_day: Vec<&str> = decimal_day.split_terminator(".").collect();

    // get fractional day
    let mut day_fraction: f64 = (".".to_owned() + full_day[1])
        .parse::<f64>()
        .expect("Could not parse day fraction.");

    // calc hours minutes, seconds milliseconds
    let hours: f64 = (day_fraction * 24.0).floor();
    day_fraction -= hours / 24.0;
    let minutes: f64 = (day_fraction * 24.0 * 60.0).floor();
    day_fraction -= minutes / (24.0 * 60.0);
    let seconds: f64 = (day_fraction * 24.0 * 60.0 * 60.0).floor();
    day_fraction -= seconds / (24.0 * 60.0 * 60.0);
    let milliseconds: f64 = ((day_fraction * 24.0 * 60.0 * 60.0 * 1000.0) + 0.5).floor();

    // create date time and correct time on jan 1st
    let date: NaiveDate = NaiveDate::from_ymd_opt(epoch_year, 1, 1).unwrap();
    let time: NaiveTime = NaiveTime::from_hms_milli_opt(
        hours as u32,
        minutes as u32,
        seconds as u32,
        milliseconds as u32,
    )
    .unwrap();
    let mut date_time = NaiveDateTime::new(date, time);

    // add days - 1 to date_time to get correct day, remove 1 because the days start from 0 and our datetime starts from 1.
    let days = full_day[0]
        .to_string()
        .parse::<i64>()
        .expect("Could not parse full days.");
    date_time += Duration::days(days - 1);

    return date_time.and_utc();
}

fn parse_decimal_point_assumed(input: String) -> f64 {
    if input.contains('+') || input.contains('-') && !input.starts_with('-') || input.matches('-').count() == 2 {
        let exp_index: usize;
        if let Some(index) = input.rfind('+') {
            exp_index = index;
        } else {
            exp_index = input.rfind('-').unwrap();
        }

        let base;
        if input.starts_with('-') {
            base = format!("-0.{}", &input[1..exp_index])
                .parse::<f64>()
                .expect("Could not parse base.");
        } else {
            base = format!("0.{}", &input[0..exp_index])
                .parse::<f64>()
                .expect("Could not parse base.");
        }
        let exponent;
        if input[exp_index..].starts_with('+') {
            exponent = input[(exp_index + 1)..].parse::<f64>()
            .expect("Could not parse exponent.")
        } else {
            exponent = input[exp_index..].parse::<f64>()
            .expect("Could not parse exponent.")
        }
        return format!("{:.8}",base * 10f64.powf(exponent)).parse::<f64>().expect("Could not parse rounded decimal point assumed.")
    } else if input.contains('-') {
        return format!("-0.{}", input)
            .parse::<f64>()
            .expect("Could not parse decimal point assumed value at parse_decimal_point_assumed")
    } else {
        return format!("0.{}", input)
            .parse::<f64>()
            .expect("Could not parse decimal point assumed value at parse_decimal_point_assumed")
    }
}
