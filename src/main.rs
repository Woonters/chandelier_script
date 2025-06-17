mod parser;
mod writer;
use std::path::Path;

use chrono::DateTime;
use chrono::TimeDelta;
use chrono::TimeZone;

#[derive(Clone, Debug)]
struct ScheduleElement<Tz: TimeZone> {
    timestamp: DateTime<Tz>,
    height: isize,
}

fn generate_points<Tz: TimeZone>(items: &[ScheduleElement<Tz>]) -> Vec<ScheduleElement<Tz>> {
    // we take the first item (it's our starting position)
    let mut out = vec![items[0].clone()];
    let mut index = 1;
    loop {
        // is the next element a different height?
        if items[index].height != out.last().unwrap().height {
            // take the difference calculate the number of mins
            let diff_delta = (items[index].height - out.last().unwrap().height) * 30;
            let delta = TimeDelta::new(diff_delta as i64, 0)
                .expect("The difference in height is outside of reasonable range, check the csv");
            // for some reason although checked_sub and checked_add don't mutate DateTime they both take ownership
            // of self, so I have to do so much more cloning than one would think proper
            let ts = items[index].timestamp.clone();
            let ts2 = ts.clone();
            let pre = ts.checked_sub_signed(delta).expect("Time is outside of UTC availabilty, are you sure you have set the correct time in the csv");
            let post = ts2.checked_add_signed(delta).expect("Time is outside of UTC availabilty, are you sure you have set the correct time in the csv");
            out.push(ScheduleElement {
                timestamp: pre,
                height: out.last().unwrap().height,
            });
            out.push(ScheduleElement {
                timestamp: post,
                height: items[index].height,
            });
            index += 1;
        } else {
            index += 1;
        }
        if index > (items.len() - 1) {
            return out;
        }
    }
}

fn main() {
    let input = parser::parse_csv(Path::new("schedule.csv"));
    let output = generate_points(&input);
    writer::write_csv(Path::new("output.csv"), &output);
}
