extern crate nfd;

use nfd::Response;

fn main() {
    let result = nfd::dialog().filter("jpg").open().unwrap_or_else(|e| {
        panic!(e);
    });

    match result {
        Response::Okay(file_path) => println!("File path = {:?}", file_path),
        Response::Cancel => println!("User canceled"),
        _ => (),
    }
}
