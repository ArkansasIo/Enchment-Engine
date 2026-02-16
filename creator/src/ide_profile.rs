#[derive(Clone, Debug)]
pub struct IdeFeature {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Debug)]
pub struct IdeFeatureCategory {
    pub name: &'static str,
    pub features: Vec<IdeFeature>,
}

#[derive(Clone, Debug)]
pub struct IdeProfile {
    pub name: &'static str,
    pub categories: Vec<IdeFeatureCategory>,
}

impl IdeProfile {
    pub fn unreal_like() -> Self {
        Self {
            name: "Unreal-like Game IDE",
            categories: vec![
                IdeFeatureCategory {
                    name: "Workspace",
                    features: vec![
                        IdeFeature {
                            name: "Docked Multi-Panel Layout",
                            description: "Viewport center, utility panels, and persistent sidebars.",
                        },
                        IdeFeature {
                            name: "Top/Left/Right Toolbars",
                            description: "Quick action buttons and mode selection with stateful highlights.",
                        },
                        IdeFeature {
                            name: "Theme Presets",
                            description: "Dark, light, and slate-style editor appearance modes.",
                        },
                    ],
                },
                IdeFeatureCategory {
                    name: "Authoring",
                    features: vec![
                        IdeFeature {
                            name: "2D/3D Editing Tools",
                            description: "Selection, geometry, terrain, render, and entity toolchains.",
                        },
                        IdeFeature {
                            name: "Code + Visual Scripting",
                            description: "Node and code modules for gameplay and content logic.",
                        },
                        IdeFeature {
                            name: "Data/Config/Info Panels",
                            description: "Structured game data, runtime settings, and diagnostics.",
                        },
                    ],
                },
                IdeFeatureCategory {
                    name: "Systems",
                    features: vec![
                        IdeFeature {
                            name: "Town Generator Pipeline",
                            description: "Preset-driven town generation, overlays, bake, import, export.",
                        },
                        IdeFeature {
                            name: "RPG/MMORPG Runtime",
                            description: "Progression, loot, events, and simulation tick operations.",
                        },
                        IdeFeature {
                            name: "Undo/Redo + Config Persistence",
                            description: "Project-integrated state history and layout persistence.",
                        },
                    ],
                },
                IdeFeatureCategory {
                    name: "Build & Runtime",
                    features: vec![
                        IdeFeature {
                            name: "Play/Pause/Stop",
                            description: "In-editor runtime control for fast gameplay iteration.",
                        },
                        IdeFeature {
                            name: "2D/3D Export Stubs",
                            description: "Packaging entry points for project deployment flows.",
                        },
                        IdeFeature {
                            name: "Performance Options",
                            description: "Target FPS, tick rate, and viewport option controls.",
                        },
                    ],
                },
            ],
        }
    }
}
