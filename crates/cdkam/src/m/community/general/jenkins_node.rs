#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible_core::core::OptU;
use cdk_ansible_core::core::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.jenkins_node")]
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
        rename = "labels"
    )]
    pub labels: OptU<::cdk_ansible_core::core::StringOrVec>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "name"
    )]
    pub name: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "num_executors"
    )]
    pub num_executors: OptU<::cdk_ansible_core::core::IntOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "offline_message"
    )]
    pub offline_message: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "state"
    )]
    pub state: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "token"
    )]
    pub token: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "url"
    )]
    pub url: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "user"
    )]
    pub user: OptU<String>,
}
