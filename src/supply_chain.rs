pub struct SupplyChainDemo {
    matrix: Vec<Vec<Option<f32>>>,
    matrix_buffer: Vec<Vec<String>>,
    nodes: Vec<String>,
    energy: i32,
    flow: i32,
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

        SupplyChainDemo {
            matrix,
            matrix_buffer,
            nodes,
            energy: 0,
            flow: 0,
        }
    }
}

impl SupplyChainDemo {

    // pub fn calc_flow_energy(&mut self, ui: &mut egui::Ui) -> (i32, i32) {
    //     ui.add(egui::Slider::new(&mut self.energy, 0.0..=100.0).text("My value"));
    //
    // }
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
