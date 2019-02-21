# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this file,
# You can obtain one at http://mozilla.org/MPL/2.0/.

import json
import os
import rs_parsepatch as pp


def write(f, res):
    f = os.path.splitext(f)[0]
    with open(os.path.join('tests/output', f + '.json'), 'w') as Out:
        json.dump(res, Out, indent=4, separators=(',', ': '))


def read(f):
    f = os.path.splitext(f)[0]
    with open(os.path.join('tests/output', f + '.json'), 'r') as In:
        return json.load(In)


def test_pp():
    directory = 'tests/patches'
    for f in os.listdir(directory):
        with open(os.path.join(directory, f), 'rb') as In:
            patch = In.read()

        counts = pp.get_counts(patch)
        diffs = pp.get_diffs(patch)

        keys = {'new', 'deleted', 'renamed_from', 'filename', 'binary'}

        assert len(counts) == len(diffs)

        res = []
        for x, y in zip(counts, diffs):
            for key in keys:
                assert x[key] == y[key]
            lines = []
            for line in y['lines']:
                line = list(line)
                line[2] = line[2].decode('utf-8')
                lines.append(line)
            x['lines'] = lines
            res.append(x)

        assert res == read(f)
