use std::process::Command;
mod common;

fn count(file: &str, no_header: bool) -> Option<usize> {
    let mut output = Command::new(common::RSV_COMMAND);
    output.arg("count");
    if no_header {
        output.arg("--no-header");
    }

    let output = output.arg(file).output().unwrap();

    String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .next()
        .unwrap()
        .parse::<usize>()
        .ok()
}

#[test]
fn test_count_csv_no_header() {
    let n = count("D:\\code\\rsv\\tests\\data\\hotel_reservation.csv", false).unwrap();
    assert_eq!(n, 36275);
}

#[test]
fn test_count_csv_header() {
    let n = count("D:\\code\\rsv\\tests\\data\\hotel_reservation.csv", true).unwrap();
    assert_eq!(n, 36276);
}

#[test]
fn test_count_empty_csv() {
    let n = count("D:\\code\\rsv\\tests\\data\\empty.csv", false).unwrap();
    assert_eq!(n, 0);
}

#[test]
fn test_count_empty_xlsx() {
    let n = count("D:\\code\\rsv\\tests\\data\\empty.xlsx", true).unwrap();
    assert_eq!(n, 0);
}
