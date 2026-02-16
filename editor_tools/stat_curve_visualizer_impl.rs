// Stat Curve Visualizer UI Implementation Example for egui
// (Stub: extend with graphing, formula tweaking, etc.)

use egui::*;

pub struct StatCurveVisualizerImpl {
    pub status: String,
    pub formula: String,
    pub min_x: f32,
    pub max_x: f32,
    pub points: Vec<[f64; 2]>,
}

impl StatCurveVisualizerImpl {
    pub fn new() -> Self {
        Self {
            status: "Ready".to_string(),
            formula: "a * x.powf(b) + c".to_string(),
            min_x: 1.0,
            max_x: 100.0,
            points: Vec::new(),
        }
    }

    fn compute_points(&mut self, a: f32, b: f32, c: f32) {
        self.points.clear();
        let steps = 100;
        let dx = (self.max_x - self.min_x) / steps as f32;
        for i in 0..=steps {
            let x = self.min_x + i as f32 * dx;
            let y = a * x.powf(b) + c;
            self.points.push([x as f64, y as f64]);
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Stat Curve Visualizer");
            ui.label("Graph stat scaling, tweak formulas, preview results here.");
            ui.horizontal(|ui| {
                ui.label("Formula:");
                ui.text_edit_singleline(&mut self.formula);
            });
            let mut a = 1.0f32;
            let mut b = 1.0f32;
            let mut c = 0.0f32;
            ui.horizontal(|ui| {
                ui.label("a:");
                ui.add(egui::DragValue::new(&mut a).speed(0.1));
                ui.label("b:");
                ui.add(egui::DragValue::new(&mut b).speed(0.1));
                ui.label("c:");
                ui.add(egui::DragValue::new(&mut c).speed(0.1));
            });
            ui.horizontal(|ui| {
                ui.label("X range:");
                ui.add(egui::DragValue::new(&mut self.min_x).clamp_range(0.0..=self.max_x-1.0));
                ui.add(egui::DragValue::new(&mut self.max_x).clamp_range(self.min_x+1.0..=1000.0));
            });
            if ui.button("Update Graph").clicked() {
                self.compute_points(a, b, c);
                self.status = format!("Graph updated: a={a}, b={b}, c={c}");
            }
            if !self.points.is_empty() {
                egui::plot::Plot::new("stat_curve_plot")
                    .view_aspect(2.0)
                    .show(ui, |plot_ui| {
                        plot_ui.line(egui::plot::Line::new(egui::plot::PlotPoints::from(self.points.clone())));
                    });
            }
            ui.label(&self.status);
        });
    }
}
