stripe payment_intents update pi_1I8BAWBjw9m35HdrQn0SUggv \
    -d "shipping[name]"='Boaty Mc Boat face' \
    -d "shipping[address][line1]"='line 1 address' \
    -d "shipping[carrier]"='Fed Ex' \
    -d "shipping[tracking_number]"='FE1234';
