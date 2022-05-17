
#![allow(warnings)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::req_params::*;
use super::responses::*;

pub trait GetId {
    fn get_id(&self) -> String;
}

pub trait SelfTag {
    /// Serde's internally tagged enums, but where the tag is actually part of the struct (is not JSON only data).
    /// Returns the type of object as named in the external HTTP API interface.
    fn get_tag() -> &'static str;
}

pub trait PageMeta {
    fn get_from_to(&self) -> Option<(String, String)>;
    fn get_has_more(&self) -> bool;
}

pub trait PageParam {
    fn set_before(&mut self, s: String);
    fn set_after(&mut self, s: String);
}

/// Spec paths:
/// - `account`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub object: UniStrObject9D4B89,
    #[serde(rename = "type")]
    pub type_x: Option<UniStrType3680CD>,
    pub id: String,
    pub business_profile: Option<AccountBusinessProfile>,
    pub business_type: Option<UniStrBusinessType>,
    pub capabilities: Option<AccountCapabilities>,
    pub charges_enabled: Option<bool>,
    pub company: Option<LegalEntityCompany>,
    pub country: Option<String>,
    pub default_currency: Option<String>,
    pub details_submitted: Option<bool>,
    pub email: Option<String>,
    pub external_accounts: Option<ExternalAccountList0165E1>,
    pub individual: Option<Person>,
    pub payouts_enabled: Option<bool>,
    pub requirements: Option<AccountRequirements>,
    pub settings: Option<AccountSettings>,
    pub tos_acceptance: Option<AccountTOSAcceptance>,
    pub created: Option<i64>,
    pub metadata: Option<Metadata8076DB>,
}

impl GetId for Account {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `account.business_type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrBusinessType {
    #[serde(rename = "company")]
    Company,
    #[serde(rename = "government_entity")]
    GovernmentEntity,
    #[serde(rename = "individual")]
    Individual,
    #[serde(rename = "non_profit")]
    NonProfit,
}

/// Spec paths:
/// - `account.external_accounts`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalAccountList0165E1 {
    pub object: UniStrObject344B0E,
    pub data: Vec<UniPolymorphic70BAFA>,
    pub has_more: bool,
    pub url: String,
}

/// Spec paths:
/// - `account.metadata`
/// - `alipay_account.metadata`
/// - `charge.metadata`
/// - `customer.metadata`
/// - `dispute.metadata`
/// - `file_link.metadata`
/// - `issuing.authorization.metadata`
/// - `issuing.card.metadata`
/// - `issuing.cardholder.metadata`
/// - `issuing.dispute.metadata`
/// - `issuing.settlement.metadata`
/// - `issuing.transaction.metadata`
/// - `person.metadata`
/// - `price.metadata`
/// - `product.metadata`
/// - `radar.value_list.metadata`
/// - `recipient.metadata`
/// - `sku.metadata`
/// - `subscription.metadata`
/// - `subscription_item.metadata`
/// - `terminal.location.metadata`
/// - `terminal.reader.metadata`
/// - `topup.metadata`
/// - `transfer.metadata`
/// - `webhook_endpoint.metadata`
pub type Metadata8076DB = HashMap<String, String>;

/// Spec paths:
/// - `account.object`
/// - `deleted_account.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject9D4B89 {
    #[serde(rename = "account")]
    Account,
}

/// Spec paths:
/// - `account.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrType3680CD {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "express")]
    Express,
    #[serde(rename = "standard")]
    Standard,
}

/// Spec paths:
/// - `account_bacs_debit_payments_settings`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountBacsDebitPaymentsSettings {
    pub display_name: Option<String>,
}

/// Spec paths:
/// - `account_branding_settings`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountBrandingSettings {
    pub icon: Option<UniFile5BD414>,
    pub logo: Option<UniFile5BD414>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
}

/// Spec paths:
/// - `account_branding_settings.icon`
/// - `account_branding_settings.logo`
/// - `dispute_evidence.cancellation_policy`
/// - `dispute_evidence.customer_communication`
/// - `dispute_evidence.customer_signature`
/// - `dispute_evidence.duplicate_charge_documentation`
/// - `dispute_evidence.receipt`
/// - `dispute_evidence.refund_policy`
/// - `dispute_evidence.service_documentation`
/// - `dispute_evidence.shipping_documentation`
/// - `dispute_evidence.uncategorized_file`
/// - `file_link.file`
/// - `issuing_cardholder_id_document.back`
/// - `issuing_cardholder_id_document.front`
/// - `issuing_dispute_canceled_evidence.additional_documentation`
/// - `issuing_dispute_duplicate_evidence.additional_documentation`
/// - `issuing_dispute_duplicate_evidence.card_statement`
/// - `issuing_dispute_duplicate_evidence.cash_receipt`
/// - `issuing_dispute_duplicate_evidence.check_image`
/// - `issuing_dispute_fraudulent_evidence.additional_documentation`
/// - `issuing_dispute_merchandise_not_as_described_evidence.additional_documentation`
/// - `issuing_dispute_not_received_evidence.additional_documentation`
/// - `issuing_dispute_other_evidence.additional_documentation`
/// - `issuing_dispute_service_not_as_described_evidence.additional_documentation`
/// - `legal_entity_company_verification_document.back`
/// - `legal_entity_company_verification_document.front`
/// - `legal_entity_person_verification_document.back`
/// - `legal_entity_person_verification_document.front`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniFile5BD414 {
    String(String),
    File(File),
}

/// Spec paths:
/// - `account_business_profile`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountBusinessProfile {
    pub name: Option<String>,
    pub mcc: Option<String>,
    pub product_description: Option<String>,
    pub support_address: Option<Address>,
    pub support_email: Option<String>,
    pub support_phone: Option<String>,
    pub support_url: Option<String>,
    pub url: Option<String>,
}

/// Spec paths:
/// - `account_capabilities`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountCapabilities {
    pub au_becs_debit_payments: Option<UniStrStatusBA4125>,
    pub bacs_debit_payments: Option<UniStrStatusBA4125>,
    pub bancontact_payments: Option<UniStrStatusBA4125>,
    pub card_issuing: Option<UniStrStatusBA4125>,
    pub card_payments: Option<UniStrStatusBA4125>,
    pub cartes_bancaires_payments: Option<UniStrStatusBA4125>,
    pub eps_payments: Option<UniStrStatusBA4125>,
    pub fpx_payments: Option<UniStrStatusBA4125>,
    pub giropay_payments: Option<UniStrStatusBA4125>,
    pub ideal_payments: Option<UniStrStatusBA4125>,
    pub jcb_payments: Option<UniStrStatusBA4125>,
    pub legacy_payments: Option<UniStrStatusBA4125>,
    pub oxxo_payments: Option<UniStrStatusBA4125>,
    pub p24_payments: Option<UniStrStatusBA4125>,
    pub sepa_debit_payments: Option<UniStrStatusBA4125>,
    pub sofort_payments: Option<UniStrStatusBA4125>,
    pub tax_reporting_us_1099_k: Option<UniStrStatusBA4125>,
    pub tax_reporting_us_1099_misc: Option<UniStrStatusBA4125>,
    pub transfers: Option<UniStrStatusBA4125>,
}

/// Spec paths:
/// - `account_capabilities.au_becs_debit_payments`
/// - `account_capabilities.bacs_debit_payments`
/// - `account_capabilities.bancontact_payments`
/// - `account_capabilities.card_issuing`
/// - `account_capabilities.card_payments`
/// - `account_capabilities.cartes_bancaires_payments`
/// - `account_capabilities.eps_payments`
/// - `account_capabilities.fpx_payments`
/// - `account_capabilities.giropay_payments`
/// - `account_capabilities.ideal_payments`
/// - `account_capabilities.jcb_payments`
/// - `account_capabilities.legacy_payments`
/// - `account_capabilities.oxxo_payments`
/// - `account_capabilities.p24_payments`
/// - `account_capabilities.sepa_debit_payments`
/// - `account_capabilities.sofort_payments`
/// - `account_capabilities.tax_reporting_us_1099_k`
/// - `account_capabilities.tax_reporting_us_1099_misc`
/// - `account_capabilities.transfers`
/// - `mandate.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatusBA4125 {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "pending")]
    Pending,
}

/// Spec paths:
/// - `account_capability_requirements`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountCapabilityRequirements {
    pub current_deadline: Option<i64>,
    pub currently_due: Vec<String>,
    pub disabled_reason: Option<String>,
    pub errors: Vec<AccountRequirementsError>,
    pub eventually_due: Vec<String>,
    pub past_due: Vec<String>,
    pub pending_verification: Vec<String>,
}

/// Spec paths:
/// - `account_card_payments_settings`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountCardPaymentsSettings {
    pub decline_on: Option<AccountDeclineChargeOn>,
    pub statement_descriptor_prefix: Option<String>,
}

/// Spec paths:
/// - `account_dashboard_settings`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountDashboardSettings {
    pub display_name: Option<String>,
    pub timezone: Option<String>,
}

/// Spec paths:
/// - `account_decline_charge_on`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountDeclineChargeOn {
    pub avs_failure: bool,
    pub cvc_failure: bool,
}

/// Spec paths:
/// - `account_link`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountLink {
    pub object: UniStrObject4F3637,
    pub expires_at: i64,
    pub url: String,
    pub created: i64,
}

/// Spec paths:
/// - `account_link.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject4F3637 {
    #[serde(rename = "account_link")]
    AccountLink,
}

/// Spec paths:
/// - `account_payments_settings`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountPaymentsSettings {
    pub statement_descriptor: Option<String>,
    pub statement_descriptor_kana: Option<String>,
    pub statement_descriptor_kanji: Option<String>,
}

/// Spec paths:
/// - `account_payout_settings`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountPayoutSettings {
    pub debit_negative_balances: bool,
    pub schedule: TransferSchedule,
    pub statement_descriptor: Option<String>,
}

/// Spec paths:
/// - `account_requirements`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountRequirements {
    pub current_deadline: Option<i64>,
    pub currently_due: Option<Vec<String>>,
    pub disabled_reason: Option<String>,
    pub errors: Option<Vec<AccountRequirementsError>>,
    pub eventually_due: Option<Vec<String>>,
    pub past_due: Option<Vec<String>>,
    pub pending_verification: Option<Vec<String>>,
}

/// Spec paths:
/// - `account_requirements_error`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountRequirementsError {
    pub code: UniStrCode,
    pub reason: String,
    pub requirement: String,
}

/// Spec paths:
/// - `account_requirements_error.code`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrCode {
    #[serde(rename = "invalid_address_city_state_postal_code")]
    InvalidAddressCityStatePostalCode,
    #[serde(rename = "invalid_street_address")]
    InvalidStreetAddress,
    #[serde(rename = "invalid_value_other")]
    InvalidValueOther,
    #[serde(rename = "verification_document_address_mismatch")]
    VerificationDocumentAddressMismatch,
    #[serde(rename = "verification_document_address_missing")]
    VerificationDocumentAddressMissing,
    #[serde(rename = "verification_document_corrupt")]
    VerificationDocumentCorrupt,
    #[serde(rename = "verification_document_country_not_supported")]
    VerificationDocumentCountryNotSupported,
    #[serde(rename = "verification_document_dob_mismatch")]
    VerificationDocumentDobMismatch,
    #[serde(rename = "verification_document_duplicate_type")]
    VerificationDocumentDuplicateType,
    #[serde(rename = "verification_document_expired")]
    VerificationDocumentExpired,
    #[serde(rename = "verification_document_failed_copy")]
    VerificationDocumentFailedCopy,
    #[serde(rename = "verification_document_failed_greyscale")]
    VerificationDocumentFailedGreyscale,
    #[serde(rename = "verification_document_failed_other")]
    VerificationDocumentFailedOther,
    #[serde(rename = "verification_document_failed_test_mode")]
    VerificationDocumentFailedTestMode,
    #[serde(rename = "verification_document_fraudulent")]
    VerificationDocumentFraudulent,
    #[serde(rename = "verification_document_id_number_mismatch")]
    VerificationDocumentIdNumberMismatch,
    #[serde(rename = "verification_document_id_number_missing")]
    VerificationDocumentIdNumberMissing,
    #[serde(rename = "verification_document_incomplete")]
    VerificationDocumentIncomplete,
    #[serde(rename = "verification_document_invalid")]
    VerificationDocumentInvalid,
    #[serde(rename = "verification_document_issue_or_expiry_date_missing")]
    VerificationDocumentIssueOrExpiryDateMissing,
    #[serde(rename = "verification_document_manipulated")]
    VerificationDocumentManipulated,
    #[serde(rename = "verification_document_missing_back")]
    VerificationDocumentMissingBack,
    #[serde(rename = "verification_document_missing_front")]
    VerificationDocumentMissingFront,
    #[serde(rename = "verification_document_name_mismatch")]
    VerificationDocumentNameMismatch,
    #[serde(rename = "verification_document_name_missing")]
    VerificationDocumentNameMissing,
    #[serde(rename = "verification_document_nationality_mismatch")]
    VerificationDocumentNationalityMismatch,
    #[serde(rename = "verification_document_not_readable")]
    VerificationDocumentNotReadable,
    #[serde(rename = "verification_document_not_signed")]
    VerificationDocumentNotSigned,
    #[serde(rename = "verification_document_not_uploaded")]
    VerificationDocumentNotUploaded,
    #[serde(rename = "verification_document_photo_mismatch")]
    VerificationDocumentPhotoMismatch,
    #[serde(rename = "verification_document_too_large")]
    VerificationDocumentTooLarge,
    #[serde(rename = "verification_document_type_not_supported")]
    VerificationDocumentTypeNotSupported,
    #[serde(rename = "verification_failed_address_match")]
    VerificationFailedAddressMatch,
    #[serde(rename = "verification_failed_business_iec_number")]
    VerificationFailedBusinessIecNumber,
    #[serde(rename = "verification_failed_document_match")]
    VerificationFailedDocumentMatch,
    #[serde(rename = "verification_failed_id_number_match")]
    VerificationFailedIdNumberMatch,
    #[serde(rename = "verification_failed_keyed_identity")]
    VerificationFailedKeyedIdentity,
    #[serde(rename = "verification_failed_keyed_match")]
    VerificationFailedKeyedMatch,
    #[serde(rename = "verification_failed_name_match")]
    VerificationFailedNameMatch,
    #[serde(rename = "verification_failed_other")]
    VerificationFailedOther,
    #[serde(rename = "verification_failed_tax_id_match")]
    VerificationFailedTaxIdMatch,
    #[serde(rename = "verification_failed_tax_id_not_issued")]
    VerificationFailedTaxIdNotIssued,
}

/// Spec paths:
/// - `account_sepa_debit_payments_settings`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountSepaDebitPaymentsSettings {
    pub creditor_id: Option<String>,
}

/// Spec paths:
/// - `account_settings`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountSettings {
    pub bacs_debit_payments: Option<AccountBacsDebitPaymentsSettings>,
    pub branding: AccountBrandingSettings,
    pub card_payments: AccountCardPaymentsSettings,
    pub dashboard: AccountDashboardSettings,
    pub payments: AccountPaymentsSettings,
    pub payouts: Option<AccountPayoutSettings>,
    pub sepa_debit_payments: Option<AccountSepaDebitPaymentsSettings>,
}

/// Spec paths:
/// - `account_tos_acceptance`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountTOSAcceptance {
    pub date: Option<i64>,
    pub ip: Option<String>,
    pub service_agreement: Option<String>,
    pub user_agent: Option<String>,
}

/// Spec paths:
/// - `address`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

/// Spec paths:
/// - `alipay_account`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlipayAccount {
    pub object: UniStrObject2AE122,
    pub id: String,
    pub customer: Option<UniCustomerC00F6E>,
    pub fingerprint: String,
    pub payment_amount: Option<i64>,
    pub payment_currency: Option<String>,
    pub reusable: bool,
    pub used: bool,
    pub username: String,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata8076DB>,
}

impl GetId for AlipayAccount {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `alipay_account.customer`
/// - `bank_account.customer`
/// - `card.customer`
/// - `charge.customer`
/// - `checkout.session.customer`
/// - `credit_note.customer`
/// - `deleted_discount.customer`
/// - `discount.customer`
/// - `invoice.customer`
/// - `invoiceitem.customer`
/// - `order.customer`
/// - `payment_intent.customer`
/// - `promotion_code.customer`
/// - `setup_attempt.customer`
/// - `setup_intent.customer`
/// - `subscription.customer`
/// - `subscription_schedule.customer`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniCustomerC00F6E {
    String(String),
    Customer(Box<Customer>),
    DeletedCustomer(DeletedCustomer),
}

/// Spec paths:
/// - `alipay_account.object`
/// - `deleted_alipay_account.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject2AE122 {
    #[serde(rename = "alipay_account")]
    AlipayAccount,
}

/// Spec paths:
/// - `api_errors`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APIErrors {
    #[serde(rename = "type")]
    pub type_x: UniStrTypeA3B873,
    pub source: Option<UniSourceEC1AE6>,
    pub charge: Option<String>,
    pub code: Option<String>,
    pub decline_code: Option<String>,
    pub doc_url: Option<String>,
    pub message: Option<String>,
    pub param: Option<String>,
    pub payment_intent: Box<Option<PaymentIntent>>,
    pub payment_method: Box<Option<PaymentMethod>>,
    pub payment_method_type: Option<String>,
    pub setup_intent: Box<Option<SetupIntent>>,
}

/// Spec paths:
/// - `api_errors.source`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniSourceEC1AE6 {
    BankAccount(BankAccount),
    Card(Box<Card>),
    Source(Source),
}

/// Spec paths:
/// - `api_errors.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTypeA3B873 {
    #[serde(rename = "api_connection_error")]
    ApiConnectionError,
    #[serde(rename = "api_error")]
    ApiError,
    #[serde(rename = "authentication_error")]
    AuthenticationError,
    #[serde(rename = "card_error")]
    CardError,
    #[serde(rename = "idempotency_error")]
    IdempotencyError,
    #[serde(rename = "invalid_request_error")]
    InvalidRequestError,
    #[serde(rename = "rate_limit_error")]
    RateLimitError,
}

/// Spec paths:
/// - `apple_pay_domain`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplePayDomain {
    pub object: UniStrObjectBA0885,
    pub id: String,
    pub domain_name: String,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for ApplePayDomain {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `apple_pay_domain.object`
/// - `deleted_apple_pay_domain.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectBA0885 {
    #[serde(rename = "apple_pay_domain")]
    ApplePayDomain,
}

/// Spec paths:
/// - `application`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    pub object: UniStrObject938E08,
    pub id: String,
    pub name: Option<String>,
}

impl GetId for Application {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `application.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject938E08 {
    #[serde(rename = "application")]
    Application,
}

/// Spec paths:
/// - `application_fee`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformFee {
    pub object: UniStrObjectD09910,
    pub id: String,
    pub account: UniAccount,
    pub application: UniApplication,
    pub balance_transaction: Option<UniBalanceTransaction>,
    pub charge: UniCharge,
    pub originating_transaction: Option<UniCharge>,
    pub amount: i64,
    pub amount_refunded: i64,
    pub currency: String,
    pub refunded: bool,
    pub refunds: FeeRefundListC565F2,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for PlatformFee {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `application_fee.account`
/// - `bank_account.account`
/// - `capability.account`
/// - `card.account`
/// - `charge.on_behalf_of`
/// - `charge_transfer_data.destination`
/// - `connect_collection_transfer.destination`
/// - `invoice_transfer_data.destination`
/// - `payment_intent.on_behalf_of`
/// - `recipient.migrated_to`
/// - `recipient.rolled_back_from`
/// - `setup_attempt.on_behalf_of`
/// - `setup_intent.on_behalf_of`
/// - `subscription_transfer_data.destination`
/// - `transfer.destination`
/// - `transfer_data.destination`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniAccount {
    String(String),
    Account(Account),
}

/// Spec paths:
/// - `application_fee.application`
/// - `charge.application`
/// - `payment_intent.application`
/// - `setup_attempt.application`
/// - `setup_intent.application`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniApplication {
    String(String),
    Application(Application),
}

/// Spec paths:
/// - `application_fee.balance_transaction`
/// - `charge.balance_transaction`
/// - `fee_refund.balance_transaction`
/// - `issuing.transaction.balance_transaction`
/// - `payout.balance_transaction`
/// - `payout.failure_balance_transaction`
/// - `refund.balance_transaction`
/// - `refund.failure_balance_transaction`
/// - `topup.balance_transaction`
/// - `transfer.balance_transaction`
/// - `transfer_reversal.balance_transaction`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniBalanceTransaction {
    String(String),
    BalanceTransaction(BalanceTransaction),
}

/// Spec paths:
/// - `application_fee.charge`
/// - `application_fee.originating_transaction`
/// - `dispute.charge`
/// - `invoice.charge`
/// - `issuer_fraud_record.charge`
/// - `order.charge`
/// - `radar.early_fraud_warning.charge`
/// - `refund.charge`
/// - `review.charge`
/// - `sepa_debit_generated_from.charge`
/// - `transfer.destination_payment`
/// - `transfer.source_transaction`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniCharge {
    String(String),
    Charge(Box<Charge>),
}

/// Spec paths:
/// - `application_fee.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectD09910 {
    #[serde(rename = "application_fee")]
    ApplicationFee,
}

/// Spec paths:
/// - `application_fee.refunds`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeeRefundListC565F2 {
    pub object: UniStrObject344B0E,
    pub data: Vec<FeeRefund>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for FeeRefundListC565F2 {
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
/// - `balance`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance {
    pub object: UniStrObjectD816FE,
    pub available: Vec<BalanceAmount>,
    pub connect_reserved: Option<Vec<BalanceAmount>>,
    pub instant_available: Option<Vec<BalanceAmount>>,
    pub issuing: Option<BalanceDetail>,
    pub pending: Vec<BalanceAmount>,
    pub livemode: bool,
}

/// Spec paths:
/// - `balance.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectD816FE {
    #[serde(rename = "balance")]
    Balance,
}

/// Spec paths:
/// - `balance_amount`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalanceAmount {
    pub amount: i64,
    pub currency: String,
    pub source_types: Option<BalanceAmountBySourceType>,
}

/// Spec paths:
/// - `balance_amount_by_source_type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalanceAmountBySourceType {
    pub bank_account: Option<i64>,
    pub card: Option<i64>,
    pub fpx: Option<i64>,
}

/// Spec paths:
/// - `balance_detail`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalanceDetail {
    pub available: Vec<BalanceAmount>,
}

/// Spec paths:
/// - `balance_transaction`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalanceTransaction {
    pub object: UniStrObject489413,
    #[serde(rename = "type")]
    pub type_x: UniStrTypeFA7E57,
    pub id: String,
    pub source: Box<Option<UniSource231860>>,
    pub amount: i64,
    pub available_on: i64,
    pub currency: String,
    pub description: Option<String>,
    pub exchange_rate: Option<f64>,
    pub fee: i64,
    pub fee_details: Vec<Fee>,
    pub net: i64,
    pub reporting_category: String,
    pub status: String,
    pub created: i64,
}

impl GetId for BalanceTransaction {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `balance_transaction.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject489413 {
    #[serde(rename = "balance_transaction")]
    BalanceTransaction,
}

/// Spec paths:
/// - `balance_transaction.source`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniSource231860 {
    String(String),
    PlatformFee(PlatformFee),
    Charge(Box<Charge>),
    ConnectCollectionTransfer(ConnectCollectionTransfer),
    Dispute(Dispute),
    FeeRefund(FeeRefund),
    IssuingAuthorization(IssuingAuthorization),
    IssuingDispute(IssuingDispute),
    IssuingTransaction(Box<IssuingTransaction>),
    Payout(Box<Payout>),
    PlatformTax(PlatformTax),
    Refund(Box<Refund>),
    ReserveTransaction(ReserveTransaction),
    TaxDeductedAtSource(TaxDeductedAtSource),
    Topup(Topup),
    Transfer(Transfer),
    TransferReversal(TransferReversal),
}

/// Spec paths:
/// - `balance_transaction.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTypeFA7E57 {
    #[serde(rename = "adjustment")]
    Adjustment,
    #[serde(rename = "advance")]
    Advance,
    #[serde(rename = "advance_funding")]
    AdvanceFunding,
    #[serde(rename = "anticipation_repayment")]
    AnticipationRepayment,
    #[serde(rename = "application_fee")]
    ApplicationFee,
    #[serde(rename = "application_fee_refund")]
    ApplicationFeeRefund,
    #[serde(rename = "charge")]
    Charge,
    #[serde(rename = "connect_collection_transfer")]
    ConnectCollectionTransfer,
    #[serde(rename = "contribution")]
    Contribution,
    #[serde(rename = "issuing_authorization_hold")]
    IssuingAuthorizationHold,
    #[serde(rename = "issuing_authorization_release")]
    IssuingAuthorizationRelease,
    #[serde(rename = "issuing_dispute")]
    IssuingDispute,
    #[serde(rename = "issuing_transaction")]
    IssuingTransaction,
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "payment_failure_refund")]
    PaymentFailureRefund,
    #[serde(rename = "payment_refund")]
    PaymentRefund,
    #[serde(rename = "payout")]
    Payout,
    #[serde(rename = "payout_cancel")]
    PayoutCancel,
    #[serde(rename = "payout_failure")]
    PayoutFailure,
    #[serde(rename = "refund")]
    Refund,
    #[serde(rename = "refund_failure")]
    RefundFailure,
    #[serde(rename = "reserve_transaction")]
    ReserveTransaction,
    #[serde(rename = "reserved_funds")]
    ReservedFunds,
    #[serde(rename = "stripe_fee")]
    StripeFee,
    #[serde(rename = "stripe_fx_fee")]
    StripeFxFee,
    #[serde(rename = "tax_fee")]
    TaxFee,
    #[serde(rename = "topup")]
    Topup,
    #[serde(rename = "topup_reversal")]
    TopupReversal,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "transfer_cancel")]
    TransferCancel,
    #[serde(rename = "transfer_failure")]
    TransferFailure,
    #[serde(rename = "transfer_refund")]
    TransferRefund,
}

/// Spec paths:
/// - `bank_account`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BankAccount {
    pub object: UniStrObject5F868A,
    pub id: String,
    pub account: Option<UniAccount>,
    pub customer: Option<UniCustomerC00F6E>,
    pub account_holder_name: Option<String>,
    pub account_holder_type: Option<String>,
    pub available_payout_methods: Option<Vec<UniStrItemsD0C1A3>>,
    pub bank_name: Option<String>,
    pub country: String,
    pub currency: String,
    pub default_for_currency: Option<bool>,
    pub fingerprint: Option<String>,
    pub last4: String,
    pub routing_number: Option<String>,
    pub status: String,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for BankAccount {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `bank_account.available_payout_methods.items`
/// - `card.available_payout_methods.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrItemsD0C1A3 {
    #[serde(rename = "instant")]
    Instant,
    #[serde(rename = "standard")]
    Standard,
}

/// Spec paths:
/// - `bank_account.metadata`
/// - `bitcoin_receiver.metadata`
/// - `card.metadata`
/// - `checkout.session.metadata`
/// - `coupon.metadata`
/// - `credit_note.metadata`
/// - `customer_balance_transaction.metadata`
/// - `fee_refund.metadata`
/// - `invoice.metadata`
/// - `invoiceitem.metadata`
/// - `order.metadata`
/// - `payment_method.metadata`
/// - `payout.metadata`
/// - `plan.metadata`
/// - `promotion_code.metadata`
/// - `refund.metadata`
/// - `setup_intent.metadata`
/// - `source.metadata`
/// - `subscription_schedule.metadata`
/// - `tax_rate.metadata`
/// - `transfer_reversal.metadata`
pub type Metadata7CCA3C = Option<HashMap<String, String>>;

/// Spec paths:
/// - `bank_account.object`
/// - `deleted_bank_account.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject5F868A {
    #[serde(rename = "bank_account")]
    BankAccount,
}

/// Spec paths:
/// - `billing_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BillingDetails {
    pub name: Option<String>,
    pub address: Option<Address>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

