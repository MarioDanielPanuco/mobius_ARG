use egui::{InnerResponse, Widget};
use serde::{Deserialize, Serialize};
use crate::levels::{AppLevel, Level};
use crate::supply_chain::*;

// Define a static array of filepaths
static FILEPATHS: [&str; 3] = [
    "/assets/text/level_1.md",
    "/assets/text/level_2.md",
    "/assets/text/level_3.md",
];

#[derive(Serialize, Deserialize, PartialEq)]
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

    #[serde(skip)]
    image_texture: Option<egui::TextureHandle>,

    lvl_num: LevelNum,

    #[serde(skip)]
    supply_chain_demo: SupplyChainDemo,
}


impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            supply_chain_demo: SupplyChainDemo::default(),
            label: "Hello World!".to_owned(),
            value: 25,
            passed_l1: false,
            passed_l2: false,
            passed_l3: false,
            lvl_num: LevelNum::Level1,
            image_texture: None,

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
            mut passed_l1, passed_l2, passed_l3,
            lvl_num,
            image_texture,
            supply_chain_demo,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui


        let mut c_history: Vec<String> = vec![]; // TODO: add to app_state
        egui::SidePanel::left("chat_area").show(ctx, |mut ui| {
            ui.heading("Prime Intellect");

            ui.vertical(|ui| {
                ui.label("Write something: ");
                ui.text_edit_multiline(label);
            });


            if ui.button("Submit").clicked() {
                c_history.push(label.to_string());
                if label == "flame" { passed_l1 =  true; }
            }

            self.supply_chain_demo.ui(ui);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT),
                           |ui| {
                               ui.horizontal(|ui| {
                                   ui.spacing_mut().item_spacing.x = 0.0;
                                   ui.label("powered by ");
                               });
                           });
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


            ui.heading("Location");
            ui.label(format!("Starship Archemdis\n{:} thousand colonists", value));
            ui.heading("Time");

            egui::warn_if_debug_build(ui);
        });

        // egui::SidePanel::right("right_panel").show(ctx, |ui| {});
        if self.passed_l1 && self.passed_l2 && !self.passed_l3 {
            self.lvl_num = LevelNum::Level3;
        } else if self.passed_l1 && !self.passed_l2 && !self.passed_l3 {
            self.lvl_num = LevelNum::Level2;
        } else if !self.passed_l1 && !self.passed_l2 && !self.passed_l3 {
            self.lvl_num = LevelNum::Level1;
        } else {}

        match self.lvl_num {
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

use egui::{RichText, TextStyle, Color32, FontId};
use image::open;

fn display_image(ui: &mut egui::Ui, path: &str, texture_cache: &mut Option<egui::TextureHandle>) {
    let texture: &egui::TextureHandle = texture_cache.get_or_insert_with(|| {
        // Load the texture only once.
        let img = open(path).expect("Failed to open image").to_rgba8();

        // Convert to egui::ColorImage.
        let size = [img.width() as usize, img.height() as usize];
        let pixels = img.into_raw()
            .chunks_exact(4)
            .map(|chunk| egui::Color32::from_rgba_premultiplied(chunk[0], chunk[1], chunk[2], chunk[3]))
            .collect();

        let image_data = egui::ColorImage {
            size,
            pixels,
        };

        ui.ctx().load_texture(path, image_data, Default::default())
    });

    // Show the image:
    ui.image(texture, texture.size_vec2());
}

use image::GenericImageView;
use crate::supply_chain;
use crate::supply_chain::SupplyChainDemo;

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();

    let pixels: Vec<egui::Color32> = image_buffer.pixels()
        .map(|p| egui::Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3]))
        .collect();

    Ok(egui::ColorImage {
        size,
        pixels,
    })
}

fn load_image_from_memory(image_data: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();

    let pixels: Vec<egui::Color32> = image_buffer.pixels()
        .map(|p| egui::Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3]))
        .collect();

    Ok(egui::ColorImage {
        size,
        pixels,
    })
}


impl TemplateApp {
    fn level_1(&mut self, ctx: &egui::Context) {
        egui::Window::new("").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.hyperlink("webpage.com");
            });
        });
    }


    fn level_2(&mut self, ctx: &egui::Context) {
        egui::Window::new("Prometheus Corp.").show(ctx, |ui| {
            // Title
            ui.heading("Prometheus Corp."); // This is a simple label now.

            // Tagline (We'll just use a normal label for now)
            ui.label("\"Illuminating the Path to True Progress\"");
            // display_image(ui, "assets/images/corp.png", &mut self.image_texture);
            /*let path = std::path::Path::new("assets/images/corp.png");

            match load_image_from_path(&path) {
                Ok(color_image) => {
                    let texture_handle = ctx.load_texture("my_image_name", color_image, Default::default());
                    // now use texture_handle with egui's UI functions to display the image

                    ui.add(egui::Image::new(&texture_handle, texture_handle.size_vec2()));
                }
                Err(e) => {
                    println!("Error loading image: {:?}", e);
                }
            }*/

            ui.group(|ui| {
                // About Us
                ui.label("About Us:");
                ui.label("In a world teetering on the brink, Prometheus Corp stands as a beacon of hope and enlightenment. Established in 2301, our legacy stretches back to times of unprecedented turmoil and change. From those challenging times, we emerged with a singular vision: to guide humanity with the flame of innovation towards a sustainable future.");
                ui.label("Our pioneering innovations, including our globally celebrated AGI models, showcase the fusion of cutting-edge tech with sustainable ideologies. We've always been ahead of the curve, foreseeing the need for a harmonious balance between unchecked growth and mindful conservation.");

                // Our Philosophy
                ui.label("Our Philosophy:");
                ui.label("1. Balanced Growth: In a world of finite resources, unchecked expansion can lead to collapse. We champion controlled growth, making sure advancements are not just rapid but right.");
                ui.label("2. Empowerment Through Dependence: Prometheus understands that true progress isn't about unfettered access, but about quality, efficiency, and sustainability. We provide the essential, cutting out the superfluous, ensuring society gets only what it needs, when it needs, with unmatched efficiency.");
                ui.label("3. Guiding the De-growth Movement: While the concept of de-growth may seem counterintuitive to a tech corporation, we've always believed in being the stewards of change. By endorsing and guiding the de-growth movement, we aim to steer humanity towards a future where technology complements nature rather than competes with it.");

                // Why Prometheus?
                ui.label("Why Prometheus?");
                ui.label("At the heart of Prometheus is a relentless pursuit of knowledge and innovation. But unlike the myths of old, our fire isn't stolen; it's shared, albeit responsibly. Join us in our journey as we illuminate pathways, not just for progress, but for true progress that honors both man and nature.");
            });
        });
    }


    fn level_3(&mut self, ctx: &egui::Context) {
        egui::Window::new("Level 3").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("");
            });
        });
    }
}