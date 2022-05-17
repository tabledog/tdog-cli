
#![allow(warnings)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::responses::*;
use super::types::*;

/// Spec paths:
/// - `credit_note.reason`
/// - `/v1/credit_notes/preview.get.GetCreditNotesPreview.reason`
/// - `/v1/credit_notes/preview/lines.get.GetCreditNotesPreviewLines.reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrReasonAB5E91 {
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "fraudulent")]
    Fraudulent,
    #[serde(rename = "order_change")]
    OrderChange,
    #[serde(rename = "product_unsatisfactory")]
    ProductUnsatisfactory,
}

/// Spec paths:
/// - `credit_note_line_item.type`
/// - `/v1/credit_notes/preview.get.GetCreditNotesPreview.lines.items.type`
/// - `/v1/credit_notes/preview/lines.get.GetCreditNotesPreviewLines.lines.items.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrType00DF75 {
    #[serde(rename = "custom_line_item")]
    CustomLineItem,
    #[serde(rename = "invoice_line_item")]
    InvoiceLineItem,
}

/// Spec paths:
/// - `customer_acceptance.type`
/// - `/v1/terminal/readers.get.GetTerminalReaders.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTypeD153A1 {
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "online")]
    Online,
}

/// Spec paths:
/// - `invoice.collection_method`
/// - `subscription.collection_method`
/// - `subscription_schedule_phase_configuration.collection_method`
/// - `subscription_schedules_resource_default_settings.collection_method`
/// - `/v1/invoices.get.GetInvoices.collection_method`
/// - `/v1/subscriptions.get.GetSubscriptions.collection_method`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrCollectionMethod {
    #[serde(rename = "charge_automatically")]
    ChargeAutomatically,
    #[serde(rename = "send_invoice")]
    SendInvoice,
}

/// Spec paths:
/// - `issuing.authorization.status`
/// - `/v1/issuing/authorizations.get.GetIssuingAuthorizations.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatus957169 {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "reversed")]
    Reversed,
}

/// Spec paths:
/// - `issuing.card.status`
/// - `/v1/issuing/cards.get.GetIssuingCards.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatusA4138B {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "inactive")]
    Inactive,
}

/// Spec paths:
/// - `issuing.card.type`
/// - `/v1/issuing/cards.get.GetIssuingCards.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTypeA467AF {
    #[serde(rename = "physical")]
    Physical,
    #[serde(rename = "virtual")]
    Virtual,
}

/// Spec paths:
/// - `issuing.cardholder.status`
/// - `/v1/issuing/cardholders.get.GetIssuingCardholders.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatusD5D208 {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "inactive")]
    Inactive,
}

/// Spec paths:
/// - `issuing.cardholder.type`
/// - `payment_method_details_ach_debit.account_holder_type`
/// - `/v1/issuing/cardholders.get.GetIssuingCardholders.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrType947A77 {
    #[serde(rename = "company")]
    Company,
    #[serde(rename = "individual")]
    Individual,
}

/// Spec paths:
/// - `issuing.dispute.status`
/// - `/v1/issuing/disputes.get.GetIssuingDisputes.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatusE71251 {
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "lost")]
    Lost,
    #[serde(rename = "submitted")]
    Submitted,
    #[serde(rename = "unsubmitted")]
    Unsubmitted,
    #[serde(rename = "won")]
    Won,
}

/// Spec paths:
/// - `plan.interval`
/// - `recurring.interval`
/// - `subscription_pending_invoice_item_interval.interval`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_items.items.price_data.recurring.interval`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_items.items.price_data.recurring.interval`
/// - `/v1/prices.get.GetPrices.recurring.interval`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrInterval {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "year")]
    Year,
}

/// Spec paths:
/// - `plan.usage_type`
/// - `recurring.usage_type`
/// - `/v1/prices.get.GetPrices.recurring.usage_type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrUsageType {
    #[serde(rename = "licensed")]
    Licensed,
    #[serde(rename = "metered")]
    Metered,
}

/// Spec paths:
/// - `price.type`
/// - `/v1/prices.get.GetPrices.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTypeFC33AE {
    #[serde(rename = "one_time")]
    OneTime,
    #[serde(rename = "recurring")]
    Recurring,
}

/// Spec paths:
/// - `subscription_schedule_phase_configuration.proration_behavior`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_proration_behavior`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_proration_behavior`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrProrationBehavior {
    #[serde(rename = "always_invoice")]
    AlwaysInvoice,
    #[serde(rename = "create_prorations")]
    CreateProrations,
    #[serde(rename = "none")]
    None,
}

/// Spec paths:
/// - `terminal.reader.device_type`
/// - `/v1/terminal/readers.get.GetTerminalReaders.device_type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrDeviceType {
    #[serde(rename = "bbpos_chipper2x")]
    BbposChipper2x,
    #[serde(rename = "verifone_P400")]
    VerifoneP400,
}