/// Spec paths:
/// - `billing_portal.session`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PortalSession {
    pub object: UniStrObject94B3D3,
    pub id: String,
    pub customer: String,
    pub return_url: String,
    pub url: String,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for PortalSession {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `billing_portal.session.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject94B3D3 {
    #[serde(rename = "billing_portal.session")]
    BillingPortalDotSession,
}

/// Spec paths:
/// - `bitcoin_receiver`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BitcoinReceiver {
    pub object: UniStrObject041746,
    pub id: String,
    pub active: bool,
    pub amount: i64,
    pub amount_received: i64,
    pub bitcoin_amount: i64,
    pub bitcoin_amount_received: i64,
    pub bitcoin_uri: String,
    pub currency: String,
    pub customer: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub filled: bool,
    pub inbound_address: String,
    pub payment: Option<String>,
    pub refund_address: Option<String>,
    pub transactions: Option<BitcoinTransactionListC35081>,
    pub uncaptured_funds: bool,
    pub used_for_payment: Option<bool>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for BitcoinReceiver {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `bitcoin_receiver.object`
/// - `deleted_bitcoin_receiver.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject041746 {
    #[serde(rename = "bitcoin_receiver")]
    BitcoinReceiver,
}

/// Spec paths:
/// - `bitcoin_receiver.transactions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BitcoinTransactionListC35081 {
    pub object: UniStrObject344B0E,
    pub data: Vec<BitcoinTransaction>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for BitcoinTransactionListC35081 {
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
/// - `bitcoin_transaction`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BitcoinTransaction {
    pub object: UniStrObjectD009EA,
    pub id: String,
    pub amount: i64,
    pub bitcoin_amount: i64,
    pub currency: String,
    pub receiver: String,
    pub created: i64,
}

impl GetId for BitcoinTransaction {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `bitcoin_transaction.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectD009EA {
    #[serde(rename = "bitcoin_transaction")]
    BitcoinTransaction,
}

/// Spec paths:
/// - `capability`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountCapability {
    pub object: UniStrObjectADCEF8,
    pub id: String,
    pub account: UniAccount,
    pub requested: bool,
    pub requested_at: Option<i64>,
    pub requirements: Option<AccountCapabilityRequirements>,
    pub status: UniStrStatus97E37F,
}

impl GetId for AccountCapability {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `capability.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectADCEF8 {
    #[serde(rename = "capability")]
    Capability,
}

/// Spec paths:
/// - `capability.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatus97E37F {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "unrequested")]
    Unrequested,
}

/// Spec paths:
/// - `card`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub object: UniStrObjectBAC623,
    pub id: String,
    pub name: Option<String>,
    pub account: Option<UniAccount>,
    pub customer: Option<UniCustomerC00F6E>,
    pub recipient: Option<UniRecipient>,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line1_check: Option<String>,
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<String>,
    pub available_payout_methods: Option<Vec<UniStrItemsD0C1A3>>,
    pub brand: String,
    pub country: Option<String>,
    pub currency: Option<String>,
    pub cvc_check: Option<String>,
    pub default_for_currency: Option<bool>,
    pub dynamic_last4: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: Option<String>,
    pub funding: String,
    pub last4: String,
    pub tokenization_method: Option<String>,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for Card {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `card.object`
/// - `deleted_card.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectBAC623 {
    #[serde(rename = "card")]
    Card,
}

/// Spec paths:
/// - `card.recipient`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniRecipient {
    String(String),
    TransferRecipient(TransferRecipient),
}

/// Spec paths:
/// - `card_generated_from_payment_method_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardGeneratedFromPaymentMethodDetails {
    #[serde(rename = "type")]
    pub type_x: String,
    pub card_present: Option<PaymentMethodDetailsCardPresent>,
}

/// Spec paths:
/// - `card_mandate_payment_method_details`
pub type CardMandatePaymentMethodDetails = Value;

/// Spec paths:
/// - `charge`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Charge {
    pub object: UniStrObjectA84719,
    pub id: String,
    pub paid: bool,
    pub application: Option<UniApplication>,
    pub application_fee: Option<UniFee>,
    pub balance_transaction: Option<UniBalanceTransaction>,
    pub customer: Option<UniCustomerC00F6E>,
    pub invoice: Option<UniInvoice>,
    pub on_behalf_of: Option<UniAccount>,
    pub order: Option<UniOrder>,
    pub payment_intent: Option<UniPaymentIntent>,
    pub review: Option<UniReview>,
    pub source_transfer: Option<UniTransfer>,
    pub transfer: Option<UniTransfer>,
    pub amount: i64,
    pub amount_captured: i64,
    pub amount_refunded: i64,
    pub application_fee_amount: Option<i64>,
    pub billing_details: BillingDetails,
    pub calculated_statement_descriptor: Option<String>,
    pub captured: bool,
    pub currency: String,
    pub description: Option<String>,
    pub disputed: bool,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub fraud_details: Option<ChargeFraudDetails>,
    pub outcome: Option<ChargeOutcome>,
    pub payment_method: Option<String>,
    pub payment_method_details: Option<PaymentMethodDetails>,
    pub receipt_email: Option<String>,
    pub receipt_number: Option<String>,
    pub receipt_url: Option<String>,
    pub refunded: bool,
    pub refunds: RefundList4E41A7,
    pub shipping: Option<Shipping>,
    pub statement_descriptor: Option<String>,
    pub statement_descriptor_suffix: Option<String>,
    pub status: String,
    pub transfer_data: Option<ChargeTransferData>,
    pub transfer_group: Option<String>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for Charge {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `charge.application_fee`
/// - `fee_refund.fee`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniFee {
    String(String),
    PlatformFee(PlatformFee),
}

/// Spec paths:
/// - `charge.invoice`
/// - `credit_note.invoice`
/// - `customer_balance_transaction.invoice`
/// - `invoiceitem.invoice`
/// - `payment_intent.invoice`
/// - `subscription.latest_invoice`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniInvoice {
    String(String),
    Invoice(Box<Invoice>),
}

/// Spec paths:
/// - `charge.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectA84719 {
    #[serde(rename = "charge")]
    Charge,
}

/// Spec paths:
/// - `charge.order`
/// - `order_return.order`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniOrder {
    String(String),
    Order(Order),
}

/// Spec paths:
/// - `charge.payment_intent`
/// - `checkout.session.payment_intent`
/// - `dispute.payment_intent`
/// - `invoice.payment_intent`
/// - `refund.payment_intent`
/// - `review.payment_intent`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniPaymentIntent {
    String(String),
    PaymentIntent(Box<PaymentIntent>),
}

/// Spec paths:
/// - `charge.refunds`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefundList4E41A7 {
    pub object: UniStrObject344B0E,
    pub data: Vec<Refund>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for RefundList4E41A7 {
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
/// - `charge.review`
/// - `payment_intent.review`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniReview {
    String(String),
    RadarReview(RadarReview),
}

/// Spec paths:
/// - `charge.source_transfer`
/// - `charge.transfer`
/// - `transfer_reversal.transfer`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniTransfer {
    String(String),
    Transfer(Transfer),
}

/// Spec paths:
/// - `charge_fraud_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChargeFraudDetails {
    pub stripe_report: Option<String>,
    pub user_report: Option<String>,
}

/// Spec paths:
/// - `charge_outcome`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChargeOutcome {
    #[serde(rename = "type")]
    pub type_x: String,
    pub rule: Option<UniRule>,
    pub network_status: Option<String>,
    pub reason: Option<String>,
    pub risk_level: Option<String>,
    pub risk_score: Option<i64>,
    pub seller_message: Option<String>,
}

/// Spec paths:
/// - `charge_outcome.rule`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniRule {
    String(String),
    RadarRule(RadarRule),
}

/// Spec paths:
/// - `charge_transfer_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChargeTransferData {
    pub destination: UniAccount,
    pub amount: Option<i64>,
}

/// Spec paths:
/// - `checkout.session`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub object: UniStrObject67F833,
    pub client_reference_id: Option<String>,
    pub id: String,
    pub customer: Option<UniCustomerC00F6E>,
    pub payment_intent: Option<UniPaymentIntent>,
    pub setup_intent: Option<UniSetupIntent>,
    pub subscription: Option<UniSubscription>,
    pub allow_promotion_codes: Option<bool>,
    pub amount_subtotal: Option<i64>,
    pub amount_total: Option<i64>,
    pub billing_address_collection: Option<UniStrBillingAddressCollection>,
    pub cancel_url: String,
    pub currency: Option<String>,
    pub customer_email: Option<String>,
    pub line_items: Option<PaymentPagesCheckoutSessionListLineItems46D5C8>,
    pub locale: Option<UniStrLocale>,
    pub mode: UniStrMode,
    pub payment_method_types: Vec<String>,
    pub payment_status: UniStrPaymentStatus,
    pub shipping: Option<Shipping>,
    pub shipping_address_collection:
        Option<PaymentPagesPaymentPageResourcesShippingAddressCollection>,
    pub submit_type: Option<UniStrSubmitType>,
    pub success_url: String,
    pub total_details: Option<PaymentPagesCheckoutSessionTotalDetails>,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for Session {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `checkout.session.billing_address_collection`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrBillingAddressCollection {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}

/// Spec paths:
/// - `checkout.session.line_items`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentPagesCheckoutSessionListLineItems46D5C8 {
    pub object: UniStrObject344B0E,
    pub data: Vec<LineItems>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for PaymentPagesCheckoutSessionListLineItems46D5C8 {
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
/// - `checkout.session.locale`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrLocale {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "en-GB")]
    EnGB,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "es-419")]
    Es419,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "fr-CA")]
    FrCA,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "mt")]
    Mt,
    #[serde(rename = "nb")]
    Nb,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "pt-BR")]
    PtBR,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "zh")]
    Zh,
    #[serde(rename = "zh-HK")]
    ZhHK,
    #[serde(rename = "zh-TW")]
    ZhTW,
}

/// Spec paths:
/// - `checkout.session.mode`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrMode {
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "setup")]
    Setup,
    #[serde(rename = "subscription")]
    Subscription,
}

/// Spec paths:
/// - `checkout.session.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject67F833 {
    #[serde(rename = "checkout.session")]
    CheckoutDotSession,
}

/// Spec paths:
/// - `checkout.session.payment_status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrPaymentStatus {
    #[serde(rename = "no_payment_required")]
    NoPaymentRequired,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "unpaid")]
    Unpaid,
}

/// Spec paths:
/// - `checkout.session.setup_intent`
/// - `setup_attempt.setup_intent`
/// - `subscription.pending_setup_intent`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniSetupIntent {
    String(String),
    SetupIntent(Box<SetupIntent>),
}

/// Spec paths:
/// - `checkout.session.submit_type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrSubmitType {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "book")]
    Book,
    #[serde(rename = "donate")]
    Donate,
    #[serde(rename = "pay")]
    Pay,
}

/// Spec paths:
/// - `checkout.session.subscription`
/// - `invoice.subscription`
/// - `invoiceitem.subscription`
/// - `subscription_schedule.subscription`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniSubscription {
    String(String),
    Subscription(Box<Subscription>),
}

/// Spec paths:
/// - `connect_collection_transfer`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectCollectionTransfer {
    pub object: UniStrObject014065,
    pub id: String,
    pub destination: UniAccount,
    pub amount: i64,
    pub currency: String,
    pub livemode: bool,
}

impl GetId for ConnectCollectionTransfer {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `connect_collection_transfer.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject014065 {
    #[serde(rename = "connect_collection_transfer")]
    ConnectCollectionTransfer,
}

/// Spec paths:
/// - `country_spec`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountrySpec {
    pub object: UniStrObjectF72038,
    pub id: String,
    pub default_currency: String,
    pub supported_bank_account_currencies: SupportedBankAccountCurrencies,
    pub supported_payment_currencies: Vec<String>,
    pub supported_payment_methods: Vec<String>,
    pub supported_transfer_countries: Vec<String>,
    pub verification_fields: CountrySpecVerificationFields,
}

impl GetId for CountrySpec {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `country_spec.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectF72038 {
    #[serde(rename = "country_spec")]
    CountrySpec,
}

/// Spec paths:
/// - `country_spec.supported_bank_account_currencies`
pub type SupportedBankAccountCurrencies = HashMap<String, Vec<String>>;

/// Spec paths:
/// - `country_spec_verification_field_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountrySpecVerificationFieldDetails {
    pub additional: Vec<String>,
    pub minimum: Vec<String>,
}

/// Spec paths:
/// - `country_spec_verification_fields`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountrySpecVerificationFields {
    pub company: CountrySpecVerificationFieldDetails,
    pub individual: CountrySpecVerificationFieldDetails,
}

/// Spec paths:
/// - `coupon`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coupon {
    pub object: UniStrObjectDA5FE2,
    pub id: String,
    pub valid: bool,
    pub name: Option<String>,
    pub amount_off: Option<i64>,
    pub applies_to: Option<CouponAppliesTo>,
    pub currency: Option<String>,
    pub duration: UniStrDuration,
    pub duration_in_months: Option<i64>,
    pub max_redemptions: Option<i64>,
    pub percent_off: Option<f64>,
    pub redeem_by: Option<i64>,
    pub times_redeemed: i64,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for Coupon {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `coupon.duration`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrDuration {
    #[serde(rename = "forever")]
    Forever,
    #[serde(rename = "once")]
    Once,
    #[serde(rename = "repeating")]
    Repeating,
}

/// Spec paths:
/// - `coupon.object`
/// - `deleted_coupon.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectDA5FE2 {
    #[serde(rename = "coupon")]
    Coupon,
}

/// Spec paths:
/// - `coupon_applies_to`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CouponAppliesTo {
    pub products: Vec<String>,
}

/// Spec paths:
/// - `credit_note`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditNote {
    pub object: UniStrObjectD362B6,
    #[serde(rename = "type")]
    pub type_x: UniStrTypeC2237F,
    pub id: String,
    pub customer: UniCustomerC00F6E,
    pub customer_balance_transaction: Option<UniCustomerBalanceTransaction>,
    pub invoice: UniInvoice,
    pub refund: Option<UniRefund>,
    pub amount: i64,
    pub currency: String,
    pub discount_amount: i64,
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,
    pub lines: CreditNoteLinesListF7B44B,
    pub memo: Option<String>,
    pub number: String,
    pub out_of_band_amount: Option<i64>,
    pub pdf: String,
    pub reason: Option<UniStrReasonAB5E91>,
    pub status: UniStrStatus476873,
    pub subtotal: i64,
    pub tax_amounts: Vec<CreditNoteTaxAmount>,
    pub total: i64,
    pub voided_at: Option<i64>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for CreditNote {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `credit_note.customer_balance_transaction`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniCustomerBalanceTransaction {
    String(String),
    CustomerBalanceTransaction(CustomerBalanceTransaction),
}

/// Spec paths:
/// - `credit_note.lines`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditNoteLinesListF7B44B {
    pub object: UniStrObject344B0E,
    pub data: Vec<CreditNoteLineItem>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for CreditNoteLinesListF7B44B {
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
/// - `credit_note.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectD362B6 {
    #[serde(rename = "credit_note")]
    CreditNote,
}

/// Spec paths:
/// - `credit_note.refund`
/// - `order_return.refund`
/// - `transfer_reversal.destination_payment_refund`
/// - `transfer_reversal.source_refund`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniRefund {
    String(String),
    Refund(Box<Refund>),
}

/// Spec paths:
/// - `credit_note.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatus476873 {
    #[serde(rename = "issued")]
    Issued,
    #[serde(rename = "void")]
    Void,
}

/// Spec paths:
/// - `credit_note.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTypeC2237F {
    #[serde(rename = "post_payment")]
    PostPayment,
    #[serde(rename = "pre_payment")]
    PrePayment,
}

/// Spec paths:
/// - `credit_note_line_item`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditNoteLineItem {
    pub object: UniStrObjectEE08F6,
    #[serde(rename = "type")]
    pub type_x: UniStrType00DF75,
    pub id: String,
    pub amount: i64,
    pub description: Option<String>,
    pub discount_amount: i64,
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,
    pub invoice_line_item: Option<String>,
    pub quantity: Option<i64>,
    pub tax_amounts: Vec<CreditNoteTaxAmount>,
    pub tax_rates: Vec<TaxRate>,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,
    pub livemode: bool,
}

impl GetId for CreditNoteLineItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `credit_note_line_item.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectEE08F6 {
    #[serde(rename = "credit_note_line_item")]
    CreditNoteLineItem,
}

/// Spec paths:
/// - `credit_note_tax_amount`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreditNoteTaxAmount {
    pub tax_rate: UniTaxRate,
    pub amount: i64,
    pub inclusive: bool,
}

/// Spec paths:
/// - `credit_note_tax_amount.tax_rate`
/// - `invoice_tax_amount.tax_rate`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniTaxRate {
    String(String),
    TaxRate(TaxRate),
}

/// Spec paths:
/// - `customer`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    pub object: UniStrObject877A57,
    pub id: String,
    pub name: Option<String>,
    pub default_source: Box<Option<UniDefaultSource>>,
    pub address: Option<Address>,
    pub balance: Option<i64>,
    pub currency: Option<String>,
    pub delinquent: Option<bool>,
    pub description: Option<String>,
    pub discount: Option<Discount>,
    pub email: Option<String>,
    pub invoice_prefix: Option<String>,
    pub invoice_settings: Option<InvoiceSettingCustomerSetting>,
    pub next_invoice_sequence: Option<i64>,
    pub phone: Option<String>,
    pub preferred_locales: Option<Vec<String>>,
    pub shipping: Option<Shipping>,
    pub sources: Option<ApmsSourcesSourceListC6CBF8>,
    pub subscriptions: Option<SubscriptionListAFDD89>,
    pub tax_exempt: Option<UniStrTaxExempt>,
    pub tax_ids: Option<TaxIDsList05E0C9>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata8076DB>,
}

impl GetId for Customer {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `customer.default_source`
/// - `invoice.default_source`
/// - `subscription.default_source`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniDefaultSource {
    String(String),
    AlipayAccount(AlipayAccount),
    BankAccount(BankAccount),
    BitcoinReceiver(BitcoinReceiver),
    Card(Box<Card>),
    Source(Source),
}

/// Spec paths:
/// - `customer.object`
/// - `deleted_customer.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject877A57 {
    #[serde(rename = "customer")]
    Customer,
}

/// Spec paths:
/// - `customer.sources`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApmsSourcesSourceListC6CBF8 {
    pub object: UniStrObject344B0E,
    pub data: Vec<UniPolymorphic646C3F>,
    pub has_more: bool,
    pub url: String,
}

/// Spec paths:
/// - `customer.subscriptions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionListAFDD89 {
    pub object: UniStrObject344B0E,
    pub data: Vec<Subscription>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for SubscriptionListAFDD89 {
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
/// - `customer.tax_exempt`
/// - `invoice.customer_tax_exempt`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTaxExempt {
    #[serde(rename = "exempt")]
    Exempt,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "reverse")]
    Reverse,
}

/// Spec paths:
/// - `customer.tax_ids`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxIDsList05E0C9 {
    pub object: UniStrObject344B0E,
    pub data: Vec<TaxId>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for TaxIDsList05E0C9 {
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
/// - `customer_acceptance`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerAcceptance {
    #[serde(rename = "type")]
    pub type_x: UniStrTypeD153A1,
    pub accepted_at: Option<i64>,
    pub offline: Option<OfflineAcceptance>,
    pub online: Option<OnlineAcceptance>,
}

/// Spec paths:
/// - `customer_balance_transaction`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerBalanceTransaction {
    pub object: UniStrObjectA5E76E,
    #[serde(rename = "type")]
    pub type_x: UniStrTypeBC72A1,
    pub id: String,
    pub credit_note: Option<UniCreditNote>,
    pub customer: UniCustomerEDC00A,
    pub invoice: Option<UniInvoice>,
    pub amount: i64,
    pub currency: String,
    pub description: Option<String>,
    pub ending_balance: i64,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for CustomerBalanceTransaction {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `customer_balance_transaction.credit_note`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniCreditNote {
    String(String),
    CreditNote(Box<CreditNote>),
}

/// Spec paths:
/// - `customer_balance_transaction.customer`
/// - `payment_method.customer`
/// - `tax_id.customer`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniCustomerEDC00A {
    String(String),
    Customer(Box<Customer>),
}

/// Spec paths:
/// - `customer_balance_transaction.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectA5E76E {
    #[serde(rename = "customer_balance_transaction")]
    CustomerBalanceTransaction,
}

/// Spec paths:
/// - `customer_balance_transaction.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTypeBC72A1 {
    #[serde(rename = "adjustment")]
    Adjustment,
    #[serde(rename = "applied_to_invoice")]
    AppliedToInvoice,
    #[serde(rename = "credit_note")]
    CreditNote,
    #[serde(rename = "initial")]
    Initial,
    #[serde(rename = "invoice_too_large")]
    InvoiceTooLarge,
    #[serde(rename = "invoice_too_small")]
    InvoiceTooSmall,
    #[serde(rename = "migration")]
    Migration,
    #[serde(rename = "unapplied_from_invoice")]
    UnappliedFromInvoice,
    #[serde(rename = "unspent_receiver_credit")]
    UnspentReceiverCredit,
}

