// SPDX-License-Identifier: Apache-2.0

//! Crate for validating [Substrait](https://substrait.io/).
//!
//! # Usage
//!
//! The usage pattern is roughly as follows.
//!
//!  1) Build a [`Config`] structure to configure the validator. You can also
//!     just use [`std::default::Default`] if you don't need to configure
//!     anything, but you might want to at least call
//!     [`Config::add_curl_uri_resolver()`] (if you're using the `curl`
//!     feature).
//!  2) Parse the incoming `substrait.Plan` message using [`parse()`]. This
//!     creates a [ParseResult], containing a [tree](output::tree) structure
//!     corresponding to the query plan that also contains diagnostics and
//!     other annotations added by the validator.
//!  3) You can traverse the tree yourself using [ParseResult::root], or you
//!     can use one of the methods associated with [ParseResult] to obtain the
//!     validation results you need.
//!
//! Note that only the binary protobuf serialization format is supported at the
//! input; the JSON format is *not* supported. This is a limitation of `prost`,
//! the crate that was used for protobuf deserialization. If you're looking for
//! a library (or CLI) that supports more human-friendly input, check out the
//! Python bindings.
#![cfg_attr(
    feature = "private_docs",
    allow(rustdoc::private_intra_doc_links),
    doc = "
# Internal workings

*Very* roughly speaking, the validation process boils down to a conversion from
[one type of tree structure](input::proto::substrait::Plan) (including
expansions of any referenced YAML files) to [another](output::tree::Node),
using the facilities provided by the [parse module](mod@parse). This process is
documented in much more detail [here](mod@parse). Once constructed, the
resulting tree can then be [further converted](export) to a few export formats,
or the validation [diagnostics](output::diagnostic) can simply be
[extracted](ParseResult::iter_diagnostics()).

This crate only supports the binary protobuf serialization format as input;
that conversion is ultimately done [here](parse::traversal::parse_proto())
using a combination of [prost] and some unfortunate magic in
[substrait_validator_derive]. That is to say: it does *NOT* support JSON format
or variations thereof. This is because support for protobuf JSON is flaky
beyond the official bindings, likely in no small part due to all the case
conversion magic and special cases crammed into that format. Since there are no
official protobuf bindings for Rust, there is no way to do this from within the
crate without reinventing the wheel as a square.

