# SPDX-License-Identifier: Apache-2.0

from maturin import *
import shutil
import os


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
    if not os.path.isdir("local_dependencies"):
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
