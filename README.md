# rusty-wc 🚾

Learn Rust hands-on by adding a feature to a rusty `wc` clone.

> This exercise was developed for Orca Security's Rust workshop, but it's open
> source under [the license](./LICENSE). Feel free to use it for your own
> workshops or personal learning, and contribute back if you find any issues or
> improvements!

![Rusty Orca](./docs/rusty-orca_256x256.webp)

* [What's the exercise?](#whats-the-exercise)
  * [Your task](#your-task)
    * [Step 0 // Get familiar with the code and environment](#step-0--get-familiar-with-the-code-and-environment)
    * [Step 1 // Add a new feature](#step-1--add-a-new-feature)
  * [Bonus points](#bonus-points)
    * [Step 2 // Add tests to your feature](#step-2--add-tests-to-your-feature)
    * [Step 3 // Benchmarking and optimization](#step-3--benchmarking-and-optimization)
* [Installation](#installation)
  * [IDEs](#ides)
    * [VSCode](#vscode)
    * [Vim](#vim)
    * [JetBrains IDEs](#jetbrains-ides)
* [Testing](#testing)

## What's the exercise?

Learn some rust by practicing a "real-life" example of adding a feature to
this CLI. The CLI we're basing off of is Good ol' `wc`. In this repo, you'll
find you already have a basic implementation of `wc` in Rust. It includes
support for three flags: `-l`, `-w`, and `-c`, which count lines, words, and
characters, respectively.

### Your task

#### Step 0 // Get familiar with the code and environment

1. Clone the repository, follow the [Installation](#installation) instructions,
   and open the project in your favorite IDE (see [IDEs](#ides)).
2. Run the tests to make sure everything is working (see [Testing](#testing)).
3. Run the program with `cargo run -- -h` to see the help message.
4. Run the program with `cargo run -- LICENSE CONTRIBUTING.md` to see the results.
5. Skim through the code to understand how it works, focusing on
   [`./src/main.rs`](./src/main.rs). Dependencies are managed in
   [`Cargo.toml`](./Cargo.toml).

#### Step 1 // Add a new feature

The feature you'll implement is adding a `-f` flag - which will count frequency
of words in the input files, and print the top 10 most frequent words. The `-f`
flag should be mutually exclusive with the other flags. If multiple files are
provided, the frequency should be calculated for all files combined.

### Bonus points

#### Step 2 // Add tests to your feature

If you're a TDD beast, you might have done this already! Add tests for the new
feature. For the `LICENSE` file, you can use these results as a reference:

```text
 309 the
 208 of
 174 to
 165 a
 131 or
 102 you
 89 that
 86 and
 72 this
 70 in
```

And for both `LICENSE` and `CONTRIBUTING.md`:

```text
 318 the
 208 of
 182 to
 168 a
 131 or
 104 you
 93 that
 88 and
 72 this
 70 for
```

#### Step 3 // Benchmarking and optimization

Use [Criterion](https://github.com/bheisler/criterion.rs) to benchmark your `-f`
implementation. Then, re-implement it with parallelism using
[Rayon](https://github.com/rayon-rs/rayon), and with the benchmark, compare the
performance of the parallel implementation with the sequential one.

## Installation

Follow the [Rust installation guide](https://www.rust-lang.org/tools/install).

The exercise was written with rustc `1.79.0-nightly`. Check yours with
`rustc --version` and if needed, run:

```sh
rustup update
```

### IDEs

> **_NOTE:_**  We recommend initially disabling CoPilot completions while working
> on the exercise, as experiencing friction with the syntax is good for learning.
> 
> Once you've experienced some friction with the syntax, re-enable CoPilot - but
> be mindful of the suggestions it gives you, as it might not always be the best
> way to solve the problem. Make sure to thouroughly read the code it suggests
> before using it, and make sure you understand it.

#### VSCode

In VSCode, install the
[Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
extension, and make sure to install a debugger as well. Try to debug one of the
tests to make sure everything works.

There are a few recommended settings to tweak:

```json
{
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "rust-analyzer.testExplorer": true
}
```

#### Vim

Install the official [rust.vim plugin](https://github.com/rust-lang/rust.vim).

#### JetBrains IDEs

For JetBrains IDEs, consider [RustRover](https://www.jetbrains.com/rust/). It's
still in preview at the time of writing, but it should be fully released by Sep
2024.

## Testing

There are two testing suites: unit tests in the `main.rs` file and integration
tests in the `tests/` directory. To run the tests, use:

```sh
cargo test -- --nocapture
```
