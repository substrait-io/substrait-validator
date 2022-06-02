# Contributing to the Substrait validator

Contributions are welcome! Here are some ways you can help:

 - Try to break it. Get it to emit an error or warning for something you believe to be completely valid, or get it to emit nothing or only a warning for something you believe to be invalid. The validator is currently still considered to be experimental.
 - Help with reviewing PRs and keeping up with Substrait.
 - Improve the built-in plan documentation. The validator tries to emit plain English descriptions of various nodes in the tree to explain what they do, but this is far from complete.
 - Submit additional integration tests, for more and better queries. Currently about half of TPC-H exists, but the quality of the plans is very poor (no optimization, partially handwritten, may be wholly incorrect, mostly restricted to the set of things Isthmus generates).
 - Look through the issues to see if anything needs solving.
 - Or maybe we're missing something we haven't even thought of yet!

## Licensing

All contributions should be licensed under Apache 2.0. We use SPDX license headers to minimize clutter. CI will ensure that all files have such a header.

## Code style

Code style is strictly enforced for all languages using CI and (to some extent) [pre-commit](https://pre-commit.com/) via:

 - Rust: [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy).
    - At the time of writing, stable clippy (1.60 and 1.61) panics on the codebase; hopefully this will be fixed soon. In CI the toolchain for linting is pinned to 1.59 for this reason. You can use Rust 1.59 to run clippy locally as well (`rustup install 1.59.0`, `cargo +1.59.0 clippy`) or you can leave it to CI, but either way you'll have to silence pre-commit (run it, see if there are no violations other than the clippy panic, then commit using `--no-verify`).
 - Python: [black](https://github.com/psf/black) and [flake8](https://flake8.pycqa.org/en/4.0.1/).
 - C: [clang-format](https://clang.llvm.org/docs/ClangFormat.html).
 - protobuf: [buf format](https://buf.build/blog/introducing-buf-format).
 - YAML: [yamllint](https://github.com/adrienverge/yamllint).

## Development dependencies

Here's a (probably non-exhaustive) list of things you may want to have installed to develop for the validator.

 - [Rust stable](https://rustup.rs/)
 - VSCode with [rust-analyzer](https://rust-analyzer.github.io/) (or some other IDE with Rust support)
 - Python 3.x (all non-EOL versions should be supported)
 - The toolchain should be able to compile libprotobuf for you if you don't already have it, but it's probably a good idea to have a reasonably recent version installed system-wide as well
 - [pre-commit](https://pre-commit.com/), so you don't have to rely on CI to catch all your errors, and to help format your code
 - git (obviously)
 - for the C bindings: [CMake](https://cmake.org/) and a C compiler (gcc, clang, and MSVC should all work; the bindings are very lightweight)

Note: this list is probably non-exhaustive; if you find something missing from this list, please add it!

## Contributor FAQ

### What if I've never written Rust before?

Probably not a problem! Getting a Rust dev environment up and running is [very](https://rustup.rs/) [easy](https://code.visualstudio.com/docs/languages/rust), and the actual validation part of the validator is very structured once you know the patterns, and hides much of the nitty-gritty details. Unless you're planning on working on the various internal support libraries, you shouldn't run into many surprises.

### There's lots of code here. Where do I begin?

After getting all the dependencies set up and playing around with the validator [command line](https://github.com/substrait-io/substrait-validator/tree/main/py) for a bit, run `cargo doc --all-features --document-private-items --open` to generate and open the internal documentation. The toplevel `substrait_validator` crate documentation includes the overview of the validator project as a whole (i.e. including some pointers for the foreign language bindings).

### What if I'm still really confused after that?

[Ask us](https://substrait.io/community/)!
