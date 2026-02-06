use eframe::egui;

/// State of the Thomas' Cyclically Symmetric Attractor
///
/// The constant 'b' represents the disspiation of the system.
/// Chaos starts to emerge when 0.1 < b < 0.25
struct ThomasAttractor {
    b: f32,
    points: Vec<(f32, f32, f32)>,
    angle: f32,
}

impl Default for ThomasAttractor {
    fn default() -> Self {
        Self {
            b: 0.208,                      // best value to visualize bifurcation before chaos descent
            points: vec![(0.1, 0.0, 0.0)], // Starting seed
            angle: 0.0,
        }
    }
}

impl eframe::App for ThomasAttractor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // physics/math area
        let dt = 0.05; // Time step
        let (mut x, mut y, mut z) = *self.points.last().unwrap();

        // Generate 50 new points per frame for smooth animation
        for _ in 0..50 {
            let dx = y.sin() - self.b * x;
            let dy = z.sin() - self.b * y;
            let dz = x.sin() - self.b * z;

            x += dx * dt;
            y += dy * dt;
            z += dz * dt;
            self.points.push((x, y, z));
        }

        // Keep a "tail" so we don't crash the RAM (max 5000 points)
        if self.points.len() < 100_000 {
            let (mut x, mut y, mut z) = *self.points.last().unwrap();
            for _ in 0..100 {
                // 100 steps per frame makes it bloom faster
                let dx = y.sin() - self.b * x;
                let dy = z.sin() - self.b * y;
                let dz = x.sin() - self.b * z;
                x += dx * dt;
                y += dy * dt;
                z += dz * dt;
                self.points.push((x, y, z));
            }
        }

        // ui!
        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("Thomas Attractor");
            ui.add(egui::Slider::new(&mut self.b, 0.0..=1.0).text("b (Dissipation)"));
            if ui.button("Reset").clicked() {
                self.points = vec![(0.1, 0.0, 0.0)];
            }
            ui.label("Lower 'b' = More Chaos");
        });

        // render those frames!
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let rect = ui.max_rect();
            let center = rect.center();
            let scale = 50.0;

            // Simple 3D to 2D rotation & projection
            self.angle += 0.01;
            let (sin_a, cos_a) = self.angle.sin_cos();

            for i in 1..self.points.len() {
                let p1 = self.points[i - 1];
                let p2 = self.points[i];

                // Project point A
                let x1 = (p1.0 * cos_a - p1.1 * sin_a) * scale + center.x;
                let y1 = p1.2 * scale + center.y;

                // Project point B
                let x2 = (p2.0 * cos_a - p2.1 * sin_a) * scale + center.x;
                let y2 = p2.2 * scale + center.y;

                painter.line_segment(
                    [egui::pos2(x1, y1), egui::pos2(x2, y2)],
                    egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_unmultiplied(100, 200, 255, 40),
                    ),
                );
            }
        });

        ctx.request_repaint(); // Keep the animation loop running
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Chaos Visualizer",
        native_options,
        Box::new(|_| Ok(Box::<ThomasAttractor>::default())),
    )
}
