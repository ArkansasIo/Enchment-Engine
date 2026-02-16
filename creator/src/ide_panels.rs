use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum IdePanelKind {
    ContentBrowser,
    WorldOutliner,
    Details,
    OutputLog,
    Blueprint,
}

#[derive(Clone, Debug)]
pub struct IdePanelSnapshot {
    pub regions: usize,
    pub characters: usize,
    pub items: usize,
    pub tilemaps: usize,
    pub screens: usize,
    pub assets: usize,
    pub shaders: usize,
    pub avatars: usize,
    pub current_region_characters: usize,
    pub current_region_items: usize,
    pub current_project_context: String,
    pub current_content_context: String,
    pub has_town_data: bool,
    pub has_mmorpg_data: bool,
}

impl IdePanelSnapshot {
    pub fn from_editor_state(
        project: &Project,
        server_ctx: &ServerContext,
        has_town_data: bool,
        has_mmorpg_data: bool,
    ) -> Self {
        let mut current_region_characters = 0usize;
        let mut current_region_items = 0usize;
        if let Some(region) = project.get_region(&server_ctx.curr_region) {
            current_region_characters = region.characters.len();
            current_region_items = region.items.len();
        }

        Self {
            regions: project.regions.len(),
            characters: project.characters.len(),
            items: project.items.len(),
            tilemaps: project.tilemaps.len(),
            screens: project.screens.len(),
            assets: project.assets.len(),
            shaders: project.shaders.len(),
            avatars: project.avatars.len(),
            current_region_characters,
            current_region_items,
            current_project_context: format!("{:?}", server_ctx.pc),
            current_content_context: format!("{:?}", server_ctx.cc),
            has_town_data,
            has_mmorpg_data,
        }
    }
}

pub fn show_ide_panel_dialog(
    ui: &mut TheUI,
    ctx: &mut TheContext,
    kind: IdePanelKind,
    snapshot: &IdePanelSnapshot,
) {
    let (title, width, height) = match kind {
        IdePanelKind::ContentBrowser => ("Content Browser", 860, 520),
        IdePanelKind::WorldOutliner => ("World Outliner", 760, 500),
        IdePanelKind::Details => ("Details", 760, 500),
        IdePanelKind::OutputLog => ("Output Log", 920, 560),
        IdePanelKind::Blueprint => ("Blueprint", 880, 540),
    };

    let mut canvas = TheCanvas::new();
    canvas.limiter_mut().set_max_size(Vec2::new(width, height));

    let mut layout = TheTextLayout::new(TheId::named(&format!("IDE Panel {}", title)));
    layout.set_margin(Vec4::new(14, 14, 14, 14));
    layout.set_padding(8);
    layout.limiter_mut().set_max_width(width - 30);

    let mut header = TheText::new(TheId::named(&format!("IDE Panel Header {}", title)));
    header.set_text(format!("{} (Unreal-style Panel)", title));
    layout.add_pair("".to_string(), Box::new(header));

    match kind {
        IdePanelKind::ContentBrowser => {
            add_text_line(
                &mut layout,
                "Browse all project content assets and gameplay resources.",
            );
            add_text_line(
                &mut layout,
                &format!(
                    "Regions: {} | Characters: {} | Items: {} | Tilemaps: {}",
                    snapshot.regions, snapshot.characters, snapshot.items, snapshot.tilemaps
                ),
            );
            add_text_line(
                &mut layout,
                &format!(
                    "Screens: {} | Assets: {} | Shaders: {} | Avatars: {}",
                    snapshot.screens, snapshot.assets, snapshot.shaders, snapshot.avatars
                ),
            );
        }
        IdePanelKind::WorldOutliner => {
            add_text_line(
                &mut layout,
                "View and navigate entities in the active world/region hierarchy.",
            );
            add_text_line(
                &mut layout,
                &format!(
                    "Active region entities: characters={} items={}",
                    snapshot.current_region_characters, snapshot.current_region_items
                ),
            );
            add_text_line(
                &mut layout,
                &format!("Project context: {}", snapshot.current_project_context),
            );
        }
        IdePanelKind::Details => {
            add_text_line(
                &mut layout,
                "Inspect selected object settings and runtime/editor metadata.",
            );
            add_text_line(
                &mut layout,
                &format!("Project context: {}", snapshot.current_project_context),
            );
            add_text_line(
                &mut layout,
                &format!("Content context: {}", snapshot.current_content_context),
            );
            add_text_line(
                &mut layout,
                &format!(
                    "Town data loaded: {} | RPG/MMORPG data loaded: {}",
                    snapshot.has_town_data, snapshot.has_mmorpg_data
                ),
            );
        }
        IdePanelKind::OutputLog => {
            add_text_line(&mut layout, "Runtime/editor log panel for diagnostics and debugging.");
            add_text_line(
                &mut layout,
                "Use Project Tree -> Debug Log to open the dedicated docked log editor.",
            );
            add_text_line(
                &mut layout,
                "Play/Pause/Stop and simulation actions emit status updates visible in the status bar.",
            );
        }
        IdePanelKind::Blueprint => {
            add_text_line(
                &mut layout,
                "Visual scripting workflow for gameplay logic and content events.",
            );
            add_text_line(
                &mut layout,
                "Use the Blueprint editor command from the top toolbar for graph authoring.",
            );
            add_text_line(
                &mut layout,
                "Integrate with character/item scripts and MMO simulation routines.",
            );
        }
    }

    canvas.set_layout(layout);
    ui.show_dialog(title, canvas, vec![TheDialogButtonRole::Accept], ctx);
}

fn add_text_line(layout: &mut TheTextLayout, text: &str) {
    let mut line = TheText::new(TheId::named_with_id("IDE Panel Line", Uuid::new_v4()));
    line.set_text(text.to_string());
    layout.add_pair("".to_string(), Box::new(line));
}
