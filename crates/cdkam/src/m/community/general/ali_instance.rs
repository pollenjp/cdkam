#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible_core::core::OptU;
use cdk_ansible_core::core::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.ali_instance")]
    pub module: Args,
}
impl TaskModule for Module {}
#[derive(Clone, Debug, Serialize)]
pub struct Args {
    #[serde(flatten)]
    pub options: Opt,
}
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Opt {
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alicloud_access_key"
    )]
    pub alicloud_access_key: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alicloud_assume_role"
    )]
    pub alicloud_assume_role: OptU<::cdk_ansible_core::core::StringOrMap>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alicloud_assume_role_arn"
    )]
    pub alicloud_assume_role_arn: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alicloud_assume_role_session_expiration"
    )]
    pub alicloud_assume_role_session_expiration: OptU<::cdk_ansible_core::core::IntOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alicloud_assume_role_session_name"
    )]
    pub alicloud_assume_role_session_name: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alicloud_region"
    )]
    pub alicloud_region: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alicloud_secret_key"
    )]
    pub alicloud_secret_key: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "alicloud_security_token"
    )]
    pub alicloud_security_token: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "allocate_public_ip"
    )]
    pub allocate_public_ip: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "auto_renew"
    )]
    pub auto_renew: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "auto_renew_period"
    )]
    pub auto_renew_period: OptU<::cdk_ansible_core::core::IntOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "availability_zone"
    )]
    pub availability_zone: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "count"
    )]
    pub count: OptU<::cdk_ansible_core::core::IntOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "count_tag"
    )]
    pub count_tag: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "description"
    )]
    pub description: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "dry_run"
    )]
    pub dry_run: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ecs_role_name"
    )]
    pub ecs_role_name: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "force"
    )]
    pub force: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "host_name"
    )]
    pub host_name: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "image_id"
    )]
    pub image_id: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "include_data_disks"
    )]
    pub include_data_disks: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "instance_charge_type"
    )]
    pub instance_charge_type: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "instance_ids"
    )]
    pub instance_ids: OptU<::cdk_ansible_core::core::StringOrVec>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "instance_name"
    )]
    pub instance_name: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "instance_type"
    )]
    pub instance_type: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "internet_charge_type"
    )]
    pub internet_charge_type: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "key_name"
    )]
    pub key_name: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "max_bandwidth_in"
    )]
    pub max_bandwidth_in: OptU<::cdk_ansible_core::core::IntOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "max_bandwidth_out"
    )]
    pub max_bandwidth_out: OptU<::cdk_ansible_core::core::IntOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "password"
    )]
    pub password: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "period"
    )]
    pub period: OptU<::cdk_ansible_core::core::IntOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "period_unit"
    )]
    pub period_unit: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "profile"
    )]
    pub profile: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "purge_tags"
    )]
    pub purge_tags: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ram_role_name"
    )]
    pub ram_role_name: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "security_groups"
    )]
    pub security_groups: OptU<::cdk_ansible_core::core::StringOrVec>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "shared_credentials_file"
    )]
    pub shared_credentials_file: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "spot_price_limit"
    )]
    pub spot_price_limit: OptU<::serde_json::Value>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "spot_strategy"
    )]
    pub spot_strategy: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "state"
    )]
    pub state: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "system_disk_category"
    )]
    pub system_disk_category: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "system_disk_description"
    )]
    pub system_disk_description: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "system_disk_name"
    )]
    pub system_disk_name: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "system_disk_size"
    )]
    pub system_disk_size: OptU<::cdk_ansible_core::core::IntOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "tags"
    )]
    pub tags: OptU<::cdk_ansible_core::core::StringOrMap>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "unique_suffix"
    )]
    pub unique_suffix: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "user_data"
    )]
    pub user_data: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "vswitch_id"
    )]
    pub vswitch_id: OptU<String>,
}
