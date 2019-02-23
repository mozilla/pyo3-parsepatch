# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this file,
# You can obtain one at http://mozilla.org/MPL/2.0/.

import copy
import os
import rs_parsepatch as pp
import whatthepatch as wp


def rm_bin(data):
    return [x for x in data if not x['binary']]


def get_filename(name):
    if name.startswith('a/') or name.startswith('b/'):
        return name[2:]
    return name


def test_pp():
    directory = 'tests/patches'
    for f in os.listdir(directory):
        with open(os.path.join(directory, f), 'rb') as In:
            patch = In.read()

        counts = pp.get_counts(patch)
        diffs = pp.get_diffs(patch)
        lines = pp.get_lines(patch)

        # compare counts & diffs
        assert len(diffs) == len(counts)
        for count, ppd in zip(counts, diffs):
            diff = copy.deepcopy(ppd)
            del diff['lines']
            diff.update({'added': 0, 'deleted': 0})
            for n, o, _ in ppd['lines']:
                if n is None:
                    diff['added'] += 1
                elif o is None:
                    diff['deleted'] += 1
            assert diff == count

        # compare lines & diffs
        assert len(diffs) == len(lines)
        for line, ppd in zip(lines, diffs):
            diff = copy.deepcopy(ppd)
            del diff['lines']
            diff.update({'added': [], 'deleted': []})
            for n, o, _ in ppd['lines']:
                if n is None:
                    diff['added'].append(o)
                elif o is None:
                    diff['deleted'].append(n)
            assert diff == line

        # we do that because wtp output doesn't contain info on bin files
        diffs = rm_bin(diffs)

        wp_diffs = wp.parse_patch(patch.decode('utf-8'))
        wp_diffs = list(wp_diffs)

        assert len(wp_diffs) == len(diffs)

        # compare wtp and pp outputs
        for ppd, wpd in zip(diffs, wp_diffs):
            wnew_path = get_filename(wpd.header.new_path)
            wold_path = get_filename(wpd.header.old_path)

            assert ppd['filename'] == wnew_path
            if wnew_path != wold_path:
                assert ppd['renamed_from'] == wold_path
            else:
                assert ppd['renamed_from'] is None

            if not wpd.changes:
                assert not ppd['lines']
                continue

            changes = list(wpd.changes)
            assert len(ppd['lines']) == len(changes)
            for pline, wline in zip(ppd['lines'], changes):
                wn, wo, wc = wline
                pn, po, pc = pline

                assert wn == pn
                assert wo == po

                pc = pc.decode('utf-8')
                assert wc == pc