/// Spec paths:
/// - `deleted_account`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedAccount {
    pub object: UniStrObject9D4B89,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedAccount {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_alipay_account`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlipayDeletedAccount {
    pub object: UniStrObject2AE122,
    pub id: String,
    pub deleted: bool,
}

impl GetId for AlipayDeletedAccount {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_apple_pay_domain`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedApplePayDomain {
    pub object: UniStrObjectBA0885,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedApplePayDomain {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_bank_account`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedBankAccount {
    pub object: UniStrObject5F868A,
    pub id: String,
    pub currency: Option<String>,
    pub deleted: bool,
}

impl GetId for DeletedBankAccount {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_bitcoin_receiver`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BitcoinDeletedReceiver {
    pub object: UniStrObject041746,
    pub id: String,
    pub deleted: bool,
}

impl GetId for BitcoinDeletedReceiver {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_card`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedCard {
    pub object: UniStrObjectBAC623,
    pub id: String,
    pub currency: Option<String>,
    pub deleted: bool,
}

impl GetId for DeletedCard {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_coupon`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedCoupon {
    pub object: UniStrObjectDA5FE2,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedCoupon {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_customer`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedCustomer {
    pub object: UniStrObject877A57,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedCustomer {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_discount`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedDiscount {
    pub object: UniStrObjectE5311B,
    pub id: String,
    pub customer: Option<UniCustomerC00F6E>,
    pub promotion_code: Option<UniPromotionCode>,
    pub checkout_session: Option<String>,
    pub coupon: Coupon,
    pub deleted: bool,
    pub invoice: Option<String>,
    pub invoice_item: Option<String>,
    pub start: i64,
    pub subscription: Option<String>,
}

impl GetId for DeletedDiscount {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_discount.object`
/// - `discount.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectE5311B {
    #[serde(rename = "discount")]
    Discount,
}

/// Spec paths:
/// - `deleted_discount.promotion_code`
/// - `discount.promotion_code`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniPromotionCode {
    String(String),
    PromotionCode(PromotionCode),
}

/// Spec paths:
/// - `deleted_external_account`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniPolymorphicB71A40 {
    DeletedBankAccount(DeletedBankAccount),
    DeletedCard(DeletedCard),
}

/// Spec paths:
/// - `deleted_invoice`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedInvoice {
    pub object: UniStrObject5E6A4A,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedInvoice {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_invoice.object`
/// - `invoice.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject5E6A4A {
    #[serde(rename = "invoice")]
    Invoice,
}

/// Spec paths:
/// - `deleted_invoiceitem`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedInvoiceItem {
    pub object: UniStrObjectA93110,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedInvoiceItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_invoiceitem.object`
/// - `invoiceitem.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectA93110 {
    #[serde(rename = "invoiceitem")]
    Invoiceitem,
}

/// Spec paths:
/// - `deleted_payment_source`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniPolymorphic94C032 {
    AlipayDeletedAccount(AlipayDeletedAccount),
    DeletedBankAccount(DeletedBankAccount),
    BitcoinDeletedReceiver(BitcoinDeletedReceiver),
    DeletedCard(DeletedCard),
}

/// Spec paths:
/// - `deleted_person`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedPerson {
    pub object: UniStrObjectAFEDA0,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedPerson {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_person.object`
/// - `person.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectAFEDA0 {
    #[serde(rename = "person")]
    Person,
}

/// Spec paths:
/// - `deleted_plan`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedPlan {
    pub object: UniStrObjectB95344,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedPlan {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_plan.object`
/// - `plan.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectB95344 {
    #[serde(rename = "plan")]
    Plan,
}

/// Spec paths:
/// - `deleted_price`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedPrice {
    pub object: UniStrObjectDB9846,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedPrice {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_price.object`
/// - `price.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectDB9846 {
    #[serde(rename = "price")]
    Price,
}

/// Spec paths:
/// - `deleted_product`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedProduct {
    pub object: UniStrObject04CB4A,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedProduct {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_product.object`
/// - `product.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject04CB4A {
    #[serde(rename = "product")]
    Product,
}

/// Spec paths:
/// - `deleted_radar.value_list`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadarListDeletedList {
    pub object: UniStrObjectF61F9F,
    pub id: String,
    pub deleted: bool,
}

impl GetId for RadarListDeletedList {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_radar.value_list.object`
/// - `radar.value_list.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectF61F9F {
    #[serde(rename = "radar.value_list")]
    RadarDotValueList,
}

/// Spec paths:
/// - `deleted_radar.value_list_item`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadarListDeletedListItem {
    pub object: UniStrObject2EE88E,
    pub id: String,
    pub deleted: bool,
}

impl GetId for RadarListDeletedListItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_radar.value_list_item.object`
/// - `radar.value_list_item.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject2EE88E {
    #[serde(rename = "radar.value_list_item")]
    RadarDotValueListItem,
}

/// Spec paths:
/// - `deleted_recipient`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedTransferRecipient {
    pub object: UniStrObject00DC3E,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedTransferRecipient {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_recipient.object`
/// - `recipient.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject00DC3E {
    #[serde(rename = "recipient")]
    Recipient,
}

/// Spec paths:
/// - `deleted_sku`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedSku {
    pub object: UniStrObject97B705,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedSku {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_sku.object`
/// - `sku.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject97B705 {
    #[serde(rename = "sku")]
    Sku,
}

/// Spec paths:
/// - `deleted_subscription_item`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedSubscriptionItem {
    pub object: UniStrObject36C70C,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedSubscriptionItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_subscription_item.object`
/// - `subscription_item.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject36C70C {
    #[serde(rename = "subscription_item")]
    SubscriptionItem,
}

/// Spec paths:
/// - `deleted_tax_id`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedTaxId {
    pub object: UniStrObject397A62,
    pub id: String,
    pub deleted: bool,
}

impl GetId for DeletedTaxId {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_tax_id.object`
/// - `tax_id.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject397A62 {
    #[serde(rename = "tax_id")]
    TaxId,
}

/// Spec paths:
/// - `deleted_terminal.location`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerminalLocationDeletedLocation {
    pub object: UniStrObject95542E,
    pub id: String,
    pub deleted: bool,
}

impl GetId for TerminalLocationDeletedLocation {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_terminal.location.object`
/// - `terminal.location.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject95542E {
    #[serde(rename = "terminal.location")]
    TerminalDotLocation,
}

/// Spec paths:
/// - `deleted_terminal.reader`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerminalReaderDeletedReader {
    pub object: UniStrObjectEAD5C5,
    pub id: String,
    pub deleted: bool,
}

impl GetId for TerminalReaderDeletedReader {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_terminal.reader.object`
/// - `terminal.reader.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectEAD5C5 {
    #[serde(rename = "terminal.reader")]
    TerminalDotReader,
}

/// Spec paths:
/// - `deleted_webhook_endpoint`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationWebhookEndpointDeleted {
    pub object: UniStrObjectBDDC67,
    pub id: String,
    pub deleted: bool,
}

impl GetId for NotificationWebhookEndpointDeleted {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `deleted_webhook_endpoint.object`
/// - `webhook_endpoint.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectBDDC67 {
    #[serde(rename = "webhook_endpoint")]
    WebhookEndpoint,
}

/// Spec paths:
/// - `delivery_estimate`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeliveryEstimate {
    #[serde(rename = "type")]
    pub type_x: String,
    pub date: Option<String>,
    pub earliest: Option<String>,
    pub latest: Option<String>,
}

/// Spec paths:
/// - `discount`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Discount {
    pub object: UniStrObjectE5311B,
    pub id: String,
    pub customer: Option<UniCustomerC00F6E>,
    pub promotion_code: Option<UniPromotionCode>,
    pub checkout_session: Option<String>,
    pub coupon: Coupon,
    pub end: Option<i64>,
    pub invoice: Option<String>,
    pub invoice_item: Option<String>,
    pub start: i64,
    pub subscription: Option<String>,
}

impl GetId for Discount {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `discounts_resource_discount_amount`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscountsResourceDiscountAmount {
    pub discount: UniItems6F859C,
    pub amount: i64,
}

/// Spec paths:
/// - `discounts_resource_discount_amount.discount`
/// - `invoice.discounts.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniItems6F859C {
    String(String),
    Discount(Discount),
    DeletedDiscount(DeletedDiscount),
}

/// Spec paths:
/// - `dispute`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dispute {
    pub object: UniStrObjectB4F7D7,
    pub id: String,
    pub charge: UniCharge,
    pub payment_intent: Option<UniPaymentIntent>,
    pub amount: i64,
    pub balance_transactions: Vec<BalanceTransaction>,
    pub currency: String,
    pub evidence: DisputeEvidence,
    pub evidence_details: DisputeEvidenceDetails,
    pub is_charge_refundable: bool,
    pub reason: String,
    pub status: UniStrStatusEEF70E,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for Dispute {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `dispute.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectB4F7D7 {
    #[serde(rename = "dispute")]
    Dispute,
}

/// Spec paths:
/// - `dispute.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatusEEF70E {
    #[serde(rename = "charge_refunded")]
    ChargeRefunded,
    #[serde(rename = "lost")]
    Lost,
    #[serde(rename = "needs_response")]
    NeedsResponse,
    #[serde(rename = "under_review")]
    UnderReview,
    #[serde(rename = "warning_closed")]
    WarningClosed,
    #[serde(rename = "warning_needs_response")]
    WarningNeedsResponse,
    #[serde(rename = "warning_under_review")]
    WarningUnderReview,
    #[serde(rename = "won")]
    Won,
}

/// Spec paths:
/// - `dispute_evidence`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisputeEvidence {
    pub duplicate_charge_id: Option<String>,
    pub cancellation_policy: Option<UniFile5BD414>,
    pub customer_communication: Option<UniFile5BD414>,
    pub customer_signature: Option<UniFile5BD414>,
    pub duplicate_charge_documentation: Option<UniFile5BD414>,
    pub receipt: Option<UniFile5BD414>,
    pub refund_policy: Option<UniFile5BD414>,
    pub service_documentation: Option<UniFile5BD414>,
    pub shipping_documentation: Option<UniFile5BD414>,
    pub uncategorized_file: Option<UniFile5BD414>,
    pub access_activity_log: Option<String>,
    pub billing_address: Option<String>,
    pub cancellation_policy_disclosure: Option<String>,
    pub cancellation_rebuttal: Option<String>,
    pub customer_email_address: Option<String>,
    pub customer_name: Option<String>,
    pub customer_purchase_ip: Option<String>,
    pub duplicate_charge_explanation: Option<String>,
    pub product_description: Option<String>,
    pub refund_policy_disclosure: Option<String>,
    pub refund_refusal_explanation: Option<String>,
    pub service_date: Option<String>,
    pub shipping_address: Option<String>,
    pub shipping_carrier: Option<String>,
    pub shipping_date: Option<String>,
    pub shipping_tracking_number: Option<String>,
    pub uncategorized_text: Option<String>,
}

/// Spec paths:
/// - `dispute_evidence_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisputeEvidenceDetails {
    pub due_by: Option<i64>,
    pub has_evidence: bool,
    pub past_due: bool,
    pub submission_count: i64,
}

/// Spec paths:
/// - `ephemeral_key`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EphemeralKey {
    pub object: UniStrObjectB95117,
    pub id: String,
    pub expires: i64,
    pub secret: Option<String>,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for EphemeralKey {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `ephemeral_key.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectB95117 {
    #[serde(rename = "ephemeral_key")]
    EphemeralKey,
}

/// Spec paths:
/// - `error`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Error {
    pub error: Box<APIErrors>,
}

/// Spec paths:
/// - `event`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationEvent {
    pub object: UniStrObject6D0693,
    #[serde(rename = "type")]
    pub type_x: String,
    pub id: String,
    pub account: Option<String>,
    pub api_version: Option<String>,
    pub data: NotificationEventData,
    pub pending_webhooks: i64,
    pub request: Option<NotificationEventRequest>,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for NotificationEvent {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `event.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject6D0693 {
    #[serde(rename = "event")]
    Event,
}

/// Spec paths:
/// - `exchange_rate`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExchangeRate {
    pub object: UniStrObject51F9DF,
    pub id: String,
    pub rates: Rates,
}

impl GetId for ExchangeRate {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `exchange_rate.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject51F9DF {
    #[serde(rename = "exchange_rate")]
    ExchangeRate,
}

/// Spec paths:
/// - `exchange_rate.rates`
pub type Rates = HashMap<String, f64>;

/// Spec paths:
/// - `fee`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fee {
    #[serde(rename = "type")]
    pub type_x: String,
    pub amount: i64,
    pub application: Option<String>,
    pub currency: String,
    pub description: Option<String>,
}

/// Spec paths:
/// - `fee_refund`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeeRefund {
    pub object: UniStrObjectC7795E,
    pub id: String,
    pub balance_transaction: Option<UniBalanceTransaction>,
    pub fee: UniFee,
    pub amount: i64,
    pub currency: String,
    pub created: i64,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for FeeRefund {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `fee_refund.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectC7795E {
    #[serde(rename = "fee_refund")]
    FeeRefund,
}

/// Spec paths:
/// - `file`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub object: UniStrObjectE55DBC,
    #[serde(rename = "type")]
    pub type_x: Option<String>,
    pub id: String,
    pub expires_at: Option<i64>,
    pub filename: Option<String>,
    pub links: Option<FileFileLinkList>,
    pub purpose: UniStrPurpose58AA54,
    pub size: i64,
    pub title: Option<String>,
    pub url: Option<String>,
    pub created: i64,
}

impl GetId for File {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `file.links`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileFileLinkList {
    pub object: UniStrObject344B0E,
    pub data: Vec<FileLink>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for FileFileLinkList {
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
/// - `file.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectE55DBC {
    #[serde(rename = "file")]
    File,
}

/// Spec paths:
/// - `file.purpose`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrPurpose58AA54 {
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
    #[serde(rename = "identity_document")]
    IdentityDocument,
    #[serde(rename = "pci_document")]
    PciDocument,
    #[serde(rename = "tax_document_user_upload")]
    TaxDocumentUserUpload,
}

/// Spec paths:
/// - `file_link`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileLink {
    pub object: UniStrObject325187,
    pub id: String,
    pub file: UniFile5BD414,
    pub expired: bool,
    pub expires_at: Option<i64>,
    pub url: Option<String>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for FileLink {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `file_link.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject325187 {
    #[serde(rename = "file_link")]
    FileLink,
}

/// Spec paths:
/// - `financial_reporting_finance_report_run_run_parameters`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FinancialReportingFinanceReportRunRunParameters {
    pub columns: Option<Vec<String>>,
    pub connected_account: Option<String>,
    pub currency: Option<String>,
    pub interval_end: Option<i64>,
    pub interval_start: Option<i64>,
    pub payout: Option<String>,
    pub reporting_category: Option<String>,
    pub timezone: Option<String>,
}

/// Spec paths:
/// - `inventory`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    #[serde(rename = "type")]
    pub type_x: String,
    pub quantity: Option<i64>,
    pub value: Option<String>,
}

/// Spec paths:
/// - `invoice`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invoice {
    pub object: UniStrObject5E6A4A,
    pub amount_paid: i64,
    pub id: Option<String>,
    pub paid: bool,
    pub charge: Option<UniCharge>,
    pub customer: UniCustomerC00F6E,
    pub default_payment_method: Option<UniPaymentMethod>,
    pub default_source: Box<Option<UniDefaultSource>>,
    pub discounts: Option<Vec<UniItems6F859C>>,
    pub payment_intent: Option<UniPaymentIntent>,
    pub subscription: Option<UniSubscription>,
    pub account_country: Option<String>,
    pub account_name: Option<String>,
    pub amount_due: i64,
    pub amount_remaining: i64,
    pub application_fee_amount: Option<i64>,
    pub attempt_count: i64,
    pub attempted: bool,
    pub auto_advance: Option<bool>,
    pub billing_reason: Option<UniStrBillingReason>,
    pub collection_method: Option<UniStrCollectionMethod>,
    pub currency: String,
    pub custom_fields: Option<Vec<InvoiceSettingCustomField>>,
    pub customer_address: Option<Address>,
    pub customer_email: Option<String>,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub customer_shipping: Option<Shipping>,
    pub customer_tax_exempt: Option<UniStrTaxExempt>,
    pub customer_tax_ids: Option<Vec<InvoicesResourceInvoiceTaxID>>,
    pub default_tax_rates: Vec<TaxRate>,
    pub description: Option<String>,
    pub discount: Option<Discount>,
    pub due_date: Option<i64>,
    pub ending_balance: Option<i64>,
    pub footer: Option<String>,
    pub hosted_invoice_url: Option<String>,
    pub invoice_pdf: Option<String>,
    pub last_finalization_error: Box<Option<APIErrors>>,
    pub lines: InvoiceLinesList95797E,
    pub next_payment_attempt: Option<i64>,
    pub number: Option<String>,
    pub period_end: i64,
    pub period_start: i64,
    pub post_payment_credit_notes_amount: i64,
    pub pre_payment_credit_notes_amount: i64,
    pub receipt_number: Option<String>,
    pub starting_balance: i64,
    pub statement_descriptor: Option<String>,
    pub status: Option<UniStrStatusDA9FBD>,
    pub status_transitions: InvoicesStatusTransitions,
    pub subscription_proration_date: Option<i64>,
    pub subtotal: i64,
    pub tax: Option<i64>,
    pub threshold_reason: Option<InvoiceThresholdReason>,
    pub total: i64,
    pub total_discount_amounts: Option<Vec<DiscountsResourceDiscountAmount>>,
    pub total_tax_amounts: Vec<InvoiceTaxAmount>,
    pub transfer_data: Option<InvoiceTransferData>,
    pub webhooks_delivered_at: Option<i64>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for Invoice {
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()
    }
}

/// Spec paths:
/// - `invoice.billing_reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrBillingReason {
    #[serde(rename = "automatic_pending_invoice_item_invoice")]
    AutomaticPendingInvoiceItemInvoice,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "subscription")]
    Subscription,
    #[serde(rename = "subscription_create")]
    SubscriptionCreate,
    #[serde(rename = "subscription_cycle")]
    SubscriptionCycle,
    #[serde(rename = "subscription_threshold")]
    SubscriptionThreshold,
    #[serde(rename = "subscription_update")]
    SubscriptionUpdate,
    #[serde(rename = "upcoming")]
    Upcoming,
}

/// Spec paths:
/// - `invoice.default_payment_method`
/// - `invoice_setting_customer_setting.default_payment_method`
/// - `mandate.payment_method`
/// - `payment_intent.payment_method`
/// - `payment_method_details_bancontact.generated_sepa_debit`
/// - `payment_method_details_ideal.generated_sepa_debit`
/// - `payment_method_details_sofort.generated_sepa_debit`
/// - `setup_attempt.payment_method`
/// - `setup_attempt_payment_method_details_bancontact.generated_sepa_debit`
/// - `setup_attempt_payment_method_details_ideal.generated_sepa_debit`
/// - `setup_attempt_payment_method_details_sofort.generated_sepa_debit`
/// - `setup_intent.payment_method`
/// - `subscription.default_payment_method`
/// - `subscription_schedule_phase_configuration.default_payment_method`
/// - `subscription_schedules_resource_default_settings.default_payment_method`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniPaymentMethod {
    String(String),
    PaymentMethod(Box<PaymentMethod>),
}

/// Spec paths:
/// - `invoice.lines`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceLinesList95797E {
    pub object: UniStrObject344B0E,
    pub data: Vec<InvoiceLineItem>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for InvoiceLinesList95797E {
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
/// - `invoice.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatusDA9FBD {
    #[serde(rename = "deleted")]
    Deleted,
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
/// - `invoice_item_threshold_reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceItemThresholdReason {
    pub line_item_ids: Vec<String>,
    pub usage_gte: i64,
}

/// Spec paths:
/// - `invoice_line_item_period`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceLineItemPeriod {
    pub end: i64,
    pub start: i64,
}

/// Spec paths:
/// - `invoice_setting_custom_field`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceSettingCustomField {
    pub name: String,
    pub value: String,
}

/// Spec paths:
/// - `invoice_setting_customer_setting`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceSettingCustomerSetting {
    pub default_payment_method: Option<UniPaymentMethod>,
    pub custom_fields: Option<Vec<InvoiceSettingCustomField>>,
    pub footer: Option<String>,
}

/// Spec paths:
/// - `invoice_setting_subscription_schedule_setting`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceSettingSubscriptionScheduleSetting {
    pub days_until_due: Option<i64>,
}

/// Spec paths:
/// - `invoice_tax_amount`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceTaxAmount {
    pub tax_rate: UniTaxRate,
    pub amount: i64,
    pub inclusive: bool,
}

/// Spec paths:
/// - `invoice_threshold_reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceThresholdReason {
    pub amount_gte: Option<i64>,
    pub item_reasons: Vec<InvoiceItemThresholdReason>,
}

/// Spec paths:
/// - `invoice_transfer_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceTransferData {
    pub destination: UniAccount,
    pub amount: Option<i64>,
}

/// Spec paths:
/// - `invoiceitem`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceItem {
    pub object: UniStrObjectA93110,
    pub id: String,
    pub customer: UniCustomerC00F6E,
    pub discounts: Option<Vec<UniItemsE47473>>,
    pub invoice: Option<UniInvoice>,
    pub subscription: Option<UniSubscription>,
    pub amount: i64,
    pub currency: String,
    pub date: i64,
    pub description: Option<String>,
    pub discountable: bool,
    pub period: InvoiceLineItemPeriod,
    pub price: Option<Price>,
    pub proration: bool,
    pub quantity: i64,
    pub subscription_item: Option<String>,
    pub tax_rates: Option<Vec<TaxRate>>,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for InvoiceItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `invoiceitem.discounts.items`
/// - `line_item.discounts.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniItemsE47473 {
    String(String),
    Discount(Discount),
}

/// Spec paths:
/// - `invoices_resource_invoice_tax_id`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoicesResourceInvoiceTaxID {
    #[serde(rename = "type")]
    pub type_x: UniStrType805F73,
    pub value: Option<String>,
}

/// Spec paths:
/// - `invoices_resource_invoice_tax_id.type`
/// - `tax_id.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrType805F73 {
    #[serde(rename = "ae_trn")]
    AeTrn,
    #[serde(rename = "au_abn")]
    AuAbn,
    #[serde(rename = "br_cnpj")]
    BrCnpj,
    #[serde(rename = "br_cpf")]
    BrCpf,
    #[serde(rename = "ca_bn")]
    CaBn,
    #[serde(rename = "ca_qst")]
    CaQst,
    #[serde(rename = "ch_vat")]
    ChVat,
    #[serde(rename = "cl_tin")]
    ClTin,
    #[serde(rename = "es_cif")]
    EsCif,
    #[serde(rename = "eu_vat")]
    EuVat,
    #[serde(rename = "hk_br")]
    HkBr,
    #[serde(rename = "id_npwp")]
    IdNpwp,
    #[serde(rename = "in_gst")]
    InGst,
    #[serde(rename = "jp_cn")]
    JpCn,
    #[serde(rename = "jp_rn")]
    JpRn,
    #[serde(rename = "kr_brn")]
    KrBrn,
    #[serde(rename = "li_uid")]
    LiUid,
    #[serde(rename = "mx_rfc")]
    MxRfc,
    #[serde(rename = "my_frp")]
    MyFrp,
    #[serde(rename = "my_itn")]
    MyItn,
    #[serde(rename = "my_sst")]
    MySst,
    #[serde(rename = "no_vat")]
    NoVat,
    #[serde(rename = "nz_gst")]
    NzGst,
    #[serde(rename = "ru_inn")]
    RuInn,
    #[serde(rename = "ru_kpp")]
    RuKpp,
    #[serde(rename = "sa_vat")]
    SaVat,
    #[serde(rename = "sg_gst")]
    SgGst,
    #[serde(rename = "sg_uen")]
    SgUen,
    #[serde(rename = "th_vat")]
    ThVat,
    #[serde(rename = "tw_vat")]
    TwVat,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "us_ein")]
    UsEin,
    #[serde(rename = "za_vat")]
    ZaVat,
}

/// Spec paths:
/// - `invoices_status_transitions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoicesStatusTransitions {
    pub finalized_at: Option<i64>,
    pub marked_uncollectible_at: Option<i64>,
    pub paid_at: Option<i64>,
    pub voided_at: Option<i64>,
}

/// Spec paths:
/// - `issuer_fraud_record`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuerFraudRecord {
    pub object: UniStrObject94010F,
    pub id: String,
    pub charge: UniCharge,
    pub actionable: bool,
    pub fraud_type: String,
    pub has_liability_shift: bool,
    pub post_date: i64,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for IssuerFraudRecord {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `issuer_fraud_record.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject94010F {
    #[serde(rename = "issuer_fraud_record")]
    IssuerFraudRecord,
}

/// Spec paths:
/// - `issuing.authorization`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingAuthorization {
    pub object: UniStrObjectE07375,
    pub id: String,
    pub cardholder: Option<UniCardholder>,
    pub amount: i64,
    pub amount_details: Option<IssuingAuthorizationAmountDetails>,
    pub approved: bool,
    pub authorization_method: UniStrAuthorizationMethod,
    pub balance_transactions: Vec<BalanceTransaction>,
    pub card: Box<IssuingCard>,
    pub currency: String,
    pub merchant_amount: i64,
    pub merchant_currency: String,
    pub merchant_data: IssuingAuthorizationMerchantData,
    pub pending_request: Option<IssuingAuthorizationPendingRequest>,
    pub request_history: Vec<IssuingAuthorizationRequest>,
    pub status: UniStrStatus957169,
    pub transactions: Vec<IssuingTransaction>,
    pub verification_data: IssuingAuthorizationVerificationData,
    pub wallet: Option<String>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for IssuingAuthorization {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `issuing.authorization.authorization_method`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrAuthorizationMethod {
    #[serde(rename = "chip")]
    Chip,
    #[serde(rename = "contactless")]
    Contactless,
    #[serde(rename = "keyed_in")]
    KeyedIn,
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "swipe")]
    Swipe,
}

/// Spec paths:
/// - `issuing.authorization.cardholder`
/// - `issuing.transaction.cardholder`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniCardholder {
    String(String),
    IssuingCardholder(IssuingCardholder),
}

/// Spec paths:
/// - `issuing.authorization.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectE07375 {
    #[serde(rename = "issuing.authorization")]
    IssuingDotAuthorization,
}

/// Spec paths:
/// - `issuing.card`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCard {
    pub object: UniStrObjectDBA102,
    #[serde(rename = "type")]
    pub type_x: UniStrTypeA467AF,
    pub id: String,
    pub replaced_by: Option<UniReplacedBy>,
    pub replacement_for: Option<UniReplacedBy>,
    pub brand: String,
    pub cancellation_reason: Option<UniStrCancellationReason6C5A3F>,
    pub cardholder: IssuingCardholder,
    pub currency: String,
    pub cvc: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub last4: String,
    pub number: Option<String>,
    pub replacement_reason: Option<UniStrReplacementReason>,
    pub shipping: Option<IssuingCardShipping>,
    pub spending_controls: IssuingCardAuthorizationControls,
    pub status: UniStrStatusA4138B,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for IssuingCard {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `issuing.card.cancellation_reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrCancellationReason6C5A3F {
    #[serde(rename = "lost")]
    Lost,
    #[serde(rename = "stolen")]
    Stolen,
}

/// Spec paths:
/// - `issuing.card.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectDBA102 {
    #[serde(rename = "issuing.card")]
    IssuingDotCard,
}

/// Spec paths:
/// - `issuing.card.replaced_by`
/// - `issuing.card.replacement_for`
/// - `issuing.transaction.card`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniReplacedBy {
    String(String),
    IssuingCard(Box<IssuingCard>),
}

/// Spec paths:
/// - `issuing.card.replacement_reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrReplacementReason {
    #[serde(rename = "damaged")]
    Damaged,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "lost")]
    Lost,
    #[serde(rename = "stolen")]
    Stolen,
}

/// Spec paths:
/// - `issuing.cardholder`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardholder {
    pub object: UniStrObject9677B5,
    #[serde(rename = "type")]
    pub type_x: UniStrType947A77,
    pub id: String,
    pub name: String,
    pub billing: IssuingCardholderAddress,
    pub company: Option<IssuingCardholderCompany>,
    pub email: Option<String>,
    pub individual: Option<IssuingCardholderIndividual>,
    pub phone_number: Option<String>,
    pub requirements: IssuingCardholderRequirements,
    pub spending_controls: Option<IssuingCardholderAuthorizationControls>,
    pub status: UniStrStatusD5D208,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for IssuingCardholder {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `issuing.cardholder.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject9677B5 {
    #[serde(rename = "issuing.cardholder")]
    IssuingDotCardholder,
}

/// Spec paths:
/// - `issuing.dispute`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingDispute {
    pub object: UniStrObjectB5E468,
    pub id: String,
    pub transaction: UniTransaction,
    pub amount: Option<i64>,
    pub balance_transactions: Option<Vec<BalanceTransaction>>,
    pub currency: Option<String>,
    pub evidence: Option<IssuingDisputeEvidence>,
    pub status: Option<UniStrStatusE71251>,
    pub created: Option<i64>,
    pub livemode: bool,
    pub metadata: Option<Metadata8076DB>,
}

impl GetId for IssuingDispute {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `issuing.dispute.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectB5E468 {
    #[serde(rename = "issuing.dispute")]
    IssuingDotDispute,
}

/// Spec paths:
/// - `issuing.dispute.transaction`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniTransaction {
    String(String),
    IssuingTransaction(Box<IssuingTransaction>),
}

/// Spec paths:
/// - `issuing.settlement`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingSettlement {
    pub object: UniStrObject0316F5,
    pub id: String,
    pub bin: String,
    pub clearing_date: i64,
    pub currency: String,
    pub interchange_fees: i64,
    pub net_total: i64,
    pub network: UniStrNetwork143EDB,
    pub network_fees: i64,
    pub network_settlement_identifier: String,
    pub settlement_service: String,
    pub transaction_count: i64,
    pub transaction_volume: i64,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for IssuingSettlement {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `issuing.settlement.network`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrNetwork143EDB {
    #[serde(rename = "visa")]
    Visa,
}

/// Spec paths:
/// - `issuing.settlement.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject0316F5 {
    #[serde(rename = "issuing.settlement")]
    IssuingDotSettlement,
}

/// Spec paths:
/// - `issuing.transaction`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingTransaction {
    pub object: UniStrObjectE479DB,
    #[serde(rename = "type")]
    pub type_x: UniStrType39A4A4,
    pub id: String,
    pub authorization: Option<UniAuthorization>,
    pub balance_transaction: Option<UniBalanceTransaction>,
    pub card: UniReplacedBy,
    pub cardholder: Option<UniCardholder>,
    pub dispute: Option<UniDispute>,
    pub amount: i64,
    pub amount_details: Option<IssuingTransactionAmountDetails>,
    pub currency: String,
    pub merchant_amount: i64,
    pub merchant_currency: String,
    pub merchant_data: IssuingAuthorizationMerchantData,
    pub purchase_details: Option<IssuingTransactionPurchaseDetails>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for IssuingTransaction {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `issuing.transaction.authorization`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniAuthorization {
    String(String),
    IssuingAuthorization(IssuingAuthorization),
}

/// Spec paths:
/// - `issuing.transaction.dispute`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniDispute {
    String(String),
    IssuingDispute(IssuingDispute),
}

/// Spec paths:
/// - `issuing.transaction.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectE479DB {
    #[serde(rename = "issuing.transaction")]
    IssuingDotTransaction,
}

/// Spec paths:
/// - `issuing.transaction.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrType39A4A4 {
    #[serde(rename = "capture")]
    Capture,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "refund")]
    Refund,
}

/// Spec paths:
/// - `issuing_authorization_amount_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingAuthorizationAmountDetails {
    pub atm_fee: Option<i64>,
}

/// Spec paths:
/// - `issuing_authorization_merchant_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingAuthorizationMerchantData {
    pub network_id: String,
    pub name: Option<String>,
    pub category: String,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

/// Spec paths:
/// - `issuing_authorization_pending_request`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingAuthorizationPendingRequest {
    pub amount: i64,
    pub amount_details: Option<IssuingAuthorizationAmountDetails>,
    pub currency: String,
    pub is_amount_controllable: bool,
    pub merchant_amount: i64,
    pub merchant_currency: String,
}

/// Spec paths:
/// - `issuing_authorization_request`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingAuthorizationRequest {
    pub amount: i64,
    pub amount_details: Option<IssuingAuthorizationAmountDetails>,
    pub approved: bool,
    pub currency: String,
    pub merchant_amount: i64,
    pub merchant_currency: String,
    pub reason: UniStrReasonEDBD38,
    pub created: i64,
}

/// Spec paths:
/// - `issuing_authorization_request.reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrReasonEDBD38 {
    #[serde(rename = "account_disabled")]
    AccountDisabled,
    #[serde(rename = "card_active")]
    CardActive,
    #[serde(rename = "card_inactive")]
    CardInactive,
    #[serde(rename = "cardholder_inactive")]
    CardholderInactive,
    #[serde(rename = "cardholder_verification_required")]
    CardholderVerificationRequired,
    #[serde(rename = "insufficient_funds")]
    InsufficientFunds,
    #[serde(rename = "not_allowed")]
    NotAllowed,
    #[serde(rename = "spending_controls")]
    SpendingControls,
    #[serde(rename = "suspected_fraud")]
    SuspectedFraud,
    #[serde(rename = "verification_failed")]
    VerificationFailed,
    #[serde(rename = "webhook_approved")]
    WebhookApproved,
    #[serde(rename = "webhook_declined")]
    WebhookDeclined,
    #[serde(rename = "webhook_timeout")]
    WebhookTimeout,
}

/// Spec paths:
/// - `issuing_authorization_verification_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingAuthorizationVerificationData {
    pub address_line1_check: UniStrCvcCheck,
    pub address_postal_code_check: UniStrCvcCheck,
    pub cvc_check: UniStrCvcCheck,
    pub expiry_check: UniStrCvcCheck,
}

/// Spec paths:
/// - `issuing_authorization_verification_data.address_line1_check`
/// - `issuing_authorization_verification_data.address_postal_code_check`
/// - `issuing_authorization_verification_data.cvc_check`
/// - `issuing_authorization_verification_data.expiry_check`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrCvcCheck {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "mismatch")]
    Mismatch,
    #[serde(rename = "not_provided")]
    NotProvided,
}

/// Spec paths:
/// - `issuing_card_authorization_controls`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardAuthorizationControls {
    pub allowed_categories: Option<Vec<UniStrItems6A79F4>>,
    pub blocked_categories: Option<Vec<UniStrItems6A79F4>>,
    pub spending_limits: Option<Vec<IssuingCardSpendingLimit>>,
    pub spending_limits_currency: Option<String>,
}