/// Spec paths:
/// - `/v1/3d_secure/{three_d_secure}.get.Get3dSecureThreeDSecure`
/// - `/v1/account.get.GetAccount`
/// - `/v1/account/bank_accounts/{id}.get.GetAccountBankAccountsId`
/// - `/v1/account/capabilities.get.GetAccountCapabilities`
/// - `/v1/account/capabilities/{capability}.get.GetAccountCapabilitiesCapability`
/// - `/v1/account/external_accounts/{id}.get.GetAccountExternalAccountsId`
/// - `/v1/account/people/{person}.get.GetAccountPeoplePerson`
/// - `/v1/account/persons/{person}.get.GetAccountPersonsPerson`
/// - `/v1/accounts/{account}.get.GetAccountsAccount`
/// - `/v1/accounts/{account}/bank_accounts/{id}.get.GetAccountsAccountBankAccountsId`
/// - `/v1/accounts/{account}/capabilities.get.GetAccountsAccountCapabilities`
/// - `/v1/accounts/{account}/capabilities/{capability}.get.GetAccountsAccountCapabilitiesCapability`
/// - `/v1/accounts/{account}/external_accounts/{id}.get.GetAccountsAccountExternalAccountsId`
/// - `/v1/accounts/{account}/people/{person}.get.GetAccountsAccountPeoplePerson`
/// - `/v1/accounts/{account}/persons/{person}.get.GetAccountsAccountPersonsPerson`
/// - `/v1/apple_pay/domains/{domain}.get.GetApplePayDomainsDomain`
/// - `/v1/application_fees/{fee}/refunds/{id}.get.GetApplicationFeesFeeRefundsId`
/// - `/v1/application_fees/{id}.get.GetApplicationFeesId`
/// - `/v1/balance.get.GetBalance`
/// - `/v1/balance/history/{id}.get.GetBalanceHistoryId`
/// - `/v1/balance_transactions/{id}.get.GetBalanceTransactionsId`
/// - `/v1/bitcoin/receivers/{id}.get.GetBitcoinReceiversId`
/// - `/v1/charges/{charge}.get.GetChargesCharge`
/// - `/v1/charges/{charge}/dispute.get.GetChargesChargeDispute`
/// - `/v1/charges/{charge}/refunds/{refund}.get.GetChargesChargeRefundsRefund`
/// - `/v1/checkout/sessions/{session}.get.GetCheckoutSessionsSession`
/// - `/v1/country_specs/{country}.get.GetCountrySpecsCountry`
/// - `/v1/coupons/{coupon}.get.GetCouponsCoupon`
/// - `/v1/credit_notes/{id}.get.GetCreditNotesId`
/// - `/v1/customers/{customer}.get.GetCustomersCustomer`
/// - `/v1/customers/{customer}/balance_transactions/{transaction}.get.GetCustomersCustomerBalanceTransactionsTransaction`
/// - `/v1/customers/{customer}/bank_accounts/{id}.get.GetCustomersCustomerBankAccountsId`
/// - `/v1/customers/{customer}/cards/{id}.get.GetCustomersCustomerCardsId`
/// - `/v1/customers/{customer}/discount.get.GetCustomersCustomerDiscount`
/// - `/v1/customers/{customer}/sources/{id}.get.GetCustomersCustomerSourcesId`
/// - `/v1/customers/{customer}/subscriptions/{subscription_exposed_id}.get.GetCustomersCustomerSubscriptionsSubscriptionExposedId`
/// - `/v1/customers/{customer}/subscriptions/{subscription_exposed_id}/discount.get.GetCustomersCustomerSubscriptionsSubscriptionExposedIdDiscount`
/// - `/v1/customers/{customer}/tax_ids/{id}.get.GetCustomersCustomerTaxIdsId`
/// - `/v1/disputes/{dispute}.get.GetDisputesDispute`
/// - `/v1/events/{id}.get.GetEventsId`
/// - `/v1/exchange_rates/{rate_id}.get.GetExchangeRatesRateId`
/// - `/v1/file_links/{link}.get.GetFileLinksLink`
/// - `/v1/files/{file}.get.GetFilesFile`
/// - `/v1/invoiceitems/{invoiceitem}.get.GetInvoiceitemsInvoiceitem`
/// - `/v1/invoices/{invoice}.get.GetInvoicesInvoice`
/// - `/v1/issuer_fraud_records/{issuer_fraud_record}.get.GetIssuerFraudRecordsIssuerFraudRecord`
/// - `/v1/issuing/authorizations/{authorization}.get.GetIssuingAuthorizationsAuthorization`
/// - `/v1/issuing/cardholders/{cardholder}.get.GetIssuingCardholdersCardholder`
/// - `/v1/issuing/cards/{card}.get.GetIssuingCardsCard`
/// - `/v1/issuing/disputes/{dispute}.get.GetIssuingDisputesDispute`
/// - `/v1/issuing/settlements/{settlement}.get.GetIssuingSettlementsSettlement`
/// - `/v1/issuing/transactions/{transaction}.get.GetIssuingTransactionsTransaction`
/// - `/v1/mandates/{mandate}.get.GetMandatesMandate`
/// - `/v1/order_returns/{id}.get.GetOrderReturnsId`
/// - `/v1/orders/{id}.get.GetOrdersId`
/// - `/v1/payment_methods/{payment_method}.get.GetPaymentMethodsPaymentMethod`
/// - `/v1/payouts/{payout}.get.GetPayoutsPayout`
/// - `/v1/plans/{plan}.get.GetPlansPlan`
/// - `/v1/prices/{price}.get.GetPricesPrice`
/// - `/v1/products/{id}.get.GetProductsId`
/// - `/v1/promotion_codes/{promotion_code}.get.GetPromotionCodesPromotionCode`
/// - `/v1/radar/early_fraud_warnings/{early_fraud_warning}.get.GetRadarEarlyFraudWarningsEarlyFraudWarning`
/// - `/v1/radar/value_list_items/{item}.get.GetRadarValueListItemsItem`
/// - `/v1/radar/value_lists/{value_list}.get.GetRadarValueListsValueList`
/// - `/v1/recipients/{id}.get.GetRecipientsId`
/// - `/v1/refunds/{refund}.get.GetRefundsRefund`
/// - `/v1/reporting/report_runs/{report_run}.get.GetReportingReportRunsReportRun`
/// - `/v1/reporting/report_types.get.GetReportingReportTypes`
/// - `/v1/reporting/report_types/{report_type}.get.GetReportingReportTypesReportType`
/// - `/v1/reviews/{review}.get.GetReviewsReview`
/// - `/v1/sigma/scheduled_query_runs/{scheduled_query_run}.get.GetSigmaScheduledQueryRunsScheduledQueryRun`
/// - `/v1/skus/{id}.get.GetSkusId`
/// - `/v1/sources/{source}/mandate_notifications/{mandate_notification}.get.GetSourcesSourceMandateNotificationsMandateNotification`
/// - `/v1/sources/{source}/source_transactions/{source_transaction}.get.GetSourcesSourceSourceTransactionsSourceTransaction`
/// - `/v1/subscription_items/{item}.get.GetSubscriptionItemsItem`
/// - `/v1/subscription_schedules/{schedule}.get.GetSubscriptionSchedulesSchedule`
/// - `/v1/subscriptions/{subscription_exposed_id}.get.GetSubscriptionsSubscriptionExposedId`
/// - `/v1/tax_rates/{tax_rate}.get.GetTaxRatesTaxRate`
/// - `/v1/terminal/locations/{location}.get.GetTerminalLocationsLocation`
/// - `/v1/terminal/readers/{reader}.get.GetTerminalReadersReader`
/// - `/v1/tokens/{token}.get.GetTokensToken`
/// - `/v1/topups/{topup}.get.GetTopupsTopup`
/// - `/v1/transfers/{transfer}.get.GetTransfersTransfer`
/// - `/v1/transfers/{transfer}/reversals/{id}.get.GetTransfersTransferReversalsId`
/// - `/v1/webhook_endpoints/{webhook_endpoint}.get.GetWebhookEndpointsWebhookEndpoint`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccount {
    pub expand: Option<Vec<String>>,
}

