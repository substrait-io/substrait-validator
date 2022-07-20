#!/usr/bin/python3
# SPDX-License-Identifier: Apache-2.0

"""
Waits for a crate to appear on crates.io, since cargo publish doesn't.
See https://github.com/rust-lang/cargo/issues/9507
"""


import urllib.request
import json
import sys
import time
import argparse


def crate_version_exists(crate, version):
    """Returns whether the given version of the given crate exists on the
    crates.io index."""

    # Fetch the version info for the crate.
    with urllib.request.urlopen(
        "https://raw.githubusercontent.com/rust-lang/crates.io-index/"
        f"master/{crate[0:2]}/{crate[2:4]}/{crate}"
    ) as f:
        data = f.read()

    # Parse version info.
    versions = list(map(json.loads, data.decode("utf-8").strip().split("\n")))

    # Check whether the requested version exists.
    for version_data in versions:
        if version_data.get("vers", None) == version:
            return True
    return False


def check(crate, version):
    """Exits with code 0 if the given crate + version is found."""

    try:
        if crate_version_exists(crate, version):
            print("Crate found!")
            sys.exit(0)
    except Exception as e:
        print(f"{type(e).__name__}: {e}")


if __name__ == "__main__":

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("crate", type=str, help="the crate to wait for")
    parser.add_argument("version", type=str, help="the version to wait for")
    parser.add_argument("timeout", type=int, help="number of seconds to wait")
    args = parser.parse_args()

    print("Checking...")
    check(args.crate, args.version)

    wait_remain = args.timeout
    period = 10
    while wait_remain > 0:
        print(f"Waiting {period}s...", flush=True)
        time.sleep(period)
        wait_remain -= period
        period = int(period * 1.25)

        print("Checking...", flush=True)
        check(args.crate, args.version)

    print("Timeout expired!")
    sys.exit(1)
