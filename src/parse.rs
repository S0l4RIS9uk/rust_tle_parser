use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};


#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
    last_updated_epoch: i64
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


// Splits a large string into groups of 3.
pub fn split_tle(tles: String) -> Vec<String> {
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


// Takes a 3 line element as a string and parses it into a TLE struct.
pub fn parse_tle(tle: &String) -> TLE {
    let mut lines = tle.lines();

    let name = lines.next().expect("Expected TLE Name Line").trim().to_string();
    let line1 = lines.next().expect("Expected TLE Line 1").trim();
    let line2 = lines.next().expect("Expected TLE Line 2").trim();
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
        eccentricity: parse_decimal_point_assumed(line2[26..33].trim().to_string()),
        argument_of_perigee: line2[34..42].trim()
            .parse::<f64>()
            .expect("Could not parse argument_of_perigee."),
        mean_anomaly: line2[42..51].trim()
            .parse::<f64>()
            .expect("Could not parse mean_anomaly."),
        mean_motion: line2[52..63].trim()
            .parse::<f64>()
            .expect("Could not parse mean_motion."),
        revolution_number: line2[63..68].trim()
            .parse::<u32>()
            .expect("Could not parse revolution_number."),
        last_updated_epoch: Utc::now().timestamp()
    };
    return parsed_tle;
}


// Parses a string into a utc chrono::DateTime object
fn get_epoch_from_tle(tle_epoch: String) -> DateTime<Utc> {
    println!("{}", tle_epoch );
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

// Parses a decimal point assumed string into a float.
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
        // 15 dp is the general accuracy of a f64.
        return format!("{:.15}",base * 10f64.powf(exponent)).parse::<f64>().expect("Could not parse rounded decimal point assumed.")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch_parser() {
        assert_eq!(1718748281, get_epoch_from_tle("24170.91992694".to_string()).timestamp())
    }

    #[test]
    fn test_decimal_point_parser() {
        assert_eq!(0.00014141 as f64, parse_decimal_point_assumed("14141-3".to_string()));

        assert_eq!(parse_decimal_point_assumed("00000-0".to_string()), 0.0 as f64);

        assert_eq!(parse_decimal_point_assumed("-36258-4".to_string()), -0.36258e-4);
    }

    #[test]
    fn test_tle_parse() {
        let mut raw_tle = 
        "ISS (ZARYA)
        1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990
        2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.49165514212791";
        
        let tle = parse_tle(&raw_tle.to_string());

        assert_eq!(tle.name, "ISS (ZARYA)".to_string());
        assert_eq!(tle.satellite_number, 25544);
        assert_eq!(tle.classification, 'U');
        assert_eq!(tle.international_designator, "98067A".to_string());
        assert_eq!(tle.epoch, 1581654459);
        assert_eq!(tle.date_time, "2020-02-14T04:27:39.231+00:00".to_string());
        assert_eq!(tle.first_derivative_mean_motion, 0.00000950);
        assert_eq!(tle.second_derivative_mean_motion, 0.0);
        assert_eq!(tle.drag_term, 0.25302e-4);
        assert_eq!(tle.ephemeris_type, 0);
        assert_eq!(tle.element_number, 999);
        assert_eq!(tle.inclination, 51.6443);
        assert_eq!(tle.right_ascension, 242.0161);
        assert_eq!(tle.eccentricity, 0.0004885);
        assert_eq!(tle.argument_of_perigee, 264.6060);
        assert_eq!(tle.mean_anomaly, 207.3845);
        assert_eq!(tle.mean_motion, 15.49165514);
        assert_eq!(tle.revolution_number, 21279);

        raw_tle = "GRUS-1A
        1 43890U 18111Q   20044.88470557  .00000320  00000-0  36258-4 0  9993
        2 43890  97.7009 312.6237 0003899   7.8254 352.3026 14.92889838 61757";
        
        let tle = parse_tle(&raw_tle.to_string());

        assert_eq!(tle.name, "GRUS-1A");
        assert_eq!(tle.satellite_number, 43890);
        assert_eq!(tle.classification, 'U');
        assert_eq!(tle.international_designator, "18111Q");
        assert_eq!(tle.epoch, 1581628438);
        assert_eq!(tle.first_derivative_mean_motion, 0.00000320);
        assert_eq!(tle.second_derivative_mean_motion, 0.0);
        assert_eq!(tle.drag_term, 0.36258e-4);
        assert_eq!(tle.ephemeris_type, 0);
        assert_eq!(tle.element_number, 999);
        // 2nd line
        assert_eq!(tle.inclination, 97.7009);
        assert_eq!(tle.right_ascension, 312.6237);
        assert_eq!(tle.eccentricity, 0.0003899);
        assert_eq!(tle.argument_of_perigee, 7.8254);
        assert_eq!(tle.mean_anomaly, 352.3026);
        assert_eq!(tle.mean_motion, 14.92889838);
        assert_eq!(tle.revolution_number, 6175);
    }
}