/// Spec paths:
/// - `issuing_card_authorization_controls.allowed_categories.items`
/// - `issuing_card_authorization_controls.blocked_categories.items`
/// - `issuing_card_spending_limit.categories.items`
/// - `issuing_cardholder_authorization_controls.allowed_categories.items`
/// - `issuing_cardholder_authorization_controls.blocked_categories.items`
/// - `issuing_cardholder_spending_limit.categories.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrItems6A79F4 {
    #[serde(rename = "ac_refrigeration_repair")]
    AcRefrigerationRepair,
    #[serde(rename = "accounting_bookkeeping_services")]
    AccountingBookkeepingServices,
    #[serde(rename = "advertising_services")]
    AdvertisingServices,
    #[serde(rename = "agricultural_cooperative")]
    AgriculturalCooperative,
    #[serde(rename = "airlines_air_carriers")]
    AirlinesAirCarriers,
    #[serde(rename = "airports_flying_fields")]
    AirportsFlyingFields,
    #[serde(rename = "ambulance_services")]
    AmbulanceServices,
    #[serde(rename = "amusement_parks_carnivals")]
    AmusementParksCarnivals,
    #[serde(rename = "antique_reproductions")]
    AntiqueReproductions,
    #[serde(rename = "antique_shops")]
    AntiqueShops,
    #[serde(rename = "aquariums")]
    Aquariums,
    #[serde(rename = "architectural_surveying_services")]
    ArchitecturalSurveyingServices,
    #[serde(rename = "art_dealers_and_galleries")]
    ArtDealersAndGalleries,
    #[serde(rename = "artists_supply_and_craft_shops")]
    ArtistsSupplyAndCraftShops,
    #[serde(rename = "auto_and_home_supply_stores")]
    AutoAndHomeSupplyStores,
    #[serde(rename = "auto_body_repair_shops")]
    AutoBodyRepairShops,
    #[serde(rename = "auto_paint_shops")]
    AutoPaintShops,
    #[serde(rename = "auto_service_shops")]
    AutoServiceShops,
    #[serde(rename = "automated_cash_disburse")]
    AutomatedCashDisburse,
    #[serde(rename = "automated_fuel_dispensers")]
    AutomatedFuelDispensers,
    #[serde(rename = "automobile_associations")]
    AutomobileAssociations,
    #[serde(rename = "automotive_parts_and_accessories_stores")]
    AutomotivePartsAndAccessoriesStores,
    #[serde(rename = "automotive_tire_stores")]
    AutomotiveTireStores,
    #[serde(rename = "bail_and_bond_payments")]
    BailAndBondPayments,
    #[serde(rename = "bakeries")]
    Bakeries,
    #[serde(rename = "bands_orchestras")]
    BandsOrchestras,
    #[serde(rename = "barber_and_beauty_shops")]
    BarberAndBeautyShops,
    #[serde(rename = "betting_casino_gambling")]
    BettingCasinoGambling,
    #[serde(rename = "bicycle_shops")]
    BicycleShops,
    #[serde(rename = "billiard_pool_establishments")]
    BilliardPoolEstablishments,
    #[serde(rename = "boat_dealers")]
    BoatDealers,
    #[serde(rename = "boat_rentals_and_leases")]
    BoatRentalsAndLeases,
    #[serde(rename = "book_stores")]
    BookStores,
    #[serde(rename = "books_periodicals_and_newspapers")]
    BooksPeriodicalsAndNewspapers,
    #[serde(rename = "bowling_alleys")]
    BowlingAlleys,
    #[serde(rename = "bus_lines")]
    BusLines,
    #[serde(rename = "business_secretarial_schools")]
    BusinessSecretarialSchools,
    #[serde(rename = "buying_shopping_services")]
    BuyingShoppingServices,
    #[serde(rename = "cable_satellite_and_other_pay_television_and_radio")]
    CableSatelliteAndOtherPayTelevisionAndRadio,
    #[serde(rename = "camera_and_photographic_supply_stores")]
    CameraAndPhotographicSupplyStores,
    #[serde(rename = "candy_nut_and_confectionery_stores")]
    CandyNutAndConfectioneryStores,
    #[serde(rename = "car_and_truck_dealers_new_used")]
    CarAndTruckDealersNewUsed,
    #[serde(rename = "car_and_truck_dealers_used_only")]
    CarAndTruckDealersUsedOnly,
    #[serde(rename = "car_rental_agencies")]
    CarRentalAgencies,
    #[serde(rename = "car_washes")]
    CarWashes,
    #[serde(rename = "carpentry_services")]
    CarpentryServices,
    #[serde(rename = "carpet_upholstery_cleaning")]
    CarpetUpholsteryCleaning,
    #[serde(rename = "caterers")]
    Caterers,
    #[serde(rename = "charitable_and_social_service_organizations_fundraising")]
    CharitableAndSocialServiceOrganizationsFundraising,
    #[serde(rename = "chemicals_and_allied_products")]
    ChemicalsAndAlliedProducts,
    #[serde(rename = "child_care_services")]
    ChildCareServices,
    #[serde(rename = "childrens_and_infants_wear_stores")]
    ChildrensAndInfantsWearStores,
    #[serde(rename = "chiropodists_podiatrists")]
    ChiropodistsPodiatrists,
    #[serde(rename = "chiropractors")]
    Chiropractors,
    #[serde(rename = "cigar_stores_and_stands")]
    CigarStoresAndStands,
    #[serde(rename = "civic_social_fraternal_associations")]
    CivicSocialFraternalAssociations,
    #[serde(rename = "cleaning_and_maintenance")]
    CleaningAndMaintenance,
    #[serde(rename = "clothing_rental")]
    ClothingRental,
    #[serde(rename = "colleges_universities")]
    CollegesUniversities,
    #[serde(rename = "commercial_equipment")]
    CommercialEquipment,
    #[serde(rename = "commercial_footwear")]
    CommercialFootwear,
    #[serde(rename = "commercial_photography_art_and_graphics")]
    CommercialPhotographyArtAndGraphics,
    #[serde(rename = "commuter_transport_and_ferries")]
    CommuterTransportAndFerries,
    #[serde(rename = "computer_network_services")]
    ComputerNetworkServices,
    #[serde(rename = "computer_programming")]
    ComputerProgramming,
    #[serde(rename = "computer_repair")]
    ComputerRepair,
    #[serde(rename = "computer_software_stores")]
    ComputerSoftwareStores,
    #[serde(rename = "computers_peripherals_and_software")]
    ComputersPeripheralsAndSoftware,
    #[serde(rename = "concrete_work_services")]
    ConcreteWorkServices,
    #[serde(rename = "construction_materials")]
    ConstructionMaterials,
    #[serde(rename = "consulting_public_relations")]
    ConsultingPublicRelations,
    #[serde(rename = "correspondence_schools")]
    CorrespondenceSchools,
    #[serde(rename = "cosmetic_stores")]
    CosmeticStores,
    #[serde(rename = "counseling_services")]
    CounselingServices,
    #[serde(rename = "country_clubs")]
    CountryClubs,
    #[serde(rename = "courier_services")]
    CourierServices,
    #[serde(rename = "court_costs")]
    CourtCosts,
    #[serde(rename = "credit_reporting_agencies")]
    CreditReportingAgencies,
    #[serde(rename = "cruise_lines")]
    CruiseLines,
    #[serde(rename = "dairy_products_stores")]
    DairyProductsStores,
    #[serde(rename = "dance_hall_studios_schools")]
    DanceHallStudiosSchools,
    #[serde(rename = "dating_escort_services")]
    DatingEscortServices,
    #[serde(rename = "dentists_orthodontists")]
    DentistsOrthodontists,
    #[serde(rename = "department_stores")]
    DepartmentStores,
    #[serde(rename = "detective_agencies")]
    DetectiveAgencies,
    #[serde(rename = "digital_goods_applications")]
    DigitalGoodsApplications,
    #[serde(rename = "digital_goods_games")]
    DigitalGoodsGames,
    #[serde(rename = "digital_goods_large_volume")]
    DigitalGoodsLargeVolume,
    #[serde(rename = "digital_goods_media")]
    DigitalGoodsMedia,
    #[serde(rename = "direct_marketing_catalog_merchant")]
    DirectMarketingCatalogMerchant,
    #[serde(rename = "direct_marketing_combination_catalog_and_retail_merchant")]
    DirectMarketingCombinationCatalogAndRetailMerchant,
    #[serde(rename = "direct_marketing_inbound_telemarketing")]
    DirectMarketingInboundTelemarketing,
    #[serde(rename = "direct_marketing_insurance_services")]
    DirectMarketingInsuranceServices,
    #[serde(rename = "direct_marketing_other")]
    DirectMarketingOther,
    #[serde(rename = "direct_marketing_outbound_telemarketing")]
    DirectMarketingOutboundTelemarketing,
    #[serde(rename = "direct_marketing_subscription")]
    DirectMarketingSubscription,
    #[serde(rename = "direct_marketing_travel")]
    DirectMarketingTravel,
    #[serde(rename = "discount_stores")]
    DiscountStores,
    #[serde(rename = "doctors")]
    Doctors,
    #[serde(rename = "door_to_door_sales")]
    DoorToDoorSales,
    #[serde(rename = "drapery_window_covering_and_upholstery_stores")]
    DraperyWindowCoveringAndUpholsteryStores,
    #[serde(rename = "drinking_places")]
    DrinkingPlaces,
    #[serde(rename = "drug_stores_and_pharmacies")]
    DrugStoresAndPharmacies,
    #[serde(rename = "drugs_drug_proprietaries_and_druggist_sundries")]
    DrugsDrugProprietariesAndDruggistSundries,
    #[serde(rename = "dry_cleaners")]
    DryCleaners,
    #[serde(rename = "durable_goods")]
    DurableGoods,
    #[serde(rename = "duty_free_stores")]
    DutyFreeStores,
    #[serde(rename = "eating_places_restaurants")]
    EatingPlacesRestaurants,
    #[serde(rename = "educational_services")]
    EducationalServices,
    #[serde(rename = "electric_razor_stores")]
    ElectricRazorStores,
    #[serde(rename = "electrical_parts_and_equipment")]
    ElectricalPartsAndEquipment,
    #[serde(rename = "electrical_services")]
    ElectricalServices,
    #[serde(rename = "electronics_repair_shops")]
    ElectronicsRepairShops,
    #[serde(rename = "electronics_stores")]
    ElectronicsStores,
    #[serde(rename = "elementary_secondary_schools")]
    ElementarySecondarySchools,
    #[serde(rename = "employment_temp_agencies")]
    EmploymentTempAgencies,
    #[serde(rename = "equipment_rental")]
    EquipmentRental,
    #[serde(rename = "exterminating_services")]
    ExterminatingServices,
    #[serde(rename = "family_clothing_stores")]
    FamilyClothingStores,
    #[serde(rename = "fast_food_restaurants")]
    FastFoodRestaurants,
    #[serde(rename = "financial_institutions")]
    FinancialInstitutions,
    #[serde(rename = "fines_government_administrative_entities")]
    FinesGovernmentAdministrativeEntities,
    #[serde(rename = "fireplace_fireplace_screens_and_accessories_stores")]
    FireplaceFireplaceScreensAndAccessoriesStores,
    #[serde(rename = "floor_covering_stores")]
    FloorCoveringStores,
    #[serde(rename = "florists")]
    Florists,
    #[serde(rename = "florists_supplies_nursery_stock_and_flowers")]
    FloristsSuppliesNurseryStockAndFlowers,
    #[serde(rename = "freezer_and_locker_meat_provisioners")]
    FreezerAndLockerMeatProvisioners,
    #[serde(rename = "fuel_dealers_non_automotive")]
    FuelDealersNonAutomotive,
    #[serde(rename = "funeral_services_crematories")]
    FuneralServicesCrematories,
    #[serde(rename = "furniture_home_furnishings_and_equipment_stores_except_appliances")]
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    #[serde(rename = "furniture_repair_refinishing")]
    FurnitureRepairRefinishing,
    #[serde(rename = "furriers_and_fur_shops")]
    FurriersAndFurShops,
    #[serde(rename = "general_services")]
    GeneralServices,
    #[serde(rename = "gift_card_novelty_and_souvenir_shops")]
    GiftCardNoveltyAndSouvenirShops,
    #[serde(rename = "glass_paint_and_wallpaper_stores")]
    GlassPaintAndWallpaperStores,
    #[serde(rename = "glassware_crystal_stores")]
    GlasswareCrystalStores,
    #[serde(rename = "golf_courses_public")]
    GolfCoursesPublic,
    #[serde(rename = "government_services")]
    GovernmentServices,
    #[serde(rename = "grocery_stores_supermarkets")]
    GroceryStoresSupermarkets,
    #[serde(rename = "hardware_equipment_and_supplies")]
    HardwareEquipmentAndSupplies,
    #[serde(rename = "hardware_stores")]
    HardwareStores,
    #[serde(rename = "health_and_beauty_spas")]
    HealthAndBeautySpas,
    #[serde(rename = "hearing_aids_sales_and_supplies")]
    HearingAidsSalesAndSupplies,
    #[serde(rename = "heating_plumbing_a_c")]
    HeatingPlumbingAC,
    #[serde(rename = "hobby_toy_and_game_shops")]
    HobbyToyAndGameShops,
    #[serde(rename = "home_supply_warehouse_stores")]
    HomeSupplyWarehouseStores,
    #[serde(rename = "hospitals")]
    Hospitals,
    #[serde(rename = "hotels_motels_and_resorts")]
    HotelsMotelsAndResorts,
    #[serde(rename = "household_appliance_stores")]
    HouseholdApplianceStores,
    #[serde(rename = "industrial_supplies")]
    IndustrialSupplies,
    #[serde(rename = "information_retrieval_services")]
    InformationRetrievalServices,
    #[serde(rename = "insurance_default")]
    InsuranceDefault,
    #[serde(rename = "insurance_underwriting_premiums")]
    InsuranceUnderwritingPremiums,
    #[serde(rename = "intra_company_purchases")]
    IntraCompanyPurchases,
    #[serde(rename = "jewelry_stores_watches_clocks_and_silverware_stores")]
    JewelryStoresWatchesClocksAndSilverwareStores,
    #[serde(rename = "landscaping_services")]
    LandscapingServices,
    #[serde(rename = "laundries")]
    Laundries,
    #[serde(rename = "laundry_cleaning_services")]
    LaundryCleaningServices,
    #[serde(rename = "legal_services_attorneys")]
    LegalServicesAttorneys,
    #[serde(rename = "luggage_and_leather_goods_stores")]
    LuggageAndLeatherGoodsStores,
    #[serde(rename = "lumber_building_materials_stores")]
    LumberBuildingMaterialsStores,
    #[serde(rename = "manual_cash_disburse")]
    ManualCashDisburse,
    #[serde(rename = "marinas_service_and_supplies")]
    MarinasServiceAndSupplies,
    #[serde(rename = "masonry_stonework_and_plaster")]
    MasonryStoneworkAndPlaster,
    #[serde(rename = "massage_parlors")]
    MassageParlors,
    #[serde(rename = "medical_and_dental_labs")]
    MedicalAndDentalLabs,
    #[serde(rename = "medical_dental_ophthalmic_and_hospital_equipment_and_supplies")]
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    #[serde(rename = "medical_services")]
    MedicalServices,
    #[serde(rename = "membership_organizations")]
    MembershipOrganizations,
    #[serde(rename = "mens_and_boys_clothing_and_accessories_stores")]
    MensAndBoysClothingAndAccessoriesStores,
    #[serde(rename = "mens_womens_clothing_stores")]
    MensWomensClothingStores,
    #[serde(rename = "metal_service_centers")]
    MetalServiceCenters,
    #[serde(rename = "miscellaneous")]
    Miscellaneous,
    #[serde(rename = "miscellaneous_apparel_and_accessory_shops")]
    MiscellaneousApparelAndAccessoryShops,
    #[serde(rename = "miscellaneous_auto_dealers")]
    MiscellaneousAutoDealers,
    #[serde(rename = "miscellaneous_business_services")]
    MiscellaneousBusinessServices,
    #[serde(rename = "miscellaneous_food_stores")]
    MiscellaneousFoodStores,
    #[serde(rename = "miscellaneous_general_merchandise")]
    MiscellaneousGeneralMerchandise,
    #[serde(rename = "miscellaneous_general_services")]
    MiscellaneousGeneralServices,
    #[serde(rename = "miscellaneous_home_furnishing_specialty_stores")]
    MiscellaneousHomeFurnishingSpecialtyStores,
    #[serde(rename = "miscellaneous_publishing_and_printing")]
    MiscellaneousPublishingAndPrinting,
    #[serde(rename = "miscellaneous_recreation_services")]
    MiscellaneousRecreationServices,
    #[serde(rename = "miscellaneous_repair_shops")]
    MiscellaneousRepairShops,
    #[serde(rename = "miscellaneous_specialty_retail")]
    MiscellaneousSpecialtyRetail,
    #[serde(rename = "mobile_home_dealers")]
    MobileHomeDealers,
    #[serde(rename = "motion_picture_theaters")]
    MotionPictureTheaters,
    #[serde(rename = "motor_freight_carriers_and_trucking")]
    MotorFreightCarriersAndTrucking,
    #[serde(rename = "motor_homes_dealers")]
    MotorHomesDealers,
    #[serde(rename = "motor_vehicle_supplies_and_new_parts")]
    MotorVehicleSuppliesAndNewParts,
    #[serde(rename = "motorcycle_shops_and_dealers")]
    MotorcycleShopsAndDealers,
    #[serde(rename = "motorcycle_shops_dealers")]
    MotorcycleShopsDealers,
    #[serde(rename = "music_stores_musical_instruments_pianos_and_sheet_music")]
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    #[serde(rename = "news_dealers_and_newsstands")]
    NewsDealersAndNewsstands,
    #[serde(rename = "non_fi_money_orders")]
    NonFiMoneyOrders,
    #[serde(rename = "non_fi_stored_value_card_purchase_load")]
    NonFiStoredValueCardPurchaseLoad,
    #[serde(rename = "nondurable_goods")]
    NondurableGoods,
    #[serde(rename = "nurseries_lawn_and_garden_supply_stores")]
    NurseriesLawnAndGardenSupplyStores,
    #[serde(rename = "nursing_personal_care")]
    NursingPersonalCare,
    #[serde(rename = "office_and_commercial_furniture")]
    OfficeAndCommercialFurniture,
    #[serde(rename = "opticians_eyeglasses")]
    OpticiansEyeglasses,
    #[serde(rename = "optometrists_ophthalmologist")]
    OptometristsOphthalmologist,
    #[serde(rename = "orthopedic_goods_prosthetic_devices")]
    OrthopedicGoodsProstheticDevices,
    #[serde(rename = "osteopaths")]
    Osteopaths,
    #[serde(rename = "package_stores_beer_wine_and_liquor")]
    PackageStoresBeerWineAndLiquor,
    #[serde(rename = "paints_varnishes_and_supplies")]
    PaintsVarnishesAndSupplies,
    #[serde(rename = "parking_lots_garages")]
    ParkingLotsGarages,
    #[serde(rename = "passenger_railways")]
    PassengerRailways,
    #[serde(rename = "pawn_shops")]
    PawnShops,
    #[serde(rename = "pet_shops_pet_food_and_supplies")]
    PetShopsPetFoodAndSupplies,
    #[serde(rename = "petroleum_and_petroleum_products")]
    PetroleumAndPetroleumProducts,
    #[serde(rename = "photo_developing")]
    PhotoDeveloping,
    #[serde(rename = "photographic_photocopy_microfilm_equipment_and_supplies")]
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    #[serde(rename = "photographic_studios")]
    PhotographicStudios,
    #[serde(rename = "picture_video_production")]
    PictureVideoProduction,
    #[serde(rename = "piece_goods_notions_and_other_dry_goods")]
    PieceGoodsNotionsAndOtherDryGoods,
    #[serde(rename = "plumbing_heating_equipment_and_supplies")]
    PlumbingHeatingEquipmentAndSupplies,
    #[serde(rename = "political_organizations")]
    PoliticalOrganizations,
    #[serde(rename = "postal_services_government_only")]
    PostalServicesGovernmentOnly,
    #[serde(rename = "precious_stones_and_metals_watches_and_jewelry")]
    PreciousStonesAndMetalsWatchesAndJewelry,
    #[serde(rename = "professional_services")]
    ProfessionalServices,
    #[serde(rename = "public_warehousing_and_storage")]
    PublicWarehousingAndStorage,
    #[serde(rename = "quick_copy_repro_and_blueprint")]
    QuickCopyReproAndBlueprint,
    #[serde(rename = "railroads")]
    Railroads,
    #[serde(rename = "real_estate_agents_and_managers_rentals")]
    RealEstateAgentsAndManagersRentals,
    #[serde(rename = "record_stores")]
    RecordStores,
    #[serde(rename = "recreational_vehicle_rentals")]
    RecreationalVehicleRentals,
    #[serde(rename = "religious_goods_stores")]
    ReligiousGoodsStores,
    #[serde(rename = "religious_organizations")]
    ReligiousOrganizations,
    #[serde(rename = "roofing_siding_sheet_metal")]
    RoofingSidingSheetMetal,
    #[serde(rename = "secretarial_support_services")]
    SecretarialSupportServices,
    #[serde(rename = "security_brokers_dealers")]
    SecurityBrokersDealers,
    #[serde(rename = "service_stations")]
    ServiceStations,
    #[serde(rename = "sewing_needlework_fabric_and_piece_goods_stores")]
    SewingNeedleworkFabricAndPieceGoodsStores,
    #[serde(rename = "shoe_repair_hat_cleaning")]
    ShoeRepairHatCleaning,
    #[serde(rename = "shoe_stores")]
    ShoeStores,
    #[serde(rename = "small_appliance_repair")]
    SmallApplianceRepair,
    #[serde(rename = "snowmobile_dealers")]
    SnowmobileDealers,
    #[serde(rename = "special_trade_services")]
    SpecialTradeServices,
    #[serde(rename = "specialty_cleaning")]
    SpecialtyCleaning,
    #[serde(rename = "sporting_goods_stores")]
    SportingGoodsStores,
    #[serde(rename = "sporting_recreation_camps")]
    SportingRecreationCamps,
    #[serde(rename = "sports_and_riding_apparel_stores")]
    SportsAndRidingApparelStores,
    #[serde(rename = "sports_clubs_fields")]
    SportsClubsFields,
    #[serde(rename = "stamp_and_coin_stores")]
    StampAndCoinStores,
    #[serde(rename = "stationary_office_supplies_printing_and_writing_paper")]
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    #[serde(rename = "stationery_stores_office_and_school_supply_stores")]
    StationeryStoresOfficeAndSchoolSupplyStores,
    #[serde(rename = "swimming_pools_sales")]
    SwimmingPoolsSales,
    #[serde(rename = "t_ui_travel_germany")]
    TUiTravelGermany,
    #[serde(rename = "tailors_alterations")]
    TailorsAlterations,
    #[serde(rename = "tax_payments_government_agencies")]
    TaxPaymentsGovernmentAgencies,
    #[serde(rename = "tax_preparation_services")]
    TaxPreparationServices,
    #[serde(rename = "taxicabs_limousines")]
    TaxicabsLimousines,
    #[serde(rename = "telecommunication_equipment_and_telephone_sales")]
    TelecommunicationEquipmentAndTelephoneSales,
    #[serde(rename = "telecommunication_services")]
    TelecommunicationServices,
    #[serde(rename = "telegraph_services")]
    TelegraphServices,
    #[serde(rename = "tent_and_awning_shops")]
    TentAndAwningShops,
    #[serde(rename = "testing_laboratories")]
    TestingLaboratories,
    #[serde(rename = "theatrical_ticket_agencies")]
    TheatricalTicketAgencies,
    #[serde(rename = "timeshares")]
    Timeshares,
    #[serde(rename = "tire_retreading_and_repair")]
    TireRetreadingAndRepair,
    #[serde(rename = "tolls_bridge_fees")]
    TollsBridgeFees,
    #[serde(rename = "tourist_attractions_and_exhibits")]
    TouristAttractionsAndExhibits,
    #[serde(rename = "towing_services")]
    TowingServices,
    #[serde(rename = "trailer_parks_campgrounds")]
    TrailerParksCampgrounds,
    #[serde(rename = "transportation_services")]
    TransportationServices,
    #[serde(rename = "travel_agencies_tour_operators")]
    TravelAgenciesTourOperators,
    #[serde(rename = "truck_stop_iteration")]
    TruckStopIteration,
    #[serde(rename = "truck_utility_trailer_rentals")]
    TruckUtilityTrailerRentals,
    #[serde(rename = "typesetting_plate_making_and_related_services")]
    TypesettingPlateMakingAndRelatedServices,
    #[serde(rename = "typewriter_stores")]
    TypewriterStores,
    #[serde(rename = "u_s_federal_government_agencies_or_departments")]
    USFederalGovernmentAgenciesOrDepartments,
    #[serde(rename = "uniforms_commercial_clothing")]
    UniformsCommercialClothing,
    #[serde(rename = "used_merchandise_and_secondhand_stores")]
    UsedMerchandiseAndSecondhandStores,
    #[serde(rename = "utilities")]
    Utilities,
    #[serde(rename = "variety_stores")]
    VarietyStores,
    #[serde(rename = "veterinary_services")]
    VeterinaryServices,
    #[serde(rename = "video_amusement_game_supplies")]
    VideoAmusementGameSupplies,
    #[serde(rename = "video_game_arcades")]
    VideoGameArcades,
    #[serde(rename = "video_tape_rental_stores")]
    VideoTapeRentalStores,
    #[serde(rename = "vocational_trade_schools")]
    VocationalTradeSchools,
    #[serde(rename = "watch_jewelry_repair")]
    WatchJewelryRepair,
    #[serde(rename = "welding_repair")]
    WeldingRepair,
    #[serde(rename = "wholesale_clubs")]
    WholesaleClubs,
    #[serde(rename = "wig_and_toupee_stores")]
    WigAndToupeeStores,
    #[serde(rename = "wires_money_orders")]
    WiresMoneyOrders,
    #[serde(rename = "womens_accessory_and_specialty_shops")]
    WomensAccessoryAndSpecialtyShops,
    #[serde(rename = "womens_ready_to_wear_stores")]
    WomensReadyToWearStores,
    #[serde(rename = "wrecking_and_salvage_yards")]
    WreckingAndSalvageYards,
}

/// Spec paths:
/// - `issuing_card_shipping`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardShipping {
    #[serde(rename = "type")]
    pub type_x: UniStrTypeF00519,
    pub name: String,
    pub address: Address,
    pub carrier: Option<UniStrCarrier>,
    pub eta: Option<i64>,
    pub service: UniStrService,
    pub status: Option<UniStrStatus24CF53>,
    pub tracking_number: Option<String>,
    pub tracking_url: Option<String>,
}

/// Spec paths:
/// - `issuing_card_shipping.carrier`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrCarrier {
    #[serde(rename = "fedex")]
    Fedex,
    #[serde(rename = "usps")]
    Usps,
}

/// Spec paths:
/// - `issuing_card_shipping.service`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrService {
    #[serde(rename = "express")]
    Express,
    #[serde(rename = "priority")]
    Priority,
    #[serde(rename = "standard")]
    Standard,
}

/// Spec paths:
/// - `issuing_card_shipping.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatus24CF53 {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "returned")]
    Returned,
    #[serde(rename = "shipped")]
    Shipped,
}

/// Spec paths:
/// - `issuing_card_shipping.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTypeF00519 {
    #[serde(rename = "bulk")]
    Bulk,
    #[serde(rename = "individual")]
    Individual,
}

/// Spec paths:
/// - `issuing_card_spending_limit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardSpendingLimit {
    pub amount: i64,
    pub categories: Option<Vec<UniStrItems6A79F4>>,
    pub interval: UniStrInterval98ED8A,
}

/// Spec paths:
/// - `issuing_card_spending_limit.interval`
/// - `issuing_cardholder_spending_limit.interval`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrInterval98ED8A {
    #[serde(rename = "all_time")]
    AllTime,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "per_authorization")]
    PerAuthorization,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "yearly")]
    Yearly,
}

/// Spec paths:
/// - `issuing_cardholder_address`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardholderAddress {
    pub address: Address,
}

/// Spec paths:
/// - `issuing_cardholder_authorization_controls`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardholderAuthorizationControls {
    pub allowed_categories: Option<Vec<UniStrItems6A79F4>>,
    pub blocked_categories: Option<Vec<UniStrItems6A79F4>>,
    pub spending_limits: Option<Vec<IssuingCardholderSpendingLimit>>,
    pub spending_limits_currency: Option<String>,
}

/// Spec paths:
/// - `issuing_cardholder_company`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardholderCompany {
    pub tax_id_provided: bool,
}

/// Spec paths:
/// - `issuing_cardholder_id_document`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardholderIdDocument {
    pub back: Option<UniFile5BD414>,
    pub front: Option<UniFile5BD414>,
}

/// Spec paths:
/// - `issuing_cardholder_individual`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardholderIndividual {
    pub dob: Option<IssuingCardholderIndividualDOB>,
    pub first_name: String,
    pub last_name: String,
    pub verification: Option<IssuingCardholderVerification>,
}

/// Spec paths:
/// - `issuing_cardholder_individual_dob`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardholderIndividualDOB {
    pub day: Option<i64>,
    pub month: Option<i64>,
    pub year: Option<i64>,
}

/// Spec paths:
/// - `issuing_cardholder_requirements`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardholderRequirements {
    pub disabled_reason: Option<UniStrDisabledReason>,
    pub past_due: Option<Vec<UniStrItems6966B7>>,
}

/// Spec paths:
/// - `issuing_cardholder_requirements.disabled_reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrDisabledReason {
    #[serde(rename = "listed")]
    Listed,
    #[serde(rename = "rejected.listed")]
    RejectedDotListed,
    #[serde(rename = "under_review")]
    UnderReview,
}

/// Spec paths:
/// - `issuing_cardholder_requirements.past_due.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrItems6966B7 {
    #[serde(rename = "company.tax_id")]
    CompanyDotTaxId,
    #[serde(rename = "individual.dob.day")]
    IndividualDotDobDotDay,
    #[serde(rename = "individual.dob.month")]
    IndividualDotDobDotMonth,
    #[serde(rename = "individual.dob.year")]
    IndividualDotDobDotYear,
    #[serde(rename = "individual.first_name")]
    IndividualDotFirstName,
    #[serde(rename = "individual.last_name")]
    IndividualDotLastName,
    #[serde(rename = "individual.verification.document")]
    IndividualDotVerificationDotDocument,
}

/// Spec paths:
/// - `issuing_cardholder_spending_limit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardholderSpendingLimit {
    pub amount: i64,
    pub categories: Option<Vec<UniStrItems6A79F4>>,
    pub interval: UniStrInterval98ED8A,
}

/// Spec paths:
/// - `issuing_cardholder_verification`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingCardholderVerification {
    pub document: Option<IssuingCardholderIdDocument>,
}

/// Spec paths:
/// - `issuing_dispute_canceled_evidence`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingDisputeCanceledEvidence {
    pub additional_documentation: Option<UniFile5BD414>,
    pub canceled_at: Option<i64>,
    pub cancellation_policy_provided: Option<bool>,
    pub cancellation_reason: Option<String>,
    pub expected_at: Option<i64>,
    pub explanation: Option<String>,
    pub product_description: Option<String>,
    pub product_type: Option<UniStrProductType>,
    pub return_status: Option<UniStrReturnStatus>,
    pub returned_at: Option<i64>,
}

/// Spec paths:
/// - `issuing_dispute_canceled_evidence.product_type`
/// - `issuing_dispute_not_received_evidence.product_type`
/// - `issuing_dispute_other_evidence.product_type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrProductType {
    #[serde(rename = "merchandise")]
    Merchandise,
    #[serde(rename = "service")]
    Service,
}

