use std::process::Command;
mod common;

fn end_row_of_head(file: &str, no_header: bool) -> String {
    let mut output = Command::new(common::RSV_COMMAND);
    output.arg("head").arg("-n").arg("10");
    if no_header {
        output.arg("--no-header");
    }

    let output = output.arg(file).output().unwrap();

    String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .filter(|i| !i.is_empty())
        .last()
        .unwrap()
        .to_owned()
}

#[test]
fn test_head_csv_no_header() {
    let r = end_row_of_head("D:\\code\\rsv\\tests\\data\\hotel_reservation.csv", false);
    assert_eq!(
        r.as_str(),
        "INN00010,2,0,Meal Plan 1,Room_Type 4,Online,133.44,3"
    );
}

#[test]
fn test_head_csv_header() {
    let r = end_row_of_head("D:\\code\\rsv\\tests\\data\\hotel_reservation.csv", true);
    assert_eq!(
        r.as_str(),
        "INN00009,3,0,Meal Plan 1,Room_Type 1,Offline,96.9,1"
    );
}

#[test]
fn test_head_excel_no_header() {
    let r = end_row_of_head("D:\\code\\rsv\\tests\\data\\hotel_reservation.xlsx", false);
    assert_eq!(
        r.as_str(),
        "INN00010,2,0,Meal Plan 1,Room_Type 4,Online,133.44,3"
    );
}
#[test]

fn test_head_excel_header() {
    let r = end_row_of_head("D:\\code\\rsv\\tests\\data\\hotel_reservation.xlsx", true);
    assert_eq!(
        r.as_str(),
        "INN00009,3,0,Meal Plan 1,Room_Type 1,Offline,96.9,1"
    );
}
