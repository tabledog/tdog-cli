Alternative ways of doing "compile time" converting of Rust type data into SQL rows (create, insert, update etc).

## Things I dislike about the current approach:

- Updates may not work as by default the entire struct is placed into an insert.
- It takes effort to copy an API struct into a Row struct - this data is mostly copied.
	- Good:
		- Provides a point of transforming the data into SQL friendly values.
		- Structs are understood by the IDE and the intitial Rust passes so are usually checked fast/before macros.





## A1

- A macro to convert an API stuct into an enum, one variant per cell.

- It can do this recrusively.

```rust

macro_convert_to_cells!(APICustomer);

// Outputs:

enum RowCustomer {
	FieldA(Option<i64>),
}

// Issue: enum needed for value too?
let map:  HashMap<Key, &'static str> = hashMap::new();
map.insert(Key::First, "first");
map.insert(Key::Second, "second");

```

- This would allow any set of cells to make up a table, and would work well for foreign keys.
- Updates can be represented as a subset.
- Issue: Creates `create` at runtime.
- Issue: `Vec<RowCustomer>` could have duplicates.
- Issue: No `dot` syntax to read/write fields of a struct.
