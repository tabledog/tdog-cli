
#![allow(warnings)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::req_params::*;
use super::types::*;

/// Spec paths:
/// - `account.external_accounts.data.items`
/// - `external_account`
/// - `/v1/account/external_accounts.get.200.GetAccountExternalAccountsRes.data.items`
/// - `/v1/accounts/{account}/external_accounts.get.200.GetAccountsAccountExternalAccountsRes.data.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniPolymorphic70BAFA {
    BankAccount(BankAccount),
    Card(Box<Card>),
}

/// Spec paths:
/// - `account.external_accounts.object`
/// - `application_fee.refunds.object`
/// - `bitcoin_receiver.transactions.object`
/// - `charge.refunds.object`
/// - `checkout.session.line_items.object`
/// - `credit_note.lines.object`
/// - `customer.sources.object`
/// - `customer.subscriptions.object`
/// - `customer.tax_ids.object`
/// - `file.links.object`
/// - `invoice.lines.object`
/// - `order.returns.object`
/// - `payment_intent.charges.object`
/// - `radar.value_list.list_items.object`
/// - `recipient.cards.object`
/// - `subscription.items.object`
/// - `transfer.reversals.object`
/// - `/v1/account/capabilities.get.200.GetAccountCapabilitiesRes.object`
/// - `/v1/account/external_accounts.get.200.GetAccountExternalAccountsRes.object`
/// - `/v1/account/people.get.200.GetAccountPeopleRes.object`
/// - `/v1/account/persons.get.200.GetAccountPersonsRes.object`
/// - `/v1/accounts.get.200.GetAccountsRes.object`
/// - `/v1/accounts/{account}/capabilities.get.200.GetAccountsAccountCapabilitiesRes.object`
/// - `/v1/accounts/{account}/external_accounts.get.200.GetAccountsAccountExternalAccountsRes.object`
/// - `/v1/accounts/{account}/people.get.200.GetAccountsAccountPeopleRes.object`
/// - `/v1/accounts/{account}/persons.get.200.GetAccountsAccountPersonsRes.object`
/// - `/v1/apple_pay/domains.get.200.GetApplePayDomainsRes.object`
/// - `/v1/application_fees.get.200.GetApplicationFeesRes.object`
/// - `/v1/application_fees/{id}/refunds.get.200.GetApplicationFeesIdRefundsRes.object`
/// - `/v1/balance/history.get.200.GetBalanceHistoryRes.object`
/// - `/v1/balance_transactions.get.200.GetBalanceTransactionsRes.object`
/// - `/v1/bitcoin/receivers.get.200.GetBitcoinReceiversRes.object`
/// - `/v1/bitcoin/receivers/{receiver}/transactions.get.200.GetBitcoinReceiversReceiverTransactionsRes.object`
/// - `/v1/bitcoin/transactions.get.200.GetBitcoinTransactionsRes.object`
/// - `/v1/charges.get.200.GetChargesRes.object`
/// - `/v1/charges/{charge}/refunds.get.200.GetChargesChargeRefundsRes.object`
/// - `/v1/checkout/sessions.get.200.GetCheckoutSessionsRes.object`
/// - `/v1/checkout/sessions/{session}/line_items.get.200.GetCheckoutSessionsSessionLineItemsRes.object`
/// - `/v1/country_specs.get.200.GetCountrySpecsRes.object`
/// - `/v1/coupons.get.200.GetCouponsRes.object`
/// - `/v1/credit_notes.get.200.GetCreditNotesRes.object`
/// - `/v1/credit_notes/preview/lines.get.200.GetCreditNotesPreviewLinesRes.object`
/// - `/v1/credit_notes/{credit_note}/lines.get.200.GetCreditNotesCreditNoteLinesRes.object`
/// - `/v1/customers.get.200.GetCustomersRes.object`
/// - `/v1/customers/{customer}/balance_transactions.get.200.GetCustomersCustomerBalanceTransactionsRes.object`
/// - `/v1/customers/{customer}/bank_accounts.get.200.GetCustomersCustomerBankAccountsRes.object`
/// - `/v1/customers/{customer}/cards.get.200.GetCustomersCustomerCardsRes.object`
/// - `/v1/customers/{customer}/sources.get.200.GetCustomersCustomerSourcesRes.object`
/// - `/v1/customers/{customer}/subscriptions.get.200.GetCustomersCustomerSubscriptionsRes.object`
/// - `/v1/customers/{customer}/tax_ids.get.200.GetCustomersCustomerTaxIdsRes.object`
/// - `/v1/disputes.get.200.GetDisputesRes.object`
/// - `/v1/events.get.200.GetEventsRes.object`
/// - `/v1/exchange_rates.get.200.GetExchangeRatesRes.object`
/// - `/v1/file_links.get.200.GetFileLinksRes.object`
/// - `/v1/files.get.200.GetFilesRes.object`
/// - `/v1/invoiceitems.get.200.GetInvoiceitemsRes.object`
/// - `/v1/invoices.get.200.GetInvoicesRes.object`
/// - `/v1/invoices/upcoming/lines.get.200.GetInvoicesUpcomingLinesRes.object`
/// - `/v1/invoices/{invoice}/lines.get.200.GetInvoicesInvoiceLinesRes.object`
/// - `/v1/issuer_fraud_records.get.200.GetIssuerFraudRecordsRes.object`
/// - `/v1/issuing/authorizations.get.200.GetIssuingAuthorizationsRes.object`
/// - `/v1/issuing/cardholders.get.200.GetIssuingCardholdersRes.object`
/// - `/v1/issuing/cards.get.200.GetIssuingCardsRes.object`
/// - `/v1/issuing/disputes.get.200.GetIssuingDisputesRes.object`
/// - `/v1/issuing/settlements.get.200.GetIssuingSettlementsRes.object`
/// - `/v1/issuing/transactions.get.200.GetIssuingTransactionsRes.object`
/// - `/v1/order_returns.get.200.GetOrderReturnsRes.object`
/// - `/v1/orders.get.200.GetOrdersRes.object`
/// - `/v1/payment_intents.get.200.GetPaymentIntentsRes.object`
/// - `/v1/payment_methods.get.200.GetPaymentMethodsRes.object`
/// - `/v1/payouts.get.200.GetPayoutsRes.object`
/// - `/v1/plans.get.200.GetPlansRes.object`
/// - `/v1/prices.get.200.GetPricesRes.object`
/// - `/v1/products.get.200.GetProductsRes.object`
/// - `/v1/promotion_codes.get.200.GetPromotionCodesRes.object`
/// - `/v1/radar/early_fraud_warnings.get.200.GetRadarEarlyFraudWarningsRes.object`
/// - `/v1/radar/value_list_items.get.200.GetRadarValueListItemsRes.object`
/// - `/v1/radar/value_lists.get.200.GetRadarValueListsRes.object`
/// - `/v1/recipients.get.200.GetRecipientsRes.object`
/// - `/v1/refunds.get.200.GetRefundsRes.object`
/// - `/v1/reporting/report_runs.get.200.GetReportingReportRunsRes.object`
/// - `/v1/reporting/report_types.get.200.GetReportingReportTypesRes.object`
/// - `/v1/reviews.get.200.GetReviewsRes.object`
/// - `/v1/setup_attempts.get.200.GetSetupAttemptsRes.object`
/// - `/v1/setup_intents.get.200.GetSetupIntentsRes.object`
/// - `/v1/sigma/scheduled_query_runs.get.200.GetSigmaScheduledQueryRunsRes.object`
/// - `/v1/skus.get.200.GetSkusRes.object`
/// - `/v1/sources/{source}/source_transactions.get.200.GetSourcesSourceSourceTransactionsRes.object`
/// - `/v1/subscription_items.get.200.GetSubscriptionItemsRes.object`
/// - `/v1/subscription_items/{subscription_item}/usage_record_summaries.get.200.GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRes.object`
/// - `/v1/subscription_schedules.get.200.GetSubscriptionSchedulesRes.object`
/// - `/v1/subscriptions.get.200.GetSubscriptionsRes.object`
/// - `/v1/tax_rates.get.200.GetTaxRatesRes.object`
/// - `/v1/terminal/locations.get.200.GetTerminalLocationsRes.object`
/// - `/v1/terminal/readers.get.200.GetTerminalReadersRes.object`
/// - `/v1/topups.get.200.GetTopupsRes.object`
/// - `/v1/transfers.get.200.GetTransfersRes.object`
/// - `/v1/transfers/{id}/reversals.get.200.GetTransfersIdReversalsRes.object`
/// - `/v1/webhook_endpoints.get.200.GetWebhookEndpointsRes.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject344B0E {
    #[serde(rename = "list")]
    List,
}

