# How testing works.

## Intro

- Timelines are created with a separate JS based repo.
	- JS used as:
		- Enables faster iteration.
		- No compile step.
		- Stripe docs are JS heavy.
		- Stripe libs have good TS type hints.
		- JS/JSON DSL syntax makes it easy to create graphs of objects.
		- It can be used for other Stripe DL systems.
		- This represents an end users integration with Stripe.

- The Rust tests use the state the JS code writes to the file system instead of interacting directly with the Stripe
  API.
	- This enables:
		- Faster iteration of tests as there is no network latency.
		- Changing and re-running Rust tests whilst keeping the exact same "Stripe API interactions".
			- Some parts of the API are non-deterministic, like the ordering of events, payment failures or network
			  issues.
		- Inspecting and debugging state and graph of relations using non-changing ID's.
			- The ID's do not change between runs making following the graph connections easier.
	- Important: If any Rust download logic changes, the JS code will have to be run again (as it calls the Rust CLI and
	  stores the state in a SQLite file).
		- Assumption: For a given git commit, the SQLite files have been created with the same download code. This
		  ensures if the download logic changes, the result in the SQLite download can be tracked alongside it.

## Terms.

- `EventSeq`
	- Event Sequence.
	- AKA Timeline.
	- An event sequence is an ordered list representing API interactions with the Stripe API.
		- It is a list of Stripe events and SQLite DB files representing account downloads.
	- Any given point in the event list can be "tagged".
	- This is defined by the Timelines defined in the `stripe-simulator` repo.
	- At each tag point, a full Stripe account download is done with the TD CLI.
		- This snapshots the account at that point in time.
			- A tag creates a "pause" where Rust test assertions can be run.


- `TagSeq`
	- Tag Sequence.
	- An event sequence can have any number of tags.
	- A tag sequence is an ordered subset of those tags that make sense together (E.g. `c`, `u`, `d` defines a create
	  update and delete lifecycle, but the event sequence writer can create any tags they need).
	- This enables a single event sequence to have many tags, some of which are unrelated to each other, and are
	  ultimately used to allow the Rust tests to pause at any point and make assertions about database state.
	- Tags are used to avoid using numeric indexes (which change when new API interactions are added before or after
	  resulting in tests that are referencing constantly changing timeline positions).

- `path`, `path_part`
	- A `path` is theoretical end user advancement along a given tag sequence.
	- A `path_part` allows advancing along that timeline, and then pausing to inspect the DB state to ensure
	  assertions/constraints are met.
	- Regex-like syntax is used for brevity.
	- Important: `path` and `path_parts` allows testing the synchronicity of the two Stripe API's:
		- (1) Download.
			- These are direct object listings.
		- (2) Apply events.
			- JSON object read from the event stream.
		- The same state can be read from both API's, but both differ in subtle ways; the Rust tests ensure that they
		  result in the same database state and **ultimately the same query results for end users**.
		- Stripe deletes events after 30 days.
	- Example:
	- TagSeq = `c,u,d`, "Use only these tags, ignore the others"
	- `path` = `(c),u,d`, "This is the full execution, ensure the test proceeds along this path only"
	- You would then walk this `path` using the `path_parts`:
		- `exec.dl("(c)")`
			- "Simulate a user downloading all events in tag c"
			- Run assertions against DB file
		- `exec.apply("u")`
			- "Simulate a user starting with a DB with all data in set c, and applying events in set u to the DB"
			- Run assertions against DB file
		- `exec.apply("d")`
			- Run assertions against DB file


- `Exec`
	- Execution.
	- This keeps track of the above parts during the actual test execution, and makes general assertions at each stage
	  like:
		- Ensure SQL relations are correct.
			- Cannot be done with SQL foreign keys because the FK invariants of the API are too loose.
		- Ensure no events are skipped.


- `steps`
	- A timeline is broken up into parts with tags. Each part is a step.

- Walk
	- A nickname for common tag seq/path combinations.
		- Starting the download at different stages, applying different sets of events.
	- Focused on the lifecycle of objects.
	- Prevents re-writing the same ones in many tests, allows comparison between different objects tests.
	- Walks are:
		- 1, "(),c,u,d"
		- 2_a, "(c),u,d"
		- 2_b, "(c),u-u,d"
		- 3_a, "(u),d"
		- 3_b, "(u),d-d",
		- 4, "(d),d"
	- Explanation:
		- 1, Start with blank db, apply events c, then u, then d. (create, update, delete)
		- 2_a, Start with data in set c from the download in the database, apply events from the start of time and
		  including set u, and then set d.
		- 2_b, Same as above, but exclude the events in set c (u-u means a range of set u inclusive).