/// Spec paths:
/// - `/v1/account/external_accounts.get.GetAccountExternalAccounts`
/// - `/v1/accounts/{account}/external_accounts.get.GetAccountsAccountExternalAccounts`
/// - `/v1/charges/{charge}/refunds.get.GetChargesChargeRefunds`
/// - `/v1/customers/{customer}/bank_accounts.get.GetCustomersCustomerBankAccounts`
/// - `/v1/customers/{customer}/cards.get.GetCustomersCustomerCards`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetChargesChargeRefunds {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetChargesChargeRefunds {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/account/people.get.GetAccountPeople`
/// - `/v1/account/persons.get.GetAccountPersons`
/// - `/v1/accounts/{account}/people.get.GetAccountsAccountPeople`
/// - `/v1/accounts/{account}/persons.get.GetAccountsAccountPersons`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountPeople {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub relationship: Option<AllPeopleRelationshipSpecs>,
    pub starting_after: Option<String>,
}

impl PageParam for GetAccountPeople {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/account/people.get.GetAccountPeople.relationship`
/// - `/v1/account/persons.get.GetAccountPersons.relationship`
/// - `/v1/accounts/{account}/people.get.GetAccountsAccountPeople.relationship`
/// - `/v1/accounts/{account}/persons.get.GetAccountsAccountPersons.relationship`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllPeopleRelationshipSpecs {
    pub director: Option<bool>,
    pub executive: Option<bool>,
    pub owner: Option<bool>,
    pub representative: Option<bool>,
}

/// Spec paths:
/// - `/v1/accounts.get.GetAccounts`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccounts {
    pub created: Option<UniCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetAccounts {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/accounts.get.GetAccounts.created`
/// - `/v1/application_fees.get.GetApplicationFees.created`
/// - `/v1/balance/history.get.GetBalanceHistory.available_on`
/// - `/v1/balance/history.get.GetBalanceHistory.created`
/// - `/v1/balance_transactions.get.GetBalanceTransactions.available_on`
/// - `/v1/balance_transactions.get.GetBalanceTransactions.created`
/// - `/v1/charges.get.GetCharges.created`
/// - `/v1/coupons.get.GetCoupons.created`
/// - `/v1/customers.get.GetCustomers.created`
/// - `/v1/disputes.get.GetDisputes.created`
/// - `/v1/events.get.GetEvents.created`
/// - `/v1/file_links.get.GetFileLinks.created`
/// - `/v1/files.get.GetFiles.created`
/// - `/v1/invoiceitems.get.GetInvoiceitems.created`
/// - `/v1/invoices.get.GetInvoices.created`
/// - `/v1/invoices.get.GetInvoices.due_date`
/// - `/v1/issuing/authorizations.get.GetIssuingAuthorizations.created`
/// - `/v1/issuing/cardholders.get.GetIssuingCardholders.created`
/// - `/v1/issuing/cards.get.GetIssuingCards.created`
/// - `/v1/issuing/disputes.get.GetIssuingDisputes.created`
/// - `/v1/issuing/settlements.get.GetIssuingSettlements.created`
/// - `/v1/issuing/transactions.get.GetIssuingTransactions.created`
/// - `/v1/order_returns.get.GetOrderReturns.created`
/// - `/v1/orders.get.GetOrders.created`
/// - `/v1/orders.get.GetOrders.status_transitions.canceled`
/// - `/v1/orders.get.GetOrders.status_transitions.fulfilled`
/// - `/v1/orders.get.GetOrders.status_transitions.paid`
/// - `/v1/orders.get.GetOrders.status_transitions.returned`
/// - `/v1/payment_intents.get.GetPaymentIntents.created`
/// - `/v1/payouts.get.GetPayouts.arrival_date`
/// - `/v1/payouts.get.GetPayouts.created`
/// - `/v1/plans.get.GetPlans.created`
/// - `/v1/prices.get.GetPrices.created`
/// - `/v1/products.get.GetProducts.created`
/// - `/v1/promotion_codes.get.GetPromotionCodes.created`
/// - `/v1/radar/value_list_items.get.GetRadarValueListItems.created`
/// - `/v1/radar/value_lists.get.GetRadarValueLists.created`
/// - `/v1/recipients.get.GetRecipients.created`
/// - `/v1/refunds.get.GetRefunds.created`
/// - `/v1/reporting/report_runs.get.GetReportingReportRuns.created`
/// - `/v1/reviews.get.GetReviews.created`
/// - `/v1/setup_attempts.get.GetSetupAttempts.created`
/// - `/v1/setup_intents.get.GetSetupIntents.created`
/// - `/v1/subscription_schedules.get.GetSubscriptionSchedules.canceled_at`
/// - `/v1/subscription_schedules.get.GetSubscriptionSchedules.completed_at`
/// - `/v1/subscription_schedules.get.GetSubscriptionSchedules.created`
/// - `/v1/subscription_schedules.get.GetSubscriptionSchedules.released_at`
/// - `/v1/subscriptions.get.GetSubscriptions.created`
/// - `/v1/subscriptions.get.GetSubscriptions.current_period_end`
/// - `/v1/subscriptions.get.GetSubscriptions.current_period_start`
/// - `/v1/tax_rates.get.GetTaxRates.created`
/// - `/v1/topups.get.GetTopups.amount`
/// - `/v1/topups.get.GetTopups.created`
/// - `/v1/transfers.get.GetTransfers.created`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniCreated {
    RangeQuerySpecs(RangeQuerySpecs),
    I64(i64),
}

