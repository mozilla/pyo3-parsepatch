# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this file,
# You can obtain one at http://mozilla.org/MPL/2.0/.

import copy
import os
import rs_parsepatch as pp
import whatthepatch as wp


def get_filename(name):
    if name.startswith("a/") or name.startswith("b/"):
        return name[2:]
    return name


def get_patch():
    directory = "tests/patches"
    for f in os.listdir(directory):
        path = os.path.join(directory, f)
        print("Test patch: {}".format(path))
        with open(path, "rb") as In:
            patch = In.read()
            yield patch


def read(patch):
    counts = pp.get_counts(patch)
    diffs = pp.get_diffs(patch)
    lines = pp.get_lines(patch)

    # compare counts & diffs
    assert len(diffs) == len(counts)
    for count, ppd in zip(counts, diffs):
        diff = copy.deepcopy(ppd)
        del diff["lines"]
        diff.update({"added_lines": 0, "deleted_lines": 0})
        for n, o, _ in ppd["lines"]:
            if n is None:
                diff["added_lines"] += 1
            elif o is None:
                diff["deleted_lines"] += 1
        assert diff == count

    # compare lines & diffs
    assert len(diffs) == len(lines)
    for line, ppd in zip(lines, diffs):
        diff = copy.deepcopy(ppd)
        del diff["lines"]
        diff.update({"added_lines": [], "deleted_lines": []})
        for n, o, _ in ppd["lines"]:
            if n is None:
                diff["added_lines"].append(o)
            elif o is None:
                diff["deleted_lines"].append(n)
        assert diff == line

    if not isinstance(patch, str):
        patch = patch.decode("utf-8")

    wp_diffs = wp.parse_patch(patch)
    wp_diffs = list(wp_diffs)

    assert len(wp_diffs) == len(diffs)

    # compare wtp and pp outputs
    for ppd, wpd in zip(diffs, wp_diffs):
        wnew_path = get_filename(wpd.header.new_path)
        wold_path = get_filename(wpd.header.old_path)

        assert ppd["filename"] == wnew_path
        if wnew_path != wold_path:
            assert ppd["renamed_from"] == wold_path or ppd["copied_from"] == wold_path
        else:
            assert ppd["renamed_from"] is None

        if not wpd.changes:
            assert not ppd["lines"]
            continue

        changes = list(wpd.changes)
        assert len(ppd["lines"]) == len(changes)
        for pline, wline in zip(ppd["lines"], changes):
            wn, wo, wc, whc = wline
            pn, po, pc = pline

            assert wn == pn
            assert wo == po

            pc = pc.decode("utf-8")
            assert wc == pc


def test_pp():
    for patch in get_patch():
        read(patch)

    for patch in get_patch():
        patch = patch.decode("utf-8")
        read(patch)

    for patch in get_patch():
        patch = bytearray(patch)
        read(patch)