/// Spec paths:
/// - `issuing_dispute_canceled_evidence.return_status`
/// - `issuing_dispute_merchandise_not_as_described_evidence.return_status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrReturnStatus {
    #[serde(rename = "merchant_rejected")]
    MerchantRejected,
    #[serde(rename = "successful")]
    Successful,
}

/// Spec paths:
/// - `issuing_dispute_duplicate_evidence`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingDisputeDuplicateEvidence {
    pub additional_documentation: Option<UniFile5BD414>,
    pub card_statement: Option<UniFile5BD414>,
    pub cash_receipt: Option<UniFile5BD414>,
    pub check_image: Option<UniFile5BD414>,
    pub explanation: Option<String>,
    pub original_transaction: Option<String>,
}

/// Spec paths:
/// - `issuing_dispute_evidence`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingDisputeEvidence {
    pub canceled: Option<IssuingDisputeCanceledEvidence>,
    pub duplicate: Option<IssuingDisputeDuplicateEvidence>,
    pub fraudulent: Option<IssuingDisputeFraudulentEvidence>,
    pub merchandise_not_as_described: Option<IssuingDisputeMerchandiseNotAsDescribedEvidence>,
    pub not_received: Option<IssuingDisputeNotReceivedEvidence>,
    pub other: Option<IssuingDisputeOtherEvidence>,
    pub reason: UniStrReason,
    pub service_not_as_described: Option<IssuingDisputeServiceNotAsDescribedEvidence>,
}

/// Spec paths:
/// - `issuing_dispute_evidence.reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrReason {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "fraudulent")]
    Fraudulent,
    #[serde(rename = "merchandise_not_as_described")]
    MerchandiseNotAsDescribed,
    #[serde(rename = "not_received")]
    NotReceived,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "service_not_as_described")]
    ServiceNotAsDescribed,
}

/// Spec paths:
/// - `issuing_dispute_fraudulent_evidence`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingDisputeFraudulentEvidence {
    pub additional_documentation: Option<UniFile5BD414>,
    pub explanation: Option<String>,
}

/// Spec paths:
/// - `issuing_dispute_merchandise_not_as_described_evidence`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingDisputeMerchandiseNotAsDescribedEvidence {
    pub additional_documentation: Option<UniFile5BD414>,
    pub explanation: Option<String>,
    pub received_at: Option<i64>,
    pub return_description: Option<String>,
    pub return_status: Option<UniStrReturnStatus>,
    pub returned_at: Option<i64>,
}

/// Spec paths:
/// - `issuing_dispute_not_received_evidence`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingDisputeNotReceivedEvidence {
    pub additional_documentation: Option<UniFile5BD414>,
    pub expected_at: Option<i64>,
    pub explanation: Option<String>,
    pub product_description: Option<String>,
    pub product_type: Option<UniStrProductType>,
}

/// Spec paths:
/// - `issuing_dispute_other_evidence`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingDisputeOtherEvidence {
    pub additional_documentation: Option<UniFile5BD414>,
    pub explanation: Option<String>,
    pub product_description: Option<String>,
    pub product_type: Option<UniStrProductType>,
}

/// Spec paths:
/// - `issuing_dispute_service_not_as_described_evidence`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingDisputeServiceNotAsDescribedEvidence {
    pub additional_documentation: Option<UniFile5BD414>,
    pub canceled_at: Option<i64>,
    pub cancellation_reason: Option<String>,
    pub explanation: Option<String>,
    pub received_at: Option<i64>,
}

/// Spec paths:
/// - `issuing_transaction_amount_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingTransactionAmountDetails {
    pub atm_fee: Option<i64>,
}

/// Spec paths:
/// - `issuing_transaction_flight_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingTransactionFlightData {
    pub departure_at: Option<i64>,
    pub passenger_name: Option<String>,
    pub refundable: Option<bool>,
    pub segments: Option<Vec<IssuingTransactionFlightDataLeg>>,
    pub travel_agency: Option<String>,
}

/// Spec paths:
/// - `issuing_transaction_flight_data_leg`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingTransactionFlightDataLeg {
    pub arrival_airport_code: Option<String>,
    pub carrier: Option<String>,
    pub departure_airport_code: Option<String>,
    pub flight_number: Option<String>,
    pub service_class: Option<String>,
    pub stopover_allowed: Option<bool>,
}

/// Spec paths:
/// - `issuing_transaction_fuel_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingTransactionFuelData {
    #[serde(rename = "type")]
    pub type_x: String,
    pub unit: String,
    pub unit_cost_decimal: String,
    pub volume_decimal: Option<String>,
}

/// Spec paths:
/// - `issuing_transaction_lodging_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingTransactionLodgingData {
    pub check_in_at: Option<i64>,
    pub nights: Option<i64>,
}

/// Spec paths:
/// - `issuing_transaction_purchase_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingTransactionPurchaseDetails {
    pub flight: Option<IssuingTransactionFlightData>,
    pub fuel: Option<IssuingTransactionFuelData>,
    pub lodging: Option<IssuingTransactionLodgingData>,
    pub receipt: Option<Vec<IssuingTransactionReceiptData>>,
    pub reference: Option<String>,
}

/// Spec paths:
/// - `issuing_transaction_receipt_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuingTransactionReceiptData {
    pub description: Option<String>,
    pub quantity: Option<f64>,
    pub total: Option<i64>,
    pub unit_cost: Option<i64>,
}

/// Spec paths:
/// - `item`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineItems {
    pub object: UniStrObjectDE5E3A,
    pub id: String,
    pub amount_subtotal: Option<i64>,
    pub amount_total: Option<i64>,
    pub currency: String,
    pub description: String,
    pub discounts: Option<Vec<LineItemsDiscountAmount>>,
    pub price: Price,
    pub quantity: Option<i64>,
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}

impl GetId for LineItems {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `item.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectDE5E3A {
    #[serde(rename = "item")]
    Item,
}

/// Spec paths:
/// - `legal_entity_company`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegalEntityCompany {
    pub name: Option<String>,
    pub address: Option<Address>,
    pub address_kana: Option<LegalEntityJapanAddress>,
    pub address_kanji: Option<LegalEntityJapanAddress>,
    pub directors_provided: Option<bool>,
    pub executives_provided: Option<bool>,
    pub name_kana: Option<String>,
    pub name_kanji: Option<String>,
    pub owners_provided: Option<bool>,
    pub phone: Option<String>,
    pub structure: Option<UniStrStructure>,
    pub tax_id_provided: Option<bool>,
    pub tax_id_registrar: Option<String>,
    pub vat_id_provided: Option<bool>,
    pub verification: Option<LegalEntityCompanyVerification>,
}

/// Spec paths:
/// - `legal_entity_company.structure`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStructure {
    #[serde(rename = "government_instrumentality")]
    GovernmentInstrumentality,
    #[serde(rename = "governmental_unit")]
    GovernmentalUnit,
    #[serde(rename = "incorporated_non_profit")]
    IncorporatedNonProfit,
    #[serde(rename = "limited_liability_partnership")]
    LimitedLiabilityPartnership,
    #[serde(rename = "multi_member_llc")]
    MultiMemberLlc,
    #[serde(rename = "private_company")]
    PrivateCompany,
    #[serde(rename = "private_corporation")]
    PrivateCorporation,
    #[serde(rename = "private_partnership")]
    PrivatePartnership,
    #[serde(rename = "public_company")]
    PublicCompany,
    #[serde(rename = "public_corporation")]
    PublicCorporation,
    #[serde(rename = "public_partnership")]
    PublicPartnership,
    #[serde(rename = "sole_proprietorship")]
    SoleProprietorship,
    #[serde(rename = "tax_exempt_government_instrumentality")]
    TaxExemptGovernmentInstrumentality,
    #[serde(rename = "unincorporated_association")]
    UnincorporatedAssociation,
    #[serde(rename = "unincorporated_non_profit")]
    UnincorporatedNonProfit,
}

/// Spec paths:
/// - `legal_entity_company_verification`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegalEntityCompanyVerification {
    pub document: LegalEntityCompanyVerificationDocument,
}

/// Spec paths:
/// - `legal_entity_company_verification_document`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegalEntityCompanyVerificationDocument {
    pub back: Option<UniFile5BD414>,
    pub front: Option<UniFile5BD414>,
    pub details: Option<String>,
    pub details_code: Option<String>,
}

/// Spec paths:
/// - `legal_entity_dob`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegalEntityDOB {
    pub day: Option<i64>,
    pub month: Option<i64>,
    pub year: Option<i64>,
}

/// Spec paths:
/// - `legal_entity_japan_address`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegalEntityJapanAddress {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
    pub town: Option<String>,
}

/// Spec paths:
/// - `legal_entity_person_verification`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegalEntityPersonVerification {
    pub additional_document: Option<LegalEntityPersonVerificationDocument>,
    pub details: Option<String>,
    pub details_code: Option<String>,
    pub document: Option<LegalEntityPersonVerificationDocument>,
    pub status: String,
}

/// Spec paths:
/// - `legal_entity_person_verification_document`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegalEntityPersonVerificationDocument {
    pub back: Option<UniFile5BD414>,
    pub front: Option<UniFile5BD414>,
    pub details: Option<String>,
    pub details_code: Option<String>,
}

/// Spec paths:
/// - `light_account_logout`
pub type LightAccountLogout = Value;

/// Spec paths:
/// - `line_item`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvoiceLineItem {
    pub object: UniStrObject0B3778,
    #[serde(rename = "type")]
    pub type_x: UniStrType5768DB,
    pub id: String,
    pub discounts: Option<Vec<UniItemsE47473>>,
    pub amount: i64,
    pub currency: String,
    pub description: Option<String>,
    pub discount_amounts: Option<Vec<DiscountsResourceDiscountAmount>>,
    pub discountable: bool,
    pub invoice_item: Option<String>,
    pub period: InvoiceLineItemPeriod,
    pub price: Option<Price>,
    pub proration: bool,
    pub quantity: Option<i64>,
    pub subscription: Option<String>,
    pub subscription_item: Option<String>,
    pub tax_amounts: Option<Vec<InvoiceTaxAmount>>,
    pub tax_rates: Option<Vec<TaxRate>>,
    pub livemode: bool,
    pub metadata: MetadataE2C46F,
}

impl GetId for InvoiceLineItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `line_item.metadata`
pub type MetadataE2C46F = HashMap<String, String>;

/// Spec paths:
/// - `line_item.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject0B3778 {
    #[serde(rename = "line_item")]
    LineItem,
}

/// Spec paths:
/// - `line_item.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrType5768DB {
    #[serde(rename = "invoiceitem")]
    Invoiceitem,
    #[serde(rename = "subscription")]
    Subscription,
}

/// Spec paths:
/// - `line_items_discount_amount`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineItemsDiscountAmount {
    pub amount: i64,
    pub discount: Discount,
}

/// Spec paths:
/// - `line_items_tax_amount`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineItemsTaxAmount {
    pub amount: i64,
    pub rate: TaxRate,
}

/// Spec paths:
/// - `login_link`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginLink {
    pub object: UniStrObjectCEBEB6,
    pub url: String,
    pub created: i64,
}

/// Spec paths:
/// - `login_link.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectCEBEB6 {
    #[serde(rename = "login_link")]
    LoginLink,
}

/// Spec paths:
/// - `mandate`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mandate {
    pub object: UniStrObject166EB7,
    #[serde(rename = "type")]
    pub type_x: UniStrType312965,
    pub id: String,
    pub payment_method: UniPaymentMethod,
    pub customer_acceptance: CustomerAcceptance,
    pub multi_use: Option<MandateMultiUse>,
    pub payment_method_details: MandatePaymentMethodDetails,
    pub single_use: Option<MandateSingleUse>,
    pub status: UniStrStatusBA4125,
    pub livemode: bool,
}

impl GetId for Mandate {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `mandate.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject166EB7 {
    #[serde(rename = "mandate")]
    Mandate,
}

/// Spec paths:
/// - `mandate.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrType312965 {
    #[serde(rename = "multi_use")]
    MultiUse,
    #[serde(rename = "single_use")]
    SingleUse,
}

/// Spec paths:
/// - `mandate_au_becs_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MandateAuBecsDebit {
    pub url: String,
}

/// Spec paths:
/// - `mandate_bacs_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MandateBacsDebit {
    pub network_status: UniStrNetworkStatus,
    pub reference: String,
    pub url: String,
}

/// Spec paths:
/// - `mandate_bacs_debit.network_status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrNetworkStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "refused")]
    Refused,
    #[serde(rename = "revoked")]
    Revoked,
}

/// Spec paths:
/// - `mandate_multi_use`
pub type MandateMultiUse = Value;

/// Spec paths:
/// - `mandate_payment_method_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MandatePaymentMethodDetails {
    #[serde(rename = "type")]
    pub type_x: String,
    pub au_becs_debit: Option<MandateAuBecsDebit>,
    pub bacs_debit: Option<MandateBacsDebit>,
    pub card: Option<CardMandatePaymentMethodDetails>,
    pub sepa_debit: Option<MandateSepaDebit>,
}

/// Spec paths:
/// - `mandate_sepa_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MandateSepaDebit {
    pub reference: String,
    pub url: String,
}

/// Spec paths:
/// - `mandate_single_use`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MandateSingleUse {
    pub amount: i64,
    pub currency: String,
}

/// Spec paths:
/// - `networks`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Networks {
    pub available: Vec<String>,
    pub preferred: Option<String>,
}

/// Spec paths:
/// - `notification_event_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationEventData {
    pub object: Box<UniNotificationEventDataObject>,
    pub previous_attributes: Option<PreviousAttributes>,
}

/// Spec paths:
/// - `notification_event_data.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniNotificationEventDataObject {
    Account(Account),
    PlatformFee(PlatformFee),
    Balance(Balance),
    AccountCapability(AccountCapability),
    Charge(Box<Charge>),
    Session(Session),
    Coupon(Coupon),
    CreditNote(Box<CreditNote>),
    Customer(Box<Customer>),
    Discount(Discount),
    Dispute(Dispute),
    RadarEarlyFraudWarning(RadarEarlyFraudWarning),
    UniPolymorphic70BAFA(UniPolymorphic70BAFA),
    FeeRefund(FeeRefund),
    File(File),
    Invoice(Box<Invoice>),
    InvoiceItem(InvoiceItem),
    IssuingAuthorization(IssuingAuthorization),
    IssuingCard(Box<IssuingCard>),
    IssuingCardholder(IssuingCardholder),
    IssuingDispute(IssuingDispute),
    IssuingTransaction(Box<IssuingTransaction>),
    Mandate(Mandate),
    Order(Order),
    OrderReturn(OrderReturn),
    PaymentIntent(Box<PaymentIntent>),
    PaymentMethod(Box<PaymentMethod>),
    Payout(Box<Payout>),
    Person(Person),
    Plan(Plan),
    Price(Price),
    Product(Product),
    PromotionCode(PromotionCode),
    TransferRecipient(TransferRecipient),
    Refund(Box<Refund>),
    ReportingReportRun(ReportingReportRun),
    ReportingReportType(ReportingReportType),
    RadarReview(RadarReview),
    ScheduledQueryRun(ScheduledQueryRun),
    SetupIntent(Box<SetupIntent>),
    Sku(Sku),
    Source(Source),
    SourceTransaction(SourceTransaction),
    Subscription(Box<Subscription>),
    SubscriptionSchedule(SubscriptionSchedule),
    TaxId(TaxId),
    TaxRate(TaxRate),
    Topup(Topup),
    Transfer(Transfer),
    UnknownEvent(UnknownEvent),
}

/// Spec paths:
/// - `notification_event_data.object.anyOf.49`
pub type UnknownEvent = Value;

/// Spec paths:
/// - `notification_event_data.previous_attributes`
pub type PreviousAttributes = Value;

/// Spec paths:
/// - `notification_event_request`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationEventRequest {
    pub id: Option<String>,
    pub idempotency_key: Option<String>,
}

/// Spec paths:
/// - `offline_acceptance`
pub type OfflineAcceptance = Value;

/// Spec paths:
/// - `online_acceptance`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnlineAcceptance {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Spec paths:
/// - `order`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub object: UniStrObject43B2E2,
    pub id: String,
    pub upstream_id: Option<String>,
    pub charge: Option<UniCharge>,
    pub customer: Option<UniCustomerC00F6E>,
    pub amount: i64,
    pub amount_returned: Option<i64>,
    pub application: Option<String>,
    pub application_fee: Option<i64>,
    pub currency: String,
    pub email: Option<String>,
    pub external_coupon_code: Option<String>,
    pub items: Vec<OrderItem>,
    pub returns: Option<OrdersResourceOrderReturnList>,
    pub selected_shipping_method: Option<String>,
    pub shipping: Option<Shipping>,
    pub shipping_methods: Option<Vec<ShippingMethod>>,
    pub status: String,
    pub status_transitions: Option<StatusTransitions>,
    pub created: i64,
    pub updated: Option<i64>,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for Order {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `order.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject43B2E2 {
    #[serde(rename = "order")]
    Order,
}

/// Spec paths:
/// - `order.returns`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrdersResourceOrderReturnList {
    pub object: UniStrObject344B0E,
    pub data: Vec<OrderReturn>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for OrdersResourceOrderReturnList {
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
/// - `order_item`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderItem {
    pub object: UniStrObject347C00,
    #[serde(rename = "type")]
    pub type_x: String,
    pub parent: Option<UniParent>,
    pub amount: i64,
    pub currency: String,
    pub description: String,
    pub quantity: Option<i64>,
}

/// Spec paths:
/// - `order_item.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject347C00 {
    #[serde(rename = "order_item")]
    OrderItem,
}

/// Spec paths:
/// - `order_item.parent`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniParent {
    String(String),
    Sku(Sku),
}

/// Spec paths:
/// - `order_return`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderReturn {
    pub object: UniStrObject765808,
    pub id: String,
    pub order: Option<UniOrder>,
    pub refund: Option<UniRefund>,
    pub amount: i64,
    pub currency: String,
    pub items: Vec<OrderItem>,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for OrderReturn {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `order_return.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject765808 {
    #[serde(rename = "order_return")]
    OrderReturn,
}

/// Spec paths:
/// - `package_dimensions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageDimensions {
    pub height: f64,
    pub length: f64,
    pub weight: f64,
    pub width: f64,
}

/// Spec paths:
/// - `payment_flows_private_payment_methods_alipay`
pub type PaymentFlowsPrivatePaymentMethodsAlipay = Value;

/// Spec paths:
/// - `payment_flows_private_payment_methods_alipay_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentFlowsPrivatePaymentMethodsAlipayDetails {
    pub transaction_id: Option<String>,
    pub fingerprint: Option<String>,
}

/// Spec paths:
/// - `payment_intent`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentIntent {
    pub object: UniStrObjectD4FACD,
    pub id: String,
    pub application: Option<UniApplication>,
    pub customer: Option<UniCustomerC00F6E>,
    pub invoice: Option<UniInvoice>,
    pub on_behalf_of: Option<UniAccount>,
    pub payment_method: Option<UniPaymentMethod>,
    pub review: Option<UniReview>,
    pub amount: i64,
    pub amount_capturable: Option<i64>,
    pub amount_received: Option<i64>,
    pub application_fee_amount: Option<i64>,
    pub canceled_at: Option<i64>,
    pub cancellation_reason: Option<UniStrCancellationReasonB1131D>,
    pub capture_method: UniStrCaptureMethod,
    pub charges: Option<PaymentFlowsPaymentIntentResourceChargeList>,
    pub client_secret: Option<String>,
    pub confirmation_method: UniStrCaptureMethod,
    pub currency: String,
    pub description: Option<String>,
    pub last_payment_error: Box<Option<APIErrors>>,
    pub next_action: Option<PaymentIntentNextAction>,
    pub payment_method_options: Option<PaymentIntentPaymentMethodOptions>,
    pub payment_method_types: Vec<String>,
    pub receipt_email: Option<String>,
    pub setup_future_usage: Option<UniStrSetupFutureUsage>,
    pub shipping: Option<Shipping>,
    pub statement_descriptor: Option<String>,
    pub statement_descriptor_suffix: Option<String>,
    pub status: UniStrStatus2A8B48,
    pub transfer_data: Option<TransferData>,
    pub transfer_group: Option<String>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata3CAB08>,
}

impl GetId for PaymentIntent {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `payment_intent.cancellation_reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrCancellationReasonB1131D {
    #[serde(rename = "abandoned")]
    Abandoned,
    #[serde(rename = "automatic")]
    Automatic,
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "failed_invoice")]
    FailedInvoice,
    #[serde(rename = "fraudulent")]
    Fraudulent,
    #[serde(rename = "requested_by_customer")]
    RequestedByCustomer,
    #[serde(rename = "void_invoice")]
    VoidInvoice,
}

/// Spec paths:
/// - `payment_intent.capture_method`
/// - `payment_intent.confirmation_method`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrCaptureMethod {
    #[serde(rename = "automatic")]
    Automatic,
    #[serde(rename = "manual")]
    Manual,
}

/// Spec paths:
/// - `payment_intent.charges`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentFlowsPaymentIntentResourceChargeList {
    pub object: UniStrObject344B0E,
    pub data: Vec<Charge>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for PaymentFlowsPaymentIntentResourceChargeList {
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
/// - `payment_intent.metadata`
pub type Metadata3CAB08 = HashMap<String, String>;

/// Spec paths:
/// - `payment_intent.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectD4FACD {
    #[serde(rename = "payment_intent")]
    PaymentIntent,
}

/// Spec paths:
/// - `payment_intent.setup_future_usage`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrSetupFutureUsage {
    #[serde(rename = "off_session")]
    OffSession,
    #[serde(rename = "on_session")]
    OnSession,
}

/// Spec paths:
/// - `payment_intent.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatus2A8B48 {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "requires_action")]
    RequiresAction,
    #[serde(rename = "requires_capture")]
    RequiresCapture,
    #[serde(rename = "requires_confirmation")]
    RequiresConfirmation,
    #[serde(rename = "requires_payment_method")]
    RequiresPaymentMethod,
    #[serde(rename = "succeeded")]
    Succeeded,
}

/// Spec paths:
/// - `payment_intent_next_action`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentIntentNextAction {
    #[serde(rename = "type")]
    pub type_x: String,
    pub alipay_handle_redirect: Option<PaymentIntentNextActionAlipayHandleRedirect>,
    pub oxxo_display_details: Option<PaymentIntentNextActionDisplayOxxoDetails>,
    pub redirect_to_url: Option<PaymentIntentNextActionRedirectToUrl>,
    pub use_stripe_sdk: Option<UseStripeSdkDAE5BB>,
}

/// Spec paths:
/// - `payment_intent_next_action.use_stripe_sdk`
pub type UseStripeSdkDAE5BB = Value;

/// Spec paths:
/// - `payment_intent_next_action_alipay_handle_redirect`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentIntentNextActionAlipayHandleRedirect {
    pub native_data: Option<String>,
    pub native_url: Option<String>,
    pub return_url: Option<String>,
    pub url: Option<String>,
}

/// Spec paths:
/// - `payment_intent_next_action_display_oxxo_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentIntentNextActionDisplayOxxoDetails {
    pub expires_after: Option<i64>,
    pub hosted_voucher_url: Option<String>,
    pub number: Option<String>,
}

/// Spec paths:
/// - `payment_intent_next_action_redirect_to_url`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentIntentNextActionRedirectToUrl {
    pub return_url: Option<String>,
    pub url: Option<String>,
}

/// Spec paths:
/// - `payment_intent_payment_method_options`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentIntentPaymentMethodOptions {
    pub alipay: Option<PaymentMethodOptionsAlipay>,
    pub bancontact: Option<PaymentMethodOptionsBancontact>,
    pub card: Option<PaymentIntentPaymentMethodOptionsCard>,
    pub oxxo: Option<PaymentMethodOptionsOxxo>,
    pub p24: Option<PaymentMethodOptionsP24>,
    pub sofort: Option<PaymentMethodOptionsSofort>,
}

/// Spec paths:
/// - `payment_intent_payment_method_options_card`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentIntentPaymentMethodOptionsCard {
    pub installments: Option<PaymentMethodOptionsCardInstallments>,
    pub network: Option<UniStrNetworkA1E06C>,
    pub request_three_d_secure: Option<UniStrRequestThreeDSecure>,
}

/// Spec paths:
/// - `payment_intent_payment_method_options_card.network`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrNetworkA1E06C {
    #[serde(rename = "amex")]
    Amex,
    #[serde(rename = "cartes_bancaires")]
    CartesBancaires,
    #[serde(rename = "diners")]
    Diners,
    #[serde(rename = "discover")]
    Discover,
    #[serde(rename = "interac")]
    Interac,
    #[serde(rename = "jcb")]
    Jcb,
    #[serde(rename = "mastercard")]
    Mastercard,
    #[serde(rename = "unionpay")]
    Unionpay,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "visa")]
    Visa,
}

/// Spec paths:
/// - `payment_intent_payment_method_options_card.request_three_d_secure`
/// - `setup_intent_payment_method_options_card.request_three_d_secure`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrRequestThreeDSecure {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "automatic")]
    Automatic,
    #[serde(rename = "challenge_only")]
    ChallengeOnly,
}

/// Spec paths:
/// - `payment_method`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethod {
    pub object: UniStrObject8D07E7,
    #[serde(rename = "type")]
    pub type_x: UniStrType661F1A,
    pub id: String,
    pub customer: Option<UniCustomerEDC00A>,
    pub alipay: Option<PaymentFlowsPrivatePaymentMethodsAlipay>,
    pub au_becs_debit: Option<PaymentMethodAuBecsDebit>,
    pub bacs_debit: Option<PaymentMethodBacsDebit>,
    pub bancontact: Option<PaymentMethodBancontact>,
    pub billing_details: BillingDetails,
    pub card: Option<PaymentMethodCard>,
    pub card_present: Option<PaymentMethodCardPresent>,
    pub eps: Option<PaymentMethodEps>,
    pub fpx: Option<PaymentMethodFpx>,
    pub giropay: Option<PaymentMethodGiropay>,
    pub grabpay: Option<PaymentMethodGrabpay>,
    pub ideal: Option<PaymentMethodIdeal>,
    pub interac_present: Option<PaymentMethodInteracPresent>,
    pub oxxo: Option<PaymentMethodOxxo>,
    pub p24: Option<PaymentMethodP24>,
    pub sepa_debit: Option<PaymentMethodSepaDebit>,
    pub sofort: Option<PaymentMethodSofort>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for PaymentMethod {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `payment_method.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject8D07E7 {
    #[serde(rename = "payment_method")]
    PaymentMethod,
}

/// Spec paths:
/// - `payment_method.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrType661F1A {
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
    #[serde(rename = "card_present")]
    CardPresent,
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
    #[serde(rename = "interac_present")]
    InteracPresent,
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
/// - `payment_method_au_becs_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodAuBecsDebit {
    pub bsb_number: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
}

/// Spec paths:
/// - `payment_method_bacs_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodBacsDebit {
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub sort_code: Option<String>,
}

/// Spec paths:
/// - `payment_method_bancontact`
pub type PaymentMethodBancontact = Value;

/// Spec paths:
/// - `payment_method_card`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodCard {
    pub brand: String,
    pub checks: Option<PaymentMethodCardChecks>,
    pub country: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: Option<String>,
    pub funding: String,
    pub generated_from: Option<PaymentMethodCardGeneratedCard>,
    pub last4: String,
    pub networks: Option<Networks>,
    pub three_d_secure_usage: Option<ThreeDSecureUsage>,
    pub wallet: Option<PaymentMethodCardWallet>,
}

/// Spec paths:
/// - `payment_method_card_checks`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodCardChecks {
    pub address_line1_check: Option<String>,
    pub address_postal_code_check: Option<String>,
    pub cvc_check: Option<String>,
}

/// Spec paths:
/// - `payment_method_card_generated_card`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodCardGeneratedCard {
    pub charge: Option<String>,
    pub payment_method_details: Option<CardGeneratedFromPaymentMethodDetails>,
}

/// Spec paths:
/// - `payment_method_card_present`
pub type PaymentMethodCardPresent = Value;

/// Spec paths:
/// - `payment_method_card_wallet`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodCardWallet {
    #[serde(rename = "type")]
    pub type_x: UniStrType0A6777,
    pub amex_express_checkout: Option<PaymentMethodCardWalletAmexExpressCheckout>,
    pub apple_pay: Option<PaymentMethodCardWalletApplePay>,
    pub dynamic_last4: Option<String>,
    pub google_pay: Option<PaymentMethodCardWalletGooglePay>,
    pub masterpass: Option<PaymentMethodCardWalletMasterpass>,
    pub samsung_pay: Option<PaymentMethodCardWalletSamsungPay>,
    pub visa_checkout: Option<PaymentMethodCardWalletVisaCheckout>,
}

/// Spec paths:
/// - `payment_method_card_wallet.type`
/// - `payment_method_details_card_wallet.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrType0A6777 {
    #[serde(rename = "amex_express_checkout")]
    AmexExpressCheckout,
    #[serde(rename = "apple_pay")]
    ApplePay,
    #[serde(rename = "google_pay")]
    GooglePay,
    #[serde(rename = "masterpass")]
    Masterpass,
    #[serde(rename = "samsung_pay")]
    SamsungPay,
    #[serde(rename = "visa_checkout")]
    VisaCheckout,
}

