#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""Wrapper script to download and run a suitable version of ANTLR for
generating or verifying the Rust bindings for a given grammar."""

import urllib.request
import os
import sys
import hashlib
import logging
import tempfile
import shutil
import filecmp
import subprocess
import difflib
import argparse


# NOTE: the Rust bindings for ANTLR are not (yet) official, so we need to
# download a forked ANTLR build.
ANTLR_URL = "https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar"
ANTLR_SHA1 = "775d24ac1ad5df1eb0ed0e802f0fb2a5aeace43c"


class Failure(Exception):
    """Used for fatal errors."""


def fail(msg):
    """Logs and throws an error message."""
    logging.error(msg)
    raise Failure(msg)


def download_file(fname, url):
    """Downloads a file if it does not already exist."""
    if not os.path.isfile(fname):
        logging.info(f"Downloading {fname}...")
        urllib.request.urlretrieve(ANTLR_URL, fname)


def verify_file_hash(fname, hash_str):
    """Verifies the hash of a (downloaded) file."""
    logging.info(f"Verifying {fname}...")
    with open(fname, "rb") as f:
        file_hash = hashlib.sha1()
        while chunk := f.read(8192):
            file_hash.update(chunk)
    actual = file_hash.hexdigest()
    if hash_str != actual:
        fail(f"Verification failed; hash should be {hash_str} but was {actual}")


def verify_file_identical(new, old):
    """Verifies that two text files are identical, printing a diff if not."""
    logging.info(f"Verifying {new} against {old}...")
    if not os.path.isfile(new):
        fail(f"{new} does not exist")
    if not os.path.isfile(old):
        fail(f"{old} does not exist")
    if not filecmp.cmp(new, old, shallow=False):
        with open(new, "r") as f:
            new_data = f.readlines()
        with open(old, "r") as f:
            old_data = f.readlines()
        sys.stdout.writelines(difflib.unified_diff(old_data, new_data, old, new))
        fail(f"{new} is different, see diff")


def run_antlr(antlr, grammar, output_dir, verify=False, java="java"):
    """Runns the given ANTLR JAR on the given grammar, sending outputs to
    output_dir. If verify is set, instead of copying the newly-generated files,
    this checks that there are no differences between the newly and previously
    generated files."""
    logging.info("Running ANTLR...")

    # Determine the names of the generated files that we're interested in.
    name = os.path.basename(grammar).split(".")[0].lower()
    expected_files = [f"{name}lexer.rs", f"{name}parser.rs", f"{name}listener.rs"]

    # Run in a temporary directory, because ANTLR spams random files we didn't
    # ask for in its working directory.
    with tempfile.TemporaryDirectory() as generate_dir:
        shutil.copyfile(grammar, os.path.join(generate_dir, os.path.basename(grammar)))
        subprocess.run(
            [
                java,
                "-jar",
                os.path.realpath(antlr),
                "-Dlanguage=Rust",
                os.path.basename(grammar),
            ],
            cwd=generate_dir,
        )

        logging.info("Copying/verifying output files...")
        for expected_file in expected_files:
            src = os.path.join(generate_dir, expected_file)
            dest = os.path.join(output_dir, expected_file)
            if not os.path.isfile(src):
                fail(f"ANTLR failed to generate {expected_file}")
            with open(src, "r+") as f:
                data = f.read()
                data = (
                    "// SPDX-License-Identifier: Apache-2.0\n"
                    "#![allow(clippy::all)]\n"
                    "#![cfg_attr(rustfmt, rustfmt_skip)]\n"
                    f"{data}"
                )
                f.seek(0)
                f.write(data)
            if verify:
                verify_file_identical(src, dest)
            else:
                if os.path.exists(dest):
                    os.unlink(dest)
                shutil.copyfile(src, dest)


def main(*args):
    """Utility to generate Rust bindings for an ANTLR grammar."""
    parser = argparse.ArgumentParser(description=main.__doc__)
    parser.add_argument(
        "--antlr",
        metavar="antlr.jar",
        default=os.path.join(os.path.dirname(os.path.realpath(__file__)), "antlr.jar"),
        help="alternate location for the ANTLR jar",
    )
    parser.add_argument(
        "--no-download",
        action="store_true",
        help="don't attempt to download the ANTLR jar",
    )
    parser.add_argument(
        "--no-verify",
        action="store_true",
        help="don't attempt to verify the hash of the ANTLR jar",
    )
    parser.add_argument(
        "--java", default="java", help="path to java executable to call ANTLR with"
    )
    parser.add_argument(
        "--ci-check",
        action="store_true",
        help="instead of regenerating the files, assert that the files do not need to be regenerated",
    )
    parser.add_argument("grammar", help="the .g4 grammar file to generate")
    parser.add_argument(
        "dest_dir", default=".", nargs="?", help="where to copy the generated files to"
    )
    args = parser.parse_args(args)

    logging.basicConfig(level=logging.INFO)

    # Acquire ANTLR jar.
    if args.no_download:
        if not os.path.isfile(args.antlr):
            parser.error(f"{args.antlr} does not exist and auto-download is disabled")
    else:
        download_file(args.antlr, ANTLR_URL)
    if not args.no_verify:
        verify_file_hash(args.antlr, ANTLR_SHA1)

    # Run ANTLR.
    if not os.path.isfile(args.grammar):
        parser.error(f"{args.grammar} does not exist")
    run_antlr(
        args.antlr, args.grammar, args.dest_dir, verify=args.ci_check, java=args.java
    )


if __name__ == "__main__":
    try:
        main(*sys.argv[1:])
        logging.info("Done")
    except Failure:
        logging.info("Returning failure exit status")
        sys.exit(1)
