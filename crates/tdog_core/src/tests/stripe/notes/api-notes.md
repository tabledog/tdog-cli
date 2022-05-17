# Intro

This document will list general notes about the API.

Less obvious things that took time to work out, but which the documentation leaves out.



### PaymentMethod vs PaymentMethodDetails

https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-stripe_account
 
```
payment_method_details.card.network
string
Identifies which network this charge ***was processed on***. Can be amex, cartes_bancaires, diners, discover, interac, jcb, mastercard, unionpay, visa, or unknown.
```

- It seems a `payment_method_details` is for after a successful charge, and `payment_method` is a pending charge.
- They contain around 10% differences in their trees.
- I'm not sure if they can both go into the same table.

https://stripe.com/docs/api/payment_methods/object#payment_method_object-card-networks

```
card.networks
hash
Contains information about card networks ***that can be used to process*** the payment.
```

This difference is quite strange as they are mixing up state transitions with data types.

PaymentMethods can only have one customer, and it can not be moved (enforced at Intent creation time).


### Standalone objects vs contained.

There seems to be a difference between stand alone objects and contained ones.
- E.g.
    - Price created inline as a child to a subscription vs standalone.
    - PaymentMethodDetails (when attached to an Intent; no metadata) vs PaymentMethod (when standalone or attached to a customer; has metadata).
     