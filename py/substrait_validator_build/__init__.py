# SPDX-License-Identifier: Apache-2.0

from maturin import *
import shutil
import os
import sys


_PATHS = [
    (os.path.join("..", "proto"), "proto"),
    (os.path.join("..", "substrait", "proto"), "proto"),
    (os.path.join("..", "substrait", "text"), "text"),
    (os.path.join("..", "LICENSE"), "LICENSE"),
    (None, "protoc_out"),
    (None, "substrait_validator/substrait"),
    (None, "substrait_validator/__pycache__"),
]


def clean():
    for _, path in _PATHS:
        if os.path.isdir(path):
            shutil.rmtree(path)
        elif os.path.isfile(path):
            os.unlink(path)


def _copytree(source, dest):
    if os.path.isdir(source):
        if not os.path.isdir(dest):
            os.makedirs(dest)
        files = os.listdir(source)
        for f in files:
            _copytree(os.path.join(source, f), os.path.join(dest, f))
    elif os.path.isfile(source):
        shutil.copyfile(source, dest)
    else:
        rel_path = os.path.relpath(source, "..")
        abs_path = os.path.realpath(source)
        path = os.path.join("(git-root)", rel_path)
        msg = f"Could not find {path}."
        if rel_path.startswith("substrait"):
            msg = f"{msg} Did you check out submodules?"
        msg = f"{msg} Full path = {abs_path}"
        raise FileNotFoundError(msg)


def populate():
    clean()
    for source, dest in _PATHS:
        if source is not None:
            _copytree(source, dest)


def _prepare():
    # If the local_dependencies directory exists, pip is building the package
    # from a source distribution. In that case, the build environment is
    # already as it should be.
    if os.path.isdir("local_dependencies"):
        return

    # Outside of building from specially-prepared sdist packages, we don't
    # support out-of-tree builds, because that breaks Maturin and Cargo. The
    # error messages you'd get if you were to try would be cryptic at best, so
    # we detect this and fail more gently here.
    if os.path.basename(os.path.realpath(".")) != "py":
        print(
            "\n"
            "=================================================================\n"
            "Out-of-tree build detected. This is not supported. Please\n"
            "configure pip (or whatever build system you're using) to build\n"
            "in-tree, for example using an editable install.\n"
            "See https://github.com/substrait-io/substrait-validator/issues/35\n"
            "=================================================================\n",
            file=sys.stderr,
        )
        sys.exit(1)

    populate()


_maturin_prepare_metadata_for_build_wheel = (
    prepare_metadata_for_build_wheel  # noqa: F405
)


def prepare_metadata_for_build_wheel(*args, **kwargs):
    _prepare()
    return _maturin_prepare_metadata_for_build_wheel(*args, **kwargs)


_maturin_build_wheel = build_wheel  # noqa: F405


def build_wheel(*args, **kwargs):
    _prepare()
    return _maturin_build_wheel(*args, **kwargs)


_maturin_build_sdist = build_sdist  # noqa: F405


def build_sdist(*args, **kwargs):
    _prepare()
    return _maturin_build_sdist(*args, **kwargs)
