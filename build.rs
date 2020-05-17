use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub type EAW = &'static str;
pub const FULL: EAW = "Full";
pub const HALF: EAW = "Half";
pub const WIDE: EAW = "Wide";
pub const NARROW: EAW = "Narrow";
pub const AMBIGUOUS: EAW = "Ambiguous";
pub const NEUTRAL: EAW = "Neutral";

fn main() {
    let r = BufReader::new(File::open("EastAsianWidth.txt").unwrap());
    let records = read_records(r);
    let records = fill_unassigned(records);
    let records = compress(records);

    let mut w = BufWriter::new(File::create("src/solver.rs").unwrap());
    write_solver(&mut w, &records);
}

#[derive(Debug)]
struct Record {
    pub start: u32,
    pub end: u32,
    pub eaw: EAW,
}

fn parse_record(line: &str) -> Option<Record> {
    let line = if let Some(p) = line.find('#') {
        &line[0..p]
    } else {
        &line
    };
    let line = line.trim();
    let ss = line
        .split(";")
        .flat_map(|s| s.split(".."))
        .collect::<Vec<_>>();
    let (start, end, eaw);
    match ss.len() {
        1 => return None,
        2 => {
            start = parse_hex(ss[0]);
            end = parse_hex(ss[0]) + 1;
            eaw = parse_eaw(ss[1]);
        }
        3 => {
            start = parse_hex(ss[0]);
            end = parse_hex(ss[1]) + 1;
            eaw = parse_eaw(ss[2]);
        }
        _ => panic!(),
    }

    return Some(Record { start, end, eaw });

    fn parse_hex(s: &str) -> u32 {
        u32::from_str_radix(s, 16).unwrap()
    }
    fn parse_eaw(s: &str) -> EAW {
        match s {
            "F" => FULL,
            "H" => HALF,
            "W" => WIDE,
            "Na" => NARROW,
            "A" => AMBIGUOUS,
            "N" => NEUTRAL,
            _ => panic!(),
        }
    }
}

fn read_records(r: impl BufRead) -> Vec<Record> {
    r.lines()
        .filter_map(|line| parse_record(&line.unwrap()))
        .collect()
}

fn fill_unassigned(mut records: Vec<Record>) -> Vec<Record> {
    let mut result = Vec::new();

    records.sort_by_key(|x| x.start);

    if let Some(first) = records.first() {
        if 0 < first.start {
            result.push(Record {
                start: 0,
                end: first.start,
                eaw: NEUTRAL,
            });
        }
    }

    for (record, next) in records.iter().zip(records.iter().skip(1)) {
        if record.end == next.start {
            result.push(Record {
                start: record.start,
                end: record.end,
                eaw: record.eaw,
            });
        } else if record.end < next.start {
            result.push(Record {
                start: record.start,
                end: record.end,
                eaw: record.eaw,
            });
            result.push(Record {
                start: record.end,
                end: next.start,
                eaw: NEUTRAL,
            });
        } else {
            unreachable!();
        }
    }

    if let Some(last) = records.last() {
        result.push(Record {
            start: last.start,
            end: last.end,
            eaw: last.eaw,
        });
        result.push(Record {
            start: last.end,
            end: 0x110000,
            eaw: NEUTRAL,
        });
    }

    return result;
}

fn compress(mut records: Vec<Record>) -> Vec<Record> {
    let mut i = 1usize;
    while i < records.len() {
        assert_eq!(records[i - 1].end, records[i].start);
        if records[i - 1].eaw == records[i].eaw {
            let a = records.remove(i);
            records[i - 1].end = a.end;
        } else {
            i += 1;
        }
    }
    return records;
}

fn write_solver(w: &mut impl Write, records: &[Record]) {
    enum Fragment<'a> {
        Slice(&'a [Record]),
        Line(String),
    }

    writeln!(w, "use crate::common::EastAsianWidth;").unwrap();
    writeln!(w, "use crate::common::EastAsianWidth::*;").unwrap();
    writeln!(w).unwrap();
    writeln!(w, "pub fn solve_eaw(code_point: u32) -> EastAsianWidth {{").unwrap();

    let mut stack = vec![(Fragment::Slice(records), 1usize)];
    while let Some(top) = stack.pop() {
        match top {
            (Fragment::Slice(slice), depth) => match slice.len() {
                0 => unreachable!(),
                1 => stack.push((Fragment::Line(format!("return {};", slice[0].eaw)), depth)),
                _ => {
                    let mid = slice.len() / 2;
                    stack.push((Fragment::Line(format!("}}")), depth));
                    stack.push((Fragment::Slice(&slice[mid..]), depth + 1));
                    stack.push((Fragment::Line(format!("}} else {{")), depth));
                    stack.push((Fragment::Slice(&slice[0..mid]), depth + 1));
                    stack.push((
                        Fragment::Line(format!("if code_point < 0x{:X} {{", slice[mid].start)),
                        depth,
                    ));
                }
            },
            (Fragment::Line(line), depth) => {
                let indent = "    ".repeat(depth);
                writeln!(w, "{}{}", indent, line).unwrap();
            }
        }
    }

    writeln!(w, "}}").unwrap();
}
