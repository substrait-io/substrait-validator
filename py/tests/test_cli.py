# SPDX-License-Identifier: Apache-2.0

from click.testing import CliRunner
from substrait_validator import cli
from data import BASIC_PLAN, COMPLEX_PLAN
import tempfile
import json
import pprint
from os.path import join as pjoin
from os.path import isfile


def run(*args):
    return CliRunner().invoke(cli, args)


def test_no_args():
    result = run()
    assert result.exit_code == 2
    assert "Missing input file." in result.output


def test_mconvert_auto():
    """Test -mconvert with automatic format deduction from file extensions."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, "plan.json"), "w") as f:
            f.write(BASIC_PLAN)

        def convert(src, dest):
            assert (
                run(pjoin(tmp, src), "-O", pjoin(tmp, dest), "-mconvert").exit_code == 0
            )

        convert("plan.json", "plan.proto")

        with open(pjoin(tmp, "plan.proto"), "rb") as f:
            a = f.read()

        convert("plan.proto", "plan.yaml")
        convert("plan.yaml", "plan.jdot")
        convert("plan.jdot", "plan.json")
        convert("plan.json", "plan.bin")

        with open(pjoin(tmp, "plan.bin"), "rb") as f:
            b = f.read()

        assert a == b


def test_mconvert_manual():
    """Test -mconvert with automatic format deduction from file extensions."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, "data"), "w") as f:
            f.write(BASIC_PLAN)

        def convert(in_type, out_type):
            assert (
                run(
                    pjoin(tmp, "data"),
                    "-O",
                    pjoin(tmp, "data"),
                    "-mconvert",
                    "--in-type",
                    in_type,
                    "--out-type",
                    out_type,
                ).exit_code
                == 0
            )

        convert("json", "proto")

        with open(pjoin(tmp, "data"), "rb") as f:
            a = f.read()

        convert("proto", "yaml")
        convert("yaml", "jdot")
        convert("jdot", "json")
        convert("json", "proto")

        with open(pjoin(tmp, "data"), "rb") as f:
            b = f.read()

        assert a == b


def test_mconvert_complex():
    """Test -mconvert with a complex plan."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, "data"), "w") as f:
            f.write(COMPLEX_PLAN)

        def convert(in_type, out_type):
            assert (
                run(
                    pjoin(tmp, "data"),
                    "-O",
                    pjoin(tmp, "data"),
                    "-mconvert",
                    "--in-type",
                    in_type,
                    "--out-type",
                    out_type,
                ).exit_code
                == 0
            )

        convert("json", "proto")

        with open(pjoin(tmp, "data"), "rb") as f:
            a = f.read()

        convert("proto", "yaml")
        convert("yaml", "jdot")
        convert("jdot", "json")
        convert("json", "proto")

        with open(pjoin(tmp, "data"), "rb") as f:
            b = f.read()

        assert a == b


def test_valid_invalid():
    """Test exit code based on validity for various modes using diagnostic
    level overrides to force an outcome."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, "plan.json"), "w") as f:
            f.write(BASIC_PLAN)

        # Test all corner cases.
        def x(mode, level):
            return run(
                pjoin(tmp, "plan.json"),
                "-m",
                mode,
                "--diagnostic-level",
                "0",
                level,
                level,
            ).exit_code

        assert x("ignore", "error") == 0
        assert x("loose", "error") == 1
        assert x("loose", "warning") == 0
        assert x("strict", "warning") == 1
        assert x("strict", "info") == 0

        # Default should be -mloose.
        def x(level):
            return run(
                pjoin(tmp, "plan.json"), "--diagnostic-level", "0", level, level
            ).exit_code

        assert x("info") == 0
        assert x("warning") == 0
        assert x("error") == 1


def test_verbosity():
    """Test verbosity using diagnostic level overrides."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, "plan.json"), "w") as f:
            f.write(BASIC_PLAN)

        # Test all corner cases.
        def x(verbosity, level):
            return run(
                pjoin(tmp, "plan.json"),
                "-v",
                verbosity,
                "--diagnostic-level",
                "0",
                level,
                level,
            ).output.split(maxsplit=1)[:1]

        assert x("quiet", "error") == []
        assert x("fatal", "error") == ["Fatal"]
        assert x("error", "error") == ["Error"]
        assert x("error", "warn") == []
        assert x("warn", "warn") == ["Warning"]
        assert x("warn", "info") == []
        assert x("info", "info") == ["Info"]


def test_export():
    """Test export logic."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, "plan.json"), "w") as f:
            f.write(BASIC_PLAN)

        def x(output, level):
            return run(
                pjoin(tmp, "plan.json"),
                "-O",
                pjoin(tmp, output),
                "--diagnostic-level",
                "0",
                level,
                level,
            ).exit_code

        def y(output):
            assert x(output, "error") == 1
            assert not isfile(pjoin(tmp, output))
            assert x(output, "info") == 0
            with open(pjoin(tmp, output), "rb") as f:
                return f.read()

        assert y("output.proto")[0] == 10
        assert y("output.json").startswith(b'{\n  "root":')
        assert y("output.yaml").startswith(b"root:")
        assert y("output.jdot").startswith(b"@macros")
        assert b"<!DOCTYPE html>" in y("output.html")
        assert y("output.txt").startswith(b"Info")


def test_urn_resolution():
    """Test extension URN resolution logic."""
    with tempfile.TemporaryDirectory() as tmp:
        with open(pjoin(tmp, "plan.json"), "w") as f:
            f.write(
                json.dumps(
                    {
                        "extensionUrns": [
                            {
                                "extension_urn_anchor": 1,
                                "urn": "extension:io.substrait:extension_types",
                            }
                        ]
                    }
                )
            )

        def x(*args):
            return run(
                pjoin(tmp, "plan.json"),
                "-verror",  # verbosity error
                "--diagnostic-level",
                "2002",
                "error",
                "error",  # YAML resolution failure -> error
                "--diagnostic-level",
                "2001",
                "error",
                "error",  # YAML resolution disabled -> error
                "--diagnostic-level",
                "0",
                "info",
                "info",  # all other diagnostics -> info
                "--urn-depth",
                "-1",  # opt in to URN resolution, this will be the default
                *args
            ).exit_code

        # The standard extensions are bundled into the validator, so this URN
        # resolves out of the box.
        assert x() == 0

        # Remapping the URN to None disables resolution, so we expect a
        # failure.
        assert x("--override-urn", "extension:io.substrait:*", "-") == 1
