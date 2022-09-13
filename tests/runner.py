#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

import os
import pathlib
import subprocess
import shutil
import sys
import click
import yaml
import json
import re


def destringify_ident(s):
    """Converts potentially stringified identifiers to strings."""
    if s.startswith('"') and s.endswith('"'):
        return s[1:-1].replace('\\"', '"').replace("\\\\", "\\")
    return s


def path_element_field(f):
    """Returns the serialization of a field path element."""
    return {"Field": {"field": f}}


def path_element_repeated(f, i):
    """Returns the serialization of a repeated field path element."""
    return {"Repeated": {"field": f, "index": i}}


def path_element_oneof(f, v):
    """Returns the serialization of a oneof field path element."""
    return {"Oneof": {"field": f, "variant": v}}


def path_element_index(i):
    """Returns the serialization of an array index path element."""
    return {"Index": {"index": i}}


def convert_if_int(x):
    """Convert string x to an integer of possible, otherwise keep it as or
    convert it to a string."""
    try:
        return int(x)
    except ValueError:
        return str(x)


def parse_path_element(s):
    """Parses the Rust path element syntax to its serialized form."""
    ident_re = r'([a-zA-Z_][a-zA-Z0-9_]*|"(?:[^\\]|\\[\\"])*")'
    index_re = r"\[([1-9][0-9]*|0)\]"
    field_mat = re.fullmatch(ident_re, s)
    if field_mat:
        return path_element_field(destringify_ident(field_mat.group(1)))
    oneof_mat = re.fullmatch(ident_re + "<" + ident_re + ">", s)
    if oneof_mat:
        return path_element_oneof(
            destringify_ident(oneof_mat.group(1)), destringify_ident(oneof_mat.group(2))
        )
    repeated_mat = re.fullmatch(ident_re + index_re, s)
    if repeated_mat:
        return path_element_repeated(
            destringify_ident(repeated_mat.group(1)), int(repeated_mat.group(2))
        )
    index_mat = re.fullmatch(index_re, s)
    if index_mat:
        return path_element_index(int(index_mat.group(1)))
    raise ValueError(f"failed to parse {s} as path element")


def parse_diag_overrides(diags):
    """Parses and checks the input syntax for diagnostic overrides into the
    Rust/serde syntax."""
    diag_overrides = []
    if diags is not None:
        if not isinstance(diags, list):
            raise Exception("diags key must map to a list")
        for diag in diags:
            diag_data = {}

            code = diag.pop("code", None)
            if not isinstance(code, int):
                raise Exception("diags[].code must be an integer")
            diag_data["code"] = code

            level = diag.pop("min", "i")
            if level not in ("i", "w", "e"):
                raise Exception('diags[].min must be either "i", "w", or "e"')
            diag_data["min"] = level

            level = diag.pop("max", "e")
            if level not in ("i", "w", "e"):
                raise Exception('diags[].max must be either "i", "w", or "e"')
            diag_data["max"] = level

            if diag:
                raise Exception(
                    "Found unknown key(s) in diag[]: {}".format(", ".join(diag.keys()))
                )
            diag_overrides.append(diag_data)
    return diag_overrides


def strip_test_tags(data, path=(), yaml_counter=None):
    """
    Modifies data recursively, yielding a flattened set of instruction triple:

     - Pops all "[sub_path]__test" keys from the given data. For each popped
       value, yields a ('test', path + sub_path, test_data) triple. sub_path
       may be left blank, or may be a .-separated list of key names and list
       indices.
     - Replaces all "<name>__yaml" keys with "<name>", replacing their value
       with "test:<index>.yaml", where index is a unique integer index within
       the plan. For each replaced value, the original yaml data is recursively
       stripped using 'data' for the path element (this is how it will appear
       in the validator output tree) and then yielded in the form of a
       ('yaml', index, data) triple.
    """
    if yaml_counter is None:
        yaml_counter = [0]
    if isinstance(data, dict):
        # Handle __test keys.
        keys = []
        for key in data.keys():
            if not isinstance(key, str):
                raise Exception(
                    "found non-string key at {}".format(".".join(map(str, path)))
                )
            if key.endswith("__test"):
                keys.append(key)
        for key in keys:
            test_data = data.pop(key)
            sub_path = tuple(map(convert_if_int, key.rsplit("__")[0].split(".")))
            if sub_path == ("",):
                sub_path = ()
            yield ("test", path + sub_path, test_data)

        # Handle __yaml keys.
        keys = []
        for key in data.keys():
            if key.endswith("__yaml"):
                keys.append(key)
        for key in keys:
            index = yaml_counter[0]
            yaml_counter[0] += 1
            yaml_data = data.pop(key)
            new_key = key.rsplit("__")[0]
            data[new_key] = f"test:{index}.yaml"
            for x in strip_test_tags(yaml_data, path + (new_key, "data"), yaml_counter):
                yield x
            yield ("yaml", index, yaml_data)

        # Traverse into dict.
        for key, value in data.items():
            for x in strip_test_tags(value, path + (key,), yaml_counter):
                yield x
    elif isinstance(data, list):
        # Traverse into list.
        for index, value in enumerate(data):
            for x in strip_test_tags(value, path + (index,), yaml_counter):
                yield x


