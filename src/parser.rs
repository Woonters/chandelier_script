use core::panic;
use std::path::Path;

use chrono::{DateTime, Utc};
use csv;
#[doc = r"Parsing the input CSV"]
use winnow::Result;
use winnow::{
    Parser,
    ascii::digit1,
    combinator::{separated, separated_pair},
    token::literal,
};

use crate::ScheduleElement;

pub fn parse_csv_alt(filename: &Path) -> Vec<ScheduleElement<Utc>> {
    let mut csv_file = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)
        .expect("Couldn't find the csv file, Have you given the correct path?");
    let mut out = Vec::new();
    for result in csv_file.records() {
        let record = result.unwrap();
        out.push(ScheduleElement {
            timestamp: record.get(0).unwrap().parse::<DateTime<Utc>>().unwrap(),
            height: record.get(1).unwrap()[1..].parse().unwrap(),
        });
    }
    out
}

pub fn parse_csv(filename: &Path) -> Vec<ScheduleElement<Utc>> {
    let csv_file = std::fs::read_to_string(filename)
        .expect("Error getting csv file have you given the correct path?");
    match _parse_csv(&mut &csv_file[..]) {
        Ok(v) => v,
        Err(_) => panic!("Error reading File, is the csv formatted correctly"),
    }
}

fn _parse_csv(input: &mut &str) -> Result<Vec<ScheduleElement<Utc>>> {
    separated(0.., csv_line, literal("\n")).parse_next(input)
}

fn csv_line(input: &mut &str) -> Result<ScheduleElement<Utc>> {
    separated_pair(time_parser, literal(", "), parse_number)
        .map(|(timestamp, height)| ScheduleElement { timestamp, height })
        .parse_next(dbg!(input))
}

fn parse_number(input: &mut &str) -> Result<isize> {
    digit1.parse_to().parse_next(input)
}
fn time_parser(input: &mut &str) -> Result<DateTime<Utc>> {
    input.parse_to::<DateTime<Utc>>().parse_next(input)
}

#[cfg(test)]
mod tests {

    use chrono::{Datelike, Timelike};

    use super::{csv_line, *};

    #[test]
    fn total() {
        let parsed = parse_csv(Path::new("test_csv.csv"));
        assert_eq!(
            parsed.len(),
            4,
            "csv wasn't parsed into the correct number of lines"
        );
    }

    #[test]
    fn timestamp() {
        let mut time = "2025-01-01T01:00:00Z";
        let time_parsed = time_parser(&mut time);
        assert!(time_parsed.is_ok());
        assert_eq!(time_parsed.unwrap().year(), 2025);
        assert_eq!(time, "");
    }

    #[test]
    fn number() {
        let mut num = "15";
        let num_parsed = parse_number(&mut num);
        assert!(num_parsed.is_ok());
        assert_eq!(num_parsed.unwrap(), 15, "Numbers were not the same");
        assert_eq!(num, "");
    }

    #[test]
    fn test_csv_line() {
        let mut line = "2025-01-01T01:00:00Z, 15";
        let line_parsed = csv_line(&mut line);
        assert!(line_parsed.is_ok());
        let parsed = line_parsed.unwrap();
        assert_eq!(parsed.height, 15);
        assert_eq!(parsed.timestamp.hour(), 1);
        assert_eq!(line, "");
    }
}