/// Spec paths:
/// - `customer.sources.data.items`
/// - `/v1/customers/{customer}/sources.get.200.GetCustomersCustomerSourcesRes.data.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniPolymorphic646C3F {
    AlipayAccount(AlipayAccount),
    BankAccount(BankAccount),
    BitcoinReceiver(BitcoinReceiver),
    Card(Box<Card>),
    Source(Source),
}

/// Spec paths:
/// - `/v1/account/capabilities.get.200.GetAccountCapabilitiesRes`
/// - `/v1/accounts/{account}/capabilities.get.200.GetAccountsAccountCapabilitiesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListAccountCapability {
    pub object: UniStrObject344B0E,
    pub data: Vec<AccountCapability>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for ListAccountCapability {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/account/external_accounts.get.200.GetAccountExternalAccountsRes`
/// - `/v1/accounts/{account}/external_accounts.get.200.GetAccountsAccountExternalAccountsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalAccountListADE54B {
    pub object: UniStrObject344B0E,
    pub data: Vec<UniPolymorphic70BAFA>,
    pub has_more: bool,
    pub url: String,
}

/// Spec paths:
/// - `/v1/account/people.get.200.GetAccountPeopleRes`
/// - `/v1/account/persons.get.200.GetAccountPersonsRes`
/// - `/v1/accounts/{account}/people.get.200.GetAccountsAccountPeopleRes`
/// - `/v1/accounts/{account}/persons.get.200.GetAccountsAccountPersonsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountPeopleRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<Person>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetAccountPeopleRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/accounts.get.200.GetAccountsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<Account>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetAccountsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/apple_pay/domains.get.200.GetApplePayDomainsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplePayDomainList {
    pub object: UniStrObject344B0E,
    pub data: Vec<ApplePayDomain>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for ApplePayDomainList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/application_fees.get.200.GetApplicationFeesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetApplicationFeesRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<PlatformFee>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetApplicationFeesRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/application_fees/{id}/refunds.get.200.GetApplicationFeesIdRefundsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeeRefundListFDC0D1 {
    pub object: UniStrObject344B0E,
    pub data: Vec<FeeRefund>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for FeeRefundListFDC0D1 {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/balance/history.get.200.GetBalanceHistoryRes`
/// - `/v1/balance_transactions.get.200.GetBalanceTransactionsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalanceTransactionsList {
    pub object: UniStrObject344B0E,
    pub data: Vec<BalanceTransaction>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for BalanceTransactionsList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/bitcoin/receivers.get.200.GetBitcoinReceiversRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBitcoinReceiversRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<BitcoinReceiver>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetBitcoinReceiversRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/bitcoin/receivers/{receiver}/transactions.get.200.GetBitcoinReceiversReceiverTransactionsRes`
/// - `/v1/bitcoin/transactions.get.200.GetBitcoinTransactionsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BitcoinTransactionListC3C538 {
    pub object: UniStrObject344B0E,
    pub data: Vec<BitcoinTransaction>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for BitcoinTransactionListC3C538 {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/charges.get.200.GetChargesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetChargesRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<Charge>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetChargesRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/charges/{charge}/refunds.get.200.GetChargesChargeRefundsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefundListBBCF51 {
    pub object: UniStrObject344B0E,
    pub data: Vec<Refund>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for RefundListBBCF51 {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/checkout/sessions.get.200.GetCheckoutSessionsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentPagesCheckoutSessionList {
    pub object: UniStrObject344B0E,
    pub data: Vec<Session>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for PaymentPagesCheckoutSessionList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/checkout/sessions/{session}/line_items.get.200.GetCheckoutSessionsSessionLineItemsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentPagesCheckoutSessionListLineItems404A63 {
    pub object: UniStrObject344B0E,
    pub data: Vec<LineItems>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for PaymentPagesCheckoutSessionListLineItems404A63 {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/country_specs.get.200.GetCountrySpecsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCountrySpecsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<CountrySpec>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetCountrySpecsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/coupons.get.200.GetCouponsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCouponsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<Coupon>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetCouponsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/credit_notes.get.200.GetCreditNotesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditNotesList {
    pub object: UniStrObject344B0E,
    pub data: Vec<CreditNote>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for CreditNotesList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/credit_notes/preview/lines.get.200.GetCreditNotesPreviewLinesRes`
/// - `/v1/credit_notes/{credit_note}/lines.get.200.GetCreditNotesCreditNoteLinesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditNoteLinesList34EE1C {
    pub object: UniStrObject344B0E,
    pub data: Vec<CreditNoteLineItem>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for CreditNoteLinesList34EE1C {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/customers.get.200.GetCustomersRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCustomersRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<Customer>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetCustomersRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/customers/{customer}.get.200.GetCustomersCustomerRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniGetCustomersCustomerRes {
    Customer(Box<Customer>),
    DeletedCustomer(DeletedCustomer),
}

/// Spec paths:
/// - `/v1/customers/{customer}/balance_transactions.get.200.GetCustomersCustomerBalanceTransactionsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerBalanceTransactionList {
    pub object: UniStrObject344B0E,
    pub data: Vec<CustomerBalanceTransaction>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for CustomerBalanceTransactionList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/customers/{customer}/bank_accounts.get.200.GetCustomersCustomerBankAccountsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BankAccountList {
    pub object: UniStrObject344B0E,
    pub data: Vec<BankAccount>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for BankAccountList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/customers/{customer}/cards.get.200.GetCustomersCustomerCardsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardList81180B {
    pub object: UniStrObject344B0E,
    pub data: Vec<Card>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for CardList81180B {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/customers/{customer}/sources.get.200.GetCustomersCustomerSourcesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApmsSourcesSourceListF0771E {
    pub object: UniStrObject344B0E,
    pub data: Vec<UniPolymorphic646C3F>,
    pub has_more: bool,
    pub url: String,
}

/// Spec paths:
/// - `/v1/customers/{customer}/subscriptions.get.200.GetCustomersCustomerSubscriptionsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionList5B5899 {
    pub object: UniStrObject344B0E,
    pub data: Vec<Subscription>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for SubscriptionList5B5899 {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/customers/{customer}/tax_ids.get.200.GetCustomersCustomerTaxIdsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxIDsListAFDA6E {
    pub object: UniStrObject344B0E,
    pub data: Vec<TaxId>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for TaxIDsListAFDA6E {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/disputes.get.200.GetDisputesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDisputesRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<Dispute>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetDisputesRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/events.get.200.GetEventsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationEventList {
    pub object: UniStrObject344B0E,
    pub data: Vec<NotificationEvent>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for NotificationEventList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/exchange_rates.get.200.GetExchangeRatesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetExchangeRatesRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<ExchangeRate>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetExchangeRatesRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/file_links.get.200.GetFileLinksRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFileLinksRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<FileLink>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetFileLinksRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/files.get.200.GetFilesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFilesRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<File>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetFilesRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/invoiceitems.get.200.GetInvoiceitemsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetInvoiceitemsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<InvoiceItem>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetInvoiceitemsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/invoices.get.200.GetInvoicesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoicesList {
    pub object: UniStrObject344B0E,
    pub data: Vec<Invoice>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for InvoicesList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.as_ref().unwrap().clone(),
            self.data.last().unwrap().id.as_ref().unwrap().clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/invoices/upcoming/lines.get.200.GetInvoicesUpcomingLinesRes`
/// - `/v1/invoices/{invoice}/lines.get.200.GetInvoicesInvoiceLinesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceLinesList9B8534 {
    pub object: UniStrObject344B0E,
    pub data: Vec<InvoiceLineItem>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for InvoiceLinesList9B8534 {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/issuer_fraud_records.get.200.GetIssuerFraudRecordsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadarIssuerFraudRecordList {
    pub object: UniStrObject344B0E,
    pub data: Vec<IssuerFraudRecord>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for RadarIssuerFraudRecordList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/issuing/authorizations.get.200.GetIssuingAuthorizationsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIssuingAuthorizationsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<IssuingAuthorization>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetIssuingAuthorizationsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/issuing/cardholders.get.200.GetIssuingCardholdersRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIssuingCardholdersRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<IssuingCardholder>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetIssuingCardholdersRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/issuing/cards.get.200.GetIssuingCardsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIssuingCardsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<IssuingCard>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetIssuingCardsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/issuing/disputes.get.200.GetIssuingDisputesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingDisputeList {
    pub object: UniStrObject344B0E,
    pub data: Vec<IssuingDispute>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for IssuingDisputeList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/issuing/settlements.get.200.GetIssuingSettlementsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIssuingSettlementsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<IssuingSettlement>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetIssuingSettlementsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/issuing/transactions.get.200.GetIssuingTransactionsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetIssuingTransactionsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<IssuingTransaction>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetIssuingTransactionsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/order_returns.get.200.GetOrderReturnsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetOrderReturnsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<OrderReturn>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetOrderReturnsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/orders.get.200.GetOrdersRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetOrdersRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<Order>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetOrdersRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/payment_intents.get.200.GetPaymentIntentsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentFlowsPaymentIntentList {
    pub object: UniStrObject344B0E,
    pub data: Vec<PaymentIntent>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for PaymentFlowsPaymentIntentList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/payment_methods.get.200.GetPaymentMethodsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentFlowsPaymentMethodList {
    pub object: UniStrObject344B0E,
    pub data: Vec<PaymentMethod>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for PaymentFlowsPaymentMethodList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/payouts.get.200.GetPayoutsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayoutList {
    pub object: UniStrObject344B0E,
    pub data: Vec<Payout>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for PayoutList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/plans.get.200.GetPlansRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlanList {
    pub object: UniStrObject344B0E,
    pub data: Vec<Plan>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for PlanList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/prices.get.200.GetPricesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceList {
    pub object: UniStrObject344B0E,
    pub data: Vec<Price>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for PriceList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/products.get.200.GetProductsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProductsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<Product>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetProductsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/promotion_codes.get.200.GetPromotionCodesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPromotionCodesRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<PromotionCode>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetPromotionCodesRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/radar/early_fraud_warnings.get.200.GetRadarEarlyFraudWarningsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadarEarlyFraudWarningList {
    pub object: UniStrObject344B0E,
    pub data: Vec<RadarEarlyFraudWarning>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for RadarEarlyFraudWarningList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/radar/value_list_items.get.200.GetRadarValueListItemsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRadarValueListItemsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<RadarListListItem>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetRadarValueListItemsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/radar/value_lists.get.200.GetRadarValueListsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRadarValueListsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<RadarListList>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetRadarValueListsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/recipients.get.200.GetRecipientsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRecipientsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<TransferRecipient>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetRecipientsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/recipients/{id}.get.200.GetRecipientsIdRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniGetRecipientsIdRes {
    TransferRecipient(TransferRecipient),
    DeletedTransferRecipient(DeletedTransferRecipient),
}

/// Spec paths:
/// - `/v1/refunds.get.200.GetRefundsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRefundsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<Refund>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetRefundsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/reporting/report_runs.get.200.GetReportingReportRunsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetReportingReportRunsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<ReportingReportRun>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetReportingReportRunsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/reporting/report_types.get.200.GetReportingReportTypesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinancialReportingFinanceReportTypeList {
    pub object: UniStrObject344B0E,
    pub data: Vec<ReportingReportType>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for FinancialReportingFinanceReportTypeList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/reviews.get.200.GetReviewsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetReviewsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<RadarReview>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetReviewsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/setup_attempts.get.200.GetSetupAttemptsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentFlowsSetupIntentSetupAttemptList {
    pub object: UniStrObject344B0E,
    pub data: Vec<PaymentFlowsSetupIntentSetupAttempt>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for PaymentFlowsSetupIntentSetupAttemptList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/setup_intents.get.200.GetSetupIntentsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentFlowsSetupIntentList {
    pub object: UniStrObject344B0E,
    pub data: Vec<SetupIntent>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for PaymentFlowsSetupIntentList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/sigma/scheduled_query_runs.get.200.GetSigmaScheduledQueryRunsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSigmaScheduledQueryRunsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<ScheduledQueryRun>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetSigmaScheduledQueryRunsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/skus.get.200.GetSkusRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSkusRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<Sku>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetSkusRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/skus/{id}.get.200.GetSkusIdRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniGetSkusIdRes {
    Sku(Sku),
    DeletedSku(DeletedSku),
}

/// Spec paths:
/// - `/v1/sources/{source}/source_transactions.get.200.GetSourcesSourceSourceTransactionsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApmsSourcesSourceTransactionList {
    pub object: UniStrObject344B0E,
    pub data: Vec<SourceTransaction>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for ApmsSourcesSourceTransactionList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/subscription_items.get.200.GetSubscriptionItemsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSubscriptionItemsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<SubscriptionItem>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetSubscriptionItemsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/subscription_items/{subscription_item}/usage_record_summaries.get.200.GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<UsageRecordSummary>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/subscription_schedules.get.200.GetSubscriptionSchedulesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSubscriptionSchedulesRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<SubscriptionSchedule>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetSubscriptionSchedulesRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/subscriptions.get.200.GetSubscriptionsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSubscriptionsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<Subscription>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetSubscriptionsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/tax_rates.get.200.GetTaxRatesRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTaxRatesRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<TaxRate>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetTaxRatesRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/terminal/locations.get.200.GetTerminalLocationsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerminalLocationLocationList {
    pub object: UniStrObject344B0E,
    pub data: Vec<TerminalLocationLocation>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for TerminalLocationLocationList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/terminal/readers.get.200.GetTerminalReadersRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerminalReaderRetrieveReader {
    pub object: UniStrObject344B0E,
    pub data: Vec<TerminalReaderReader>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for TerminalReaderRetrieveReader {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/topups.get.200.GetTopupsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopupList {
    pub object: UniStrObject344B0E,
    pub data: Vec<Topup>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for TopupList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/transfers.get.200.GetTransfersRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferList {
    pub object: UniStrObject344B0E,
    pub data: Vec<Transfer>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for TransferList {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/transfers/{id}/reversals.get.200.GetTransfersIdReversalsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferReversalList620BF1 {
    pub object: UniStrObject344B0E,
    pub data: Vec<TransferReversal>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for TransferReversalList620BF1 {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}

/// Spec paths:
/// - `/v1/webhook_endpoints.get.200.GetWebhookEndpointsRes`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetWebhookEndpointsRes {
    pub object: UniStrObject344B0E,
    pub data: Vec<NotificationWebhookEndpoint>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for GetWebhookEndpointsRes {
    fn get_from_to(&self) -> Option<(String, String)> {
        if self.data.len() == 0 {
            return None;
        }
        Some((
            self.data.first().unwrap().id.clone(),
            self.data.last().unwrap().id.clone(),
        ))
    }
    fn get_has_more(&self) -> bool {
        self.has_more
    }
}