/// Spec paths:
/// - `/v1/accounts.get.GetAccounts.created.anyOf.0`
/// - `/v1/application_fees.get.GetApplicationFees.created.anyOf.0`
/// - `/v1/balance/history.get.GetBalanceHistory.available_on.anyOf.0`
/// - `/v1/balance/history.get.GetBalanceHistory.created.anyOf.0`
/// - `/v1/balance_transactions.get.GetBalanceTransactions.available_on.anyOf.0`
/// - `/v1/balance_transactions.get.GetBalanceTransactions.created.anyOf.0`
/// - `/v1/charges.get.GetCharges.created.anyOf.0`
/// - `/v1/coupons.get.GetCoupons.created.anyOf.0`
/// - `/v1/customers.get.GetCustomers.created.anyOf.0`
/// - `/v1/disputes.get.GetDisputes.created.anyOf.0`
/// - `/v1/events.get.GetEvents.created.anyOf.0`
/// - `/v1/file_links.get.GetFileLinks.created.anyOf.0`
/// - `/v1/files.get.GetFiles.created.anyOf.0`
/// - `/v1/invoiceitems.get.GetInvoiceitems.created.anyOf.0`
/// - `/v1/invoices.get.GetInvoices.created.anyOf.0`
/// - `/v1/invoices.get.GetInvoices.due_date.anyOf.0`
/// - `/v1/issuing/authorizations.get.GetIssuingAuthorizations.created.anyOf.0`
/// - `/v1/issuing/cardholders.get.GetIssuingCardholders.created.anyOf.0`
/// - `/v1/issuing/cards.get.GetIssuingCards.created.anyOf.0`
/// - `/v1/issuing/disputes.get.GetIssuingDisputes.created.anyOf.0`
/// - `/v1/issuing/settlements.get.GetIssuingSettlements.created.anyOf.0`
/// - `/v1/issuing/transactions.get.GetIssuingTransactions.created.anyOf.0`
/// - `/v1/order_returns.get.GetOrderReturns.created.anyOf.0`
/// - `/v1/orders.get.GetOrders.created.anyOf.0`
/// - `/v1/orders.get.GetOrders.status_transitions.canceled.anyOf.0`
/// - `/v1/orders.get.GetOrders.status_transitions.fulfilled.anyOf.0`
/// - `/v1/orders.get.GetOrders.status_transitions.paid.anyOf.0`
/// - `/v1/orders.get.GetOrders.status_transitions.returned.anyOf.0`
/// - `/v1/payment_intents.get.GetPaymentIntents.created.anyOf.0`
/// - `/v1/payouts.get.GetPayouts.arrival_date.anyOf.0`
/// - `/v1/payouts.get.GetPayouts.created.anyOf.0`
/// - `/v1/plans.get.GetPlans.created.anyOf.0`
/// - `/v1/prices.get.GetPrices.created.anyOf.0`
/// - `/v1/products.get.GetProducts.created.anyOf.0`
/// - `/v1/promotion_codes.get.GetPromotionCodes.created.anyOf.0`
/// - `/v1/radar/value_list_items.get.GetRadarValueListItems.created.anyOf.0`
/// - `/v1/radar/value_lists.get.GetRadarValueLists.created.anyOf.0`
/// - `/v1/recipients.get.GetRecipients.created.anyOf.0`
/// - `/v1/refunds.get.GetRefunds.created.anyOf.0`
/// - `/v1/reporting/report_runs.get.GetReportingReportRuns.created.anyOf.0`
/// - `/v1/reviews.get.GetReviews.created.anyOf.0`
/// - `/v1/setup_attempts.get.GetSetupAttempts.created.anyOf.0`
/// - `/v1/setup_intents.get.GetSetupIntents.created.anyOf.0`
/// - `/v1/subscription_schedules.get.GetSubscriptionSchedules.canceled_at.anyOf.0`
/// - `/v1/subscription_schedules.get.GetSubscriptionSchedules.completed_at.anyOf.0`
/// - `/v1/subscription_schedules.get.GetSubscriptionSchedules.created.anyOf.0`
/// - `/v1/subscription_schedules.get.GetSubscriptionSchedules.released_at.anyOf.0`
/// - `/v1/subscriptions.get.GetSubscriptions.created.anyOf.0`
/// - `/v1/subscriptions.get.GetSubscriptions.current_period_end.anyOf.0`
/// - `/v1/subscriptions.get.GetSubscriptions.current_period_start.anyOf.0`
/// - `/v1/tax_rates.get.GetTaxRates.created.anyOf.0`
/// - `/v1/topups.get.GetTopups.amount.anyOf.0`
/// - `/v1/topups.get.GetTopups.created.anyOf.0`
/// - `/v1/transfers.get.GetTransfers.created.anyOf.0`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RangeQuerySpecs {
    pub gt: Option<i64>,
    pub gte: Option<i64>,
    pub lt: Option<i64>,
    pub lte: Option<i64>,
}

/// Spec paths:
/// - `/v1/apple_pay/domains.get.GetApplePayDomains`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetApplePayDomains {
    pub domain_name: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetApplePayDomains {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/application_fees.get.GetApplicationFees`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetApplicationFees {
    pub created: Option<UniCreated>,
    pub charge: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetApplicationFees {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/application_fees/{id}/refunds.get.GetApplicationFeesIdRefunds`
/// - `/v1/checkout/sessions/{session}/line_items.get.GetCheckoutSessionsSessionLineItems`
/// - `/v1/country_specs.get.GetCountrySpecs`
/// - `/v1/credit_notes/{credit_note}/lines.get.GetCreditNotesCreditNoteLines`
/// - `/v1/customers/{customer}/balance_transactions.get.GetCustomersCustomerBalanceTransactions`
/// - `/v1/customers/{customer}/subscriptions.get.GetCustomersCustomerSubscriptions`
/// - `/v1/customers/{customer}/tax_ids.get.GetCustomersCustomerTaxIds`
/// - `/v1/exchange_rates.get.GetExchangeRates`
/// - `/v1/invoices/{invoice}/lines.get.GetInvoicesInvoiceLines`
/// - `/v1/sigma/scheduled_query_runs.get.GetSigmaScheduledQueryRuns`
/// - `/v1/sources/{source}/source_transactions.get.GetSourcesSourceSourceTransactions`
/// - `/v1/subscription_items/{subscription_item}/usage_record_summaries.get.GetSubscriptionItemsSubscriptionItemUsageRecordSummaries`
/// - `/v1/terminal/locations.get.GetTerminalLocations`
/// - `/v1/transfers/{id}/reversals.get.GetTransfersIdReversals`
/// - `/v1/webhook_endpoints.get.GetWebhookEndpoints`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCountrySpecs {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetCountrySpecs {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/balance/history.get.GetBalanceHistory`
/// - `/v1/balance_transactions.get.GetBalanceTransactions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBalanceHistory {
    #[serde(rename = "type")]
    pub type_x: Option<String>,
    pub available_on: Option<UniCreated>,
    pub created: Option<UniCreated>,
    pub currency: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payout: Option<String>,
    pub source: Option<String>,
    pub starting_after: Option<String>,
}

