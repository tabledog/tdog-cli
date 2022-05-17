# Intro

This file contains SQLite queries to:
- Verify the DB state is correct during testing.
- Form the basis of the query library for the web docs.
- Highlight the best way to query certain objects/states, or clarify ambiguous queries (to ensure users know how to get the correct answer and avoid issues with incorrect queries being blamed on TD).
- To understand the performance of queries, esp in relation to JSON functions.



## Action taken for each event. 

```sql
SELECT
	run_id,
	e.id,
	TYPE,
	w.action,
	w.write_ids,
	data_object_object,
	data_object_id
FROM
	notification_events e
	LEFT JOIN td_stripe_apply_events w ON e.id = w.event_id
```

 
## Discounts: check which discounts are deleted but still reference an object.

- E.g. Updating a discount on a subscription will delete the old one, but TD sees that as an update to keep all discount->coupon connections for historical queries.
    - Users should always join from the parent object to the discount.
 
```sql
select
	(select discount from customers where id=d.customer) c_d,
	(select discounts from invoices where id=d.invoice) i_d,
	(select discounts from invoiceitems where id=d.invoice_item) ii_d,
	(select discount from subscriptions where id=d.subscription) s_d,
	d.*
from discounts d
```