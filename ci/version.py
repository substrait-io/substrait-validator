#!/usr/bin/python3
# SPDX-License-Identifier: Apache-2.0

import subprocess
import sys
import os


def set_version(current_version, new_version, dry_run=False):
    """Tries to update the version from current_version to new_version."""
    print(f"Modifying version from {current_version} to {new_version}...")
    if dry_run:
        print("  (dry run, won't actually make the change)")

    # Read the template.
    with open("version-diff-template", "r") as f:
        template = f.read()

    # Fill out the template.
    patch = template.format(frm=current_version, to=new_version).encode("utf-8")

    # Try to apply the patch.
    output = subprocess.run(["git", "apply", "--check"], cwd="..", input=patch)
    if output.returncode:
        print("Patch is invalid. Something is out of sync: check the version")
        print("file and try regenerating the patch template.")
        sys.exit(1)
    print("Generated patch seems valid.")

    # Stop here if this is a dry run.
    if dry_run:
        return

    # Apply the patch.
    subprocess.run(["git", "apply"], cwd="..", input=patch, check=True)

    # Update the current version file.
    with open("version", "w") as f:
        f.write(new_version)

    print("Version modified.")


def escape(s):
    """Escapes { and } sequences in a diff template file."""
    return s.replace("{", "{{").replace("}", "}}")


if __name__ == "__main__":

    # The rest of the script assumes that we're running from the script
    # directory, so make sure that's where we are.
    os.chdir(os.path.dirname(os.path.realpath(__file__)))

    # Read the current version.
    with open("version", "r") as f:
        current_version = f.read().strip()

    if len(sys.argv) == 2 and sys.argv[1] == "get":
        print(current_version)
        sys.exit(0)

    if len(sys.argv) == 3 and sys.argv[1] == "set":
        new_version = sys.argv[2]
        if new_version == current_version:
            print("Error: that's already the current version.")
            sys.exit(1)
        set_version(current_version, new_version)
        sys.exit(0)

    if len(sys.argv) == 2 and sys.argv[1] == "check":
        test_version = "3.1.4"
        if current_version == test_version:
            test_version = "2.7.1"
        set_version(current_version, test_version, dry_run=True)
        sys.exit(0)

    if len(sys.argv) == 2 and sys.argv[1] == "update":
        print("Change all the version numbers in the repository, EXCEPT the")
        print("current_version file in this directory, such that git diff")
        print("will give a valid diff (make sure the repo is clean before")
        print("doing this). The new version doesn't matter; it just serves")
        print("as a marker and will be reverted. When done, enter the version")
        print("you changed the current version into here.")
        print()
        dummy_version = input("New/dummy version: ").strip()
        print()
        if dummy_version == current_version:
            print("Current and new/dummy version must differ")
            sys.exit(1)

        # Ask git for the diff, expected to correspond to a version change from
        # current_version to dummy_version.
        output = subprocess.run(["git", "diff"], capture_output=True)
        if output.returncode:
            error = output.stderr.decode("utf-8")
            print(f"Failed to get diff:\n{error}")
            sys.exit(1)
        diff = output.stdout.decode("utf-8")

        # Parse the diff and convert it to a format-style template.
        lines = diff.strip().split("\n")
        template_lines = []
        num_changes = 0
        skip = False
        for index, line in enumerate(lines):
            if skip:
                skip = False
                continue
            if line.startswith("-") and not line.startswith("---"):
                if index == len(lines) - 1:
                    print("Error: - line at end of `git diff`")
                    sys.exit(1)
                frm = line[1:]
                if not lines[index + 1].startswith("+"):
                    print(
                        f"Error: expecting -+ line pairs on lines {index+1} "
                        f"and {index+2} of `git diff`"
                    )
                    sys.exit(1)
                to = lines[index + 1][1:]
                if (
                    frm.replace(current_version, dummy_version) != to
                    or to.replace(dummy_version, current_version) != frm
                ):
                    print(
                        f"Error: diff at lines {index+1} and {index+2} of git "
                        f"diff is not just a change from {current_version} to "
                        f"{dummy_version}"
                    )
                    sys.exit(1)
                template_lines.append(
                    "-" + escape(frm).replace(current_version, "{frm}")
                )
                template_lines.append("+" + escape(to).replace(dummy_version, "{to}"))
                skip = True
                num_changes += 1
            else:
                template_lines.append(escape(line))
        template = "\n".join(template_lines) + "\n"
        print(f"Found {num_changes} version instances.")

        # If no lines changed, the user probably didn't understand how this
        # works.
        if not num_changes:
            print("Error: `git diff` returned empty or invalid diff.")
            print(
                "Did you manually change the version from "
                f"{current_version} to {dummy_version}?"
            )
            sys.exit(1)

        # Write the template.
        with open("version-diff-template", "w") as f:
            f.write(template)
        print("Wrote updated template.")

        # Try to apply the reverse diff.
        set_version(dummy_version, current_version)
        sys.exit(0)

    # Arguments invalid, print usage.
    me = sys.argv[0] if sys.argv else "version.py"
    print("Usage:")
    print()
    print(f"{me} get")
    print("  Returns the current version.")
    print()
    print(f"{me} set <version>")
    print("  Updates the version to the given value.")
    print()
    print(f"{me} check")
    print("  Checks patch template consistency.")
    print()
    print(f"{me} update")
    print("  Interactively updates the patchfile template used to update")
    print("  the version")
    sys.exit(2)
