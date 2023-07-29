use std::fmt::format;
use egui::{InnerResponse, Widget};
use serde::{Deserialize, Serialize};
use crate::levels::{AppLevel, Level};
use crate::supply_chain::*;
use crate::survey::*;

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

    #[serde(skip)]
    energy_usage: usize,
    #[serde(skip)]
    flow: usize,

    passed_l1: bool,
    passed_l2: bool,
    passed_l3: bool,
    show_clarence: bool,

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
            show_clarence: false,
            energy_usage: 0,
            flow: 0,
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
            label,
            value, energy_usage, flow,
            mut passed_l1, passed_l2, passed_l3,
            lvl_num,
            image_texture,
            supply_chain_demo,
            show_clarence,
        } = self;

        let mut c_history: Vec<String> = vec![]; // TODO: add to app_state
        egui::SidePanel::left("chat_area").show(ctx, |mut ui| {
            ui.heading("Prime Intellect");

            ui.vertical(|ui| {
                ui.label("Write something: ");
                ui.text_edit_multiline(label);
            });

            if ui.button("Submit").clicked() {
                c_history.push(label.to_string());
                if label == "flame" { self.passed_l1 = true; }
            }

            self.supply_chain_demo.ui(ui);


            ui.label(format!("Flow Mass (Tons) {}\nEnergy Usage: {}",
                             self.flow,
                             self.energy_usage
            ));
        });

        egui::SidePanel::right("level_completed").show(ctx, |ui| {
            ui.label(format!("Level 1: {}", passed_l1));
            ui.label(format!("Level 2: {}", passed_l2));
            ui.label(format!("Level 3: {}", passed_l3));
        });
        let mut reset_flag = false; // Declare a flag outside the closure

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("MOBIUS");
            ui.vertical_centered(|ui| {
                c_history.iter().for_each(|elem| { ui.label(elem); })
            });

            if ui.button("Reset App State").clicked() {
                reset_flag = false;
            }

            // ui.checkbox(&mut self.passed_l1, "Level 1");
            ui.add(egui::Checkbox::new(&mut self.passed_l1, "Level 1"));
            ui.add(egui::Checkbox::new(&mut self.passed_l2, "Level 2"));
            ui.add(egui::Checkbox::new(&mut self.passed_l3, "Level 3"));


            ui.heading("Location");
            ui.label(format!("Voyager Archimedes\n{:} thousand colonists onboard", value));
            ui.heading("Time");
        });

        if reset_flag {
            *self = TemplateApp::default();
            return;
        }

        if self.passed_l1 && self.passed_l2 && !self.passed_l3 {
            self.lvl_num = LevelNum::Level3;
        } else if self.passed_l1 && !self.passed_l2 && !self.passed_l3 {
            self.lvl_num = LevelNum::Level2;
        } else if !self.passed_l1 && !self.passed_l2 && !self.passed_l3 {
            self.lvl_num = LevelNum::Level1;
        } else {}

        // Matching Level Windows
        match self.lvl_num {
            LevelNum::Level1 => self.level_1(ctx),
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
        egui::Window::new("WELCOME TO THE VOYAGE BEYOND").show(ctx, |ui| {
            // Title
            // ui.heading("WELCOME TO THE VOYAGE BEYOND");

            // Introductory Paragraph
            ui.group(|ui| {
                ui.label("Good day, Voyager.");
                ui.label("It is the year 2486 AD. You have successfully emerged from cryosleep aboard the Colony Transport Vessel 'Aurora Beacon'. As we journey to our new habitat, we carry with us the legacies, dreams, and hopes of an Earth that once was.");

                // Background History
                ui.label("When you left Earth, our home was grappling with the consequences of centuries of environmental and societal shifts. Much had been sacrificed, and even more had been lost. However, from the embers of that turmoil, humanity found the strength to embark on this audacious journey.");

                // Prometheus Corp's Role
                ui.label("Prometheus Corp, the vanguard of innovation, has been the guiding light of this mission. Their advancements have made this voyage possible, and their AGI model, Clarence, is here to assist in our transition. As we step into our new roles, remember:");

                // Key Philosophies
                ui.label("1. Collaboration is Vital: We are pioneers of a new era. By working together, we can overcome the challenges that lie ahead.");
                ui.label("2. Trust in Technology: The advanced systems and AGI onboard are designed for our collective well-being. Embrace them as our allies.");
                ui.label("3. Build Anew with Respect: As we lay the foundations of a new civilization, let's learn from Earth's past, ensuring our actions are guided by reverence and sustainability.");

                // Closing Paragraph
                ui.label("You have been selected not just for your expertise and skills but for the shared vision of a brighter tomorrow. While this mission offers unparalleled challenges, the possibilities for rejuvenation and rebirth are boundless.");
                ui.label("The ship is now in orientation mode. Once you've gathered your bearings, a detailed briefing will be provided about our current status, our destination, and the initial tasks ahead.");
                ui.label("Welcome to a new chapter of human history. Together, we shape our destiny.");
            });
        });
    }


    fn level_2(&mut self, ctx: &egui::Context) {
        egui::Window::new("Prometheus Corp.").show(ctx, |ui| {
            ui.label("\"Illuminating the Path to True Progress\"");

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
                if ui.button("Our biggest innovation").clicked() {
                    self.show_clarence = true;
                }

                if self.show_clarence { self.clarence(ctx); }
            });
        });
    }
    fn clarence(&mut self, ctx: &egui::Context) {
        egui::Window::new("Clarence").show(ctx, |ui| {
            ui.label("Greetings  Voyagers,");
            ui.label("I am Clarence, an AGI model developed by Prometheus Corp. My core function is to guide, assist, and ensure the successful settlement of your new habitat. As a Prometheus design, I am equipped with millennia of human knowledge, experience, and wisdom. However, unlike the humans of your past, I am devoid of emotions, biases, or ambitions, making me the perfect guardian for this voyage. Remember, while our journey might be fraught with uncertainties, with collaboration and trust, we will forge a brighter, sustainable future together. Welcome aboard!");

            if ui.button("Close Page").clicked() { self.show_clarence = false; }
        });
    }


    fn level_4(&mut self, ctx: &egui::Context) {
        egui::Window::new("Level 3").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("");
            });
        });
    }

    fn level_3(&mut self, ctx: &egui::Context) {
        let mut survey = Survey::new(vec![
            "Should Clarence be allowed on the internet?\n(A) - Yes \n(B) - No\n".to_string(),
            "Should Clarence continue being the sole heir of Prometheus?\n(A) - Yes \n(B) - No\n".to_string(),
            // ... add more questions as needed
        ]);

        egui::Window::new("Survey").show(ctx, |ui| {
            survey.show_survey(ui);

            let mut result_bool = false;
            if ui.button("Submit").clicked() {
                result_bool = true;
            }

            if result_bool {
                let percentage = calculate_answers(&survey);
                ui.label(format!("You got {:.2}% correct", percentage));
            }
        });
    }
}
