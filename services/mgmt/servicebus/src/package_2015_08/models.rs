#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceCreateOrUpdateParameters {
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NamespaceResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<namespace_properties::Status>,
    #[serde(rename = "createdAt", skip_serializing)]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing)]
    pub updated_at: Option<String>,
    #[serde(rename = "serviceBusEndpoint", skip_serializing)]
    pub service_bus_endpoint: Option<String>,
    #[serde(rename = "createACSNamespace", skip_serializing_if = "Option::is_none")]
    pub create_acs_namespace: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
pub mod namespace_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Unknown,
        Creating,
        Created,
        Activating,
        Enabling,
        Active,
        Disabling,
        Disabled,
        SoftDeleting,
        SoftDeleted,
        Removing,
        Removed,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
    pub tier: sku::Tier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Basic,
        Standard,
        Premium,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        Standard,
        Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleCreateOrUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedAccessAuthorizationRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SharedAccessAuthorizationRuleResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedAccessAuthorizationRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleProperties {
    pub rights: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceListKeys {
    #[serde(rename = "primaryConnectionString", skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[serde(rename = "secondaryConnectionString", skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
    #[serde(rename = "primaryKey", skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[serde(rename = "keyName", skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateKeysParameters {
    #[serde(rename = "Policykey", skip_serializing_if = "Option::is_none")]
    pub policykey: Option<regenerate_keys_parameters::Policykey>,
}
pub mod regenerate_keys_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Policykey {
        PrimaryKey,
        SecondaryKey,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueCreateOrUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueueProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QueueResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueueProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueProperties {
    #[serde(rename = "lockDuration", skip_serializing_if = "Option::is_none")]
    pub lock_duration: Option<String>,
    #[serde(rename = "accessedAt", skip_serializing)]
    pub accessed_at: Option<String>,
    #[serde(rename = "autoDeleteOnIdle", skip_serializing_if = "Option::is_none")]
    pub auto_delete_on_idle: Option<String>,
    #[serde(rename = "entityAvailabilityStatus", skip_serializing_if = "Option::is_none")]
    pub entity_availability_status: Option<EntityAvailabilityStatus>,
    #[serde(rename = "createdAt", skip_serializing)]
    pub created_at: Option<String>,
    #[serde(rename = "defaultMessageTimeToLive", skip_serializing_if = "Option::is_none")]
    pub default_message_time_to_live: Option<String>,
    #[serde(rename = "duplicateDetectionHistoryTimeWindow", skip_serializing_if = "Option::is_none")]
    pub duplicate_detection_history_time_window: Option<String>,
    #[serde(rename = "enableBatchedOperations", skip_serializing_if = "Option::is_none")]
    pub enable_batched_operations: Option<bool>,
    #[serde(rename = "deadLetteringOnMessageExpiration", skip_serializing_if = "Option::is_none")]
    pub dead_lettering_on_message_expiration: Option<bool>,
    #[serde(rename = "enableExpress", skip_serializing_if = "Option::is_none")]
    pub enable_express: Option<bool>,
    #[serde(rename = "enablePartitioning", skip_serializing_if = "Option::is_none")]
    pub enable_partitioning: Option<bool>,
    #[serde(rename = "isAnonymousAccessible", skip_serializing_if = "Option::is_none")]
    pub is_anonymous_accessible: Option<bool>,
    #[serde(rename = "maxDeliveryCount", skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
    #[serde(rename = "maxSizeInMegabytes", skip_serializing_if = "Option::is_none")]
    pub max_size_in_megabytes: Option<i64>,
    #[serde(rename = "messageCount", skip_serializing)]
    pub message_count: Option<i64>,
    #[serde(rename = "countDetails", skip_serializing_if = "Option::is_none")]
    pub count_details: Option<MessageCountDetails>,
    #[serde(rename = "requiresDuplicateDetection", skip_serializing_if = "Option::is_none")]
    pub requires_duplicate_detection: Option<bool>,
    #[serde(rename = "requiresSession", skip_serializing_if = "Option::is_none")]
    pub requires_session: Option<bool>,
    #[serde(rename = "sizeInBytes", skip_serializing)]
    pub size_in_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EntityStatus>,
    #[serde(rename = "supportOrdering", skip_serializing_if = "Option::is_none")]
    pub support_ordering: Option<bool>,
    #[serde(rename = "updatedAt", skip_serializing)]
    pub updated_at: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageCountDetails {
    #[serde(rename = "activeMessageCount", skip_serializing)]
    pub active_message_count: Option<i64>,
    #[serde(rename = "deadLetterMessageCount", skip_serializing)]
    pub dead_letter_message_count: Option<i64>,
    #[serde(rename = "scheduledMessageCount", skip_serializing)]
    pub scheduled_message_count: Option<i64>,
    #[serde(rename = "transferDeadLetterMessageCount", skip_serializing)]
    pub transfer_dead_letter_message_count: Option<i64>,
    #[serde(rename = "transferMessageCount", skip_serializing)]
    pub transfer_message_count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicCreateOrUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TopicProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TopicResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<TopicProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicProperties {
    #[serde(rename = "accessedAt", skip_serializing)]
    pub accessed_at: Option<String>,
    #[serde(rename = "autoDeleteOnIdle", skip_serializing_if = "Option::is_none")]
    pub auto_delete_on_idle: Option<String>,
    #[serde(rename = "entityAvailabilityStatus", skip_serializing_if = "Option::is_none")]
    pub entity_availability_status: Option<EntityAvailabilityStatus>,
    #[serde(rename = "createdAt", skip_serializing)]
    pub created_at: Option<String>,
    #[serde(rename = "countDetails", skip_serializing_if = "Option::is_none")]
    pub count_details: Option<MessageCountDetails>,
    #[serde(rename = "defaultMessageTimeToLive", skip_serializing_if = "Option::is_none")]
    pub default_message_time_to_live: Option<String>,
    #[serde(rename = "duplicateDetectionHistoryTimeWindow", skip_serializing_if = "Option::is_none")]
    pub duplicate_detection_history_time_window: Option<String>,
    #[serde(rename = "enableBatchedOperations", skip_serializing_if = "Option::is_none")]
    pub enable_batched_operations: Option<bool>,
    #[serde(rename = "enableExpress", skip_serializing_if = "Option::is_none")]
    pub enable_express: Option<bool>,
    #[serde(rename = "enablePartitioning", skip_serializing_if = "Option::is_none")]
    pub enable_partitioning: Option<bool>,
    #[serde(rename = "filteringMessagesBeforePublishing", skip_serializing_if = "Option::is_none")]
    pub filtering_messages_before_publishing: Option<bool>,
    #[serde(rename = "isAnonymousAccessible", skip_serializing_if = "Option::is_none")]
    pub is_anonymous_accessible: Option<bool>,
    #[serde(rename = "isExpress", skip_serializing_if = "Option::is_none")]
    pub is_express: Option<bool>,
    #[serde(rename = "maxSizeInMegabytes", skip_serializing_if = "Option::is_none")]
    pub max_size_in_megabytes: Option<i64>,
    #[serde(rename = "requiresDuplicateDetection", skip_serializing_if = "Option::is_none")]
    pub requires_duplicate_detection: Option<bool>,
    #[serde(rename = "sizeInBytes", skip_serializing)]
    pub size_in_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EntityStatus>,
    #[serde(rename = "subscriptionCount", skip_serializing)]
    pub subscription_count: Option<i32>,
    #[serde(rename = "supportOrdering", skip_serializing_if = "Option::is_none")]
    pub support_ordering: Option<bool>,
    #[serde(rename = "updatedAt", skip_serializing)]
    pub updated_at: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionCreateOrUpdateParameters {
    pub location: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubscriptionResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionProperties {
    #[serde(rename = "accessedAt", skip_serializing)]
    pub accessed_at: Option<String>,
    #[serde(rename = "autoDeleteOnIdle", skip_serializing_if = "Option::is_none")]
    pub auto_delete_on_idle: Option<String>,
    #[serde(rename = "countDetails", skip_serializing_if = "Option::is_none")]
    pub count_details: Option<MessageCountDetails>,
    #[serde(rename = "createdAt", skip_serializing)]
    pub created_at: Option<String>,
    #[serde(rename = "defaultMessageTimeToLive", skip_serializing_if = "Option::is_none")]
    pub default_message_time_to_live: Option<String>,
    #[serde(rename = "deadLetteringOnFilterEvaluationExceptions", skip_serializing_if = "Option::is_none")]
    pub dead_lettering_on_filter_evaluation_exceptions: Option<bool>,
    #[serde(rename = "deadLetteringOnMessageExpiration", skip_serializing_if = "Option::is_none")]
    pub dead_lettering_on_message_expiration: Option<bool>,
    #[serde(rename = "enableBatchedOperations", skip_serializing_if = "Option::is_none")]
    pub enable_batched_operations: Option<bool>,
    #[serde(rename = "entityAvailabilityStatus", skip_serializing_if = "Option::is_none")]
    pub entity_availability_status: Option<EntityAvailabilityStatus>,
    #[serde(rename = "isReadOnly", skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    #[serde(rename = "lockDuration", skip_serializing_if = "Option::is_none")]
    pub lock_duration: Option<String>,
    #[serde(rename = "maxDeliveryCount", skip_serializing_if = "Option::is_none")]
    pub max_delivery_count: Option<i32>,
    #[serde(rename = "messageCount", skip_serializing)]
    pub message_count: Option<i64>,
    #[serde(rename = "requiresSession", skip_serializing_if = "Option::is_none")]
    pub requires_session: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EntityStatus>,
    #[serde(rename = "updatedAt", skip_serializing)]
    pub updated_at: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EntityAvailabilityStatus {
    Available,
    Limited,
    Renaming,
    Restoring,
    Unknown,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EntityStatus {
    Active,
    Creating,
    Deleting,
    Disabled,
    ReceiveDisabled,
    Renaming,
    Restoring,
    SendDisabled,
    Unknown,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UnavailableReason {
    None,
    InvalidName,
    SubscriptionIsDisabled,
    NameInUse,
    NameInLockdown,
    TooManyNamespaceInCurrentSubscription,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailability {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<UnavailableReason>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing)]
        pub provider: Option<String>,
        #[serde(skip_serializing)]
        pub resource: Option<String>,
        #[serde(skip_serializing)]
        pub operation: Option<String>,
    }
}
