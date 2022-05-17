
# General heuristics for mapping API data to row data.

- @todo/maybe Encode these rules into an enum variant, automate the code.
- @todo/maybe Encode these into traits so they are obvious.


A. If an object has an API URL that it can be listed at, use that instead of using Stripe's `expand`.

    - Exceptions:
        - If the object is only ever a child of the parent object being requested, expand instead.
        - If the listing URL requires an ID in the URL (must request many lists per ID instead of one global list).

    - Assumptions:
        - Any `expand` items that return lists contain 100% of items - they do not require extra HTTP requests.
        
B. Flatten simple JSON objects.
    
    - This is how Sigma works, although some fields are missing.
    - E.g. `{x: {y: "scalar"}}` becomes x_y = "scalar".
    
    - Exceptions:
        - When the JSON object has an array at any level, or is deeply nested.
   

- Every query must be right by default.
    - Or: It should be extremely unlikely that a SQL report gives users invalid data (that are inputs to decisions/tax returns etc).
        - Possible causes:
            - 1. User error (from non-obvious SQL state).
            - 2. TD logic error (not syncing 100% of a sets items).
    - Given a SQL user with moderate skill, what ever their first instinct to query the data is must be correct.
    - E.g. parents limiting lists to 10 items must be explicit (no docs for "these child rows are limited to 10").
        - Opt in with a CLI flag for non-obvious behavior.
        - E.g. sub->sub items, inv->inv items.
            - Event stream is lossy; needs direct download to get extra data.

    - Principle: Download/event symmetry.
        - State that is visible from the download should continue to be written to with apply events.
            - E.g. Checkout sessions have no create event, so when they are created in payment_status=unpaid, this state is only visible from the download db.
                - This can be misleading to a user who has a (download, apply_events, ...), *because they do not know they only have <100% of the rows for payment_status=unpaid*.
    
    - Principle: Opt in to incorrect queries.
        - The user can opt out of download/event symmetry by setting an option.
            - E.g. in the case of being able to get the data they need from the download only, and have no need for the apply events. 
    

- Why relations are tested in Rust based tests instead of relying on the SQL engines FK constraints.
    - Because Stripes event stream does not have read tx's, events can be applied in any batch which could result in a FK constraint being violated.
        - In contrast, the Rust tests are testing those relations exist for known points in time/event batches. 

   
# Maybe

- JSON string vs array of FKs.
    - Is it better to have:
        - `["a", "b", "c"]`, where the strings are FKs, OR:
        - `[{}, {}, {}]`, where the values are copies of objects that also exist in their own table?
        
        
# Testing methods
- Use SQLite/SQL as a "dynamic language" over the SQLite db file to ensure many nested relations during testing.
    - Instead of trying to pull in the SQLite database into a giant Rust hashmap with limited ability to test relations (simulate end user joins).
        - It is much more time consuming to write the Rust code than it is the SQL code.
        - The SQL code is also closer to what the end user will be using.     
    
    


