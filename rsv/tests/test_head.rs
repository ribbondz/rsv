use execute::shell;
mod common;
use crate::common::{data_path, rsv};

fn end_row_of_head(file: &str, no_header: bool, io: bool) -> String {
    let rsv = rsv();
    let cmd = match (no_header, io) {
        (true, false) => format!("{rsv} head -n 10 --no-header {file}"),
        (false, false) => format!("{rsv} head -n 10 {file}"),
        (true, true) => format!("{rsv} slice -l 10 {file} | {rsv} head -n 2 --no-header"),
        (false, true) => format!("{rsv} slice -l 10 {file} | {rsv} head -n 2"),
    };

    // println!("111111 {:?}", cmd);
    let mut cmd = shell(cmd);
    let output = cmd.output().unwrap();

    String::from_utf8(output.stdout)
        .unwrap()
        .split('\n').rfind(|i| !i.is_empty())
        .unwrap()
        .to_owned()
}

#[test]
fn test_head_csv_header() {
    let r = end_row_of_head(&data_path("hotel_reservation.csv"), false, false);
    assert_eq!(
        r.as_str(),
        "INN00010,2,0,Meal Plan 1,Room_Type 4,Online,133.44,3"
    );
}

#[test]
fn test_head_csv_no_header() {
    let r = end_row_of_head(&data_path("hotel_reservation.csv"), true, false);
    assert_eq!(
        r.as_str(),
        "INN00009,3,0,Meal Plan 1,Room_Type 1,Offline,96.9,1"
    );
}

#[test]
fn test_head_excel_header() {
    let r = end_row_of_head(&data_path("hotel_reservation.xlsx"), false, false);
    assert_eq!(
        r.as_str(),
        "INN00010,2,0,Meal Plan 1,Room_Type 4,Online,133.44,3"
    );
}
#[test]

fn test_head_excel_no_header() {
    let r = end_row_of_head(&data_path("hotel_reservation.xlsx"), true, false);
    assert_eq!(
        r.as_str(),
        "INN00009,3,0,Meal Plan 1,Room_Type 1,Offline,96.9,1"
    );
}

#[test]
fn test_head_io_header() {
    let r = end_row_of_head(&data_path("hotel_reservation.csv"), false, true);
    assert_eq!(
        r.as_str(),
        "INN00002,2,0,Not Selected,Room_Type 1,Online,106.68,1"
    );
}
#[test]

fn test_head_io_no_header() {
    let r = end_row_of_head(&data_path("hotel_reservation.csv"), true, true);
    assert_eq!(
        r.as_str(),
        "INN00001,2,0,Meal Plan 1,Room_Type 1,Offline,65,0"
    );
}
