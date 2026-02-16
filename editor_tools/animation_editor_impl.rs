// Animation Editor UI Implementation Example for egui
// (Stub: extend with frame assignment, FPS, preview, etc.)


use egui::*;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use std::fs;
use image::GenericImageView;

pub struct AnimationEditorImpl {
    pub status: String,
    pub frames: Vec<PathBuf>,
    pub current_frame: usize,
    pub fps: u32,
    pub last_frame_time: Option<Instant>,
    pub playing: bool,
    pub loaded_textures: Vec<Option<egui::TextureHandle>>,
}

impl AnimationEditorImpl {
    pub fn new() -> Self {
        Self {
            status: "Ready".to_string(),
            frames: Vec::new(),
            current_frame: 0,
            fps: 6,
            last_frame_time: None,
            playing: false,
            loaded_textures: Vec::new(),
        }
    }

    fn load_frame_texture(&mut self, ctx: &egui::Context, idx: usize) {
        if idx >= self.frames.len() {
            return;
        }
        if self.loaded_textures.len() <= idx {
            self.loaded_textures.resize(self.frames.len(), None);
        }
        if self.loaded_textures[idx].is_some() {
            return;
        }
        let path = &self.frames[idx];
        if let Ok(img) = image::open(path) {
            let size = [img.width() as usize, img.height() as usize];
            let rgba = img.to_rgba8();
            let pixels = rgba.as_flat_samples();
            let tex = ctx.load_texture(
                format!("anim_frame_{}", idx),
                egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
                egui::TextureOptions::default(),
            );
            self.loaded_textures[idx] = Some(tex);
        } else {
            self.status = format!("Failed to load image: {}", path.display());
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Animation Editor");
            ui.horizontal(|ui| {
                if ui.button(if self.playing { "Pause" } else { "Play" }).clicked() {
                    self.playing = !self.playing;
                    if self.playing {
                        self.last_frame_time = Some(Instant::now());
                    }
                }
                if ui.button("Prev").clicked() {
                    self.playing = false;
                    if self.current_frame > 0 {
                        self.current_frame -= 1;
                    }
                }
                if ui.button("Next").clicked() {
                    self.playing = false;
                    if self.current_frame + 1 < self.frames.len() {
                        self.current_frame += 1;
                    }
                }
                ui.label(format!("Frame {}/{}", self.current_frame + 1, self.frames.len()));
                ui.add(egui::Slider::new(&mut self.fps, 1..=60).text("FPS"));
            });

            // Frame list UI
            egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                let mut remove_idx = None;
                for (i, path) in self.frames.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(path.file_name().unwrap_or_default().to_string_lossy());
                        if ui.button("Remove").clicked() {
                            remove_idx = Some(i);
                        }
                        if ui.button("Up").clicked() && i > 0 {
                            self.frames.swap(i, i - 1);
                            self.loaded_textures.swap(i, i - 1);
                        }
                        if ui.button("Down").clicked() && i + 1 < self.frames.len() {
                            self.frames.swap(i, i + 1);
                            self.loaded_textures.swap(i, i + 1);
                        }
                    });
                }
                if let Some(idx) = remove_idx {
                    self.frames.remove(idx);
                    self.loaded_textures.remove(idx);
                    if self.current_frame >= self.frames.len() && !self.frames.is_empty() {
                        self.current_frame = self.frames.len() - 1;
                    }
                }
            });

            if ui.button("Add Frame (PNG)").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("PNG", &["png"]).pick_file() {
                    self.frames.push(path);
                    self.loaded_textures.push(None);
                }
            }

            // Animation preview
            if !self.frames.is_empty() {
                self.load_frame_texture(ctx, self.current_frame);
                if let Some(Some(tex)) = self.loaded_textures.get(self.current_frame) {
                    ui.image(tex, tex.size_vec2());
                }
            } else {
                ui.label("No frames loaded.");
            }

            // Animation playback logic
            if self.playing && !self.frames.is_empty() {
                let now = Instant::now();
                let frame_time = Duration::from_secs_f32(1.0 / self.fps as f32);
                if let Some(last) = self.last_frame_time {
                    if now.duration_since(last) >= frame_time {
                        self.current_frame = (self.current_frame + 1) % self.frames.len();
                        self.last_frame_time = Some(now);
                        ctx.request_repaint();
                    }
                }
            }

            ui.label(&self.status);
        });
    }
}
