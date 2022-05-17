# Intro

This file is used to explain the design behind the `relations.rs` as longer form text is easier to read/write in a .md file.


# What is a "foreign key"?

The SQL standard uses the terms "parent" and "child" to label the two vertices and one edge.

This is misleading as parent/child are terms used in trees, but two rows can form a circular reference, both being each others parent:

- E.g.
    - `discount(discount_id, customer_id)`
    - `customer(customer_id, discount_id)`


I think better terms would be "owner" and "copy"; "owner" is the owner of the ID, and copy is a copy of it in another row.

The aim of FK constraints are to ensure that every copy of an ID connects back to its owner.

This in turn ensures the consistency of the database as a single unit, and allows queries to be correct without the query writer having to be aware of possible inconsistencies.


# TD goals.

There are two high level goals related to FK constraints:

1. Correct queries at all times.
    - Queries against TD tables should be correct *and not require the query writer to have a list of exceptions in their head*.
        - AKA: The intuitive/common/standard SQL query pattern should work without having to know details about Stripes underlying API. 

2. SQL tooling that can parse relations.
    - Code generation (JOOQ, etc).
        - Converts a SQL schema into ORM code.
    - Database GUI (jump to relatives, code completion).



Goal 1 is essential, goal 2 may be targeted later by providing a SQL schema with the FK relations just for code gen (not to be used at runtime).



# Why SQL-based FK constraints cannot be used.

## There are no "read transactions" with Stripes network API. 

There are two boolean attributes:

- `read-tx`
    - When reading from the SAAS API, is the data being read:
        - Atomic.
            - Downloading groups of objects are all consistent/have valid FK constraints.
        - Isolated.
            - Protected from seeing the effects of writes.
    - Stripe FK constraints are "eventually consistent" because there is no read tx.
        - They may be 1% inconsistent in a single tx (dl/events apply), but the next tx will in most cases add the correct objects to make the graph complete.
    
- `ordered`
    - When reading from the SAAS API, is the order of the downloads deterministic?
        - E.g. if you re-run the process 10 times, the order of SQL writes will be exactly the same?


Truth table, possible combinations:

- read-tx=1
- ordered=1
    - When: test Stripe data, events

- read-tx=1
- ordered=0
    - When: test Stripe data, download    


- read-tx=0
- ordered=0
    - When: prod Stripe account, download


- read-tx=0
- ordered=1
    - When: prod Stripe account, events.


Notes:
- During testing a read tx can be emulated by:
    - Not allowing writes to the Stripe account.
    - Applying events in known atomic units (child and parent events are always applied together).


- `ordered` is always 0 for download.
    - During downloads, the listing of objects is done concurrently, so it is impossible to know when one list finishes downloading and another begins.
        - Circular references also mean that two objects depending on one another may be incorrect.
            - `read-tx`: The lists can change whilst they are being downloaded.


- For events, `ordered` is always 1, but `read-tx=0` means that the event list may stop at any point.
    - You never know if the list of objects you just downloaded have valid FK constraints.
    - E.g. given an event list of.
        - A.
        - `child.created`
        - B.
        - `parent.created`
        - C.
    - The `stripe list events` network call may stop at point B, which means a FK constraint on table `child` would fail.
    

## Some constraints are not enforced by Stripe.

E.g. a customer can be deleted, but its child rows can still exist *with the ID set to the deleted customer*.
- Subscriptions.
- Invoices.


## JSON based FK lists also need to be enforced.

- JSON arrays of FK's like `[a, b, c]` are used, which cannot be enforced by FK constraints.
    - These are useful because:
        - They reduce tables that only contain relations.
        - They make it easy to operate on a parents child objects without having to join (e.g. a Python script reading the ID's to write to them via the Stripe API).
        - They make it clear that those FK ids are written in sync with the rest of the row; there is no async gap between them.
            - This acts as kind of a "double check".
                - The parent contains the IDs in a JSON array; the child rows should all exist AND point back to their parent with a FK.

## Each SQL engine implements FK's slightly differently. 

Checking the relations in Rust means that the logic can be customized and be the same across different engines.

E.g: there should be no issues of FK constraints differing over 5 different SQL engines.





# Fix, A

## Steps

- Use Rust code to check relations with queries (and not use native SQL FK constraints at all).

- Use the fact that a read tx can be emulated during testing to check the FK relations, including JSON arrays.

- Enable checking the relations at the end of each TX.
    - Enabled for testing, possibly enabled for debugging in the future against prod Stripe accounts.
    - Could add a CLI for checking these relations for customer debugging.

- On first download against a prod account, immediately run an "apply events" inside the same tx.
    - This allows transitioning from ordered=0 (download) to ordered=1 (events).
        - Removes some of the possible FK inconsistencies from Stripe data being written to whilst also being downloaded. 
          

## What this achieves.

- Correct queries at all times.
    - Basic FK checking to protect against obvious errors.
        - A list of exceptions where FK constraints are not concrete (customer can be deleted, but subscription still references it).
    
    - Much easier to write tests, as all relations are checked by default.
        - Test code can focus on specific object invariants.
    
    - A "debug mode".
        - A tool to understand/list possible errors in a given database.
            - It is possible that future Stripe API changes could invalidate some assumptions about event ordering/relations. 



# Fix, B
- Still use native SQL FK constraints, but disable them per connection.
    - This will work for SQLite, but not server side SQL engines as they maintain the invariants over all connections, globally.
    
# Fix, C
- Only use native SQL for FK constraints that are constant (E.g. not (customer, sub), as the customer row can be deleted).   
    - This could work, but has issues:
        - When the Stripe account is constantly being written to:
            - Downloads could end up inconsistent.
                - But apply_events can be run inside the same dl tx.
                    - This would mean the FK constraints that would fail would be the ones on the leading edge of the event list (child event, no parent event). 
                        - apply_events could be retried in order to wait for the parent event.
                            - But for high throughput Stripe accounts (ecommerce stores on black friday), this retry loop might not end for hours.
                                - And may result in reactive functions based on polling SQL not receiving input state to process when they are intended to run every second.
                                    - Queries can also be written to work in spite of incorrect FK relations.



## Issues:
- `read-tx` is always 0, which means FK constraints may be invalid.
    - This is the best that can be done when the underlying API does not support read tx's.
    - It should only ever be a few objects on the leading edge of the events list that get cut off, which should have minimial impact on queries.
    - Stripe may be triggering/listing events in atomic units, so this may not be an issue.
        

# Questions


## Q: Why not add FK constraints to Rust structs using attributes?

Rust macros can only access AST data for the node they are applied to and its descendants.

This means you can define a FK on a Rust struct, but it would not be aware of the other Rust struct.

You could pass the Rust struct name, or create some kind of trait naming convention.

But it makes it more difficult to get a set of edges than can then be used as input to many other processes. 

Also, it does not seem to make sense for a FK edge to be define on one of the two nodes, as both of them are essential.


# Issues/improvement

## Polymorphic ID's.

With the current method it is only possible to check that the owner exists when it's ID is copied to another table as a FK.

Some Stripe objects contain one of many possible ID's (`balance_transaction.source`, which can be almost any Stripe object).

It could be possible to determine the ID type, map that to a table, and check its owner exists.

Issues:
- Not all ID's identify a single table they can be in (some prefixes can be primary keys of multiple tables). 

Ignore for now, as the objects with polymorphic ID's seem to be the exception/not primary objects.
 