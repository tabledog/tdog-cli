const fs = require('fs');
const path = require('path');


/**
 * This file processes Rust source code to try and infer SQL relations between Rust structs based on the names, E.g (owner Rust struct name, copy Rust struct field name) combos.
 * - Intended to be used via CLI to gather meta data for writing the relation data into a Rust file, `relations.rs`.
 *      - Much faster than grepping each type manually via IDE.
 *      - The order of adding relations is alphabetical by owner - much easier to encode relations in Rust with a reference list to follow.
 *      - Any changes will show in the JSON diff.
 *
 * - Assumption: Stripes convention of naming FK's with the same name as the PK owner holds for 100% of relations.
 *
 *
 * Usage: `cd this-dir; node list-relations.js > relations.json`
 */

// @see https://stackoverflow.com/a/49601340/4949386
function readFilesSync(dir) {
    const files = [];

    fs.readdirSync(dir).forEach(filename => {
        const name = path.parse(filename).name;
        const ext = path.parse(filename).ext;
        const abs_file_path = path.resolve(dir, filename);
        const stat = fs.statSync(abs_file_path);
        const isFile = stat.isFile();

        const as_string = fs.readFileSync(abs_file_path, 'utf8');

        if (isFile) {
            files.push({
                abs_file_path,
                name,
                ext,
                as_string
            });
        }
    });

    files.sort((a, b) => {
        // natural sort alphanumeric strings
        // https://stackoverflow.com/a/38641281
        return a.name.localeCompare(b.name, undefined, {numeric: true, sensitivity: 'base'});
    });

    return files;
}

// Example matches:
//      - `request_id: i.request.as_ref().and_then(|x| x.id.clone()),`
//      - `customer,`
//          - E.g. `customer` is a variable binding with the same name as a struct field.
const field_re = /^\s+[a-z0-9_]+(:.+?|,)$/gm;

const get_relations = () => {
    const files = readFilesSync('./types');


    // Extract lines where a Rust struct field is written to.
    for (const x of files) {
        x.rust_fields = [...x.as_string.matchAll(field_re)].map(x => x[0].trim());
    }


    const o = [];
    const all_file_names = files.map(x => x.name).sort();
    for (const owner of all_file_names) {
        if (owner === "mod") {
            continue;
        }
        const copies = [];

        for (const x2 of files) {
            if (x2.name === owner) {
                continue;
            }

            const matches = [];
            for (const f of x2.rust_fields) {
                if (f.includes(owner)) {
                    matches.push(f);
                }
            }

            if (matches.length > 0) {
                copies.push({
                    file_name: x2.name,
                    matches
                });
            }
        }

        o.push({
            owner_file_name: owner,
            copies
        });
    }

    return {
        relations: o
    }
};


console.log(JSON.stringify(get_relations(), null, 4));
