use execute::shell;
mod common;
use crate::common::{data_path, rsv};

fn end_row_of_cmd(cmd: &str, file: &str) -> String {
    // println!("111111 {:?}", cmd);
    let cmd = cmd.replace("rsv", &rsv());
    let cmd = cmd.replace("file", &data_path(file));
    let cmd = cmd.replace("FILE", &data_path(file));

    let mut cmd = shell(cmd);
    let output = cmd.output().unwrap();

    String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .filter(|i| !i.is_empty())
        .last()
        .unwrap()
        .to_owned()
}

#[test]
fn test_frequency_one_column_csv() {
    let cmd = "rsv frequency -c 1 -n 2 file";
    assert_eq!(end_row_of_cmd(cmd, "hotel_reservation.csv"), "1,7695");
}

#[test]
fn test_frequency_one_column_csv_no_header() {
    let cmd = "rsv frequency -c 1 -n 2 file --no-header";
    assert_eq!(end_row_of_cmd(cmd, "hotel_reservation.csv"), "1,7695");
}

#[test]
fn test_frequency_one_column_excel() {
    let cmd = "rsv frequency -c 1 -n 2 file";
    assert_eq!(end_row_of_cmd(cmd, "hotel_reservation.xlsx"), "1,7695");
}

#[test]
fn test_frequency_one_column_excel_no_header() {
    let cmd = "rsv frequency -c 1 -n 2 file --no-header";
    assert_eq!(end_row_of_cmd(cmd, "hotel_reservation.xlsx"), "1,7695");
}

#[test]
fn test_frequency_two_column_csv() {
    let cmd = "rsv frequency -c 1,2 -n 2 file";
    assert_eq!(end_row_of_cmd(cmd, "hotel_reservation.csv"), "1,0,7551");
}

#[test]
fn test_frequency_two_column_excel() {
    let cmd = "rsv frequency -c 1,2 -n 2 file";
    assert_eq!(end_row_of_cmd(cmd, "hotel_reservation.xlsx"), "1,0,7551");
}

#[test]
fn test_frequency_two_column_ascending_csv() {
    let cmd = "rsv frequency -c 1,2 -n 6 -a file";
    assert_eq!(end_row_of_cmd(cmd, "hotel_reservation.csv"), "0,3,5");
}

#[test]
fn test_frequency_two_column_ascending_excel() {
    let cmd = "rsv frequency -c 1,2 -n 6 -a file";
    assert_eq!(end_row_of_cmd(cmd, "hotel_reservation.xlsx"), "0,3,5");
}
