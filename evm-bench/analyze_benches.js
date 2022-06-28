#!/usr/bin/env node

const assert = require('assert');
const fs = require('fs');
const path = require('path');


const stdinBuffer = fs.readFileSync(0, 'utf-8'); // STDIN_FILENO = 0
assert(stdinBuffer);

const bench_path = stdinBuffer.toString().split('\n').filter(x => !!x).slice(-1)[0];
assert(bench_path);

const bench_data = fs.readFileSync(bench_path, 'utf-8');
const bench_config = fs.readFileSync(__dirname + '/build/benches.json', 'utf-8');

const benches = JSON.parse(bench_data);
const config = JSON.parse(bench_config);

const db_read = 25_000_000;
const db_write = 100_000_000;

const output = benches
    .map(({ name, reads, writes, weight }) => {
        const used_gas = config[name]['used_gas'];
        const total_weight = weight + reads * db_read + writes * db_write;
        const ratio = parseInt((total_weight / used_gas).toString());
        return {
            name,
            reads,
            writes,
            weight,
            total_weight,
            used_gas,
            ratio
        };
    })
    .sort((a, b) => b.ratio - a.ratio);

assert(output.length > 0);

console.table(output);

let ratio = output[0].ratio;
// round up ratio
ratio = Math.ceil(ratio / 1_000) * 1_000;

console.log('Ratio', ratio);

const file = `// This file is part of Selendra.

// Copyright (C) 2020-2021 Selendra.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

pub const RATIO: u64 = ${ratio};
`;

const output_path = process.argv.slice(2)[0];
if (output_path) {
    const file_path = path.isAbsolute(output_path) ? output_path : path.resolve(process.cwd(), output_path);
    fs.writeFileSync(file_path, file);
}
