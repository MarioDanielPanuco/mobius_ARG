pub struct SupplyChainDemo {
    // Represents the adjacency matrix
    matrix: Vec<Vec<i32>>,
    matrix_buffer: Vec<Vec<String>>,
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
        let matrix_buffer = matrix.iter().map(|row| {
            row.iter().map(|&value| value.to_string()).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        SupplyChainDemo { matrix, matrix_buffer, nodes }
    }
}

const MIN_WEIGHT: i32 = 0;
const MAX_WEIGHT: i32 = 10;
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

                        for j in 0..self.matrix_buffer[i].len() {
                            row.col(|ui| {
                                if let Ok(updated_value) = self.matrix_buffer[i][j].parse::<i32>() {
                                    // Check if the entered value is within the range
                                    if (MIN_WEIGHT..=MAX_WEIGHT).contains(&updated_value) {
                                        self.matrix[i][j] = updated_value;
                                    } else {
                                        // This is a simple example, but you could provide feedback to the user here
                                        // indicating that the entered value is out of range.
                                        // Reset buffer to old matrix value
                                        self.matrix_buffer[i][j] = self.matrix[i][j].to_string();
                                    }
                                }
                            });
                        }
                    });
                }
            });
    }
}