/// Spec paths:
/// - `payment_method_card_wallet_amex_express_checkout`
pub type PaymentMethodCardWalletAmexExpressCheckout = Value;

/// Spec paths:
/// - `payment_method_card_wallet_apple_pay`
pub type PaymentMethodCardWalletApplePay = Value;

/// Spec paths:
/// - `payment_method_card_wallet_google_pay`
pub type PaymentMethodCardWalletGooglePay = Value;

/// Spec paths:
/// - `payment_method_card_wallet_masterpass`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodCardWalletMasterpass {
    pub name: Option<String>,
    pub billing_address: Option<Address>,
    pub email: Option<String>,
    pub shipping_address: Option<Address>,
}

/// Spec paths:
/// - `payment_method_card_wallet_samsung_pay`
pub type PaymentMethodCardWalletSamsungPay = Value;

/// Spec paths:
/// - `payment_method_card_wallet_visa_checkout`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodCardWalletVisaCheckout {
    pub name: Option<String>,
    pub billing_address: Option<Address>,
    pub email: Option<String>,
    pub shipping_address: Option<Address>,
}

/// Spec paths:
/// - `payment_method_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetails {
    #[serde(rename = "type")]
    pub type_x: String,
    pub ach_credit_transfer: Option<PaymentMethodDetailsAchCreditTransfer>,
    pub ach_debit: Option<PaymentMethodDetailsAchDebit>,
    pub alipay: Option<PaymentFlowsPrivatePaymentMethodsAlipayDetails>,
    pub au_becs_debit: Option<PaymentMethodDetailsAuBecsDebit>,
    pub bacs_debit: Option<PaymentMethodDetailsBacsDebit>,
    pub bancontact: Option<PaymentMethodDetailsBancontact>,
    pub card: Option<PaymentMethodDetailsCard>,
    pub card_present: Option<PaymentMethodDetailsCardPresent>,
    pub eps: Option<PaymentMethodDetailsEps>,
    pub fpx: Option<PaymentMethodDetailsFpx>,
    pub giropay: Option<PaymentMethodDetailsGiropay>,
    pub grabpay: Option<PaymentMethodDetailsGrabpay>,
    pub ideal: Option<PaymentMethodDetailsIdeal>,
    pub interac_present: Option<PaymentMethodDetailsInteracPresent>,
    pub klarna: Option<PaymentMethodDetailsKlarna>,
    pub multibanco: Option<PaymentMethodDetailsMultibanco>,
    pub oxxo: Option<PaymentMethodDetailsOxxo>,
    pub p24: Option<PaymentMethodDetailsP24>,
    pub sepa_debit: Option<PaymentMethodDetailsSepaDebit>,
    pub sofort: Option<PaymentMethodDetailsSofort>,
    pub stripe_account: Option<PaymentMethodDetailsStripeAccount>,
    pub wechat: Option<PaymentMethodDetailsWechat>,
}

/// Spec paths:
/// - `payment_method_details_ach_credit_transfer`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsAchCreditTransfer {
    pub account_number: Option<String>,
    pub bank_name: Option<String>,
    pub routing_number: Option<String>,
    pub swift_code: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_ach_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsAchDebit {
    pub account_holder_type: Option<UniStrType947A77>,
    pub bank_name: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub routing_number: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_au_becs_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsAuBecsDebit {
    pub bsb_number: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub mandate: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_bacs_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsBacsDebit {
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub mandate: Option<String>,
    pub sort_code: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_bancontact`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsBancontact {
    pub generated_sepa_debit: Option<UniPaymentMethod>,
    pub generated_sepa_debit_mandate: Option<UniMandate>,
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub iban_last4: Option<String>,
    pub preferred_language: Option<UniStrPreferredLanguageD97AA3>,
    pub verified_name: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_bancontact.generated_sepa_debit_mandate`
/// - `payment_method_details_ideal.generated_sepa_debit_mandate`
/// - `payment_method_details_sofort.generated_sepa_debit_mandate`
/// - `setup_attempt_payment_method_details_bancontact.generated_sepa_debit_mandate`
/// - `setup_attempt_payment_method_details_ideal.generated_sepa_debit_mandate`
/// - `setup_attempt_payment_method_details_sofort.generated_sepa_debit_mandate`
/// - `setup_intent.mandate`
/// - `setup_intent.single_use_mandate`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniMandate {
    String(String),
    Mandate(Mandate),
}

/// Spec paths:
/// - `payment_method_details_bancontact.preferred_language`
/// - `payment_method_options_bancontact.preferred_language`
/// - `setup_attempt_payment_method_details_bancontact.preferred_language`
/// - `setup_attempt_payment_method_details_sofort.preferred_language`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrPreferredLanguageD97AA3 {
    #[serde(rename = "de")]
    De,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "nl")]
    Nl,
}

/// Spec paths:
/// - `payment_method_details_card`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsCard {
    pub brand: Option<String>,
    pub checks: Option<PaymentMethodDetailsCardChecks>,
    pub country: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub installments: Option<PaymentMethodDetailsCardInstallments>,
    pub last4: Option<String>,
    pub network: Option<String>,
    pub three_d_secure: Option<ThreeDSecureDetails>,
    pub wallet: Option<PaymentMethodDetailsCardWallet>,
}

/// Spec paths:
/// - `payment_method_details_card_checks`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsCardChecks {
    pub address_line1_check: Option<String>,
    pub address_postal_code_check: Option<String>,
    pub cvc_check: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_card_installments`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsCardInstallments {
    pub plan: Option<PaymentMethodDetailsCardInstallmentsPlan>,
}

/// Spec paths:
/// - `payment_method_details_card_installments_plan`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsCardInstallmentsPlan {
    #[serde(rename = "type")]
    pub type_x: UniStrType16A5D0,
    pub count: Option<i64>,
    pub interval: Option<UniStrIntervalC620AD>,
}

/// Spec paths:
/// - `payment_method_details_card_installments_plan.interval`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrIntervalC620AD {
    #[serde(rename = "month")]
    Month,
}

/// Spec paths:
/// - `payment_method_details_card_installments_plan.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrType16A5D0 {
    #[serde(rename = "fixed_count")]
    FixedCount,
}

/// Spec paths:
/// - `payment_method_details_card_present`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsCardPresent {
    pub brand: Option<String>,
    pub cardholder_name: Option<String>,
    pub country: Option<String>,
    pub emv_auth_data: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub generated_card: Option<String>,
    pub last4: Option<String>,
    pub network: Option<String>,
    pub read_method: Option<UniStrReadMethod>,
    pub receipt: Option<PaymentMethodDetailsCardPresentReceipt>,
}

/// Spec paths:
/// - `payment_method_details_card_present.read_method`
/// - `payment_method_details_interac_present.read_method`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrReadMethod {
    #[serde(rename = "contact_emv")]
    ContactEmv,
    #[serde(rename = "contactless_emv")]
    ContactlessEmv,
    #[serde(rename = "contactless_magstripe_mode")]
    ContactlessMagstripeMode,
    #[serde(rename = "magnetic_stripe_fallback")]
    MagneticStripeFallback,
    #[serde(rename = "magnetic_stripe_track2")]
    MagneticStripeTrack2,
}

/// Spec paths:
/// - `payment_method_details_card_present_receipt`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsCardPresentReceipt {
    pub account_type: Option<UniStrAccountType769528>,
    pub application_cryptogram: Option<String>,
    pub application_preferred_name: Option<String>,
    pub authorization_code: Option<String>,
    pub authorization_response_code: Option<String>,
    pub cardholder_verification_method: Option<String>,
    pub dedicated_file_name: Option<String>,
    pub terminal_verification_results: Option<String>,
    pub transaction_status_information: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_card_present_receipt.account_type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrAccountType769528 {
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "prepaid")]
    Prepaid,
    #[serde(rename = "unknown")]
    Unknown,
}

/// Spec paths:
/// - `payment_method_details_card_wallet`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsCardWallet {
    #[serde(rename = "type")]
    pub type_x: UniStrType0A6777,
    pub amex_express_checkout: Option<PaymentMethodDetailsCardWalletAmexExpressCheckout>,
    pub apple_pay: Option<PaymentMethodDetailsCardWalletApplePay>,
    pub dynamic_last4: Option<String>,
    pub google_pay: Option<PaymentMethodDetailsCardWalletGooglePay>,
    pub masterpass: Option<PaymentMethodDetailsCardWalletMasterpass>,
    pub samsung_pay: Option<PaymentMethodDetailsCardWalletSamsungPay>,
    pub visa_checkout: Option<PaymentMethodDetailsCardWalletVisaCheckout>,
}

/// Spec paths:
/// - `payment_method_details_card_wallet_amex_express_checkout`
pub type PaymentMethodDetailsCardWalletAmexExpressCheckout = Value;

/// Spec paths:
/// - `payment_method_details_card_wallet_apple_pay`
pub type PaymentMethodDetailsCardWalletApplePay = Value;

/// Spec paths:
/// - `payment_method_details_card_wallet_google_pay`
pub type PaymentMethodDetailsCardWalletGooglePay = Value;

/// Spec paths:
/// - `payment_method_details_card_wallet_masterpass`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsCardWalletMasterpass {
    pub name: Option<String>,
    pub billing_address: Option<Address>,
    pub email: Option<String>,
    pub shipping_address: Option<Address>,
}

/// Spec paths:
/// - `payment_method_details_card_wallet_samsung_pay`
pub type PaymentMethodDetailsCardWalletSamsungPay = Value;

/// Spec paths:
/// - `payment_method_details_card_wallet_visa_checkout`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsCardWalletVisaCheckout {
    pub name: Option<String>,
    pub billing_address: Option<Address>,
    pub email: Option<String>,
    pub shipping_address: Option<Address>,
}

/// Spec paths:
/// - `payment_method_details_eps`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsEps {
    pub verified_name: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_fpx`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsFpx {
    pub transaction_id: Option<String>,
    pub bank: UniStrBank211045,
}

/// Spec paths:
/// - `payment_method_details_fpx.bank`
/// - `payment_method_fpx.bank`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrBank211045 {
    #[serde(rename = "affin_bank")]
    AffinBank,
    #[serde(rename = "alliance_bank")]
    AllianceBank,
    #[serde(rename = "ambank")]
    Ambank,
    #[serde(rename = "bank_islam")]
    BankIslam,
    #[serde(rename = "bank_muamalat")]
    BankMuamalat,
    #[serde(rename = "bank_rakyat")]
    BankRakyat,
    #[serde(rename = "bsn")]
    Bsn,
    #[serde(rename = "cimb")]
    Cimb,
    #[serde(rename = "deutsche_bank")]
    DeutscheBank,
    #[serde(rename = "hong_leong_bank")]
    HongLeongBank,
    #[serde(rename = "hsbc")]
    Hsbc,
    #[serde(rename = "kfh")]
    Kfh,
    #[serde(rename = "maybank2e")]
    Maybank2e,
    #[serde(rename = "maybank2u")]
    Maybank2u,
    #[serde(rename = "ocbc")]
    Ocbc,
    #[serde(rename = "pb_enterprise")]
    PbEnterprise,
    #[serde(rename = "public_bank")]
    PublicBank,
    #[serde(rename = "rhb")]
    Rhb,
    #[serde(rename = "standard_chartered")]
    StandardChartered,
    #[serde(rename = "uob")]
    Uob,
}

/// Spec paths:
/// - `payment_method_details_giropay`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsGiropay {
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub verified_name: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_grabpay`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsGrabpay {
    pub transaction_id: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_ideal`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsIdeal {
    pub generated_sepa_debit: Option<UniPaymentMethod>,
    pub generated_sepa_debit_mandate: Option<UniMandate>,
    pub bank: Option<UniStrBank3AE87A>,
    pub bic: Option<UniStrBic>,
    pub iban_last4: Option<String>,
    pub verified_name: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_ideal.bank`
/// - `payment_method_ideal.bank`
/// - `setup_attempt_payment_method_details_ideal.bank`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrBank3AE87A {
    #[serde(rename = "abn_amro")]
    AbnAmro,
    #[serde(rename = "asn_bank")]
    AsnBank,
    #[serde(rename = "bunq")]
    Bunq,
    #[serde(rename = "handelsbanken")]
    Handelsbanken,
    #[serde(rename = "ing")]
    Ing,
    #[serde(rename = "knab")]
    Knab,
    #[serde(rename = "moneyou")]
    Moneyou,
    #[serde(rename = "rabobank")]
    Rabobank,
    #[serde(rename = "regiobank")]
    Regiobank,
    #[serde(rename = "sns_bank")]
    SnsBank,
    #[serde(rename = "triodos_bank")]
    TriodosBank,
    #[serde(rename = "van_lanschot")]
    VanLanschot,
}

/// Spec paths:
/// - `payment_method_details_ideal.bic`
/// - `payment_method_ideal.bic`
/// - `setup_attempt_payment_method_details_ideal.bic`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrBic {
    #[serde(rename = "ABNANL2A")]
    ABNANL2A,
    #[serde(rename = "ASNBNL21")]
    ASNBNL21,
    #[serde(rename = "BUNQNL2A")]
    BUNQNL2A,
    #[serde(rename = "FVLBNL22")]
    FVLBNL22,
    #[serde(rename = "HANDNL2A")]
    HANDNL2A,
    #[serde(rename = "INGBNL2A")]
    INGBNL2A,
    #[serde(rename = "KNABNL2H")]
    KNABNL2H,
    #[serde(rename = "MOYONL21")]
    MOYONL21,
    #[serde(rename = "RABONL2U")]
    RABONL2U,
    #[serde(rename = "RBRBNL21")]
    RBRBNL21,
    #[serde(rename = "SNSBNL2A")]
    SNSBNL2A,
    #[serde(rename = "TRIONL2U")]
    TRIONL2U,
}

/// Spec paths:
/// - `payment_method_details_interac_present`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsInteracPresent {
    pub brand: Option<String>,
    pub cardholder_name: Option<String>,
    pub country: Option<String>,
    pub emv_auth_data: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub generated_card: Option<String>,
    pub last4: Option<String>,
    pub network: Option<String>,
    pub preferred_locales: Option<Vec<String>>,
    pub read_method: Option<UniStrReadMethod>,
    pub receipt: Option<PaymentMethodDetailsInteracPresentReceipt>,
}

/// Spec paths:
/// - `payment_method_details_interac_present_receipt`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsInteracPresentReceipt {
    pub account_type: Option<UniStrAccountTypeE55AAC>,
    pub application_cryptogram: Option<String>,
    pub application_preferred_name: Option<String>,
    pub authorization_code: Option<String>,
    pub authorization_response_code: Option<String>,
    pub cardholder_verification_method: Option<String>,
    pub dedicated_file_name: Option<String>,
    pub terminal_verification_results: Option<String>,
    pub transaction_status_information: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_interac_present_receipt.account_type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrAccountTypeE55AAC {
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "savings")]
    Savings,
    #[serde(rename = "unknown")]
    Unknown,
}

/// Spec paths:
/// - `payment_method_details_klarna`
pub type PaymentMethodDetailsKlarna = Value;

/// Spec paths:
/// - `payment_method_details_multibanco`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsMultibanco {
    pub entity: Option<String>,
    pub reference: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_oxxo`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsOxxo {
    pub number: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_p24`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsP24 {
    pub reference: Option<String>,
    pub verified_name: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_sepa_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsSepaDebit {
    pub bank_code: Option<String>,
    pub branch_code: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub mandate: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_sofort`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodDetailsSofort {
    pub generated_sepa_debit: Option<UniPaymentMethod>,
    pub generated_sepa_debit_mandate: Option<UniMandate>,
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub country: Option<String>,
    pub iban_last4: Option<String>,
    pub preferred_language: Option<UniStrPreferredLanguage5C252C>,
    pub verified_name: Option<String>,
}

/// Spec paths:
/// - `payment_method_details_sofort.preferred_language`
/// - `payment_method_options_sofort.preferred_language`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrPreferredLanguage5C252C {
    #[serde(rename = "de")]
    De,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "pl")]
    Pl,
}

/// Spec paths:
/// - `payment_method_details_stripe_account`
pub type PaymentMethodDetailsStripeAccount = Value;

/// Spec paths:
/// - `payment_method_details_wechat`
pub type PaymentMethodDetailsWechat = Value;

/// Spec paths:
/// - `payment_method_eps`
pub type PaymentMethodEps = Value;

/// Spec paths:
/// - `payment_method_fpx`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodFpx {
    pub bank: UniStrBank211045,
}

/// Spec paths:
/// - `payment_method_giropay`
pub type PaymentMethodGiropay = Value;

/// Spec paths:
/// - `payment_method_grabpay`
pub type PaymentMethodGrabpay = Value;

/// Spec paths:
/// - `payment_method_ideal`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodIdeal {
    pub bank: Option<UniStrBank3AE87A>,
    pub bic: Option<UniStrBic>,
}

/// Spec paths:
/// - `payment_method_interac_present`
pub type PaymentMethodInteracPresent = Value;

/// Spec paths:
/// - `payment_method_options_alipay`
pub type PaymentMethodOptionsAlipay = Value;

/// Spec paths:
/// - `payment_method_options_bancontact`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodOptionsBancontact {
    pub preferred_language: UniStrPreferredLanguageD97AA3,
}

/// Spec paths:
/// - `payment_method_options_card_installments`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodOptionsCardInstallments {
    pub available_plans: Option<Vec<PaymentMethodDetailsCardInstallmentsPlan>>,
    pub enabled: bool,
    pub plan: Option<PaymentMethodDetailsCardInstallmentsPlan>,
}

/// Spec paths:
/// - `payment_method_options_oxxo`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodOptionsOxxo {
    pub expires_after_days: i64,
}

/// Spec paths:
/// - `payment_method_options_p24`
pub type PaymentMethodOptionsP24 = Value;

/// Spec paths:
/// - `payment_method_options_sofort`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodOptionsSofort {
    pub preferred_language: Option<UniStrPreferredLanguage5C252C>,
}

/// Spec paths:
/// - `payment_method_oxxo`
pub type PaymentMethodOxxo = Value;

/// Spec paths:
/// - `payment_method_p24`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodP24 {
    pub bank: Option<UniStrBank>,
}

/// Spec paths:
/// - `payment_method_p24.bank`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrBank {
    #[serde(rename = "alior_bank")]
    AliorBank,
    #[serde(rename = "bank_millennium")]
    BankMillennium,
    #[serde(rename = "bank_nowy_bfg_sa")]
    BankNowyBfgSa,
    #[serde(rename = "bank_pekao_sa")]
    BankPekaoSa,
    #[serde(rename = "banki_spbdzielcze")]
    BankiSpbdzielcze,
    #[serde(rename = "blik")]
    Blik,
    #[serde(rename = "bnp_paribas")]
    BnpParibas,
    #[serde(rename = "boz")]
    Boz,
    #[serde(rename = "citi_handlowy")]
    CitiHandlowy,
    #[serde(rename = "credit_agricole")]
    CreditAgricole,
    #[serde(rename = "envelobank")]
    Envelobank,
    #[serde(rename = "etransfer_pocztowy24")]
    EtransferPocztowy24,
    #[serde(rename = "getin_bank")]
    GetinBank,
    #[serde(rename = "ideabank")]
    Ideabank,
    #[serde(rename = "ing")]
    Ing,
    #[serde(rename = "inteligo")]
    Inteligo,
    #[serde(rename = "mbank_mtransfer")]
    MbankMtransfer,
    #[serde(rename = "nest_przelew")]
    NestPrzelew,
    #[serde(rename = "noble_pay")]
    NoblePay,
    #[serde(rename = "pbac_z_ipko")]
    PbacZIpko,
    #[serde(rename = "plus_bank")]
    PlusBank,
    #[serde(rename = "santander_przelew24")]
    SantanderPrzelew24,
    #[serde(rename = "tmobile_usbugi_bankowe")]
    TmobileUsbugiBankowe,
    #[serde(rename = "toyota_bank")]
    ToyotaBank,
    #[serde(rename = "volkswagen_bank")]
    VolkswagenBank,
}

/// Spec paths:
/// - `payment_method_sepa_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodSepaDebit {
    pub bank_code: Option<String>,
    pub branch_code: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub generated_from: Option<SepaDebitGeneratedFrom>,
    pub last4: Option<String>,
}

/// Spec paths:
/// - `payment_method_sofort`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethodSofort {
    pub country: Option<String>,
}

/// Spec paths:
/// - `payment_pages_checkout_session_total_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentPagesCheckoutSessionTotalDetails {
    pub amount_discount: i64,
    pub amount_tax: i64,
    pub breakdown: Option<PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown>,
}

/// Spec paths:
/// - `payment_pages_checkout_session_total_details_resource_breakdown`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    pub discounts: Vec<LineItemsDiscountAmount>,
    pub taxes: Vec<LineItemsTaxAmount>,
}

/// Spec paths:
/// - `payment_pages_payment_page_resources_shipping_address_collection`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentPagesPaymentPageResourcesShippingAddressCollection {
    pub allowed_countries: Vec<UniStrItems670930>,
}

/// Spec paths:
/// - `payment_pages_payment_page_resources_shipping_address_collection.allowed_countries.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrItems670930 {
    #[serde(rename = "AC")]
    AC,
    #[serde(rename = "AD")]
    AD,
    #[serde(rename = "AE")]
    AE,
    #[serde(rename = "AF")]
    AF,
    #[serde(rename = "AG")]
    AG,
    #[serde(rename = "AI")]
    AI,
    #[serde(rename = "AL")]
    AL,
    #[serde(rename = "AM")]
    AM,
    #[serde(rename = "AO")]
    AO,
    #[serde(rename = "AQ")]
    AQ,
    #[serde(rename = "AR")]
    AR,
    #[serde(rename = "AT")]
    AT,
    #[serde(rename = "AU")]
    AU,
    #[serde(rename = "AW")]
    AW,
    #[serde(rename = "AX")]
    AX,
    #[serde(rename = "AZ")]
    AZ,
    #[serde(rename = "BA")]
    BA,
    #[serde(rename = "BB")]
    BB,
    #[serde(rename = "BD")]
    BD,
    #[serde(rename = "BE")]
    BE,
    #[serde(rename = "BF")]
    BF,
    #[serde(rename = "BG")]
    BG,
    #[serde(rename = "BH")]
    BH,
    #[serde(rename = "BI")]
    BI,
    #[serde(rename = "BJ")]
    BJ,
    #[serde(rename = "BL")]
    BL,
    #[serde(rename = "BM")]
    BM,
    #[serde(rename = "BN")]
    BN,
    #[serde(rename = "BO")]
    BO,
    #[serde(rename = "BQ")]
    BQ,
    #[serde(rename = "BR")]
    BR,
    #[serde(rename = "BS")]
    BS,
    #[serde(rename = "BT")]
    BT,
    #[serde(rename = "BV")]
    BV,
    #[serde(rename = "BW")]
    BW,
    #[serde(rename = "BY")]
    BY,
    #[serde(rename = "BZ")]
    BZ,
    #[serde(rename = "CA")]
    CA,
    #[serde(rename = "CD")]
    CD,
    #[serde(rename = "CF")]
    CF,
    #[serde(rename = "CG")]
    CG,
    #[serde(rename = "CH")]
    CH,
    #[serde(rename = "CI")]
    CI,
    #[serde(rename = "CK")]
    CK,
    #[serde(rename = "CL")]
    CL,
    #[serde(rename = "CM")]
    CM,
    #[serde(rename = "CN")]
    CN,
    #[serde(rename = "CO")]
    CO,
    #[serde(rename = "CR")]
    CR,
    #[serde(rename = "CV")]
    CV,
    #[serde(rename = "CW")]
    CW,
    #[serde(rename = "CY")]
    CY,
    #[serde(rename = "CZ")]
    CZ,
    #[serde(rename = "DE")]
    DE,
    #[serde(rename = "DJ")]
    DJ,
    #[serde(rename = "DK")]
    DK,
    #[serde(rename = "DM")]
    DM,
    #[serde(rename = "DO")]
    DO,
    #[serde(rename = "DZ")]
    DZ,
    #[serde(rename = "EC")]
    EC,
    #[serde(rename = "EE")]
    EE,
    #[serde(rename = "EG")]
    EG,
    #[serde(rename = "EH")]
    EH,
    #[serde(rename = "ER")]
    ER,
    #[serde(rename = "ES")]
    ES,
    #[serde(rename = "ET")]
    ET,
    #[serde(rename = "FI")]
    FI,
    #[serde(rename = "FJ")]
    FJ,
    #[serde(rename = "FK")]
    FK,
    #[serde(rename = "FO")]
    FO,
    #[serde(rename = "FR")]
    FR,
    #[serde(rename = "GA")]
    GA,
    #[serde(rename = "GB")]
    GB,
    #[serde(rename = "GD")]
    GD,
    #[serde(rename = "GE")]
    GE,
    #[serde(rename = "GF")]
    GF,
    #[serde(rename = "GG")]
    GG,
    #[serde(rename = "GH")]
    GH,
    #[serde(rename = "GI")]
    GI,
    #[serde(rename = "GL")]
    GL,
    #[serde(rename = "GM")]
    GM,
    #[serde(rename = "GN")]
    GN,
    #[serde(rename = "GP")]
    GP,
    #[serde(rename = "GQ")]
    GQ,
    #[serde(rename = "GR")]
    GR,
    #[serde(rename = "GS")]
    GS,
    #[serde(rename = "GT")]
    GT,
    #[serde(rename = "GU")]
    GU,
    #[serde(rename = "GW")]
    GW,
    #[serde(rename = "GY")]
    GY,
    #[serde(rename = "HK")]
    HK,
    #[serde(rename = "HN")]
    HN,
    #[serde(rename = "HR")]
    HR,
    #[serde(rename = "HT")]
    HT,
    #[serde(rename = "HU")]
    HU,
    #[serde(rename = "ID")]
    ID,
    #[serde(rename = "IE")]
    IE,
    #[serde(rename = "IL")]
    IL,
    #[serde(rename = "IM")]
    IM,
    #[serde(rename = "IN")]
    IN,
    #[serde(rename = "IO")]
    IO,
    #[serde(rename = "IQ")]
    IQ,
    #[serde(rename = "IS")]
    IS,
    #[serde(rename = "IT")]
    IT,
    #[serde(rename = "JE")]
    JE,
    #[serde(rename = "JM")]
    JM,
    #[serde(rename = "JO")]
    JO,
    #[serde(rename = "JP")]
    JP,
    #[serde(rename = "KE")]
    KE,
    #[serde(rename = "KG")]
    KG,
    #[serde(rename = "KH")]
    KH,
    #[serde(rename = "KI")]
    KI,
    #[serde(rename = "KM")]
    KM,
    #[serde(rename = "KN")]
    KN,
    #[serde(rename = "KR")]
    KR,
    #[serde(rename = "KW")]
    KW,
    #[serde(rename = "KY")]
    KY,
    #[serde(rename = "KZ")]
    KZ,
    #[serde(rename = "LA")]
    LA,
    #[serde(rename = "LB")]
    LB,
    #[serde(rename = "LC")]
    LC,
    #[serde(rename = "LI")]
    LI,
    #[serde(rename = "LK")]
    LK,
    #[serde(rename = "LR")]
    LR,
    #[serde(rename = "LS")]
    LS,
    #[serde(rename = "LT")]
    LT,
    #[serde(rename = "LU")]
    LU,
    #[serde(rename = "LV")]
    LV,
    #[serde(rename = "LY")]
    LY,
    #[serde(rename = "MA")]
    MA,
    #[serde(rename = "MC")]
    MC,
    #[serde(rename = "MD")]
    MD,
    #[serde(rename = "ME")]
    ME,
    #[serde(rename = "MF")]
    MF,
    #[serde(rename = "MG")]
    MG,
    #[serde(rename = "MK")]
    MK,
    #[serde(rename = "ML")]
    ML,
    #[serde(rename = "MM")]
    MM,
    #[serde(rename = "MN")]
    MN,
    #[serde(rename = "MO")]
    MO,
    #[serde(rename = "MQ")]
    MQ,
    #[serde(rename = "MR")]
    MR,
    #[serde(rename = "MS")]
    MS,
    #[serde(rename = "MT")]
    MT,
    #[serde(rename = "MU")]
    MU,
    #[serde(rename = "MV")]
    MV,
    #[serde(rename = "MW")]
    MW,
    #[serde(rename = "MX")]
    MX,
    #[serde(rename = "MY")]
    MY,
    #[serde(rename = "MZ")]
    MZ,
    #[serde(rename = "NA")]
    NA,
    #[serde(rename = "NC")]
    NC,
    #[serde(rename = "NE")]
    NE,
    #[serde(rename = "NG")]
    NG,
    #[serde(rename = "NI")]
    NI,
    #[serde(rename = "NL")]
    NL,
    #[serde(rename = "NO")]
    NO,
    #[serde(rename = "NP")]
    NP,
    #[serde(rename = "NR")]
    NR,
    #[serde(rename = "NU")]
    NU,
    #[serde(rename = "NZ")]
    NZ,
    #[serde(rename = "OM")]
    OM,
    #[serde(rename = "PA")]
    PA,
    #[serde(rename = "PE")]
    PE,
    #[serde(rename = "PF")]
    PF,
    #[serde(rename = "PG")]
    PG,
    #[serde(rename = "PH")]
    PH,
    #[serde(rename = "PK")]
    PK,
    #[serde(rename = "PL")]
    PL,
    #[serde(rename = "PM")]
    PM,
    #[serde(rename = "PN")]
    PN,
    #[serde(rename = "PR")]
    PR,
    #[serde(rename = "PS")]
    PS,
    #[serde(rename = "PT")]
    PT,
    #[serde(rename = "PY")]
    PY,
    #[serde(rename = "QA")]
    QA,
    #[serde(rename = "RE")]
    RE,
    #[serde(rename = "RO")]
    RO,
    #[serde(rename = "RS")]
    RS,
    #[serde(rename = "RU")]
    RU,
    #[serde(rename = "RW")]
    RW,
    #[serde(rename = "SA")]
    SA,
    #[serde(rename = "SB")]
    SB,
    #[serde(rename = "SC")]
    SC,
    #[serde(rename = "SE")]
    SE,
    #[serde(rename = "SG")]
    SG,
    #[serde(rename = "SH")]
    SH,
    #[serde(rename = "SI")]
    SI,
    #[serde(rename = "SJ")]
    SJ,
    #[serde(rename = "SK")]
    SK,
    #[serde(rename = "SL")]
    SL,
    #[serde(rename = "SM")]
    SM,
    #[serde(rename = "SN")]
    SN,
    #[serde(rename = "SO")]
    SO,
    #[serde(rename = "SR")]
    SR,
    #[serde(rename = "SS")]
    SS,
    #[serde(rename = "ST")]
    ST,
    #[serde(rename = "SV")]
    SV,
    #[serde(rename = "SX")]
    SX,
    #[serde(rename = "SZ")]
    SZ,
    #[serde(rename = "TA")]
    TA,
    #[serde(rename = "TC")]
    TC,
    #[serde(rename = "TD")]
    TD,
    #[serde(rename = "TF")]
    TF,
    #[serde(rename = "TG")]
    TG,
    #[serde(rename = "TH")]
    TH,
    #[serde(rename = "TJ")]
    TJ,
    #[serde(rename = "TK")]
    TK,
    #[serde(rename = "TL")]
    TL,
    #[serde(rename = "TM")]
    TM,
    #[serde(rename = "TN")]
    TN,
    #[serde(rename = "TO")]
    TO,
    #[serde(rename = "TR")]
    TR,
    #[serde(rename = "TT")]
    TT,
    #[serde(rename = "TV")]
    TV,
    #[serde(rename = "TW")]
    TW,
    #[serde(rename = "TZ")]
    TZ,
    #[serde(rename = "UA")]
    UA,
    #[serde(rename = "UG")]
    UG,
    #[serde(rename = "US")]
    US,
    #[serde(rename = "UY")]
    UY,
    #[serde(rename = "UZ")]
    UZ,
    #[serde(rename = "VA")]
    VA,
    #[serde(rename = "VC")]
    VC,
    #[serde(rename = "VE")]
    VE,
    #[serde(rename = "VG")]
    VG,
    #[serde(rename = "VN")]
    VN,
    #[serde(rename = "VU")]
    VU,
    #[serde(rename = "WF")]
    WF,
    #[serde(rename = "WS")]
    WS,
    #[serde(rename = "XK")]
    XK,
    #[serde(rename = "YE")]
    YE,
    #[serde(rename = "YT")]
    YT,
    #[serde(rename = "ZA")]
    ZA,
    #[serde(rename = "ZM")]
    ZM,
    #[serde(rename = "ZW")]
    ZW,
    #[serde(rename = "ZZ")]
    ZZ,
}

