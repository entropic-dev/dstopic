use dstopic;
use std::fs;

fn main() {
    dstopic::parse_args::parse_args().get_matches();
    println!("{}", fs::read_to_string("./Cargo.toml").unwrap().len());


    let locale = "en-US";
    let bundle = dstopic::localization::get_resource_bundle(locale).unwrap();

    let (value, _) = bundle
        .format("hello-world", None)
        .expect("Failed to format a message");
    println!("{}", value);
}