def resolve_path(path, msg_desc):
    """Converts a JSON path to the protobuf path elements that Rust derives
    from the prost-generated structures."""
    while path:
        el, *path = path
        if isinstance(el, int):
            if msg_desc is None:
                yield path_element_index(el)
            else:
                raise Exception(
                    f"unexpected integer in path description, currently at {msg_desc.full_name}"
                )
        elif msg_desc is None:
            yield path_element_field(el)
        else:
            field_desc = msg_desc.fields_by_camelcase_name.get(el, None)
            if field_desc is None:
                field_desc = msg_desc.fields_by_name.get(el, None)
            if field_desc is None:
                raise Exception(f"unknown field {el} for {msg_desc.full_name}")
            if field_desc.label == field_desc.LABEL_REPEATED:
                if not path:
                    raise Exception(
                        f"ran out of path elements for repeated {msg_desc.full_name}"
                    )
                el2, *path = path
                if not isinstance(el2, int):
                    raise Exception(
                        f"found non-index path element for repeated {msg_desc.full_name}"
                    )
                yield path_element_repeated(field_desc.name, el2)
            else:
                if field_desc.containing_oneof is not None:
                    yield path_element_oneof(
                        field_desc.containing_oneof.name, field_desc.name
                    )
                else:
                    yield path_element_field(field_desc.name)
            msg_desc = field_desc.message_type


def parse_level_instruction(allowed_levels, path):
    """Parses an error level instruction in the input format into the
    Rust/serde instruction syntax."""
    if allowed_levels is None:
        return []

    if not isinstance(allowed_levels, (list, str)):
        raise Exception("__test.level must be a list or string")
    allowed_levels = list(allowed_levels)
    for level in allowed_levels:
        if level not in ("i", "w", "e"):
            raise Exception('__test.level[] must be either "i", "w", or "e"')
    return [dict(Level=dict(path=path, allowed_levels=allowed_levels))]


def parse_diag_instruction(diag_data, path):
    """Parses a diagnostic matching instruction in the input format into the
    Rust/serde instruction syntax."""
    if diag_data is None:
        return []

    rust_diag_data = {}
    if not isinstance(diag_data, dict):
        raise Exception("__test.diag must be a dict")

    code = diag_data.pop("code", None)
    if code is not None:
        if not isinstance(code, int):
            raise Exception("__test.diag.code must be an int")
        rust_diag_data["code"] = code

    level = diag_data.pop("level", None)
    if level is not None:
        if level not in ("i", "w", "e"):
            raise Exception('__test.diag.level must be either "i", "w", or "e"')
        rust_diag_data["level"] = level

    level = diag_data.pop("original_level", None)
    if level is not None:
        if level not in ("i", "w", "e"):
            raise Exception(
                '__test.diag.original_level must be either "i", "w", or "e"'
            )
        rust_diag_data["original_level"] = level

    msg_pattern = diag_data.pop("msg", None)
    if msg_pattern is not None:
        if not isinstance(msg_pattern, str):
            raise Exception("__test.diag.msg must be a string")
        # Convert to full glob pattern... We don't use the full
        # pattern syntax in the description because escape
        # sequences are needed for some rather common characters
        # in messages (i.e. '[', ']', and '?').
        i = 0
        glob_pattern = ""
        while i < len(msg_pattern):
            if msg_pattern[i : i + 2] == "**":
                glob_pattern += "[*]"
                i += 1
                break
            c = msg_pattern[i]
            if c in ("?", "[", "]"):
                glob_pattern += f"[{c}]"
            else:
                glob_pattern += c
            i += 1
        rust_diag_data["msg"] = glob_pattern

    element = diag_data.pop("before", None)
    if element is not None:
        if not isinstance(element, str):
            raise Exception("__test.diag.before must be a path element string")
        rust_diag_data["before"] = parse_path_element(element)

    element = diag_data.pop("after", None)
    if element is not None:
        if not isinstance(element, str):
            raise Exception("__test.diag.after must be a path element string")
        rust_diag_data["after"] = parse_path_element(element)

    if diag_data:
        raise Exception(
            "Found unknown __test.diag key(s): {}".format(", ".join(diag_data.keys()))
        )
    return [dict(Diag=dict(path=path, **rust_diag_data))]


