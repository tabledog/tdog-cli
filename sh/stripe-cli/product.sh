# stripe customers update cus_Ifq8BTooLFnrw7 -d 'invoice_settings[custom_fields][][name]=abc'

stripe products create \
    -d "name"='CLI product A' \
    -d "attributes[]"='a1';

stripe products create \
    -d "type"="good" \
    -d "name"='CLI product A' \
    -d "shippable"='true' \
    -d "attributes[]"='premium' \
    -d "package_dimensions[height]"='1.12' \
    -d "package_dimensions[width]"='2.12' \
    -d "package_dimensions[length]"='3.12' \
    -d "package_dimensions[weight]"='4.12';