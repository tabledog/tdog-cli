# Testing Stripe `apply_events`.

## Issues when testing the download/apply events logic

- It is not easily possible to:
    - Export and import data into a Stripe account.
        - Relations between types are based on object ids, which are re-created on re-insert.
    
    - Replicate the logic of a Stripe server in order to create a mock HTTP proxy.
    
- Stripe events are only stored for 28 days.
    - This makes it difficult to create a test account with the correct series of events as they will be auto deleted in a month.

- Downloads and events take a long time when testing against the Stripe server.
    - E.g. When testing download/events against a Stripe account of 100+ objects, the pagination requests are in series which can take 10 seconds+.
    - Network latency.
    
- The TD logic mapping JSON objects to SQL tables may change (meaning any db snapshots will be invalid).
    - SQL table schema could change.
    - Newer Stripe API version. 
    
    
## How the `apply_events` logic will be tested.

1. Insert data into a Stripe test account.
   
2. Download, store as a db snapshot file.

3. Mutate the data to create a series of events that test a given archetype.

4. Download those events, store in a JSON file.

5. Test against local data (db, events list) = (2, 4).


This:
- Allows re-creating the download snapshot by running the download against the Stripe account (assuming it is not changed).
- Saves the events locally so they are kept after Stripe deletes them.
- Uses local state for the tests.
    - This allows fast iterations.
    
### Possible issues with this approach.

- Stripe delete the test accounts after a year, which cannot be re-created from the DB files easily.
    - Fix: represent the entire interaction with code only (no web UI usage).

### Why write tests?

- It allows changing the processing logic and testing it immediately, rather than having to go through a functional network/Stripe web UI test every time.
- It allows collecting different "event series archetypes types"
    - It's likely that there are exceptions that are only discoverable with real world use.
    
- It provides benefits over in house solutions.
    - In house developers likely do not have enough time to ensure their code handles all cases flawlessly.
        - This would result in getting feedback from production systems, fixing bugs, and re-releasing.
        - In contrast TD is solid from the start.
        
        
- It ensures that "this set of events (equal to a Stripe server side state machine execution)" results in this set of SQL writes.