Instead, the Python bindings, generated using
[maturin](https://github.com/PyO3/maturin), include the user-facing logic for
this. This is also the primary reason why the CLI is written in Python, rather
than in Rust. When a format other than binary protobuf is passed to the Python
package, it uses the official protobuf bindings for Python to (re)serialize to
the binary format, before handing control to the Rust crate. For the return
trip, the protobuf export format (using the message tree defined in the
[substrait.validator](https://github.com/substrait-io/substrait-validator/blob\
/main/proto/substrait/validator/validator.proto) protobuf namespace) is used to
pass the parse result to Python.

C bindings also exist. These are of the not-very-user-friendly sort, however;
they exist primarily to allow the validator to be used from within the testing
frameworks of whatever language you want, provided they support calling into
C-like libraries.

## Testing strategy

Currently, this crate has (almost) no test cases of its own. This is primarily
to do with the fact that validating only part of a plan would require complex
context setup and that, ideally, the (bits of) plan for the test cases are
written in either JSON or a yet-more user-friendly variant thereof. For the
reasons given above, this can't really be done from within Rust.

Instead, tests are run using the [test-runner crate](https://github.com/\
    substrait-io/substrait-validator/tree/main/tests) and its associated Python
frontend. The Python frontend pre-parses YAML test description files into a
JSON file that's easy to read from within Rust via serde-json, after which the
Rust crate takes over to run the test cases. The pre-parsing involves
converting the JSON-as-YAML protobuf tree into the binary serialization format,
but also allows diagnostic presence checks to be inserted in the plan where
they are expected (rather than having to link up the tree paths manually) and
allows YAML extensions to be specified inline (they'll be extracted and
replaced with a special URI that the test runner understands).

The APIs for the bindings on top of the Rust crate are tested using
[pytest](https://docs.pytest.org/) (Python) and
[googletest](https://google.github.io/googletest/) (C).

## Resolving extension URIs

URI resolution deserves an honorable mention here, because it unfortunately
can't easily be hidden away in some private module: anything that uses HTTPS
must either link into the operating system's certificate store or ship its own
root certificates. The latter is sure to be a security issue, so let's restrict
ourselves to the former solution.

The problem with this is that it pollutes the Rust crate with runtime linking
shenanigans that are not at all compatible from one system to another. In
particular, we can't build universal Python packages around crates that do
this. Since we rely on Python for the CLI, this is a bit of an issue.

For this reason, URI resolution is guarded behind the `curl` feature. When the
feature is enabled, `libcurl` will be used to resolve URIs, using the system's
certificate store for HTTPS. When disabled, the crate will fall back to
resolving only `file://` URIs, unless a more capable resolver is
[installed](Config::add_uri_resolver()). The Python bindings will do just that:
they install a resolver based on Python's own
[urllib](https://docs.python.org/3/library/urllib.html).

## Build process

The build process for the crates and Python module also involves some
not-so-obvious magic, to do with shipping the Substrait protobuf and YAML
schema as appropriate. The problem is that Cargo and Python's packaging logic
require that all files shipped with the package be located within the package
source tree, which is not the case here due to the common submodule and proto
directories.

### Rust

If the [`in-git-repo` file](https://github.com/substrait-io/\
substrait-validator/blob/main/rs/in-git-repo) exists, the
[build.rs file for this crate](https://github.com/substrait-io/\
substrait-validator/blob/main/rs/build.rs) will copy the proto and schema files
from their respective source locations into `src/resources`, thus keeping them
in sync. The `in-git-repo` file is not included in the crate manifest, so this
step is skipped when the crate is compiled after being downloaded from
crates.io. Note however, that in order to release this crate, it must always
first be built: the only time during the packaging process when build.rs is
called is already on the user's machine, so the resource files won't be
synchronized by `cargo package`.

### Python

The process for Python is much the same, but handled by a
[wrapper around maturin](https://github.com/substrait-io/substrait-validator/\
blob/main/py/substrait_validator_build/__init__.py), as maturin does not expose
pre-build hooks of its own. The `in-git-repo` file isn't necessary here; we can
use the `local_dependencies` file that will be generated by the packaging tools
as part of a source distribution as a marker.

Here, too, it's important that the synchronization logic is run manually prior
to various release-like operations. This can be done by running
[prepare_build.py](https://github.com/substrait-io/substrait-validator/blob/\
main/py/prepare_build.py).

### Protobuf

In order to rely on as few external dependencies as possible, all protoc
invocations by the various parts of the build invoke the `protoc` executable
as found/compiled and exposed by [prost-build](prost-build). That is: this
protoc is also abused to generate Python bindings. Unfortunately, prost-build
is planning to [remove](https://github.com/tokio-rs/prost/pull/620) the build
logic for protoc at the time of writing (and who can blame them), so this will
need to be done differently in the future.
    "
)]

#[macro_use]
pub mod output;

#[macro_use]
mod parse;

pub mod export;
pub mod input;

mod util;

use std::str::FromStr;

use strum::IntoEnumIterator;

// Aliases for common types used on the crate interface.
pub use input::config::glob::Pattern;
pub use input::config::Config;
pub use output::comment::Comment;
pub use output::diagnostic::Classification;
pub use output::diagnostic::Diagnostic;
pub use output::diagnostic::Level;
pub use output::parse_result::ParseResult;
pub use output::parse_result::Validity;

/// Validates the given substrait.Plan message and returns the parse tree.
pub fn parse<B: prost::bytes::Buf + Clone>(buffer: B, config: &Config) -> ParseResult {
    parse::parse(buffer, config)
}

/// Returns an iterator that yields all known diagnostic classes.
pub fn iter_diagnostics() -> impl Iterator<Item = Classification> {
    Classification::iter()
}

/// Returns the version of the validator.
pub fn version() -> semver::Version {
    semver::Version::from_str(env!("CARGO_PKG_VERSION")).expect("invalid embedded crate version")
}

/// Returns the version of Substrait that this version of the validator was
/// built against.
pub fn substrait_version() -> semver::Version {
    semver::Version::from_str(include_str!("resources/substrait-version"))
        .expect("invalid embedded Substrait version")
}

/// Returns the Substrait version requirement for plans to be known to be
/// supported.
pub fn substrait_version_req() -> semver::VersionReq {
    let version = substrait_version();
    if version.major == 0 {
        semver::VersionReq::parse(&format!("={}.{}", version.major, version.minor)).unwrap()
    } else {
        semver::VersionReq::parse(&format!("={}", version.major)).unwrap()
    }
}

/// Returns the Substrait version requirement for plans to possibly be
/// supported.
pub fn substrait_version_req_loose() -> semver::VersionReq {
    let version = substrait_version();
    semver::VersionReq::parse(&format!("={}", version.major)).unwrap()
}
