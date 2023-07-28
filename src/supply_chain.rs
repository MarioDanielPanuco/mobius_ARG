pub struct SupplyChainDemo {
    // Represents the adjacency matrix
    matrix: Vec<Vec<i32>>,
    nodes: Vec<String>,
}

impl Default for SupplyChainDemo {
    fn default() -> Self {
        let nodes = vec![
            "Supplier".to_string(),
            "Manufacturer".to_string(),
            "Warehouse".to_string(),
            "Retailer".to_string(),
        ];
        let matrix = vec![
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
            vec![0, 0, 0, 0],
        ];

        SupplyChainDemo { matrix, nodes }
    }
}

impl SupplyChainDemo {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};

        let mut table = TableBuilder::new(ui)
            .resizable(true)
            .striped(true)
            .column(Column::auto());

        for _ in &self.nodes {
            table = table.column(Column::initial(80.0));
        }

        table.header(30.0, |mut header| {
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

                    for j in 0..self.matrix[i].len() {
                        row.col(|ui| {
                            // Using a string buffer to edit the integer values
                            let mut buffer = self.matrix[i][j].to_string();
                            if ui.text_edit_singleline(&mut buffer).lost_focus() {
                                if let Ok(updated_value) = buffer.parse::<i32>() {
                                    self.matrix[i][j] = updated_value;
                                }
                            }
                        });
                    }
                });
            }
        });

    }
}
