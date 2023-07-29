pub fn test(val: &mut i32) {
    *val += 1;
}

pub trait AppLevel {
    fn new(name: &'static str, context: &'static str, health: &mut usize) -> Self;

    fn name(&self) -> &'static str;
    fn context(&self) -> &'static str;
    fn user_health(&self) -> usize;

    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            ui.vertical(|ui| ui.label(self.context()));
            let mut val = 1;
            test(&mut val);
            ui.label(val.to_string());
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 10.0;
                ui.label(format!("Player health: {}", self.user_health()));
            });
        });
    }
}

pub struct Level {
    name: &'static str,
    context: &'static str,
    user_health: usize,
}

impl AppLevel for Level {
    fn new(name: &'static str, context: &'static str, health: &mut usize) -> Self {
        Level {
            name,
            context,
            user_health: *health,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
    fn context(&self) -> &'static str {
        self.context
    }
    fn user_health(&self) -> usize {
        self.user_health
    }
}
