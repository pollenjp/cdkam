#[allow(unused_imports, reason = "Some modules may have empty `options` field")]
use cdk_ansible_core::core::OptU;
use cdk_ansible_core::core::TaskModule;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Module {
    #[serde(rename = "community.general.django_createcachetable")]
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
        rename = "database"
    )]
    pub database: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "pythonpath"
    )]
    pub pythonpath: OptU<::cdk_ansible_core::core::StringOrPath>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "settings"
    )]
    pub settings: OptU<String>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "skip_checks"
    )]
    pub skip_checks: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "traceback"
    )]
    pub traceback: OptU<::cdk_ansible_core::core::BoolOrString>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "venv"
    )]
    pub venv: OptU<::cdk_ansible_core::core::StringOrPath>,
    #[serde(
        default = "OptU::default",
        skip_serializing_if = "OptU::is_unset",
        rename = "verbosity"
    )]
    pub verbosity: OptU<::cdk_ansible_core::core::IntOrString>,
}
