extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate multimap;
extern crate regex;

use chrono::prelude::*;
use multimap::MultiMap;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

#[allow(dead_code)]
fn day_3_part_1(input: &str) -> u32 {
    let records = parse_records(input);
    let sleeps_by_guard = sleeps_by_guard(&records);
    // Find longest sleeper - the one that spent the most _minutes_ asleep,
    // _not_ the one that had the most sleeps.
    if let Some((id, sleeps)) = longest_sleeper(&sleeps_by_guard) {
        // Now we have that pesky guard, we need to find out what minute they
        // are most asleep on. To do that, we need to produce the ranges of
        // every single minute they were asleep.
        if let Some((max_minute, _)) = most_frequent_minute_slept(sleeps) {
            return *id * (max_minute as u32);
        }
    }

    0
}

#[allow(dead_code)]
fn day_3_part_2(input: &str) -> u32 {
    let records = parse_records(input);
    let sleeps_by_guard = sleeps_by_guard(&records);

    if let Some((id, max_minute, _)) = most_frequently_asleep_on_same_min(&sleeps_by_guard) {
        return id * (max_minute as u32);
    }

    0
}

type GuardId = u32;

#[allow(dead_code)]
#[derive(Debug)]
struct Sleep {
  guard_id: GuardId,
  start: DateTime<Utc>,
  end: DateTime<Utc>
}

#[allow(dead_code)]
#[derive(Debug, Eq)]
struct Record {
  timestamp: DateTime<Utc>,
  info: RecordInfo
}

impl Ord for Record {
    fn cmp(&self, other: &Record) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Record) -> bool {
        self.timestamp == other.timestamp
    }
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RECORD_PATTERN: Regex = Regex::new(r"^\[(?P<timestamp>.*)\] (?P<info>.*)$").unwrap();
        }

        let caps = RECORD_PATTERN.captures(s).unwrap();
        let timestamp = Utc.datetime_from_str(&caps["timestamp"], "%Y-%m-%d %H:%M").expect("???");
        let info: RecordInfo = caps["info"].parse().expect("???");

        Ok(Record { timestamp, info })
    }
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
enum RecordInfo {
  BeginShift(GuardId),
  FallsAsleep,
  WakesUp
}

impl FromStr for RecordInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "falls asleep" {
            Ok(RecordInfo::FallsAsleep)
        } else if s == "wakes up" {
            Ok(RecordInfo::WakesUp)
        } else {
            lazy_static! {
                static ref BEGIN_SHIFT_PATTERN: Regex = Regex::new(r"^Guard #(?P<id>\d+) begins shift$").unwrap();
            }

            let caps = BEGIN_SHIFT_PATTERN.captures(s).unwrap();
            let id: u32 = caps["id"].parse().expect("???");

            Ok(RecordInfo::BeginShift(id))
        }
    }
}

fn parse_records(input: &str) -> Vec<Record> {
    let mut records: Vec<Record> = input.lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("???"))
        .collect();

    // The records _must_ be sorted before you do anything with them!
    records.sort_unstable();
    records
}

fn sleeps_by_guard(records: &Vec<Record>) -> MultiMap<GuardId, Sleep> {
    let mut sleeps_by_guard: MultiMap<GuardId, Sleep> = MultiMap::new();
    let mut current_guard: Option<GuardId> = None;
    let mut current_sleep_start: Option<DateTime<Utc>> = None;

    // State machine.
    for record in records {
        match record.info {
            RecordInfo::BeginShift(id) => {
                current_guard = Some(id);
                current_sleep_start = None;
            },
            RecordInfo::FallsAsleep => {
                current_sleep_start = Some(record.timestamp);
            },
            RecordInfo::WakesUp => {
                // Only possible if both the other events happened before hand.
                match (current_guard, current_sleep_start) {
                    (Some(id), Some(start)) => {
                        sleeps_by_guard.insert(id, Sleep {
                            guard_id: id,
                            start: start,
                            end: record.timestamp
                        });
                    },
                    _ => ()
                }
            }
        };
    }

    sleeps_by_guard
}

fn longest_sleeper(sleeps_by_guard: &MultiMap<GuardId, Sleep>) -> Option<(&GuardId, &Vec<Sleep>)> {
    sleeps_by_guard.iter_all()
        .max_by_key(|(_, sleeps)| {
            let minutes_asleep: i64 = sleeps.iter()
                .map(|sleep| (sleep.end - sleep.start).num_minutes())
                .sum();
            minutes_asleep
        })
}

fn most_frequently_asleep_on_same_min(sleeps_by_guard: &MultiMap<GuardId, Sleep>) -> Option<(GuardId, u8, u32)> {
    sleeps_by_guard.iter_all().flat_map(|(id, sleeps)| {
        most_frequent_minute_slept(sleeps)
            .map(|(max_minute, count)| (*id, max_minute, count))
    }).max_by_key(|&(_, _, count)| count)
}

fn most_frequent_minute_slept(sleeps: &Vec<Sleep>) -> Option<(u8, u32)> {
    let mut minutes_slept: HashMap<u8, u32> = HashMap::new();

    for sleep in sleeps.iter() {
        let length = (sleep.end - sleep.start).num_minutes();
        for i in 0..length {
            let min = ((sleep.start.minute() + i as u32) % 60) as u8;
            minutes_slept.entry(min)
                .and_modify(|i| { *i += 1 })
                .or_insert(1);
        }
    }

    minutes_slept.iter().max_by_key(|&(_, v)| v)
        .map(|(max_minute, count)| (*max_minute, *count))
}

#[cfg(test)]
mod tests {
    use day_3_part_1;
    use day_3_part_2;

    #[test]
    fn day_3_part_1_examples() {
        assert_eq!(day_3_part_1(include_str!("examples")), 240);
    }

    #[test]
    fn day_3_part_1_test_input() {
        assert_eq!(day_3_part_1(include_str!("input")), 12169);
    }

    #[test]
    fn day_3_part_2_examples() {
        assert_eq!(day_3_part_2(include_str!("examples")), 4455);
    }

    #[test]
    fn day_3_part_2_test_input() {
        assert_eq!(day_3_part_2(include_str!("input")), 16164);
    }
}
