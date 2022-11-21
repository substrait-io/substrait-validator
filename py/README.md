# Python bindings for validator

This directory contains a Rust/PyO3 project to generate Python bindings for the
validator library.

## Installation

The easiest way to install the validator is to get it from PyPI:

```console
user@host:~$ pip install substrait-validator
```

If you want to build manually, running something along the lines of
`pip install .` should work. You should only need to have a
[rust](https://www.rust-lang.org/tools/install) compiler installed.

If you want to create wheels or sdists of your own, you can do so using
maturin (note the manual prepare_build.py invocation, see hints):

```console
user@host:~$ pip install "maturin>=0.14,<0.15"
...
user@host:~$ ./prepare_build.py populate
user@host:~$ maturin build
...
📦 Built wheel for CPython 3.x to /path/to/substrait-validator/target/wheels/substrait_validator-x.y.z-cp3xx-cp3xx-linux_x86_64.whl
user@host:~$ maturin sdist
...
📦 Built source distribution to /path/to/substrait-validator/target/wheels/substrait_validator-x.y.z.tar.gz
```

Some hints if you run into issues:

 - This project relies on submodules, so you need to check those out first.
 - Out-of-tree builds are not supported, so you may need to beat pip into
   submission if you're using an old version or it otherwise insists on
   building from a temp directory.
 - If you get weird errors, try running `./prepare_build.py populate` manually
   first. The protobuf generation logic has to be run very early in the build
   process, and while this is done automatically for most build methods, not
   all methods provide a hook for this.

## Building wheels and source distributions

You can build wheels and source distributions using
[maturin](https://github.com/PyO3/maturin), specifically using the `build` and
`sdist` commands. However, before you can do this, you must run
`./prepare_build.py populate`. This makes local copies of some files in the
repository that live outside of this subdirectory, such as the protobuf
description files. When you use `pip` or some other tool based on
`pyproject.toml`, this will be done automatically via build system hooks, but
unfortunately maturin doesn't itself provide hooks with which this can be
automated.

## Running tests

You can test the module using `pytest` after you install it.

## Command-line usage

The module exposes a command-line program named `substrait-validator` for
running the validator manually. You can also use the tool to convert between
various serialization formats of the `substrait.Plan` message. Run
`substrait-validator --help` for more information.

## Library usage

The library essentially provides a bunch of type conversion functions at
module scope to convert between the various representations of a Substrait
plan, including the result of the validator. The most important functions are
arguably `check_plan_valid(plan, config=None)` and
`check_plan_not_invalid(plan, config=None)`, which run validation on the given
plan and throw a Python exception corresponding to the first diagnostic
returned by the validator of the highest severity encountered if the plan is
not strictly or loosely valid respectively. That is, `check_plan_valid` will
throw an exception if the plan could not be proven to be valid, while
`check_plan_not_invalid` will only throw if it could be proven to be invalid.

The `plan` argument can be a number of things:

 - `bytes`: treated as a binary serialization of `substrait.Plan`.
 - `str`: treated as a protobuf JSON serialization of `substrait.Plan`.
 - `dict`: treated as the above using Python's data model (JSON objects map
   to `dict`s, JSON arrays map to `list`s).
 - `substrait_validator.substrait.Plan`: a previously deserialized plan.
 - `substrait_validator.ResultHandle`: a previously validated plan.

`config` can be `None`/unspecified, or can be set to a
`substrait_validator.Config` object to configure the validator with.

For more information, use Python's `help()` function.
