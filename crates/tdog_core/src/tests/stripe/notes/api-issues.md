# Why this document?

- To store all the issues and inconsistencies with Stripes API.
- To easily document why TD works the way it does.
    - In order to prove a certain issue is due to the API limitations and not the TD code.
- To find the best solution by understanding the limitations.
- @todo/low Convert this to a web doc so that users know which states are missing and why.


## Issues

### General

- The events stream is lossy - it does not trigger all events or include all data.
    - E.g. a PaymentMethodDetails does not contain metadata, PaymentMethod events are not triggered until they are attached.
    
- In general the events stream is for running functions, not for being able to sync 100% of bytes to run any concievable SQL program you want.


### Prices where active=false are only listed when created directly (not via subscription `price_data`)


- https://mail.google.com/mail/u/0/#search/stripe/WhctKJWJKgQgzFwXjFKdzrTqkhnSDtKFnChFFgdBKlmvnMQmjTflVxTJvlXKnFGSrtgqhFg

``` 
Hi there,

This is Amy from Stripe, thanks for providing additional details. 

I checked req_uDj2TLegQMo3pj, and it seems that this is an intended behavior. For any Price objects individually created via the API, the call to list all active = false prices should work.

However, in our particular case, the price ID was created as a result of a subscription. They are price_data price objects, not exactly price objects created via the Create a price API. 

If you could individually create prices via the API here, what you are trying to achieve with the list should work:
https://stripe.com/docs/api/prices/create

Let me know if this helps, have a great week ahead!

Best,
Amy
```


- `price.created` is not triggered when `price_data` is used.
- Inline prices are not listed in the prices dl list.



### PaymentMethods without Intent cannot be listed.
- You must supply a (customerId, type).
- There are 16 types.
- That's 16k requests if you have 1k customers.
- https://stripe.com/docs/api/payment_methods/list?lang=curl
- API limit is 100 requests a second?
- https://stripe.com/blog/rate-limiters
- @todo/low Add a CLI option that will make these requests on download.

```
https://stripe.com/docs/api/payment_methods/attach
To attach a new PaymentMethod to a customer for future payments, we recommend you use a SetupIntent or a PaymentIntent with setup_future_usage. These approaches will perform any necessary steps to ensure that the PaymentMethod can be used in a future payment.
```


### Only the top level types are defined.

- The documentation (and likely generated clients) only documents the first few levels of types.
    - After that, they give up and just say "this is a hash, no other info".

E.g. https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-stripe_account
payment_method_details.stripe_account



### Balance transactions cannot be kept up to date with events.

- There are no cud events for balance transactions.
    - status=available/pending means you cannot poll the download list API with `starting_after`.
        - As it would miss out all the status updates.
        
- Current fix:
    - Just insert balance_transactions as a JSON string for types that include the full object to allow some level of querying.
        - Even though this may not contain 100% of data or be up to date. 



### Refunds only have a `charge.refund.updated` event, others enclosed inside Charge that limits to 10 refunds. 

- https://stripe.com/docs/api/refunds/list
- Only the last 10 refunds are listed on Charge objects.
- Only `charge.refund.updated` is triggered for Refund objects.
    - But `charge.refunded` is triggered which contains the latest refund at position 1.

- Note: refunds are never deleted.
    
    
Fix, A: Just insert refunds on the charge object, ignore updates.
    
    - Or just apply updates to the JSON array, put up with a limit of 10?
    - Charges with more than 10 refunds must be in the 99.99th percentile.
    - Take this approach, move to C on complaint?
    

Fix, B: Collect all refund objects in event set, trigger upsert on each.    
    - This would collect all refunds getting around the limit of 10 which may create missing refunds on for the last Charge event.

Fix, C:
    - On Charge insert, insert all.
    - On Charge update, upsert all.
    - On Refund update, update
    
    - Issues: 
        - Will refund.update be a `c` as its the first time it's been seen?
            - Esp for the first apply_events after download?
                - I think the SQL query will return the refund row.
        - The logic for detecing the last write will not work for (charge, refund.update, charge).
            - refund.update will look like the last write, but there may be new data in charge.
                - Not an issue as both 1 and 2 will contain the same data, which result in the second update writing the same data (aside from the update ts). 

        - This is more code than A, but is functionally the same as they are both limited to 10?
            - If the user is applying events every few seconds, all refunds will be collected from charges.                  

- This differs from the parent->child pattern of customer->source.
    - `customer.source.created` is triggered.



