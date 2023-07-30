pub struct SupplyChainDemo {
    matrix: Vec<Vec<Option<f32>>>,
    matrix_buffer: Vec<Vec<String>>,
    nodes: Vec<String>,
    pub(crate) energy: i32,
    pub(crate) flow: i32,
    augmentation_matrix: Vec<Vec<f32>>,
}

impl Default for SupplyChainDemo {
    fn default() -> Self {
        let nodes = vec![
            "Silicon Supplier".to_string(),
            "Semiconductor Manufacturer".to_string(),
            "Warehouse".to_string(),
            "Retailer".to_string(),
        ];
        let matrix = vec![
            vec![None, Some(1.0), None, None],
            vec![None, None, Some(1.0), None],
            vec![None, None, None, Some(1.0)],
            vec![None, None, None, None],
        ];
        let matrix_buffer = matrix
            .iter()
            .map(|row| {
                row.iter()
                    .map(|value| match value {
                        Some(v) => v.to_string(),
                        None => String::from("0"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let n = nodes.len();
        let n_1 = matrix[0].len();
        SupplyChainDemo {
            matrix,
            matrix_buffer,
            nodes,
            energy: 0,
            flow: 0,
            augmentation_matrix: vec![vec![1.0; n_1]; n],
        }
    }
}

impl SupplyChainDemo {

    pub fn calc_flow_energy(&mut self, ui: &mut egui::Ui) {
        // Let's create sliders for augmentation values of matrix entries.
        for i in 0..self.nodes.len() {
            for j in 0..self.matrix[0].len() {
                let mut value = self.augmentation_matrix[i][j];
                let label = format!("Augmentation for [{}][{}]", i, j);
                ui.add(egui::Slider::new(&mut value, 1.0..=10.0).text(label));
                self.augmentation_matrix[i][j] = value;
            }
        }

        // Calculate energy and flow based on the matrix and its augmentations.
        let mut total_energy = 0.0;
        let mut total_flow = 0.0;
        for i in 0..self.nodes.len() {
            for j in 0..self.matrix[0].len() {
                if let Some(value) = self.matrix[i][j] {
                    total_energy += value * self.augmentation_matrix[i][j];
                }
            }
        }

        // Assuming flow is the sum of the maximum augmented value in each column.
        for j in 0..self.matrix[0].len() {
            let mut max_val = 0.0;
            for i in 0..self.nodes.len() {
                if let Some(value) = self.matrix[i][j] {
                    let augmented_value = value * self.augmentation_matrix[i][j];
                    if augmented_value > max_val {
                        max_val = augmented_value;
                    }
                }
            }
            total_flow += max_val;
        }


        // Convert them to i32 for setting them to struct fields.
        self.energy = total_energy as i32;
        self.flow = total_flow as i32;
    }

    // ... Rest of the code remains unchanged.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};

        let mut table = TableBuilder::new(ui)
            .resizable(true)
            .striped(true)
            .column(Column::auto());

        for _ in &self.nodes {
            table = table.column(Column::initial(80.0));
        }

        table
            .header(30.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Nodes");
                });

                for node in &self.nodes {
                    header.col(|ui| {
                        ui.strong(node);
                    });
                }
            })
            .body(|mut body| {
                for (i, node) in self.nodes.iter().enumerate() {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(node);
                        });

                        for j in 0..self.matrix_buffer[i].len() {
                            row.col(|ui| {
                                let mut buffer = match self.matrix[i][j] {
                                    Some(val) => val.to_string(),
                                    None => String::from("0"),
                                };
                                if ui.text_edit_singleline(&mut buffer).lost_focus() {
                                    if let Ok(updated_value) = buffer.parse::<f32>() {
                                        self.matrix[i][j] = if updated_value == 0.0 {
                                            None
                                        } else {
                                            Some(updated_value)
                                        };
                                    }
                                }
                            });
                        }
                    });
                }
            });
    }
}