/// Spec paths:
/// - `payment_source`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniPolymorphic {
    Account(Account),
    AlipayAccount(AlipayAccount),
    BankAccount(BankAccount),
    BitcoinReceiver(BitcoinReceiver),
    Card(Box<Card>),
    Source(Source),
}

/// Spec paths:
/// - `payout`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payout {
    pub object: UniStrObject736393,
    #[serde(rename = "type")]
    pub type_x: UniStrTypeB8E5F5,
    pub id: String,
    pub balance_transaction: Option<UniBalanceTransaction>,
    pub destination: Option<UniDestination>,
    pub failure_balance_transaction: Option<UniBalanceTransaction>,
    pub original_payout: Option<UniReversedBy>,
    pub reversed_by: Option<UniReversedBy>,
    pub amount: i64,
    pub arrival_date: i64,
    pub automatic: bool,
    pub currency: String,
    pub description: Option<String>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub method: String,
    pub source_type: String,
    pub statement_descriptor: Option<String>,
    pub status: String,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for Payout {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `payout.destination`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniDestination {
    String(String),
    BankAccount(BankAccount),
    Card(Box<Card>),
    DeletedBankAccount(DeletedBankAccount),
    DeletedCard(DeletedCard),
}

/// Spec paths:
/// - `payout.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject736393 {
    #[serde(rename = "payout")]
    Payout,
}

/// Spec paths:
/// - `payout.original_payout`
/// - `payout.reversed_by`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniReversedBy {
    String(String),
    Payout(Box<Payout>),
}

/// Spec paths:
/// - `payout.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTypeB8E5F5 {
    #[serde(rename = "bank_account")]
    BankAccount,
    #[serde(rename = "card")]
    Card,
}

/// Spec paths:
/// - `period`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Period640F9C {
    pub end: Option<i64>,
    pub start: Option<i64>,
}

/// Spec paths:
/// - `person`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub object: UniStrObjectAFEDA0,
    pub id: String,
    pub account: String,
    pub address: Option<Address>,
    pub address_kana: Option<LegalEntityJapanAddress>,
    pub address_kanji: Option<LegalEntityJapanAddress>,
    pub dob: Option<LegalEntityDOB>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub first_name_kana: Option<String>,
    pub first_name_kanji: Option<String>,
    pub gender: Option<String>,
    pub id_number_provided: Option<bool>,
    pub last_name: Option<String>,
    pub last_name_kana: Option<String>,
    pub last_name_kanji: Option<String>,
    pub maiden_name: Option<String>,
    pub phone: Option<String>,
    pub political_exposure: Option<UniStrPoliticalExposure>,
    pub relationship: Option<PersonRelationship>,
    pub requirements: Option<PersonRequirements>,
    pub ssn_last_4_provided: Option<bool>,
    pub verification: Option<LegalEntityPersonVerification>,
    pub created: i64,
    pub metadata: Option<Metadata8076DB>,
}

impl GetId for Person {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `person.political_exposure`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrPoliticalExposure {
    #[serde(rename = "existing")]
    Existing,
    #[serde(rename = "none")]
    None,
}

/// Spec paths:
/// - `person_relationship`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersonRelationship {
    pub director: Option<bool>,
    pub executive: Option<bool>,
    pub owner: Option<bool>,
    pub percent_ownership: Option<f64>,
    pub representative: Option<bool>,
    pub title: Option<String>,
}

/// Spec paths:
/// - `person_requirements`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersonRequirements {
    pub currently_due: Vec<String>,
    pub errors: Vec<AccountRequirementsError>,
    pub eventually_due: Vec<String>,
    pub past_due: Vec<String>,
    pub pending_verification: Vec<String>,
}

/// Spec paths:
/// - `plan`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plan {
    pub object: UniStrObjectB95344,
    pub id: String,
    pub product: Option<UniProduct297E1E>,
    pub active: bool,
    pub aggregate_usage: Option<UniStrAggregateUsage>,
    pub amount: Option<i64>,
    pub amount_decimal: Option<String>,
    pub billing_scheme: UniStrBillingScheme,
    pub currency: String,
    pub interval: UniStrInterval,
    pub interval_count: i64,
    pub nickname: Option<String>,
    pub tiers: Option<Vec<PlanTier>>,
    pub tiers_mode: Option<UniStrTiersMode>,
    pub transform_usage: Option<TransformUsage>,
    pub trial_period_days: Option<i64>,
    pub usage_type: UniStrUsageType,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for Plan {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `plan.aggregate_usage`
/// - `recurring.aggregate_usage`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrAggregateUsage {
    #[serde(rename = "last_during_period")]
    LastDuringPeriod,
    #[serde(rename = "last_ever")]
    LastEver,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "sum")]
    Sum,
}

/// Spec paths:
/// - `plan.billing_scheme`
/// - `price.billing_scheme`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrBillingScheme {
    #[serde(rename = "per_unit")]
    PerUnit,
    #[serde(rename = "tiered")]
    Tiered,
}

/// Spec paths:
/// - `plan.product`
/// - `price.product`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniProduct297E1E {
    String(String),
    Product(Product),
    DeletedProduct(DeletedProduct),
}

/// Spec paths:
/// - `plan.tiers_mode`
/// - `price.tiers_mode`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTiersMode {
    #[serde(rename = "graduated")]
    Graduated,
    #[serde(rename = "volume")]
    Volume,
}

/// Spec paths:
/// - `plan_tier`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlanTier {
    pub flat_amount: Option<i64>,
    pub flat_amount_decimal: Option<String>,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,
    pub up_to: Option<i64>,
}

/// Spec paths:
/// - `platform_tax_fee`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformTax {
    pub object: UniStrObject1A472C,
    #[serde(rename = "type")]
    pub type_x: String,
    pub id: String,
    pub account: String,
    pub source_transaction: String,
}

impl GetId for PlatformTax {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `platform_tax_fee.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject1A472C {
    #[serde(rename = "platform_tax_fee")]
    PlatformTaxFee,
}

/// Spec paths:
/// - `price`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Price {
    pub object: UniStrObjectDB9846,
    #[serde(rename = "type")]
    pub type_x: UniStrTypeFC33AE,
    pub id: String,
    pub product: UniProduct297E1E,
    pub active: bool,
    pub billing_scheme: UniStrBillingScheme,
    pub currency: String,
    pub lookup_key: Option<String>,
    pub nickname: Option<String>,
    pub recurring: Option<Recurring>,
    pub tiers: Option<Vec<PriceTier>>,
    pub tiers_mode: Option<UniStrTiersMode>,
    pub transform_quantity: Option<TransformQuantity>,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for Price {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `price_tier`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceTier {
    pub flat_amount: Option<i64>,
    pub flat_amount_decimal: Option<String>,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,
    pub up_to: Option<i64>,
}

/// Spec paths:
/// - `product`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub object: UniStrObject04CB4A,
    pub id: String,
    pub name: String,
    pub active: bool,
    pub attributes: Option<Vec<String>>,
    pub caption: Option<String>,
    pub deactivate_on: Option<Vec<String>>,
    pub description: Option<String>,
    pub images: Vec<String>,
    pub package_dimensions: Option<PackageDimensions>,
    pub shippable: Option<bool>,
    pub statement_descriptor: Option<String>,
    pub unit_label: Option<String>,
    pub url: Option<String>,
    pub created: i64,
    pub updated: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for Product {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `promotion_code`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromotionCode {
    pub object: UniStrObjectB3640E,
    pub id: String,
    pub customer: Option<UniCustomerC00F6E>,
    pub active: bool,
    pub code: String,
    pub coupon: Coupon,
    pub expires_at: Option<i64>,
    pub max_redemptions: Option<i64>,
    pub restrictions: PromotionCodesResourceRestrictions,
    pub times_redeemed: i64,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for PromotionCode {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `promotion_code.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectB3640E {
    #[serde(rename = "promotion_code")]
    PromotionCode,
}

/// Spec paths:
/// - `promotion_codes_resource_restrictions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromotionCodesResourceRestrictions {
    pub first_time_transaction: bool,
    pub minimum_amount: Option<i64>,
    pub minimum_amount_currency: Option<String>,
}

/// Spec paths:
/// - `radar.early_fraud_warning`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadarEarlyFraudWarning {
    pub object: UniStrObject08D5B1,
    pub id: String,
    pub charge: UniCharge,
    pub actionable: bool,
    pub fraud_type: String,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for RadarEarlyFraudWarning {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `radar.early_fraud_warning.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject08D5B1 {
    #[serde(rename = "radar.early_fraud_warning")]
    RadarDotEarlyFraudWarning,
}

/// Spec paths:
/// - `radar.value_list`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadarListList {
    pub object: UniStrObjectF61F9F,
    pub id: String,
    pub name: String,
    pub alias: String,
    pub created_by: String,
    pub item_type: UniStrItemType,
    pub list_items: RadarListListItemList,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for RadarListList {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `radar.value_list.item_type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrItemType {
    #[serde(rename = "card_bin")]
    CardBin,
    #[serde(rename = "card_fingerprint")]
    CardFingerprint,
    #[serde(rename = "case_sensitive_string")]
    CaseSensitiveString,
    #[serde(rename = "country")]
    Country,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "ip_address")]
    IpAddress,
    #[serde(rename = "string")]
    String,
}

/// Spec paths:
/// - `radar.value_list.list_items`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadarListListItemList {
    pub object: UniStrObject344B0E,
    pub data: Vec<RadarListListItem>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for RadarListListItemList {
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
/// - `radar.value_list_item`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadarListListItem {
    pub object: UniStrObject2EE88E,
    pub id: String,
    pub created_by: String,
    pub value: String,
    pub value_list: String,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for RadarListListItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `radar_review_resource_location`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadarReviewResourceLocation {
    pub city: Option<String>,
    pub country: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub region: Option<String>,
}

/// Spec paths:
/// - `radar_review_resource_session`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadarReviewResourceSession {
    pub browser: Option<String>,
    pub device: Option<String>,
    pub platform: Option<String>,
    pub version: Option<String>,
}

/// Spec paths:
/// - `recipient`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferRecipient {
    pub object: UniStrObject00DC3E,
    #[serde(rename = "type")]
    pub type_x: String,
    pub id: String,
    pub name: Option<String>,
    pub default_card: Option<UniDefaultCard>,
    pub migrated_to: Option<UniAccount>,
    pub rolled_back_from: Option<UniAccount>,
    pub active_account: Option<BankAccount>,
    pub cards: Option<CardList93D321>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for TransferRecipient {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `recipient.cards`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardList93D321 {
    pub object: UniStrObject344B0E,
    pub data: Vec<Card>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for CardList93D321 {
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
/// - `recipient.default_card`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniDefaultCard {
    String(String),
    Card(Box<Card>),
}

/// Spec paths:
/// - `recurring`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recurring {
    pub aggregate_usage: Option<UniStrAggregateUsage>,
    pub interval: UniStrInterval,
    pub interval_count: i64,
    pub usage_type: UniStrUsageType,
}

/// Spec paths:
/// - `refund`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Refund {
    pub object: UniStrObject95EBF9,
    pub id: String,
    pub balance_transaction: Option<UniBalanceTransaction>,
    pub charge: Option<UniCharge>,
    pub failure_balance_transaction: Option<UniBalanceTransaction>,
    pub payment_intent: Option<UniPaymentIntent>,
    pub source_transfer_reversal: Option<UniTransferReversal>,
    pub transfer_reversal: Option<UniTransferReversal>,
    pub amount: i64,
    pub currency: String,
    pub description: Option<String>,
    pub failure_reason: Option<String>,
    pub reason: Option<String>,
    pub receipt_number: Option<String>,
    pub status: Option<String>,
    pub created: i64,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for Refund {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `refund.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject95EBF9 {
    #[serde(rename = "refund")]
    Refund,
}

/// Spec paths:
/// - `refund.source_transfer_reversal`
/// - `refund.transfer_reversal`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniTransferReversal {
    String(String),
    TransferReversal(TransferReversal),
}

/// Spec paths:
/// - `reporting.report_run`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportingReportRun {
    pub object: UniStrObject24E2D4,
    pub id: String,
    pub error: Option<String>,
    pub parameters: FinancialReportingFinanceReportRunRunParameters,
    pub report_type: String,
    pub result: Option<File>,
    pub status: String,
    pub succeeded_at: Option<i64>,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for ReportingReportRun {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `reporting.report_run.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject24E2D4 {
    #[serde(rename = "reporting.report_run")]
    ReportingDotReportRun,
}

/// Spec paths:
/// - `reporting.report_type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportingReportType {
    pub object: UniStrObjectCB017F,
    pub id: String,
    pub name: String,
    pub data_available_end: i64,
    pub data_available_start: i64,
    pub default_columns: Option<Vec<String>>,
    pub version: i64,
    pub updated: i64,
}

impl GetId for ReportingReportType {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `reporting.report_type.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectCB017F {
    #[serde(rename = "reporting.report_type")]
    ReportingDotReportType,
}

/// Spec paths:
/// - `reserve_transaction`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReserveTransaction {
    pub object: UniStrObjectF852AF,
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub description: Option<String>,
}

impl GetId for ReserveTransaction {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `reserve_transaction.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectF852AF {
    #[serde(rename = "reserve_transaction")]
    ReserveTransaction,
}

/// Spec paths:
/// - `review`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadarReview {
    pub object: UniStrObject31593E,
    pub id: String,
    pub charge: Option<UniCharge>,
    pub payment_intent: Option<UniPaymentIntent>,
    pub billing_zip: Option<String>,
    pub closed_reason: Option<UniStrClosedReason>,
    pub ip_address: Option<String>,
    pub ip_address_location: Option<RadarReviewResourceLocation>,
    pub open: bool,
    pub opened_reason: UniStrOpenedReason,
    pub reason: String,
    pub session: Option<RadarReviewResourceSession>,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for RadarReview {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `review.closed_reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrClosedReason {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "disputed")]
    Disputed,
    #[serde(rename = "refunded")]
    Refunded,
    #[serde(rename = "refunded_as_fraud")]
    RefundedAsFraud,
}

/// Spec paths:
/// - `review.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject31593E {
    #[serde(rename = "review")]
    Review,
}

/// Spec paths:
/// - `review.opened_reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrOpenedReason {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "rule")]
    Rule,
}

/// Spec paths:
/// - `rule`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadarRule {
    pub id: String,
    pub action: String,
    pub predicate: String,
}

impl GetId for RadarRule {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `scheduled_query_run`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScheduledQueryRun {
    pub object: UniStrObject7F3E77,
    pub id: String,
    pub data_load_time: i64,
    pub error: Option<SigmaScheduledQueryRunError>,
    pub file: Option<File>,
    pub result_available_until: i64,
    pub sql: String,
    pub status: String,
    pub title: String,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for ScheduledQueryRun {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `scheduled_query_run.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject7F3E77 {
    #[serde(rename = "scheduled_query_run")]
    ScheduledQueryRun,
}

/// Spec paths:
/// - `sepa_debit_generated_from`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SepaDebitGeneratedFrom {
    pub charge: Option<UniCharge>,
    pub setup_attempt: Option<UniLatestAttempt>,
}

/// Spec paths:
/// - `sepa_debit_generated_from.setup_attempt`
/// - `setup_intent.latest_attempt`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniLatestAttempt {
    String(String),
    PaymentFlowsSetupIntentSetupAttempt(PaymentFlowsSetupIntentSetupAttempt),
}

/// Spec paths:
/// - `setup_attempt`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentFlowsSetupIntentSetupAttempt {
    pub object: UniStrObject165299,
    pub id: String,
    pub application: Option<UniApplication>,
    pub customer: Option<UniCustomerC00F6E>,
    pub on_behalf_of: Option<UniAccount>,
    pub payment_method: UniPaymentMethod,
    pub setup_intent: UniSetupIntent,
    pub payment_method_details: SetupAttemptPaymentMethodDetails,
    pub setup_error: Box<Option<APIErrors>>,
    pub status: String,
    pub usage: String,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for PaymentFlowsSetupIntentSetupAttempt {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `setup_attempt.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject165299 {
    #[serde(rename = "setup_attempt")]
    SetupAttempt,
}

/// Spec paths:
/// - `setup_attempt_payment_method_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupAttemptPaymentMethodDetails {
    #[serde(rename = "type")]
    pub type_x: String,
    pub bancontact: Option<SetupAttemptPaymentMethodDetailsBancontact>,
    pub card: Option<SetupAttemptPaymentMethodDetailsCard>,
    pub ideal: Option<SetupAttemptPaymentMethodDetailsIdeal>,
    pub sofort: Option<SetupAttemptPaymentMethodDetailsSofort>,
}

/// Spec paths:
/// - `setup_attempt_payment_method_details_bancontact`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupAttemptPaymentMethodDetailsBancontact {
    pub generated_sepa_debit: Option<UniPaymentMethod>,
    pub generated_sepa_debit_mandate: Option<UniMandate>,
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub iban_last4: Option<String>,
    pub preferred_language: Option<UniStrPreferredLanguageD97AA3>,
    pub verified_name: Option<String>,
}

/// Spec paths:
/// - `setup_attempt_payment_method_details_card`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupAttemptPaymentMethodDetailsCard {
    pub three_d_secure: Option<ThreeDSecureDetails>,
}

/// Spec paths:
/// - `setup_attempt_payment_method_details_ideal`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupAttemptPaymentMethodDetailsIdeal {
    pub generated_sepa_debit: Option<UniPaymentMethod>,
    pub generated_sepa_debit_mandate: Option<UniMandate>,
    pub bank: Option<UniStrBank3AE87A>,
    pub bic: Option<UniStrBic>,
    pub iban_last4: Option<String>,
    pub verified_name: Option<String>,
}

/// Spec paths:
/// - `setup_attempt_payment_method_details_sofort`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupAttemptPaymentMethodDetailsSofort {
    pub generated_sepa_debit: Option<UniPaymentMethod>,
    pub generated_sepa_debit_mandate: Option<UniMandate>,
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub iban_last4: Option<String>,
    pub preferred_language: Option<UniStrPreferredLanguageD97AA3>,
    pub verified_name: Option<String>,
}

/// Spec paths:
/// - `setup_intent`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupIntent {
    pub object: UniStrObjectE09BD0,
    pub id: String,
    pub application: Option<UniApplication>,
    pub customer: Option<UniCustomerC00F6E>,
    pub latest_attempt: Option<UniLatestAttempt>,
    pub mandate: Option<UniMandate>,
    pub on_behalf_of: Option<UniAccount>,
    pub payment_method: Option<UniPaymentMethod>,
    pub single_use_mandate: Option<UniMandate>,
    pub cancellation_reason: Option<UniStrCancellationReason>,
    pub client_secret: Option<String>,
    pub description: Option<String>,
    pub last_setup_error: Box<Option<APIErrors>>,
    pub next_action: Option<SetupIntentNextAction>,
    pub payment_method_options: Option<SetupIntentPaymentMethodOptions>,
    pub payment_method_types: Vec<String>,
    pub status: UniStrStatus87966D,
    pub usage: String,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for SetupIntent {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `setup_intent.cancellation_reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrCancellationReason {
    #[serde(rename = "abandoned")]
    Abandoned,
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "requested_by_customer")]
    RequestedByCustomer,
}

/// Spec paths:
/// - `setup_intent.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectE09BD0 {
    #[serde(rename = "setup_intent")]
    SetupIntent,
}

/// Spec paths:
/// - `setup_intent.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatus87966D {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "requires_action")]
    RequiresAction,
    #[serde(rename = "requires_confirmation")]
    RequiresConfirmation,
    #[serde(rename = "requires_payment_method")]
    RequiresPaymentMethod,
    #[serde(rename = "succeeded")]
    Succeeded,
}

/// Spec paths:
/// - `setup_intent_next_action`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupIntentNextAction {
    #[serde(rename = "type")]
    pub type_x: String,
    pub redirect_to_url: Option<SetupIntentNextActionRedirectToUrl>,
    pub use_stripe_sdk: Option<UseStripeSdk2470E2>,
}

/// Spec paths:
/// - `setup_intent_next_action.use_stripe_sdk`
pub type UseStripeSdk2470E2 = Value;

/// Spec paths:
/// - `setup_intent_next_action_redirect_to_url`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupIntentNextActionRedirectToUrl {
    pub return_url: Option<String>,
    pub url: Option<String>,
}

/// Spec paths:
/// - `setup_intent_payment_method_options`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupIntentPaymentMethodOptions {
    pub card: Option<SetupIntentPaymentMethodOptionsCard>,
    pub sepa_debit: Option<SetupIntentPaymentMethodOptionsSepaDebit>,
}

/// Spec paths:
/// - `setup_intent_payment_method_options_card`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupIntentPaymentMethodOptionsCard {
    pub request_three_d_secure: Option<UniStrRequestThreeDSecure>,
}

/// Spec paths:
/// - `setup_intent_payment_method_options_mandate_options_sepa_debit`
pub type SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit = Value;

/// Spec paths:
/// - `setup_intent_payment_method_options_sepa_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetupIntentPaymentMethodOptionsSepaDebit {
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit>,
}

/// Spec paths:
/// - `shipping`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shipping {
    pub name: Option<String>,
    pub address: Option<Address>,
    pub carrier: Option<String>,
    pub phone: Option<String>,
    pub tracking_number: Option<String>,
}

/// Spec paths:
/// - `shipping_method`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippingMethod {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub delivery_estimate: Option<DeliveryEstimate>,
    pub description: String,
}

impl GetId for ShippingMethod {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `sigma_scheduled_query_run_error`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SigmaScheduledQueryRunError {
    pub message: String,
}

/// Spec paths:
/// - `sku`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sku {
    pub object: UniStrObject97B705,
    pub id: String,
    pub product: UniProduct2CB1D4,
    pub active: bool,
    pub attributes: Attributes95B11A,
    pub currency: String,
    pub image: Option<String>,
    pub inventory: Inventory,
    pub package_dimensions: Option<PackageDimensions>,
    pub price: i64,
    pub created: i64,
    pub updated: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for Sku {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `sku.attributes`
pub type Attributes95B11A = HashMap<String, String>;

/// Spec paths:
/// - `sku.product`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniProduct2CB1D4 {
    String(String),
    Product(Product),
}

/// Spec paths:
/// - `source`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub object: UniStrObjectF7EF2D,
    #[serde(rename = "type")]
    pub type_x: UniStrType08BCE8,
    pub id: String,
    pub ach_credit_transfer: Option<SourceTypeAchCreditTransfer>,
    pub ach_debit: Option<SourceTypeAchDebit>,
    pub alipay: Option<SourceTypeAlipay>,
    pub amount: Option<i64>,
    pub au_becs_debit: Option<SourceTypeAuBecsDebit>,
    pub bancontact: Option<SourceTypeBancontact>,
    pub card: Option<SourceTypeCard>,
    pub card_present: Option<SourceTypeCardPresent>,
    pub client_secret: String,
    pub code_verification: Option<SourceCodeVerificationFlow>,
    pub currency: Option<String>,
    pub customer: Option<String>,
    pub eps: Option<SourceTypeEps>,
    pub flow: String,
    pub giropay: Option<SourceTypeGiropay>,
    pub ideal: Option<SourceTypeIdeal>,
    pub klarna: Option<SourceTypeKlarna>,
    pub multibanco: Option<SourceTypeMultibanco>,
    pub owner: Option<SourceOwner>,
    pub p24: Option<SourceTypeP24>,
    pub receiver: Option<SourceReceiverFlow>,
    pub redirect: Option<SourceRedirectFlow>,
    pub sepa_debit: Option<SourceTypeSepaDebit>,
    pub sofort: Option<SourceTypeSofort>,
    pub source_order: Option<SourceOrder>,
    pub statement_descriptor: Option<String>,
    pub status: String,
    pub three_d_secure: Option<SourceTypeThreeDSecure>,
    pub usage: Option<String>,
    pub wechat: Option<SourceTypeWechat>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for Source {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `source.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectF7EF2D {
    #[serde(rename = "source")]
    Source,
}

/// Spec paths:
/// - `source.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrType08BCE8 {
    #[serde(rename = "ach_credit_transfer")]
    AchCreditTransfer,
    #[serde(rename = "ach_debit")]
    AchDebit,
    #[serde(rename = "alipay")]
    Alipay,
    #[serde(rename = "au_becs_debit")]
    AuBecsDebit,
    #[serde(rename = "bancontact")]
    Bancontact,
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "card_present")]
    CardPresent,
    #[serde(rename = "eps")]
    Eps,
    #[serde(rename = "giropay")]
    Giropay,
    #[serde(rename = "ideal")]
    Ideal,
    #[serde(rename = "klarna")]
    Klarna,
    #[serde(rename = "multibanco")]
    Multibanco,
    #[serde(rename = "p24")]
    P24,
    #[serde(rename = "sepa_debit")]
    SepaDebit,
    #[serde(rename = "sofort")]
    Sofort,
    #[serde(rename = "three_d_secure")]
    ThreeDSecure,
    #[serde(rename = "wechat")]
    Wechat,
}

/// Spec paths:
/// - `source_code_verification_flow`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceCodeVerificationFlow {
    pub attempts_remaining: i64,
    pub status: String,
}

