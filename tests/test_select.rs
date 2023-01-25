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

    // println!("{:?}", output.stderr);
    // println!("{:?}", String::from_utf8(output.stderr));
    // println!("{:?}", String::from_utf8_lossy(&output.stderr));
    // io::stdout().write_all(&output.stderr).unwrap();

    String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .filter(|i| !i.is_empty())
        .last()
        .unwrap()
        .to_owned()
}

#[test]
fn test_select_csv_filter0() {
    let cmd = "rsv select file";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.csv"),
        "INN36275,2,0,Meal Plan 1,Room_Type 1,Offline,161.67,0"
    );
}

#[test]
fn test_select_csv_filter1() {
    let cmd = "rsv select -f 0=INN00001 file";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.csv"),
        "INN00001,2,0,Meal Plan 1,Room_Type 1,Offline,65,0"
    );
}

#[test]
fn test_select_csv_filter2() {
    let cmd = "rsv select -f 1N=2 file";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.csv"),
        "INN36275,2,0,Meal Plan 1,Room_Type 1,Offline,161.67,0"
    );
}

#[test]
fn test_select_csv_column_filter1() {
    let cmd = "rsv select -c 0,2,4 file";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.csv"),
        "INN36275,0,Room_Type 1"
    );
}

#[test]
fn test_select_csv_column_filter2() {
    let cmd = "rsv select -f 0=INN00001 -c 0,2,4 file";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.csv"),
        "INN00001,0,Room_Type 1"
    );
}

#[test]
fn test_select_csv_column_filter3() {
    let cmd = "rsv select -f 1N=2 -c 0,2,4 file";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.csv"),
        "INN36275,0,Room_Type 1"
    );
}

#[test]
fn test_select_excel_filter0() {
    let cmd = "rsv select file";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.xlsx"),
        "INN36275,2,0,Meal Plan 1,Room_Type 1,Offline,161.67,0"
    );
}

#[test]
fn test_select_excel_filter1() {
    let cmd = "rsv select -f 0=INN00001 file";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.xlsx"),
        "INN00001,2,0,Meal Plan 1,Room_Type 1,Offline,65,0"
    );
}

#[test]
fn test_select_excel_filter2() {
    let cmd = "rsv select -f 1N=2 file";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.xlsx"),
        "INN36275,2,0,Meal Plan 1,Room_Type 1,Offline,161.67,0"
    );
}

#[test]
fn test_select_excel_column_filter1() {
    let cmd = "rsv select -c 0,2,4 file";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.xlsx"),
        "INN36275,0,Room_Type 1"
    );
}

#[test]
fn test_select_excel_column_filter2() {
    let cmd = "rsv select -f 0=INN00001 -c 0,2,4 file";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.xlsx"),
        "INN00001,0,Room_Type 1"
    );
}

#[test]
fn test_select_excel_column_filter3() {
    let cmd = "rsv select -f 1N=2 -c 0,2,4 file";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.xlsx"),
        "INN36275,0,Room_Type 1"
    );
}

#[test]
fn test_select_io_filter0() {
    let cmd = "rsv slice file | rsv select";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.csv"),
        "INN36275,2,0,Meal Plan 1,Room_Type 1,Offline,161.67,0"
    );
}

#[test]
fn test_select_io_filter1() {
    let cmd = "rsv slice file | rsv select -f 0=INN00001";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.csv"),
        "INN00001,2,0,Meal Plan 1,Room_Type 1,Offline,65,0"
    );
}

#[test]
fn test_select_io_filter2() {
    let cmd = "rsv slice file | rsv select -f 1N=2";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.csv"),
        "INN36275,2,0,Meal Plan 1,Room_Type 1,Offline,161.67,0"
    );
}

#[test]
fn test_select_io_column_filter1() {
    let cmd = "rsv slice file | rsv select -c 0,2,4";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.csv"),
        "INN36275,0,Room_Type 1"
    );
}

#[test]
fn test_select_io_column_filter2() {
    let cmd = "rsv slice file | rsv select -f 0=INN00001 -c 0,2,4";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.csv"),
        "INN00001,0,Room_Type 1"
    );
}

#[test]
fn test_select_io_column_filter3() {
    let cmd = "rsv slice file | rsv select -f 1N=2 -c 0,2,4";
    assert_eq!(
        end_row_of_cmd(cmd, "hotel_reservation.csv"),
        "INN36275,0,Room_Type 1"
    );
}
