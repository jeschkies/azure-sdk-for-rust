use crate::PartitionKeys;
use http::request::Builder;

pub(crate) mod from_headers;

pub(crate) const HEADER_VERSION: &str = "x-ms-version"; // Cow[str]
pub(crate) const HEADER_DATE: &str = "x-ms-date"; // [String]
pub(crate) const HEADER_DOCUMENTDB_IS_UPSERT: &str = "x-ms-documentdb-is-upsert"; // [bool]
pub(crate) const HEADER_INDEXING_DIRECTIVE: &str = "x-ms-indexing-directive"; // [IndexingDirective]
pub(crate) const HEADER_MAX_ITEM_COUNT: &str = "x-ms-max-item-count"; // [u64]
pub(crate) const HEADER_ITEM_COUNT: &str = "x-ms-item-count"; // [u64]
pub(crate) const HEADER_CONSISTENCY_LEVEL: &str = "x-ms-consistency-level"; // [ConsistencyLevel]
pub(crate) const HEADER_SESSION_TOKEN: &str = "x-ms-session-token"; // [ContinuationToken]
pub(crate) const HEADER_ALLOW_MULTIPLE_WRITES: &str = "x-ms-cosmos-allow-tentative-writes"; // [bool]
pub(crate) const HEADER_A_IM: &str = "A-IM"; // Cow[str]
pub(crate) const HEADER_ACTIVITY_ID: &str = "x-ms-activity-id"; // [String]
pub(crate) const HEADER_DOCUMENTDB_PARTITIONRANGEID: &str = "x-ms-documentdb-partitionkeyrangeid"; // [String]
pub(crate) const HEADER_DOCUMENTDB_PARTITIONKEY: &str = "x-ms-documentdb-partitionkey"; // [String]
pub(crate) const HEADER_NUMBER_OF_READ_REGIONS: &str = "x-ms-number-of-read-regions";
pub(crate) const HEADER_REQUEST_CHARGE: &str = "x-ms-request-charge"; // [f64]
pub(crate) const HEADER_OFFER_THROUGHPUT: &str = "x-ms-offer-throughput"; // [u64]
pub(crate) const HEADER_OFFER_TYPE: &str = "x-ms-offer-type"; // [&str]
#[allow(dead_code)]
pub(crate) const HEADER_DOCUMENTDB_ISQUERY: &str = "x-ms-documentdb-isquery"; // [bool]
pub(crate) const HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION: &str =
    "x-ms-documentdb-query-enablecrosspartition"; // [bool]
pub(crate) const HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY: &str =
    "x-ms-documentdb-query-parallelizecrosspartitionquery"; // [bool]
pub(crate) const HEADER_DOCUMENTDB_EXPIRY_SECONDS: &str = "x-ms-documentdb-expiry-seconds"; // [u64]
pub(crate) const HEADER_CONTENT_PATH: &str = "x-ms-content-path"; // [String]
pub(crate) const HEADER_ALT_CONTENT_PATH: &str = "x-ms-alt-content-path"; // [String]
pub(crate) const HEADER_LAST_STATE_CHANGE_UTC: &str = "x-ms-last-state-change-utc"; // [DateTime<UTC>]
pub(crate) const HEADER_RESOURCE_QUOTA: &str = "x-ms-resource-quota"; // [ResourceQuota]
pub(crate) const HEADER_RESOURCE_USAGE: &str = "x-ms-resource-usage"; // [ResourceQuota]
pub(crate) const HEADER_QUORUM_ACKED_LSN: &str = "x-ms-quorum-acked-lsn"; // [u64]
pub(crate) const HEADER_CURRENT_WRITE_QUORUM: &str = "x-ms-current-write-quorum"; // [u64]
pub(crate) const HEADER_CURRENT_REPLICA_SET_SIZE: &str = "x-ms-current-replica-set-size"; // [u64]
pub(crate) const HEADER_SCHEMA_VERSION: &str = "x-ms-schemaversion"; // [String]
pub(crate) const HEADER_SERVICE_VERSION: &str = "x-ms-serviceversion"; // [String]
pub(crate) const HEADER_GATEWAY_VERSION: &str = "x-ms-gatewayversion"; // [String]
pub(crate) const HEADER_COLLECTION_PARTITION_INDEX: &str = "collection-partition-index"; // [u64]
pub(crate) const HEADER_COLLECTION_SERVICE_INDEX: &str = "collection-service-index"; // [u64]
pub(crate) const HEADER_LSN: &str = "lsn"; // [u64]
pub(crate) const HEADER_GLOBAL_COMMITTED_LSN: &str = "x-ms-global-committed-lsn"; // [u64]
pub(crate) const HEADER_ITEM_LSN: &str = "x-ms-item-lsn"; // [u64]
pub(crate) const HEADER_TRANSPORT_REQUEST_ID: &str = "x-ms-transport-request-id"; // [u64]
pub(crate) const HEADER_COSMOS_LLSN: &str = "x-ms-cosmos-llsn"; // [u64]
pub(crate) const HEADER_COSMOS_ITEM_LLSN: &str = "x-ms-cosmos-item-llsn"; // [u64]
pub(crate) const HEADER_COSMOS_QUORUM_ACKED_LLSN: &str = "x-ms-cosmos-quorum-acked-llsn"; // [u64]
pub(crate) const HEADER_ROLE: &str = "x-ms-xp-role"; // [u64]
pub(crate) const HEADER_MAX_MEDIA_STORAGE_USAGE_MB: &str = "x-ms-max-media-storage-usage-mb"; // [u64]
pub(crate) const HEADER_MEDIA_STORAGE_USAGE_MB: &str = "x-ms-media-storage-usage-mb"; // [u64]

pub(crate) fn add_partition_keys_header(
    partition_keys: &PartitionKeys,
    builder: Builder,
) -> Builder {
    let serialized = partition_keys.to_json();
    builder.header(HEADER_DOCUMENTDB_PARTITIONKEY, serialized)
}