/// Spec paths:
/// - `source_mandate_notification`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceMandateNotification {
    pub object: UniStrObjectE9082A,
    #[serde(rename = "type")]
    pub type_x: String,
    pub id: String,
    pub acss_debit: Option<SourceMandateNotificationAcssDebitData>,
    pub amount: Option<i64>,
    pub bacs_debit: Option<SourceMandateNotificationBacsDebitData>,
    pub reason: String,
    pub sepa_debit: Option<SourceMandateNotificationSepaDebitData>,
    pub source: Source,
    pub status: String,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for SourceMandateNotification {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `source_mandate_notification.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectE9082A {
    #[serde(rename = "source_mandate_notification")]
    SourceMandateNotification,
}

/// Spec paths:
/// - `source_mandate_notification_acss_debit_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceMandateNotificationAcssDebitData {
    pub statement_descriptor: Option<String>,
}

/// Spec paths:
/// - `source_mandate_notification_bacs_debit_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceMandateNotificationBacsDebitData {
    pub last4: Option<String>,
}

/// Spec paths:
/// - `source_mandate_notification_sepa_debit_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceMandateNotificationSepaDebitData {
    pub creditor_identifier: Option<String>,
    pub last4: Option<String>,
    pub mandate_reference: Option<String>,
}

/// Spec paths:
/// - `source_order`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceOrder {
    pub amount: i64,
    pub currency: String,
    pub email: Option<String>,
    pub items: Option<Vec<SourceOrderItem>>,
    pub shipping: Option<Shipping>,
}

/// Spec paths:
/// - `source_order_item`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceOrderItem {
    #[serde(rename = "type")]
    pub type_x: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub parent: Option<String>,
    pub quantity: Option<i64>,
}

/// Spec paths:
/// - `source_owner`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceOwner {
    pub name: Option<String>,
    pub address: Option<Address>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub verified_address: Option<Address>,
    pub verified_email: Option<String>,
    pub verified_name: Option<String>,
    pub verified_phone: Option<String>,
}

/// Spec paths:
/// - `source_receiver_flow`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceReceiverFlow {
    pub address: Option<String>,
    pub amount_charged: i64,
    pub amount_received: i64,
    pub amount_returned: i64,
    pub refund_attributes_method: String,
    pub refund_attributes_status: String,
}

/// Spec paths:
/// - `source_redirect_flow`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceRedirectFlow {
    pub failure_reason: Option<String>,
    pub return_url: String,
    pub status: String,
    pub url: String,
}

/// Spec paths:
/// - `source_transaction`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTransaction {
    pub object: UniStrObject710D46,
    #[serde(rename = "type")]
    pub type_x: UniStrTypeA4C338,
    pub id: String,
    pub ach_credit_transfer: Option<SourceTransactionAchCreditTransferData>,
    pub amount: i64,
    pub chf_credit_transfer: Option<SourceTransactionChfCreditTransferData>,
    pub currency: String,
    pub gbp_credit_transfer: Option<SourceTransactionGbpCreditTransferData>,
    pub paper_check: Option<SourceTransactionPaperCheckData>,
    pub sepa_credit_transfer: Option<SourceTransactionSepaCreditTransferData>,
    pub source: String,
    pub status: String,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for SourceTransaction {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `source_transaction.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject710D46 {
    #[serde(rename = "source_transaction")]
    SourceTransaction,
}

/// Spec paths:
/// - `source_transaction.type`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrTypeA4C338 {
    #[serde(rename = "ach_credit_transfer")]
    AchCreditTransfer,
    #[serde(rename = "ach_debit")]
    AchDebit,
    #[serde(rename = "alipay")]
    Alipay,
    #[serde(rename = "bancontact")]
    Bancontact,
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "card_present")]
    CardPresent,
    #[serde(rename = "eps")]
    Eps,
    #[serde(rename = "giropay")]
    Giropay,
    #[serde(rename = "ideal")]
    Ideal,
    #[serde(rename = "klarna")]
    Klarna,
    #[serde(rename = "multibanco")]
    Multibanco,
    #[serde(rename = "p24")]
    P24,
    #[serde(rename = "sepa_debit")]
    SepaDebit,
    #[serde(rename = "sofort")]
    Sofort,
    #[serde(rename = "three_d_secure")]
    ThreeDSecure,
    #[serde(rename = "wechat")]
    Wechat,
}

/// Spec paths:
/// - `source_transaction_ach_credit_transfer_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTransactionAchCreditTransferData {
    pub customer_data: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub routing_number: Option<String>,
}

/// Spec paths:
/// - `source_transaction_chf_credit_transfer_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTransactionChfCreditTransferData {
    pub reference: Option<String>,
    pub sender_address_country: Option<String>,
    pub sender_address_line1: Option<String>,
    pub sender_iban: Option<String>,
    pub sender_name: Option<String>,
}

/// Spec paths:
/// - `source_transaction_gbp_credit_transfer_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTransactionGbpCreditTransferData {
    pub fingerprint: Option<String>,
    pub funding_method: Option<String>,
    pub last4: Option<String>,
    pub reference: Option<String>,
    pub sender_account_number: Option<String>,
    pub sender_name: Option<String>,
    pub sender_sort_code: Option<String>,
}

/// Spec paths:
/// - `source_transaction_paper_check_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTransactionPaperCheckData {
    pub available_at: Option<String>,
    pub invoices: Option<String>,
}

/// Spec paths:
/// - `source_transaction_sepa_credit_transfer_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTransactionSepaCreditTransferData {
    pub reference: Option<String>,
    pub sender_iban: Option<String>,
    pub sender_name: Option<String>,
}

/// Spec paths:
/// - `source_type_ach_credit_transfer`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeAchCreditTransfer {
    pub account_number: Option<String>,
    pub bank_name: Option<String>,
    pub fingerprint: Option<String>,
    pub refund_account_holder_name: Option<String>,
    pub refund_account_holder_type: Option<String>,
    pub refund_routing_number: Option<String>,
    pub routing_number: Option<String>,
    pub swift_code: Option<String>,
}

/// Spec paths:
/// - `source_type_ach_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeAchDebit {
    #[serde(rename = "type")]
    pub type_x: Option<String>,
    pub bank_name: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub routing_number: Option<String>,
}

/// Spec paths:
/// - `source_type_alipay`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeAlipay {
    pub data_string: Option<String>,
    pub native_url: Option<String>,
    pub statement_descriptor: Option<String>,
}

/// Spec paths:
/// - `source_type_au_becs_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeAuBecsDebit {
    pub bsb_number: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
}

/// Spec paths:
/// - `source_type_bancontact`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeBancontact {
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub iban_last4: Option<String>,
    pub preferred_language: Option<String>,
    pub statement_descriptor: Option<String>,
}

/// Spec paths:
/// - `source_type_card`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeCard {
    pub name: Option<String>,
    pub address_line1_check: Option<String>,
    pub address_zip_check: Option<String>,
    pub brand: Option<String>,
    pub country: Option<String>,
    pub cvc_check: Option<String>,
    pub dynamic_last4: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub last4: Option<String>,
    pub three_d_secure: Option<String>,
    pub tokenization_method: Option<String>,
}

/// Spec paths:
/// - `source_type_card_present`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeCardPresent {
    pub pos_device_id: Option<String>,
    pub application_cryptogram: Option<String>,
    pub application_preferred_name: Option<String>,
    pub authorization_code: Option<String>,
    pub authorization_response_code: Option<String>,
    pub brand: Option<String>,
    pub country: Option<String>,
    pub cvm_type: Option<String>,
    pub data_type: Option<String>,
    pub dedicated_file_name: Option<String>,
    pub emv_auth_data: Option<String>,
    pub evidence_customer_signature: Option<String>,
    pub evidence_transaction_certificate: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub last4: Option<String>,
    pub pos_entry_mode: Option<String>,
    pub read_method: Option<String>,
    pub reader: Option<String>,
    pub terminal_verification_results: Option<String>,
    pub transaction_status_information: Option<String>,
}

/// Spec paths:
/// - `source_type_eps`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeEps {
    pub reference: Option<String>,
    pub statement_descriptor: Option<String>,
}

/// Spec paths:
/// - `source_type_giropay`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeGiropay {
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub statement_descriptor: Option<String>,
}

/// Spec paths:
/// - `source_type_ideal`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeIdeal {
    pub bank: Option<String>,
    pub bic: Option<String>,
    pub iban_last4: Option<String>,
    pub statement_descriptor: Option<String>,
}

/// Spec paths:
/// - `source_type_klarna`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeKlarna {
    pub background_image_url: Option<String>,
    pub client_token: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub locale: Option<String>,
    pub logo_url: Option<String>,
    pub page_title: Option<String>,
    pub pay_later_asset_urls_descriptive: Option<String>,
    pub pay_later_asset_urls_standard: Option<String>,
    pub pay_later_name: Option<String>,
    pub pay_later_redirect_url: Option<String>,
    pub pay_now_asset_urls_descriptive: Option<String>,
    pub pay_now_asset_urls_standard: Option<String>,
    pub pay_now_name: Option<String>,
    pub pay_now_redirect_url: Option<String>,
    pub pay_over_time_asset_urls_descriptive: Option<String>,
    pub pay_over_time_asset_urls_standard: Option<String>,
    pub pay_over_time_name: Option<String>,
    pub pay_over_time_redirect_url: Option<String>,
    pub payment_method_categories: Option<String>,
    pub purchase_country: Option<String>,
    pub purchase_type: Option<String>,
    pub redirect_url: Option<String>,
    pub shipping_delay: Option<i64>,
    pub shipping_first_name: Option<String>,
    pub shipping_last_name: Option<String>,
}

/// Spec paths:
/// - `source_type_multibanco`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeMultibanco {
    pub entity: Option<String>,
    pub reference: Option<String>,
    pub refund_account_holder_address_city: Option<String>,
    pub refund_account_holder_address_country: Option<String>,
    pub refund_account_holder_address_line1: Option<String>,
    pub refund_account_holder_address_line2: Option<String>,
    pub refund_account_holder_address_postal_code: Option<String>,
    pub refund_account_holder_address_state: Option<String>,
    pub refund_account_holder_name: Option<String>,
    pub refund_iban: Option<String>,
}

/// Spec paths:
/// - `source_type_p24`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeP24 {
    pub reference: Option<String>,
}

/// Spec paths:
/// - `source_type_sepa_debit`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeSepaDebit {
    pub bank_code: Option<String>,
    pub branch_code: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub mandate_reference: Option<String>,
    pub mandate_url: Option<String>,
}

/// Spec paths:
/// - `source_type_sofort`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeSofort {
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub country: Option<String>,
    pub iban_last4: Option<String>,
    pub preferred_language: Option<String>,
    pub statement_descriptor: Option<String>,
}

/// Spec paths:
/// - `source_type_three_d_secure`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeThreeDSecure {
    pub name: Option<String>,
    pub address_line1_check: Option<String>,
    pub address_zip_check: Option<String>,
    pub authenticated: Option<bool>,
    pub brand: Option<String>,
    pub card: Option<String>,
    pub country: Option<String>,
    pub customer: Option<String>,
    pub cvc_check: Option<String>,
    pub dynamic_last4: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub last4: Option<String>,
    pub three_d_secure: Option<String>,
    pub tokenization_method: Option<String>,
}

/// Spec paths:
/// - `source_type_wechat`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceTypeWechat {
    pub prepay_id: Option<String>,
    pub qr_code_url: Option<String>,
    pub statement_descriptor: Option<String>,
}

/// Spec paths:
/// - `status_transitions`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusTransitions {
    pub paid: Option<i64>,
    pub canceled: Option<i64>,
    pub fulfiled: Option<i64>,
    pub returned: Option<i64>,
}

/// Spec paths:
/// - `subscription`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscription {
    pub object: UniStrObject59F834,
    pub id: String,
    pub customer: UniCustomerC00F6E,
    pub default_payment_method: Option<UniPaymentMethod>,
    pub default_source: Box<Option<UniDefaultSource>>,
    pub latest_invoice: Option<UniInvoice>,
    pub pending_setup_intent: Option<UniSetupIntent>,
    pub schedule: Option<UniSchedule>,
    pub application_fee_percent: Option<f64>,
    pub billing_cycle_anchor: i64,
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,
    pub cancel_at: Option<i64>,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<i64>,
    pub collection_method: Option<UniStrCollectionMethod>,
    pub current_period_end: i64,
    pub current_period_start: i64,
    pub days_until_due: Option<i64>,
    pub default_tax_rates: Option<Vec<TaxRate>>,
    pub discount: Option<Discount>,
    pub ended_at: Option<i64>,
    pub items: SubscriptionItemList,
    pub next_pending_invoice_item_invoice: Option<i64>,
    pub pause_collection: Option<SubscriptionsResourcePauseCollection>,
    pub pending_invoice_item_interval: Option<SubscriptionPendingInvoiceItemInterval>,
    pub pending_update: Option<SubscriptionsResourcePendingUpdate>,
    pub start_date: i64,
    pub status: UniStrStatusA5F622,
    pub transfer_data: Option<SubscriptionTransferData>,
    pub trial_end: Option<i64>,
    pub trial_start: Option<i64>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for Subscription {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `subscription.items`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionItemList {
    pub object: UniStrObject344B0E,
    pub data: Vec<SubscriptionItem>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for SubscriptionItemList {
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
/// - `subscription.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject59F834 {
    #[serde(rename = "subscription")]
    Subscription,
}

/// Spec paths:
/// - `subscription.schedule`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniSchedule {
    String(String),
    SubscriptionSchedule(SubscriptionSchedule),
}

/// Spec paths:
/// - `subscription.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatusA5F622 {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "canceled")]
    Canceled,
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
/// - `subscription_billing_thresholds`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionBillingThresholds {
    pub amount_gte: Option<i64>,
    pub reset_billing_cycle_anchor: Option<bool>,
}

/// Spec paths:
/// - `subscription_item`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionItem {
    pub object: UniStrObject36C70C,
    pub id: String,
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,
    pub price: Price,
    pub quantity: Option<i64>,
    pub subscription: String,
    pub tax_rates: Option<Vec<TaxRate>>,
    pub created: i64,
    pub metadata: Metadata8076DB,
}

impl GetId for SubscriptionItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `subscription_item_billing_thresholds`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionItemBillingThresholds {
    pub usage_gte: Option<i64>,
}

/// Spec paths:
/// - `subscription_pending_invoice_item_interval`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionPendingInvoiceItemInterval {
    pub interval: UniStrInterval,
    pub interval_count: i64,
}

/// Spec paths:
/// - `subscription_schedule`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionSchedule {
    pub object: UniStrObject2726CE,
    pub id: String,
    pub customer: UniCustomerC00F6E,
    pub subscription: Option<UniSubscription>,
    pub canceled_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub current_phase: Option<SubscriptionScheduleCurrentPhase>,
    pub default_settings: SubscriptionSchedulesResourceDefaultSettings,
    pub end_behavior: UniStrEndBehavior,
    pub phases: Vec<SubscriptionSchedulePhaseConfiguration>,
    pub released_at: Option<i64>,
    pub released_subscription: Option<String>,
    pub status: UniStrStatus826DFA,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for SubscriptionSchedule {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `subscription_schedule.end_behavior`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrEndBehavior {
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "renew")]
    Renew,
}

/// Spec paths:
/// - `subscription_schedule.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject2726CE {
    #[serde(rename = "subscription_schedule")]
    SubscriptionSchedule,
}

/// Spec paths:
/// - `subscription_schedule.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatus826DFA {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "not_started")]
    NotStarted,
    #[serde(rename = "released")]
    Released,
}

/// Spec paths:
/// - `subscription_schedule_add_invoice_item`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionScheduleAddInvoiceItem {
    pub price: UniPrice82FA7B,
    pub quantity: Option<i64>,
    pub tax_rates: Option<Vec<TaxRate>>,
}

/// Spec paths:
/// - `subscription_schedule_add_invoice_item.price`
/// - `subscription_schedule_configuration_item.price`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniPrice82FA7B {
    String(String),
    Price(Price),
    DeletedPrice(DeletedPrice),
}

/// Spec paths:
/// - `subscription_schedule_configuration_item`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionScheduleConfigurationItem {
    pub price: UniPrice82FA7B,
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,
    pub quantity: Option<i64>,
    pub tax_rates: Option<Vec<TaxRate>>,
}

/// Spec paths:
/// - `subscription_schedule_current_phase`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionScheduleCurrentPhase {
    pub end_date: i64,
    pub start_date: i64,
}

/// Spec paths:
/// - `subscription_schedule_phase_configuration`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionSchedulePhaseConfiguration {
    pub coupon: Option<UniCoupon>,
    pub default_payment_method: Option<UniPaymentMethod>,
    pub add_invoice_items: Vec<SubscriptionScheduleAddInvoiceItem>,
    pub application_fee_percent: Option<f64>,
    pub billing_cycle_anchor: Option<UniStrBillingCycleAnchor>,
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,
    pub collection_method: Option<UniStrCollectionMethod>,
    pub default_tax_rates: Option<Vec<TaxRate>>,
    pub end_date: i64,
    pub invoice_settings: Option<InvoiceSettingSubscriptionScheduleSetting>,
    pub items: Vec<SubscriptionScheduleConfigurationItem>,
    pub proration_behavior: UniStrProrationBehavior,
    pub start_date: i64,
    pub transfer_data: Option<SubscriptionTransferData>,
    pub trial_end: Option<i64>,
}

/// Spec paths:
/// - `subscription_schedule_phase_configuration.billing_cycle_anchor`
/// - `subscription_schedules_resource_default_settings.billing_cycle_anchor`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrBillingCycleAnchor {
    #[serde(rename = "automatic")]
    Automatic,
    #[serde(rename = "phase_start")]
    PhaseStart,
}

/// Spec paths:
/// - `subscription_schedule_phase_configuration.coupon`
#[derive(Serialize, Deserialize, Debug, Clone)]
// @todo/low Improve performance by replacing untagged.
#[serde(untagged)]
pub enum UniCoupon {
    String(String),
    Coupon(Coupon),
    DeletedCoupon(DeletedCoupon),
}

/// Spec paths:
/// - `subscription_schedules_resource_default_settings`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionSchedulesResourceDefaultSettings {
    pub default_payment_method: Option<UniPaymentMethod>,
    pub billing_cycle_anchor: UniStrBillingCycleAnchor,
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,
    pub collection_method: Option<UniStrCollectionMethod>,
    pub invoice_settings: Option<InvoiceSettingSubscriptionScheduleSetting>,
    pub transfer_data: Option<SubscriptionTransferData>,
}

/// Spec paths:
/// - `subscription_transfer_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionTransferData {
    pub destination: UniAccount,
    pub amount_percent: Option<f64>,
}

/// Spec paths:
/// - `subscriptions_resource_pause_collection`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionsResourcePauseCollection {
    pub behavior: UniStrBehavior,
    pub resumes_at: Option<i64>,
}

/// Spec paths:
/// - `subscriptions_resource_pause_collection.behavior`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrBehavior {
    #[serde(rename = "keep_as_draft")]
    KeepAsDraft,
    #[serde(rename = "mark_uncollectible")]
    MarkUncollectible,
    #[serde(rename = "void")]
    Void,
}

/// Spec paths:
/// - `subscriptions_resource_pending_update`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionsResourcePendingUpdate {
    pub billing_cycle_anchor: Option<i64>,
    pub expires_at: i64,
    pub subscription_items: Option<Vec<SubscriptionItem>>,
    pub trial_end: Option<i64>,
    pub trial_from_plan: Option<bool>,
}

/// Spec paths:
/// - `tax_deducted_at_source`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxDeductedAtSource {
    pub object: UniStrObject1FDC9D,
    pub id: String,
    pub period_end: i64,
    pub period_start: i64,
    pub tax_deduction_account_number: String,
}

impl GetId for TaxDeductedAtSource {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `tax_deducted_at_source.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject1FDC9D {
    #[serde(rename = "tax_deducted_at_source")]
    TaxDeductedAtSource,
}

/// Spec paths:
/// - `tax_id`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxId {
    pub object: UniStrObject397A62,
    #[serde(rename = "type")]
    pub type_x: UniStrType805F73,
    pub id: String,
    pub customer: Option<UniCustomerEDC00A>,
    pub country: Option<String>,
    pub value: String,
    pub verification: Option<TaxIdVerification>,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for TaxId {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `tax_id_verification`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxIdVerification {
    pub status: UniStrStatusF68D8E,
    pub verified_address: Option<String>,
    pub verified_name: Option<String>,
}

/// Spec paths:
/// - `tax_id_verification.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatusF68D8E {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "unverified")]
    Unverified,
    #[serde(rename = "verified")]
    Verified,
}

/// Spec paths:
/// - `tax_rate`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaxRate {
    pub object: UniStrObjectC1E2CE,
    pub id: String,
    pub active: bool,
    pub description: Option<String>,
    pub display_name: String,
    pub inclusive: bool,
    pub jurisdiction: Option<String>,
    pub percentage: f64,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for TaxRate {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `tax_rate.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectC1E2CE {
    #[serde(rename = "tax_rate")]
    TaxRate,
}

/// Spec paths:
/// - `terminal.connection_token`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerminalConnectionToken {
    pub object: UniStrObject1B6726,
    pub location: Option<String>,
    pub secret: String,
}

/// Spec paths:
/// - `terminal.connection_token.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject1B6726 {
    #[serde(rename = "terminal.connection_token")]
    TerminalDotConnectionToken,
}

/// Spec paths:
/// - `terminal.location`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerminalLocationLocation {
    pub object: UniStrObject95542E,
    pub id: String,
    pub address: Address,
    pub display_name: String,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for TerminalLocationLocation {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `terminal.reader`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerminalReaderReader {
    pub object: UniStrObjectEAD5C5,
    pub id: String,
    pub device_sw_version: Option<String>,
    pub device_type: UniStrDeviceType,
    pub ip_address: Option<String>,
    pub label: String,
    pub location: Option<String>,
    pub serial_number: String,
    pub status: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for TerminalReaderReader {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `three_d_secure`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreeDSecure {
    pub object: UniStrObject9E785A,
    pub id: String,
    pub amount: i64,
    pub authenticated: bool,
    pub card: Box<Card>,
    pub currency: String,
    pub redirect_url: Option<String>,
    pub status: String,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for ThreeDSecure {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `three_d_secure.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject9E785A {
    #[serde(rename = "three_d_secure")]
    ThreeDSecure,
}

/// Spec paths:
/// - `three_d_secure_details`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreeDSecureDetails {
    pub authentication_flow: Option<UniStrAuthenticationFlow>,
    pub result: UniStrResult,
    pub result_reason: Option<UniStrResultReason>,
    pub version: UniStrVersion,
}

/// Spec paths:
/// - `three_d_secure_details.authentication_flow`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrAuthenticationFlow {
    #[serde(rename = "challenge")]
    Challenge,
    #[serde(rename = "frictionless")]
    Frictionless,
}

/// Spec paths:
/// - `three_d_secure_details.result`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrResult {
    #[serde(rename = "attempt_acknowledged")]
    AttemptAcknowledged,
    #[serde(rename = "authenticated")]
    Authenticated,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "not_supported")]
    NotSupported,
    #[serde(rename = "processing_error")]
    ProcessingError,
}

/// Spec paths:
/// - `three_d_secure_details.result_reason`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrResultReason {
    #[serde(rename = "abandoned")]
    Abandoned,
    #[serde(rename = "bypassed")]
    Bypassed,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "card_not_enrolled")]
    CardNotEnrolled,
    #[serde(rename = "network_not_supported")]
    NetworkNotSupported,
    #[serde(rename = "protocol_error")]
    ProtocolError,
    #[serde(rename = "rejected")]
    Rejected,
}

/// Spec paths:
/// - `three_d_secure_details.version`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrVersion {
    #[serde(rename = "1.0.2")]
    X1Dot0Dot2,
    #[serde(rename = "2.1.0")]
    X2Dot1Dot0,
    #[serde(rename = "2.2.0")]
    X2Dot2Dot0,
}

/// Spec paths:
/// - `three_d_secure_usage`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreeDSecureUsage {
    pub supported: bool,
}

/// Spec paths:
/// - `token`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub object: UniStrObjectE216BE,
    #[serde(rename = "type")]
    pub type_x: String,
    pub id: String,
    pub bank_account: Option<BankAccount>,
    pub card: Box<Option<Card>>,
    pub client_ip: Option<String>,
    pub used: bool,
    pub created: i64,
    pub livemode: bool,
}

impl GetId for Token {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `token.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectE216BE {
    #[serde(rename = "token")]
    Token,
}

/// Spec paths:
/// - `topup`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Topup {
    pub object: UniStrObjectBACBB3,
    pub id: String,
    pub balance_transaction: Option<UniBalanceTransaction>,
    pub amount: i64,
    pub currency: String,
    pub description: Option<String>,
    pub expected_availability_date: Option<i64>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub source: Source,
    pub statement_descriptor: Option<String>,
    pub status: UniStrStatus433A86,
    pub transfer_group: Option<String>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for Topup {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `topup.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectBACBB3 {
    #[serde(rename = "topup")]
    Topup,
}

/// Spec paths:
/// - `topup.status`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrStatus433A86 {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "reversed")]
    Reversed,
    #[serde(rename = "succeeded")]
    Succeeded,
}

/// Spec paths:
/// - `transfer`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transfer {
    pub object: UniStrObjectE37CA2,
    pub id: String,
    pub balance_transaction: Option<UniBalanceTransaction>,
    pub destination: Option<UniAccount>,
    pub destination_payment: Option<UniCharge>,
    pub source_transaction: Option<UniCharge>,
    pub amount: i64,
    pub amount_reversed: i64,
    pub currency: String,
    pub description: Option<String>,
    pub reversals: TransferReversalList675B54,
    pub reversed: bool,
    pub source_type: Option<String>,
    pub transfer_group: Option<String>,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for Transfer {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `transfer.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectE37CA2 {
    #[serde(rename = "transfer")]
    Transfer,
}

/// Spec paths:
/// - `transfer.reversals`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferReversalList675B54 {
    pub object: UniStrObject344B0E,
    pub data: Vec<TransferReversal>,
    pub has_more: bool,
    pub url: String,
}

impl PageMeta for TransferReversalList675B54 {
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
/// - `transfer_data`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferData {
    pub destination: UniAccount,
    pub amount: Option<i64>,
}

/// Spec paths:
/// - `transfer_reversal`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferReversal {
    pub object: UniStrObject1EB608,
    pub id: String,
    pub balance_transaction: Option<UniBalanceTransaction>,
    pub destination_payment_refund: Option<UniRefund>,
    pub source_refund: Option<UniRefund>,
    pub transfer: UniTransfer,
    pub amount: i64,
    pub currency: String,
    pub created: i64,
    pub metadata: Option<Metadata7CCA3C>,
}

impl GetId for TransferReversal {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `transfer_reversal.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject1EB608 {
    #[serde(rename = "transfer_reversal")]
    TransferReversal,
}

/// Spec paths:
/// - `transfer_schedule`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferSchedule {
    pub delay_days: i64,
    pub interval: String,
    pub monthly_anchor: Option<i64>,
    pub weekly_anchor: Option<String>,
}

/// Spec paths:
/// - `transform_quantity`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransformQuantity {
    pub divide_by: i64,
    pub round: UniStrRound,
}

/// Spec paths:
/// - `transform_quantity.round`
/// - `transform_usage.round`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrRound {
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "up")]
    Up,
}

/// Spec paths:
/// - `transform_usage`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransformUsage {
    pub divide_by: i64,
    pub round: UniStrRound,
}

/// Spec paths:
/// - `usage_record`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageRecord {
    pub object: UniStrObject1E9E51,
    pub id: String,
    pub quantity: i64,
    pub subscription_item: String,
    pub timestamp: i64,
    pub livemode: bool,
}

impl GetId for UsageRecord {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `usage_record.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObject1E9E51 {
    #[serde(rename = "usage_record")]
    UsageRecord,
}

/// Spec paths:
/// - `usage_record_summary`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageRecordSummary {
    pub object: UniStrObjectB59933,
    pub id: String,
    pub invoice: Option<String>,
    pub period: Period640F9C,
    pub subscription_item: String,
    pub total_usage: i64,
    pub livemode: bool,
}

impl GetId for UsageRecordSummary {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Spec paths:
/// - `usage_record_summary.object`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UniStrObjectB59933 {
    #[serde(rename = "usage_record_summary")]
    UsageRecordSummary,
}

/// Spec paths:
/// - `webhook_endpoint`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationWebhookEndpoint {
    pub object: UniStrObjectBDDC67,
    pub id: String,
    pub api_version: Option<String>,
    pub application: Option<String>,
    pub description: Option<String>,
    pub enabled_events: Vec<String>,
    pub secret: Option<String>,
    pub status: String,
    pub url: String,
    pub created: i64,
    pub livemode: bool,
    pub metadata: Metadata8076DB,
}

impl GetId for NotificationWebhookEndpoint {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}
