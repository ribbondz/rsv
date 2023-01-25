use crate::common::{data_path, rsv};
use execute::shell;
mod common;

fn count(file: &str, no_header: bool, io: bool) -> Option<usize> {
    let rsv = rsv();
    let cmd = match (no_header, io) {
        (true, false) => format!("{rsv} count --no-header {file}"),
        (false, false) => format!("{rsv} count {file}"),
        (true, true) => format!("{rsv} slice -l 10 {file} | {rsv} count --no-header"),
        (false, true) => format!("{rsv} slice -l 10 {file} | {rsv} count"),
    };

    // println!("111111 {:?}", cmd);
    let mut cmd = shell(cmd);
    let output = cmd.output().unwrap();

    String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .next()
        .unwrap()
        .parse::<usize>()
        .ok()
}

#[test]
fn test_count_csv_header() {
    let n = count(&data_path("hotel_reservation.csv"), false, false).unwrap();
    assert_eq!(n, 36275);
}

#[test]
fn test_count_csv_no_header() {
    let n = count(&data_path("hotel_reservation.csv"), true, false).unwrap();
    assert_eq!(n, 36276);
}

#[test]
fn test_count_empty_csv() {
    let n = count(&data_path("empty.csv"), false, false).unwrap();
    assert_eq!(n, 0);
}

#[test]
fn test_count_empty_xlsx() {
    let n = count(&data_path("empty.xlsx"), true, false).unwrap();
    assert_eq!(n, 0);
}

#[test]
fn test_count_io_header() {
    let n = count(&data_path("hotel_reservation.csv"), false, true).unwrap();
    assert_eq!(n, 10);
}

#[test]
fn test_count_io_no_header() {
    let n = count(&data_path("hotel_reservation.csv"), true, true).unwrap();
    assert_eq!(n, 11);
}
