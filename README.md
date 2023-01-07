# Discoverer

This crate exposes a single method for recursively walking through directories to find files that match a particular extension.

## Usage

Add `discoverer` to your `Cargo.toml`:

```toml
[dependencies]
discoverer = "0.2.0"
```

Or use `cargo add discoverer`

## Example

```rs
use discoverer::discover;

fn main() {
    let discoveries = discover(&["txt"], &[
        &concat!(env!("CARGO_MANIFEST_DIR"), "/examples/fixtures")
    ]).unwrap();

    dbg!(discoveries);
}
```

## Credits

* [Ryan Chandler](https://github.com/ryangjchandler)
* [All contributors](https://github.com/php-rust-tools/discoverer/graphs/contributors)