### Coupons with valid=false cannot be listed at download time, but are used in historical calculations.
- Workaround: expand=discount, use discount.coupon.

# @todo

- Draw a global system state machine to formally define when/where edges in the graph are created.
    - Like a nested state graph.
    - Encode this into a Rust DSL/code?
        - At the moment this data is directly encoded into Rust functions, but its not possible to extract it into a data-only format, or compute over it with functions (E.g. for auto download).
        - The reason that Stripes API cannot be auto-downloaded into a local store automatically is because:
            - The API has so many inconsistencies that cannot be described in a programmatic spec.
            - Its not the main use case for the API.  


### What I would tell the Stripe developers if directing them to fix the API.
- Give every type an explicit create/update/delete event, and give the state transition a different key.

- Add meta data to the spec:
    - `has_dl_list`
        - Can this object be listed at download time (without having to give a parent object id like customer ID)?
    - `has_direct_event`
        - Does this event get created, updated and deleted events directly?
            - And not just the "connection" events like `customer.discount.*`.
                - This are about adding/removing connections, not the actual objects.
                
    - Note: it may be possible to auto-generate most of the TD code with this meta data.
        - Issue: polymorphic types, exceptions, understanding what makes a good queryable SQL schema etc.
            - These all require a human to make the judgment/mapping.


- If a child is always expanded (in both the dl object and event), and when it is updated its parent event fires an update, just insert from the parent object always?
    - Issue: Some child objects can be detached, and may need to be queryable.
    - Issue: `has_more=true` requires making DL requests whilst processing events (which could be days old).
    - E.g. invoice, invoice items (two diff types). 
        - This trade off seems worth making:
            - Ignore InvoiceItem
                - Bad
                    - When not attached they cannot be seen/queried.
                        - The end state for invoice items is to be attached to invoices, so this should not matter too much.
                - Good
                    - There is only one place to query invoice items
                    - No conflict between data available.
                 


- Make the event stream non-lossy.
    - E.g. so it can be applied to a dataset without a dependency on the current Stripe server state.
        - subscription.items has a limit of 10 items, when has_more=true, the user is expected to fetch the rest from the server (assuming the event is being processed shortly after it was triggered).



- Default/allow limit=100 when expanding list items.
    - E.g. sub.items, invoice.lines.
    - This would allow much simpler testing as you can save and re-apply the events (instead of having to download items >10 directly when processing an event with has_more=true).


### Alternative implementations to deal with these issues in a general way (instead of one by one by hand)

- A.
    - Treat the event stream as a single data structure with duplicates, and lossy events.
        - E.g. `charge.refund.updated` is a strange event as it is the only event triggered for refunds, the others are enclosed in the Charge events.
        - Break down the JSON tree into a single flat level, with a (path, scalar_value) structure.
            - Keep the connection to the original JSON.
        - Walk this data computing SQL writes needed.   
         


### In-consistent parent-child events.

E.g.
    - `customer.updated` always precedes `customer.source.deleted`
    - `customer.updates` never precedes `customer.discount.deleted`
        - Also:
            - `customer.discount.deleted` is fired even if the parent/owner is an sub, invoice or invoice item.
            
    - These inconsistencies are an issie becasue it prevents writing general code on the assumption parent/child events are handled the same accross the entire object graph, regardles of type.
        - It seems these events are just fired willy nilly, just when ever the developer decided to put them in.

    - Why this is an issue for TD code.
        - It is much easier to have each event upsert to on object type, and expect that if a parents FK to a child is removed, its update is fired along with changes on the child.
        - Not enforcing this "one event per upsert" means that the TD code must infer state and write many specifc SQL queries (determine which object is parent, update that hoping to match its current server state even though it has changed and no event has fired).


### Events for edges vs vertex
- Some events are for the actual objects themselves, and some are for the connection between object.
- E.g.
    - `customer.x.(created|updated|deleted)`
        - This is fired in a few places, but only means "x is attached to the customer".
            - Often the type still exists, it is just not discoverable (but can still be updated)



### `[]` vs null

E.g. invoice spec says `discounts` can be null, but its always `[]` for no discounts.



### Invoice items are confusing

- Different types listing directly vs as a child of invoice.
    - Child invoice:
        - Has: `object: 'line_item', type: 'invoiceitem'`
        - No events.
    - Direct listing:
        - Has: `object: 'invoiceitem'`
        - `date: 1618675316` - time it was created. Missing on line items, inconsistent with `created`.
        - Has events.
        - `unit_amount` and `amount` are the same, `unit_amount` is missing on the inv-child.

