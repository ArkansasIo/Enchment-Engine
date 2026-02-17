//! WorldMapEditor: Editor logic for planet-scale world maps (2D/3D, naming, tools)

use super::worldmap::WorldMap;
use crate::engine::map::view2d::{renderer2d::Renderer2D, overlay2d::Overlay2D, grid2d::Grid2D, camera2d::Camera2D};
use crate::engine::map::view3d::{renderer3d::Renderer3D, overlay3d::Overlay3D, grid3d::Grid3D, camera3d::Camera3D};

pub struct WorldMapEditor {
    pub worldmap: WorldMap,
    pub view_mode: WorldMapViewMode,
    pub selected_continent: Option<usize>,
    pub selected_zone: Option<usize>,
    pub selected_place: Option<usize>,
    // 2D view components
    pub renderer2d: Renderer2D,
    pub overlay2d: Overlay2D,
    pub grid2d: Grid2D,
    pub camera2d: Camera2D,
    // 3D view components
    pub renderer3d: Renderer3D,
    pub overlay3d: Overlay3D,
    pub grid3d: Grid3D,
    pub camera3d: Camera3D,
}

pub enum WorldMapViewMode {
    View2D,
    View3D,
}

impl WorldMapEditor {
    pub fn new(name: &str) -> Self {
        Self {
            worldmap: WorldMap::new(name),
            view_mode: WorldMapViewMode::View2D,
            selected_continent: None,
            selected_zone: None,
            selected_place: None,
            renderer2d: Renderer2D,
            overlay2d: Overlay2D,
            grid2d: Grid2D::new(100, 100),
            camera2d: Camera2D::new(0.0, 0.0, 1.0),
            renderer3d: Renderer3D,
            overlay3d: Overlay3D,
            grid3d: Grid3D::new(100, 100, 10),
            camera3d: Camera3D::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn set_view_mode(&mut self, mode: WorldMapViewMode) {
        self.view_mode = mode;
    }

    pub fn render(&self) {
        match self.view_mode {
            WorldMapViewMode::View2D => {
                self.renderer2d.render();
                self.overlay2d.draw();
                // Add grid/camera logic as needed
            }
            WorldMapViewMode::View3D => {
                self.renderer3d.render();
                self.overlay3d.draw();
                // Add grid/camera logic as needed
            }
        }
    }

    pub fn name_place(&mut self, idx: usize, name: String) {
        // Example: name a continent, zone, or place
        if let Some(cont) = self.worldmap.continents.get_mut(idx) {
            cont.name = name;
        }
    }

    // Add more methods for editing, partitioning, and tool logic as needed
}