impl PageParam for GetBalanceHistory {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/bitcoin/receivers.get.GetBitcoinReceivers`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBitcoinReceivers {
    pub active: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub filled: Option<bool>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub uncaptured_funds: Option<bool>,
}

impl PageParam for GetBitcoinReceivers {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/bitcoin/receivers/{receiver}/transactions.get.GetBitcoinReceiversReceiverTransactions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBitcoinReceiversReceiverTransactions {
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetBitcoinReceiversReceiverTransactions {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/bitcoin/transactions.get.GetBitcoinTransactions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBitcoinTransactions {
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub receiver: Option<String>,
    pub starting_after: Option<String>,
}

impl PageParam for GetBitcoinTransactions {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/charges.get.GetCharges`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCharges {
    pub created: Option<UniCreated>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
    pub transfer_group: Option<String>,
}

impl PageParam for GetCharges {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/checkout/sessions.get.GetCheckoutSessions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCheckoutSessions {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
    pub subscription: Option<String>,
}

impl PageParam for GetCheckoutSessions {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/coupons.get.GetCoupons`
/// - `/v1/issuing/settlements.get.GetIssuingSettlements`
/// - `/v1/reporting/report_runs.get.GetReportingReportRuns`
/// - `/v1/reviews.get.GetReviews`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCoupons {
    pub created: Option<UniCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetCoupons {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/credit_notes.get.GetCreditNotes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCreditNotes {
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: Option<String>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetCreditNotes {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/credit_notes/preview.get.GetCreditNotesPreview`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCreditNotesPreview {
    pub amount: Option<i64>,
    pub credit_amount: Option<i64>,
    pub expand: Option<Vec<String>>,
    pub invoice: String,
    pub lines: Option<Vec<CreditNoteLineItemParams>>,
    pub memo: Option<String>,
    pub out_of_band_amount: Option<i64>,
    pub reason: Option<UniStrReasonAB5E91>,
    pub refund: Option<String>,
    pub refund_amount: Option<i64>,
    pub metadata: Option<Metadata>,
}

/// Spec paths:
/// - `/v1/credit_notes/preview.get.GetCreditNotesPreview.lines.items`
/// - `/v1/credit_notes/preview/lines.get.GetCreditNotesPreviewLines.lines.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditNoteLineItemParams {
    #[serde(rename = "type")]
    pub type_x: UniStrType00DF75,
    pub tax_rates: Option<UniTaxRates>,
    pub amount: Option<i64>,
    pub description: Option<String>,
    pub invoice_line_item: Option<String>,
    pub quantity: Option<i64>,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,
}

/// Spec paths:
/// - `/v1/credit_notes/preview.get.GetCreditNotesPreview.lines.items.tax_rates`
/// - `/v1/credit_notes/preview/lines.get.GetCreditNotesPreviewLines.lines.items.tax_rates`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.invoice_items.items.tax_rates`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_default_tax_rates`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_items.items.tax_rates`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.invoice_items.items.tax_rates`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_default_tax_rates`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_items.items.tax_rates`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniTaxRates {
    VecString(Vec<String>),
    UniStr1(UniStr1),
}

/// Spec paths:
/// - `/v1/credit_notes/preview.get.GetCreditNotesPreview.lines.items.tax_rates.anyOf.1`
/// - `/v1/credit_notes/preview/lines.get.GetCreditNotesPreviewLines.lines.items.tax_rates.anyOf.1`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.discounts.anyOf.1`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.invoice_items.items.discounts.anyOf.1`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.invoice_items.items.metadata.anyOf.1`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.invoice_items.items.tax_rates.anyOf.1`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_cancel_at.anyOf.1`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_default_tax_rates.anyOf.1`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_items.items.billing_thresholds.anyOf.1`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_items.items.metadata.anyOf.1`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_items.items.tax_rates.anyOf.1`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.discounts.anyOf.1`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.invoice_items.items.discounts.anyOf.1`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.invoice_items.items.metadata.anyOf.1`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.invoice_items.items.tax_rates.anyOf.1`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_cancel_at.anyOf.1`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_default_tax_rates.anyOf.1`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_items.items.billing_thresholds.anyOf.1`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_items.items.metadata.anyOf.1`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_items.items.tax_rates.anyOf.1`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStr1 {
    #[serde(rename = "")]
    Empty,
}

/// Spec paths:
/// - `/v1/credit_notes/preview.get.GetCreditNotesPreview.metadata`
/// - `/v1/credit_notes/preview/lines.get.GetCreditNotesPreviewLines.metadata`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.invoice_items.items.metadata.anyOf.0`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_items.items.metadata.anyOf.0`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.invoice_items.items.metadata.anyOf.0`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_items.items.metadata.anyOf.0`
pub type Metadata = HashMap<String, String>;

/// Spec paths:
/// - `/v1/credit_notes/preview/lines.get.GetCreditNotesPreviewLines`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCreditNotesPreviewLines {
    pub amount: Option<i64>,
    pub credit_amount: Option<i64>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: String,
    pub limit: Option<i64>,
    pub lines: Option<Vec<CreditNoteLineItemParams>>,
    pub memo: Option<String>,
    pub out_of_band_amount: Option<i64>,
    pub reason: Option<UniStrReasonAB5E91>,
    pub refund: Option<String>,
    pub refund_amount: Option<i64>,
    pub starting_after: Option<String>,
    pub metadata: Option<Metadata>,
}

