use std::env::{consts::OS, current_exe};

pub fn rsv() -> String {
    let mut p = current_exe().unwrap();

    p.pop();
    p.pop();
    p.pop();
    p.pop();

    p.push("target");
    p.push("debug");

    if OS == "macos" || OS == "linux" {
        p.push("rsv");
    } else {
        p.push("rsv.exe");
    }

    // println!("file path: {}", p.display());
    // println!("OS: {}", OS);
    p.to_str().unwrap().to_owned()
}

pub fn data_path(f: &str) -> String {
    let mut p = current_exe().unwrap();

    p.pop();
    p.pop();
    p.pop();
    p.pop();

    p.push("rsv");
    p.push("tests");
    p.push("data");
    p.push(f);

    println!("file path: {}", p.display());
    p.to_str().unwrap().to_owned()
}
