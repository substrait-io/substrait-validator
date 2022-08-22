#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

import sys
import logging
import argparse
import antlr

"""Script for regenerating or verifying all the ANTLR-generated files of the
validator."""


def main(*args):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--ci",
        action="store_true",
        help="instead of regenerating, verify that the files don't need to be regenerated",
    )
    args = parser.parse_args(args)

    logging.basicConfig(level=logging.INFO)

    ci = ["--ci-check"] if args.ci else []
    antlr.main(
        "../src/parse/extensions/simple/derivations/SubstraitType.g4",
        "../src/parse/extensions/simple/derivations",
        *ci,
    )


if __name__ == "__main__":
    try:
        main(*sys.argv[1:])
        logging.info("Done")
    except antlr.Failure:
        logging.info("Returning failure exit status")
        sys.exit(1)
