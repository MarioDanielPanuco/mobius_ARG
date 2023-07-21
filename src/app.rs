use egui::Widget;
use egui_graphs::{Graph, GraphView};
use petgraph::Directed;
use serde::{Deserialize, Serialize};
use crate::levels::{AppLevel, Level};
use crate::graphs::*;

// Define a static array of filepaths
static FILEPATHS: [&str; 3] = [
    "/assets/text/level_1.md",
    "/assets/text/level_2.md",
    "/assets/text/level_3.md",
];

#[derive(Serialize, Deserialize)]
enum LevelNum { Level1, Level2, Level3 }

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: usize,

    passed_l1: bool,
    passed_l2: bool,
    passed_l3: bool,

    lvl_num: LevelNum,

    #[serde(skip)]
    mu_graph: Graph<(), (), Directed>,
}


impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 0,
            passed_l1: false,
            passed_l2: false,
            passed_l3: false,
            lvl_num: LevelNum::Level1,
            mu_graph: generate_graph(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}


impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label, value,
            passed_l1, passed_l2, passed_l3,
            lvl_num,
            mu_graph,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui


        let mut c_history: Vec<String> = vec![]; // TODO: add to app_state
        egui::SidePanel::left("chat_area").show(ctx, |ui| {
            ui.heading("Prime Intellect");

            ui.vertical(|ui| {
                ui.label("Write something: ");
                ui.text_edit_multiline(label);
            });


            if ui.button("Submit").clicked() {
                c_history.push(label.to_string());
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT),
                           |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                });
            });
        });

        egui::Window::new("graph").show(ctx, |ui| {
            ui.add(&mut GraphView::new(mu_graph));
        });
        egui::SidePanel::right("level_completed").show(ctx, |ui| {
            ui.label(format!("Level 1: {}", passed_l1));
            ui.label(format!("Level 2: {}", passed_l2));
            ui.label(format!("Level 3: {}", passed_l3));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");
            ui.vertical_centered(|ui| {
                c_history.iter().for_each(|elem| { ui.label(elem); })
            });

            // ui.checkbox(&mut self.passed_l1, "Level 1");
            ui.add(egui::Checkbox::new(&mut self.passed_l1, "Level 1"));
            ui.add(egui::Checkbox::new(&mut self.passed_l2, "Level 2"));
            ui.add(egui::Checkbox::new(&mut self.passed_l3, "Level 3"));

            egui::warn_if_debug_build(ui);
        });

        // egui::SidePanel::right("right_panel").show(ctx, |ui| {});

        match lvl_num {
            // Matching Level Windows
            // LevelNum::Level1 => self.level_1(ctx),
            LevelNum::Level1 => Level::new("Level 1", "Body 1", value)
                                    .show(ctx),
            LevelNum::Level2 => self.level_2(ctx),
            LevelNum::Level3 => self.level_3(ctx),
        }
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl TemplateApp {
    fn level_1(&mut self, ctx: &egui::Context) {
        egui::Window::new("did_they").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.hyperlink("webpage.com");
            });
        });
    }

    fn level_2(&mut self, ctx: &egui::Context) {
        egui::Window::new("Level 2").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("test");
            });
        });
    }

    fn level_3(&mut self, ctx: &egui::Context) {
        // Level::new("level 3", "Body", ).show(ctx);
        todo!()
    }
}