- There is no `invoice line item deleted` event.
    - There is a `invoiceitem.deleted`, but this is not the same as `invoice_line_item.deleted`, as the line item contains per line discount totals.
        - An `invoice.update` is triggered, but `lines` is limited to 10, so you would have no idea which one was deleted.
            - Does `invoiceitem.deleted` also represent `invoice_line_item.deleted`?
                - Should it be added? 

Issue: The gap between a parents event, and when its children are downloaded an inserted can be an issue.
- E.g. invoice items contribute to a sum on the parent, and if the process (upsert from event, wait 10s, dl_children and insert), then in that 10s gap the parent sum could be incorrect, and would need re-downloading.
    - General issue: no read transaction when interacting with the Stripe server (A: dl invoice, C: dl child items), at point B more child items could be added than is represented in A.sum.
        - Note: I want a read transaction on a TD database to represent an atomic state point on the Stripe server.
            - The event stream probably implicitly triggers events that are inside the same write transaction together.
                - This means when you download the events, *you are only getting valid atomic writes*, which avoids the "no read tx" issue with the download API.
                    - Inserting all this event data in one atomic write tx means that any clients opening a read tx *will be reading/querying valid atomic states only*, which is impossible to do with just downloading and caching objects and never applying the events.



### Twitter note to CJ Avilla:
"invoice items" and "invoice *line* items" are similarly named and contain some of the same data, but represent different objects.

invoice items
- you are right, the events exist and I can keep these up to date with no download.

invoice *line* items
- there are no events for these, these are the cause of the issue.
- these also contain different data to the line items (discount totals, foreign keys to subscription/sub items).
- They may link to one or both of: (invoice item, sub item).



### Stripe issue: Commit to a non-lossy event stream (missing out data and requiring a direct download).

- Event list option: `has_more_expand=true`.
    - Always list all child options.

- Extra events.
    - Trigger all missing events.


# Fixes, general

Possible approaches to fixing all of the API issues.

- Meta data for every array.
    - For every array, add:
        - can_be_listed_via_dl
            - Can this object be listed by itself in the API?
        - can_be_listed_via_dl
            - Does it get direct updates?
            - E.g. Subscription items can be listed, but do not get their own update events.        
    
    - Nesting.
        - invoice -> discount -> coupon
            - Coupons = can_be_listed_via_dl + can_be_listed_via_dl
                - So even though discounts must be inlined via JSON strings, they may nest other types that have their own tables and rows, so only need to contain their string IDs and not the entire object.

    - This meta data is similar to types for scalar values, but relates to how the types compose, *especially across the download+events interfaces as a whole*.
    - The data is also implicitly encoded, but difficult to read and write as it is encoded as the types are being discovered/written.


- Docs: Pass an empty string to remove previously-defined discounts.
    - This just seems strange/inconsistent.


### Related
- evernote:///view/14186947/s134/f5640baf-71e3-42be-ba53-608b86e0477c/f5640baf-71e3-42be-ba53-608b86e0477c/



### CustomerBalanceTransaction

- !has_direct_dl (needs customer id)
- !has_direct_event (no events)

https://stripe.com/docs/api/customer_balance_transactions/list?lang=node


Temp fix: Just use `customer.balance`, which is the current state of this value.


### Session

- `line_items` JSON key not included in downloads.
    - `CreditNote.lines` and `Invoice.lines` both include 0-10 items (TD panics if it sees >10 in an event).
    - `Subscription` includes 20 items (100%; this is the max items; no need to dl the rest).
    
- No `create` events - impossible to keep up to date.
    - This means `payment_status=unpaid` can only be download, not seen via queries on databases that have the events applied.
        - Document this or filter payment_status to match state accessible from events.

- Impossible to test without implementing the web UI.

- Name differs from other object names: `checkout.session`

- Search: dl_at_end_of_apply_events


### Mandate
- Cannot be listed.
- Only has updated event, no create.

### SKU
- sku.updated is equal to sku.created on create (x.updated is null for other types).
- sku.deleted still has active=true

### Order returns
- Not deleted on "delete all".
- Only `order_return.created`, no `order_return.updated`
    - E.g.
        - Order return created
        - Refund created.
        - Order return update should fire with order_return.refund=x, but it does not (leaving order_return.refund=null).
            - Fix: This can be queried from other paths.
            
    - Fix: `order.updated` contains the data.
    
    
# Relations

- See lib-app/src/providers/stripe/schema/relations.rs
    - Lists many issues with missing data from events or invisible objects (E.g. product created via invoice item API).