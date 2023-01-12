use tabled::builder::Builder;
use tabled::Style;

pub fn print_frequency_table(names: &Vec<String>, freq: Vec<(String, i32)>) {
    let mut builder = Builder::default();

    // header
    if !names.is_empty() {
        builder.set_columns(names);
    }

    // content
    for (key, n) in freq {
        let r = key
            .split(',')
            .map(|i| i.to_owned())
            .chain(std::iter::once(n.to_string()))
            .collect::<Vec<_>>();
        builder.add_record(r);
    }

    // build
    let mut table = builder.build();

    // style
    table.with(Style::blank());

    println!("{table}");
}