impl PageParam for GetCreditNotesPreviewLines {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/customers.get.GetCustomers`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCustomers {
    pub created: Option<UniCreated>,
    pub email: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetCustomers {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/customers/{customer}/sources.get.GetCustomersCustomerSources`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCustomersCustomerSources {
    pub object: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetCustomersCustomerSources {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/disputes.get.GetDisputes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDisputes {
    pub created: Option<UniCreated>,
    pub charge: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
}

impl PageParam for GetDisputes {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/events.get.GetEvents`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetEvents {
    #[serde(rename = "type")]
    pub type_x: Option<String>,
    pub created: Option<UniCreated>,
    pub delivery_success: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub types: Option<Vec<String>>,
}

impl PageParam for GetEvents {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/file_links.get.GetFileLinks`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFileLinks {
    pub created: Option<UniCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub expired: Option<bool>,
    pub file: Option<String>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetFileLinks {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/files.get.GetFiles`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFiles {
    pub created: Option<UniCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub purpose: Option<UniStrPurposeA778EA>,
    pub starting_after: Option<String>,
}

impl PageParam for GetFiles {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/files.get.GetFiles.purpose`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrPurposeA778EA {
    #[serde(rename = "additional_verification")]
    AdditionalVerification,
    #[serde(rename = "business_icon")]
    BusinessIcon,
    #[serde(rename = "business_logo")]
    BusinessLogo,
    #[serde(rename = "customer_signature")]
    CustomerSignature,
    #[serde(rename = "dispute_evidence")]
    DisputeEvidence,
    #[serde(rename = "document_provider_identity_document")]
    DocumentProviderIdentityDocument,
    #[serde(rename = "finance_report_run")]
    FinanceReportRun,
    #[serde(rename = "identity_document")]
    IdentityDocument,
    #[serde(rename = "pci_document")]
    PciDocument,
    #[serde(rename = "sigma_scheduled_query")]
    SigmaScheduledQuery,
    #[serde(rename = "tax_document_user_upload")]
    TaxDocumentUserUpload,
}

/// Spec paths:
/// - `/v1/invoiceitems.get.GetInvoiceitems`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetInvoiceitems {
    pub created: Option<UniCreated>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: Option<String>,
    pub limit: Option<i64>,
    pub pending: Option<bool>,
    pub starting_after: Option<String>,
}

impl PageParam for GetInvoiceitems {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/invoices.get.GetInvoices`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetInvoices {
    pub created: Option<UniCreated>,
    pub due_date: Option<UniCreated>,
    pub collection_method: Option<UniStrCollectionMethod>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<UniStrStatus536EDD>,
    pub subscription: Option<String>,
}

impl PageParam for GetInvoices {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/invoices.get.GetInvoices.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatus536EDD {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "uncollectible")]
    Uncollectible,
    #[serde(rename = "void")]
    Void,
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetInvoicesUpcoming {
    pub discounts: Option<UniDiscounts>,
    pub subscription_billing_cycle_anchor: Option<UniSubscriptionBillingCycleAnchor>,
    pub subscription_cancel_at: Option<UniSubscriptionCancelAt>,
    pub subscription_default_tax_rates: Option<UniTaxRates>,
    pub subscription_trial_end: Option<UniSubscriptionTrialEnd>,
    pub coupon: Option<String>,
    pub customer: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice_items: Option<Vec<InvoiceItemPreviewParams>>,
    pub schedule: Option<String>,
    pub subscription: Option<String>,
    pub subscription_cancel_at_period_end: Option<bool>,
    pub subscription_cancel_now: Option<bool>,
    pub subscription_items: Option<Vec<SubscriptionItemUpdateParams>>,
    pub subscription_proration_behavior: Option<UniStrProrationBehavior>,
    pub subscription_proration_date: Option<i64>,
    pub subscription_start_date: Option<i64>,
    pub subscription_trial_from_plan: Option<bool>,
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.discounts`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.invoice_items.items.discounts`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.discounts`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.invoice_items.items.discounts`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniDiscounts {
    VecDiscountsDataParam(Vec<DiscountsDataParam>),
    UniStr1(UniStr1),
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.discounts.anyOf.0.items`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.invoice_items.items.discounts.anyOf.0.items`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.discounts.anyOf.0.items`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.invoice_items.items.discounts.anyOf.0.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscountsDataParam {
    pub coupon: Option<String>,
    pub discount: Option<String>,
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.invoice_items.items`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.invoice_items.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceItemPreviewParams {
    pub discounts: Option<UniDiscounts>,
    pub metadata: Option<UniMetadata>,
    pub tax_rates: Option<UniTaxRates>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub discountable: Option<bool>,
    pub invoiceitem: Option<String>,
    pub period: Option<Period0B71D6>,
    pub price: Option<String>,
    pub price_data: Option<OneTimePriceData>,
    pub quantity: Option<i64>,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.invoice_items.items.metadata`
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_items.items.metadata`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.invoice_items.items.metadata`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_items.items.metadata`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniMetadata {
    Metadata(Metadata),
    UniStr1(UniStr1),
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.invoice_items.items.period`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.invoice_items.items.period`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Period0B71D6 {
    pub end: i64,
    pub start: i64,
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.invoice_items.items.price_data`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.invoice_items.items.price_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OneTimePriceData {
    pub currency: String,
    pub product: String,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_billing_cycle_anchor`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_billing_cycle_anchor`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniSubscriptionBillingCycleAnchor {
    UniStr03014CD(UniStr03014CD),
    I64(i64),
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_billing_cycle_anchor.anyOf.0`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_billing_cycle_anchor.anyOf.0`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStr03014CD {
    #[serde(rename = "now")]
    Now,
    #[serde(rename = "unchanged")]
    Unchanged,
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_cancel_at`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_cancel_at`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniSubscriptionCancelAt {
    I64(i64),
    UniStr1(UniStr1),
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_items.items`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_items.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionItemUpdateParams {
    pub id: Option<String>,
    pub billing_thresholds: Option<UniBillingThresholds>,
    pub metadata: Option<UniMetadata>,
    pub tax_rates: Option<UniTaxRates>,
    pub clear_usage: Option<bool>,
    pub deleted: Option<bool>,
    pub price: Option<String>,
    pub price_data: Option<RecurringPriceData>,
    pub quantity: Option<i64>,
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_items.items.billing_thresholds`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_items.items.billing_thresholds`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniBillingThresholds {
    ItemBillingThresholdsParam(ItemBillingThresholdsParam),
    UniStr1(UniStr1),
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_items.items.billing_thresholds.anyOf.0`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_items.items.billing_thresholds.anyOf.0`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemBillingThresholdsParam {
    pub usage_gte: i64,
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_items.items.price_data`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_items.items.price_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecurringPriceData {
    pub currency: String,
    pub product: String,
    pub recurring: RecurringAdhoc,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_items.items.price_data.recurring`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_items.items.price_data.recurring`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecurringAdhoc {
    pub interval: UniStrInterval,
    pub interval_count: Option<i64>,
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_trial_end`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_trial_end`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniSubscriptionTrialEnd {
    UniStr0104DCB(UniStr0104DCB),
    I64(i64),
}

/// Spec paths:
/// - `/v1/invoices/upcoming.get.GetInvoicesUpcoming.subscription_trial_end.anyOf.0`
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines.subscription_trial_end.anyOf.0`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStr0104DCB {
    #[serde(rename = "now")]
    Now,
}

/// Spec paths:
/// - `/v1/invoices/upcoming/lines.get.GetInvoicesUpcomingLines`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetInvoicesUpcomingLines {
    pub discounts: Option<UniDiscounts>,
    pub subscription_billing_cycle_anchor: Option<UniSubscriptionBillingCycleAnchor>,
    pub subscription_cancel_at: Option<UniSubscriptionCancelAt>,
    pub subscription_default_tax_rates: Option<UniTaxRates>,
    pub subscription_trial_end: Option<UniSubscriptionTrialEnd>,
    pub coupon: Option<String>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice_items: Option<Vec<InvoiceItemPreviewParams>>,
    pub limit: Option<i64>,
    pub schedule: Option<String>,
    pub starting_after: Option<String>,
    pub subscription: Option<String>,
    pub subscription_cancel_at_period_end: Option<bool>,
    pub subscription_cancel_now: Option<bool>,
    pub subscription_items: Option<Vec<SubscriptionItemUpdateParams>>,
    pub subscription_proration_behavior: Option<UniStrProrationBehavior>,
    pub subscription_proration_date: Option<i64>,
    pub subscription_start_date: Option<i64>,
    pub subscription_trial_from_plan: Option<bool>,
}

impl PageParam for GetInvoicesUpcomingLines {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/issuer_fraud_records.get.GetIssuerFraudRecords`
/// - `/v1/radar/early_fraud_warnings.get.GetRadarEarlyFraudWarnings`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIssuerFraudRecords {
    pub charge: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetIssuerFraudRecords {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/issuing/authorizations.get.GetIssuingAuthorizations`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIssuingAuthorizations {
    pub created: Option<UniCreated>,
    pub card: Option<String>,
    pub cardholder: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<UniStrStatus957169>,
}

impl PageParam for GetIssuingAuthorizations {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/issuing/cardholders.get.GetIssuingCardholders`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIssuingCardholders {
    #[serde(rename = "type")]
    pub type_x: Option<UniStrType947A77>,
    pub created: Option<UniCreated>,
    pub email: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub phone_number: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<UniStrStatusD5D208>,
}

impl PageParam for GetIssuingCardholders {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/issuing/cards.get.GetIssuingCards`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIssuingCards {
    #[serde(rename = "type")]
    pub type_x: Option<UniStrTypeA467AF>,
    pub created: Option<UniCreated>,
    pub cardholder: Option<String>,
    pub ending_before: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub expand: Option<Vec<String>>,
    pub last4: Option<String>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<UniStrStatusA4138B>,
}

impl PageParam for GetIssuingCards {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/issuing/disputes.get.GetIssuingDisputes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIssuingDisputes {
    pub created: Option<UniCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<UniStrStatusE71251>,
    pub transaction: Option<String>,
}

impl PageParam for GetIssuingDisputes {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/issuing/transactions.get.GetIssuingTransactions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIssuingTransactions {
    pub created: Option<UniCreated>,
    pub card: Option<String>,
    pub cardholder: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetIssuingTransactions {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/order_returns.get.GetOrderReturns`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetOrderReturns {
    pub created: Option<UniCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub order: Option<String>,
    pub starting_after: Option<String>,
}

impl PageParam for GetOrderReturns {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/orders.get.GetOrders`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetOrders {
    pub created: Option<UniCreated>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub ids: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub status_transitions: Option<OrderTimestampSpecs>,
    pub upstream_ids: Option<Vec<String>>,
}

impl PageParam for GetOrders {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/orders.get.GetOrders.status_transitions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderTimestampSpecs {
    pub paid: Option<UniCreated>,
    pub canceled: Option<UniCreated>,
    pub fulfilled: Option<UniCreated>,
    pub returned: Option<UniCreated>,
}

/// Spec paths:
/// - `/v1/payment_intents.get.GetPaymentIntents`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPaymentIntents {
    pub created: Option<UniCreated>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetPaymentIntents {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/payment_intents/{intent}.get.GetPaymentIntentsIntent`
/// - `/v1/setup_intents/{intent}.get.GetSetupIntentsIntent`
/// - `/v1/sources/{source}.get.GetSourcesSource`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSourcesSource {
    pub client_secret: Option<String>,
    pub expand: Option<Vec<String>>,
}

/// Spec paths:
/// - `/v1/payment_methods.get.GetPaymentMethods`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPaymentMethods {
    #[serde(rename = "type")]
    pub type_x: UniStrTypeBAE85E,
    pub customer: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetPaymentMethods {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/payment_methods.get.GetPaymentMethods.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTypeBAE85E {
    #[serde(rename = "alipay")]
    Alipay,
    #[serde(rename = "au_becs_debit")]
    AuBecsDebit,
    #[serde(rename = "bacs_debit")]
    BacsDebit,
    #[serde(rename = "bancontact")]
    Bancontact,
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "eps")]
    Eps,
    #[serde(rename = "fpx")]
    Fpx,
    #[serde(rename = "giropay")]
    Giropay,
    #[serde(rename = "grabpay")]
    Grabpay,
    #[serde(rename = "ideal")]
    Ideal,
    #[serde(rename = "oxxo")]
    Oxxo,
    #[serde(rename = "p24")]
    P24,
    #[serde(rename = "sepa_debit")]
    SepaDebit,
    #[serde(rename = "sofort")]
    Sofort,
}

/// Spec paths:
/// - `/v1/payouts.get.GetPayouts`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPayouts {
    pub arrival_date: Option<UniCreated>,
    pub created: Option<UniCreated>,
    pub destination: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}

impl PageParam for GetPayouts {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/plans.get.GetPlans`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPlans {
    pub created: Option<UniCreated>,
    pub active: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub product: Option<String>,
    pub starting_after: Option<String>,
}

impl PageParam for GetPlans {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/prices.get.GetPrices`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPrices {
    #[serde(rename = "type")]
    pub type_x: Option<UniStrTypeFC33AE>,
    pub created: Option<UniCreated>,
    pub active: Option<bool>,
    pub currency: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub lookup_keys: Option<Vec<String>>,
    pub product: Option<String>,
    pub recurring: Option<AllPricesRecurringParams>,
    pub starting_after: Option<String>,
}

impl PageParam for GetPrices {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/prices.get.GetPrices.recurring`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllPricesRecurringParams {
    pub interval: Option<UniStrInterval>,
    pub usage_type: Option<UniStrUsageType>,
}

/// Spec paths:
/// - `/v1/products.get.GetProducts`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProducts {
    pub created: Option<UniCreated>,
    pub active: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub ids: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub shippable: Option<bool>,
    pub starting_after: Option<String>,
    pub url: Option<String>,
}

impl PageParam for GetProducts {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/promotion_codes.get.GetPromotionCodes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPromotionCodes {
    pub created: Option<UniCreated>,
    pub active: Option<bool>,
    pub code: Option<String>,
    pub coupon: Option<String>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetPromotionCodes {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/radar/value_list_items.get.GetRadarValueListItems`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRadarValueListItems {
    pub created: Option<UniCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub value: Option<String>,
    pub value_list: String,
}

impl PageParam for GetRadarValueListItems {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/radar/value_lists.get.GetRadarValueLists`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRadarValueLists {
    pub created: Option<UniCreated>,
    pub alias: Option<String>,
    pub contains: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetRadarValueLists {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/recipients.get.GetRecipients`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRecipients {
    #[serde(rename = "type")]
    pub type_x: Option<UniStrType>,
    pub created: Option<UniCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub verified: Option<bool>,
}

impl PageParam for GetRecipients {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/recipients.get.GetRecipients.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrType {
    #[serde(rename = "corporation")]
    Corporation,
    #[serde(rename = "individual")]
    Individual,
}

/// Spec paths:
/// - `/v1/refunds.get.GetRefunds`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRefunds {
    pub created: Option<UniCreated>,
    pub charge: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
}

impl PageParam for GetRefunds {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/setup_attempts.get.GetSetupAttempts`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSetupAttempts {
    pub created: Option<UniCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub setup_intent: String,
    pub starting_after: Option<String>,
}

impl PageParam for GetSetupAttempts {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/setup_intents.get.GetSetupIntents`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSetupIntents {
    pub created: Option<UniCreated>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_method: Option<String>,
    pub starting_after: Option<String>,
}

impl PageParam for GetSetupIntents {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/skus.get.GetSkus`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSkus {
    pub active: Option<bool>,
    pub attributes: Option<AttributesD48725>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub ids: Option<Vec<String>>,
    pub in_stock: Option<bool>,
    pub limit: Option<i64>,
    pub product: Option<String>,
    pub starting_after: Option<String>,
}

impl PageParam for GetSkus {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/skus.get.GetSkus.attributes`
pub type AttributesD48725 = HashMap<String, String>;

/// Spec paths:
/// - `/v1/subscription_items.get.GetSubscriptionItems`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSubscriptionItems {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub subscription: String,
}

impl PageParam for GetSubscriptionItems {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/subscription_schedules.get.GetSubscriptionSchedules`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSubscriptionSchedules {
    pub canceled_at: Option<UniCreated>,
    pub completed_at: Option<UniCreated>,
    pub created: Option<UniCreated>,
    pub released_at: Option<UniCreated>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub scheduled: Option<bool>,
    pub starting_after: Option<String>,
}

impl PageParam for GetSubscriptionSchedules {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/subscriptions.get.GetSubscriptions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSubscriptions {
    pub created: Option<UniCreated>,
    pub current_period_end: Option<UniCreated>,
    pub current_period_start: Option<UniCreated>,
    pub collection_method: Option<UniStrCollectionMethod>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub price: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<UniStrStatus3EB683>,
}

impl PageParam for GetSubscriptions {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/subscriptions.get.GetSubscriptions.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatus3EB683 {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "ended")]
    Ended,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "incomplete_expired")]
    IncompleteExpired,
    #[serde(rename = "past_due")]
    PastDue,
    #[serde(rename = "trialing")]
    Trialing,
    #[serde(rename = "unpaid")]
    Unpaid,
}

/// Spec paths:
/// - `/v1/tax_rates.get.GetTaxRates`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTaxRates {
    pub created: Option<UniCreated>,
    pub active: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub inclusive: Option<bool>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

impl PageParam for GetTaxRates {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/terminal/readers.get.GetTerminalReaders`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTerminalReaders {
    pub device_type: Option<UniStrDeviceType>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub location: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<UniStrTypeD153A1>,
}

impl PageParam for GetTerminalReaders {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/topups.get.GetTopups`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTopups {
    pub amount: Option<UniCreated>,
    pub created: Option<UniCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<UniStrStatus>,
}

impl PageParam for GetTopups {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}

/// Spec paths:
/// - `/v1/topups.get.GetTopups.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatus {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "succeeded")]
    Succeeded,
}

/// Spec paths:
/// - `/v1/transfers.get.GetTransfers`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTransfers {
    pub created: Option<UniCreated>,
    pub destination: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub transfer_group: Option<String>,
}

impl PageParam for GetTransfers {
    fn set_before(&mut self, s: String) {
        self.ending_before = Some(s);
        self.starting_after = None;
    }

    fn set_after(&mut self, s: String) {
        self.starting_after = Some(s);
        self.ending_before = None;
    }
}