def parse_type_instruction(type_str, path):
    """Parses a data type check instruction in the input format into the
    Rust/serde instruction syntax."""
    if type_str is None:
        return []

    if not isinstance(type_str, str):
        raise Exception("__test.type must be a string")
    return [dict(DataType=dict(path=path, data_type=type_str))]


def parse_comment_instruction(comment_test, path):
    """Parses a comment check instruction in the input format into the
    Rust/serde instruction syntax."""
    if comment_test is None:
        return []

    if not isinstance(comment_test, str):
        raise Exception("__test.comment must be a string")
    return [dict(Comment=dict(path=path, msg=comment_test))]


def parse_instructions(test_tags, fname, proto_desc):
    """Parses and checks the syntax for instructions in the input format into
    the Rust/serde instruction syntax."""
    instructions = []
    for insn, loc, data in test_tags:
        if insn == "test":
            path = list(resolve_path(loc, proto_desc))
            for insn_type in data:

                # Handle level instructions.
                instructions.extend(
                    parse_level_instruction(insn_type.pop("level", None), path)
                )

                # Handle diag instructions.
                instructions.extend(
                    parse_diag_instruction(insn_type.pop("diag", None), path)
                )

                # Handle type instructions.
                instructions.extend(
                    parse_type_instruction(insn_type.pop("type", None), path)
                )

                # Handle comment instructions.
                instructions.extend(
                    parse_comment_instruction(insn_type.pop("comment", None), path)
                )

                if insn_type:
                    raise Exception(
                        "Found unknown __test key(s): {}".format(
                            ", ".join(insn_type.keys())
                        )
                    )

        if insn == "yaml":
            with open(f"{fname}.{loc}.yaml", "w") as f:
                f.write(yaml.safe_dump(data))

    return instructions


def compile_test(fname, data, proto_parse, proto_desc):
    """Compile test data into a bunch of test files, of which fname itself is
    the main test file and the remainder are of the form fname.<tag>.yaml,
    containing supplementary information. proto_parse should be a function
    that parses a Python dict representation of the JSON corresponding to a
    Substrait plan into its binary representation, and proto_desc must point
    to the descriptor for substrait.Plan.

    See README.md for format information."""

    # Get name.
    name = data.pop("name", None)
    if not isinstance(name, str):
        raise Exception("Missing valid test name")

    # Parse diagnostic overrides.
    diag_overrides = parse_diag_overrides(data.pop("diags", None))

    # Get plan data.
    plan = data.pop("plan", None)
    if not isinstance(plan, dict):
        raise Exception("Missing Substrait plan")

    if data:
        raise Exception(
            "Found unknown key(s) in root: {}".format(", ".join(data.keys()))
        )

    # Strip test tags from the test data.
    test_tags = list(strip_test_tags(plan))

    # strip_test_tags does post-order tree traversal, but we need the
    # instructions ordered pre-order. Easiest way to do that is to just reverse
    # the list.
    test_tags.reverse()

    # Write the converted plan for debugging purposes.
    with open(f"{fname}.plan.yaml", "w") as f:
        f.write(yaml.safe_dump(plan))

    # Parse and serialize the stripped plan using protobuf.
    plan = proto_parse(plan)

    # Parse the instructions derived from the test tags now that we know the
    # protobuf structure was found to be valid by protobuf (it generates far
    # better error messages than the path resolver does, in case something is
    # wrong in the test description).
    instructions = parse_instructions(test_tags, fname, proto_desc)

    # Write output file.
    with open(fname, "w") as f:
        f.write(
            json.dumps(
                dict(
                    name=name,
                    plan=list(plan),
                    diag_overrides=diag_overrides,
                    instructions=instructions,
                )
            )
        )


def mtime(path) -> float:
    """Yields the mtime of the given path, or 0 if it doesn't exist."""
    try:
        if os.path.isfile(path):
            return os.path.getmtime(path)
    except OSError:
        pass
    return 0.0


@click.group()
def cli():
    pass


