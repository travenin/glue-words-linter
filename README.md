# Glue words linter

This is a simple CLI tool linter for finding sentences with too many [glue words](https://en.wikipedia.org/wiki/Function_word) (> 50%) in a text given as a standard input.

This code was developed to demonstrate [Rust](https://www.rust-lang.org/) language at [Vincit's](https://www.vincit.com/) internal event.
The solution is not meant to be optimal, but rather to demonstrate some of the Rust language features.

## Install

Install Rust and Cargo for example with `rustup` (https://rustup.rs/) if you don't have them already.

To install without cloning the repository, run

```sh
cargo install --git https://github.com/travenin/glue-words-linter
```

which installs it to Cargo binary path (usually `~/.cargo/bin`).
The path should be in your `PATH` environment variable.

## Usage

To lint this `file.txt`:

```
What is this madness? What are we doing here?

I don't know, it just is what it is.
```

Run `glue-words-linter < file.txt` to lint the file.

```
$ glue-words-linter < file.txt
Line 1: What is this madness? (75%)
Line 3: I don't know, but it just is what it is. (60%)

Among 3 sentences there were 2 sticky ones (66.67%).
```

## Credits

This code was inspired by [@kopoli's](https://github.com/kopoli) demonstration in a similar event
where he implemented a similar linter in Go.

## License

This code is licensed under the MIT license.
See [LICENSE.md](./LICENSE.md) for more information.
