#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible_core::core::OptU;
use cdk_ansible_core::core::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.ipa_vault")]
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
        rename = "cn"
    )]
    pub cn: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "description"
    )]
    pub description: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ipa_host"
    )]
    pub ipa_host: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ipa_pass"
    )]
    pub ipa_pass: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ipa_port"
    )]
    pub ipa_port: OptU<::cdk_ansible_core::core::IntOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ipa_prot"
    )]
    pub ipa_prot: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ipa_timeout"
    )]
    pub ipa_timeout: OptU<::cdk_ansible_core::core::IntOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ipa_user"
    )]
    pub ipa_user: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ipavaultpublickey"
    )]
    pub ipavaultpublickey: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ipavaultsalt"
    )]
    pub ipavaultsalt: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "ipavaulttype"
    )]
    pub ipavaulttype: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "replace"
    )]
    pub replace: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "service"
    )]
    pub service: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "state"
    )]
    pub state: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "username"
    )]
    pub username: OptU<::cdk_ansible_core::core::StringOrVec>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "validate_certs"
    )]
    pub validate_certs: OptU<::cdk_ansible_core::core::BoolOrString>,
}