@cli.command(
    short_help="Runs the test suite",
    help=(
        "Runs the test suite, or only run tests matching the "
        "given glob-capable filter (matching test case names)."
    ),
)
@click.argument("filter", required=False, default="*")
@click.option(
    "--release/--no-release",
    default=False,
    help=(
        "Build Rust application in --release mode. Recompilation "
        "will take longer, but rerunning the tests will be much "
        "faster."
    ),
)
@click.option(
    "--html/--no-html",
    default=True,
    help=(
        "Enables or disables exporting HTML for the plans under "
        "test. Enabled by default."
    ),
)
def run(filter, release, html):

    # Build and run with optimizations if --release is passed.
    if release:
        release = ["--release"]
    else:
        release = []

    # Run cargo build without capturing output.
    code = subprocess.run(["cargo", "build"] + release).returncode
    if code:
        sys.exit(code)

    # Find all proto files and check if they've changed since the last run.
    click.echo("Scanning for proto files...")
    script_path = os.path.dirname(os.path.realpath(__file__))
    repo_path = os.path.realpath(os.path.join(script_path, ".."))
    proto_paths = [
        os.path.join(repo_path, "proto"),
        os.path.join(repo_path, "substrait", "proto"),
    ]
    proto_files = []
    proto_path_args = []
    for proto_path in proto_paths:
        proto_files.extend(
            pathlib.Path(os.path.join(proto_path, "substrait")).rglob("*.proto")
        )
        proto_path_args.extend(("-I", proto_path))
    proto_mtime = max(map(mtime, proto_files))
    output_path = os.path.join(script_path, "substrait")
    stamp_path = os.path.join(output_path, "__init__.py")
    stamp_mtime = mtime(stamp_path)
    if proto_mtime < stamp_mtime:
        click.echo("Protobuf bindings are up-to-date.")
    else:

        # Find the path to a protoc executable. We rely on prost for this, which is
        # capable of shipping it for most operating systems.
        click.echo("Finding protoc location...")
        protoc = subprocess.run(
            ["cargo", "run"] + release + ["-q", "--bin", "find_protoc"],
            capture_output=True,
        ).stdout.strip()

        # (Re)generate and import protobuf files and import them.
        click.echo("Generating protobuf bindings...")
        if os.path.isdir(output_path):
            shutil.rmtree(output_path)
        subprocess.check_call(
            [protoc, *proto_path_args, "--python_out", script_path, *proto_files]
        )
        for subdir in (".", "extensions", "validator"):
            fname = os.path.join(output_path, subdir, "__init__.py")
            with open(fname, "w") as f:
                f.write("\n")

    # Import the generated protobuf bindings.
    from substrait import plan_pb2

    assert os.path.samefile(plan_pb2.__file__, os.path.join(output_path, "plan_pb2.py"))
    from google.protobuf.json_format import ParseDict

    proto_desc = plan_pb2.Plan.DESCRIPTOR

    def proto_parse(data):
        return ParseDict(data, plan_pb2.Plan()).SerializeToString()

    # Rather than failing immediately when the first error occurs, store errors
    # here. The output for test files that compile without errors will then
    # still be written.
    errors = {}

    # Deserialize test input files (multiple input formats can be added here).
    click.echo("Scanning for test description files...")
    suite_path = os.path.join(script_path, "tests")
    test_inputs = {}
    for fname in pathlib.Path(suite_path).rglob("*.yaml"):
        if ".test." in fname.name:
            continue
        try:
            output_fname = str(fname) + ".test"
            if mtime(fname) >= mtime(output_fname):
                with open(fname, "r") as f:
                    test_inputs[fname] = (yaml.safe_load(f.read()), output_fname)
        except Exception as e:
            errors[fname] = ("reading", e)

    # Compile the contents of the test input files.
    if not test_inputs:
        click.echo("All test descriptions are up-to-date.")
    else:
        click.echo(f"Parsing {len(test_inputs)} test description(s)...")
        for fname, (test_input, output_fname) in test_inputs.items():
            try:
                compile_test(output_fname, test_input, proto_parse, proto_desc)
            except Exception as e:
                if os.path.isfile(output_fname):
                    os.remove(output_fname)
                errors[fname] = ("compiling", e)

    # Fail if there were any errors.
    if errors:
        for fname, (action, error) in errors.items():
            rel_path = os.path.relpath(fname, suite_path)
            click.echo(f"{type(error).__name__} while {action} {rel_path}: {error}")
        sys.exit(1)

    # Now run the test suite.
    sys.exit(
        subprocess.run(
            ["cargo", "run"] + release + ["-q", suite_path, str(int(html)), filter]
        ).returncode
    )


@cli.command(
    short_help="Removes all generated files", help="Removes all generated files."
)
def clean():
    script_path = os.path.dirname(os.path.realpath(__file__))

    # Remove generated protobuf files.
    proto_output_path = os.path.join(script_path, "substrait")
    if os.path.isdir(proto_output_path):
        shutil.rmtree(proto_output_path)

    # Remove compiled test files and test results.
    suite_path = os.path.join(script_path, "tests")
    for fname in pathlib.Path(suite_path).rglob("*.test*"):
        os.remove(fname)


if __name__ == "__main__":
    cli()
