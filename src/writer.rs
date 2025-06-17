use std::{io::Write, path::Path};

use chrono::TimeZone;

use crate::ScheduleElement;

pub fn write_csv<Tz: TimeZone>(filename: &Path, data: &[ScheduleElement<Tz>]) {
    let mut f = std::fs::File::create(filename).expect("Something went wrong creating the file");
    // data to &[u8]
    let mut out: Vec<u8> = Vec::new();
    for elem in data {
        let datetime = elem
            .timestamp
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        out.extend_from_slice(format!("{datetime}, {0}\n", elem.height).as_bytes());
    }
    f.write_all(&out).expect("Something went wrong writing");
}
