//! Unreal-style Blueprint GUI API contract used by the editor.

/// Generic contract for a Blueprint GUI API profile.
pub trait BlueprintGuiApi {
    fn api_name(&self) -> &'static str;
    fn preferred_panel(&self) -> &'static str;
    fn graph_families(&self) -> &'static [&'static str];
    fn core_tabs(&self) -> &'static [&'static str];
    fn docs_url(&self) -> &'static str;
}

/// Launch metadata produced when the Blueprint interface is requested.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlueprintLaunchInfo {
    pub api_name: &'static str,
    pub preferred_panel: &'static str,
    pub status_line: String,
    pub docs_url: &'static str,
}

/// Unreal Engine 5 inspired Blueprint GUI profile.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Unreal5BlueprintGuiApi;

const UE5_GRAPH_FAMILIES: &[&str] = &[
    "Actor Blueprint",
    "Function Graph",
    "Macro Graph",
    "Animation Blueprint",
    "Material-style Data Graph",
];

const UE5_CORE_TABS: &[&str] = &[
    "Components",
    "My Blueprint",
    "Graph",
    "Details",
    "Compiler Results",
];

impl BlueprintGuiApi for Unreal5BlueprintGuiApi {
    fn api_name(&self) -> &'static str {
        "Unreal5BlueprintGuiApi"
    }

    fn preferred_panel(&self) -> &'static str {
        "Blueprint"
    }

    fn graph_families(&self) -> &'static [&'static str] {
        UE5_GRAPH_FAMILIES
    }

    fn core_tabs(&self) -> &'static [&'static str] {
        UE5_CORE_TABS
    }

    fn docs_url(&self) -> &'static str {
        "https://www.eldiron.com/docs/creator/"
    }
}

impl Unreal5BlueprintGuiApi {
    pub fn launch_info(&self) -> BlueprintLaunchInfo {
        BlueprintLaunchInfo {
            api_name: self.api_name(),
            preferred_panel: self.preferred_panel(),
            status_line: format!(
                "{} active: tabs [{}], graphs [{}].",
                self.api_name(),
                self.core_tabs().join(", "),
                self.graph_families().join(", "),
            ),
            docs_url: self.docs_url(),
        }
    }
}

/// Launches the Blueprint editor UI profile and returns launch metadata.
pub fn launch_blueprint_editor() -> BlueprintLaunchInfo {
    Unreal5BlueprintGuiApi.launch_info()
}
