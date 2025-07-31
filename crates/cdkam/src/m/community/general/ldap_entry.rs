#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible_core::core::OptU;
use cdk_ansible_core::core::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.ldap_entry")]
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
        rename = "attributes"
    )]
    pub attributes: OptU<::cdk_ansible_core::core::StringOrMap>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "bind_dn"
    )]
    pub bind_dn: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "bind_pw"
    )]
    pub bind_pw: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ca_path"
    )]
    pub ca_path: OptU<::cdk_ansible_core::core::StringOrPath>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "client_cert"
    )]
    pub client_cert: OptU<::cdk_ansible_core::core::StringOrPath>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "client_key"
    )]
    pub client_key: OptU<::cdk_ansible_core::core::StringOrPath>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "dn"
    )]
    pub dn: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "objectClass"
    )]
    pub object_class: OptU<::cdk_ansible_core::core::StringOrVec>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "recursive"
    )]
    pub recursive: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "referrals_chasing"
    )]
    pub referrals_chasing: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "sasl_class"
    )]
    pub sasl_class: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "server_uri"
    )]
    pub server_uri: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "start_tls"
    )]
    pub start_tls: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "state"
    )]
    pub state: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "validate_certs"
    )]
    pub validate_certs: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "xorder_discovery"
    )]
    pub xorder_discovery: OptU<String>,
}
