# stripe customers update cus_Ifq8BTooLFnrw7 -d 'invoice_settings[custom_fields][][name]=abc'

stripe subscriptions update sub_Ifq8j27SHCINgy \
    -d "cancel_at"=1641840226 \
    -d "collection_method"=send_invoice \
    -d "days_until_due"=99 \
    -d "pending_invoice_item_interval[interval]"=day \
    -d "pending_invoice_item_interval[interval_count]"=2 \
    -d "default_payment_method"=pm_1I4US3Bjw9m35Hdr9q2qLqZl;