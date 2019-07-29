# nfd-rs [![](http://meritbadge.herokuapp.com/nfd)](https://crates.io/crates/nfd) [![](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/saurvs/nfd-rs/blob/master/LICENSE.md)

`nfd-rs` is a Rust binding to the library [nativefiledialog](https://github.com/mlabbe/nativefiledialog), that provides a convenient cross-platform interface to opening file dialogs on Linux, OS X and Windows.

This crate has been tested on Mac, Window and Linux (Ubuntu 14.04) and supports single/mutliple and save dialogs, notice APIs may break with newer versions.

## Usage

* Add the dependency `nfd` in your ```Cargo.toml```
  ```toml
  [dependencies]
  nfd = "0.0.4"
  ```

* Open a single file dialog
  ```rust
  extern crate nfd;

  use nfd::Response;

  fn main() {

    let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
    	panic!(e);
    });

    match result {
        Response::Okay(file_path) => println!("File path = {:?}", file_path),
        Response::OkayMultiple(files) => println!("Files {:?}", files),
        Response::Cancel => println!("User canceled"),
    }
  }
  ```

* Open a multi file dialog using builder with jpg files as filter
  ```rust
  extern crate nfd;

  use nfd::Response;

  fn main() {

    let result = nfd::dialog_multiple().filter("jpg").open().unwrap_or_else(|e| {
    	panic!(e);
    });

    match result {
        Response::Okay(file_path) => println!("File path = {:?}", file_path),
        Response::OkayMultiple(files) => println!("Files {:?}", files),
        Response::Cancel => println!("User canceled"),
    }
  }
  ```

## Screenshot

![Cocoa on El Capitan](screenshots/cocoa_el_capitan.png?raw=true)
