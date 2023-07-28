use egui::*;
use std::collections::HashMap;

static CORRECT_ANSWERS: [AnswerOption; 5] =
    [AnswerOption::C, AnswerOption::B, AnswerOption::A, AnswerOption::C, AnswerOption::A];

// The `AnswerOption` and `Survey` structs have already been defined, so we don't need to redefine them.
#[derive(PartialEq, Debug, Clone)]
pub enum AnswerOption {
    A, B, C, D,
}

pub struct Survey {
    questions: Vec<String>,
    answers: HashMap<usize, AnswerOption>,
}

impl Survey {
    pub fn new(questions: Vec<String>) -> Self {
        Survey { questions, answers: HashMap::new()}
    }
}


pub fn calculate_answers(survey: &Survey) -> f32 {
    let min_len = std::cmp::min(survey.answers.len(), CORRECT_ANSWERS.len());
    let mut count = 0;

    for i in 0..min_len {
        if let Some(answer) = survey.answers.get(&i) {
            if *answer == CORRECT_ANSWERS[i] {
                count += 1;
            }
        }
    }

    count as f32 / survey.answers.len() as f32 * 100.0
}

impl Survey {
    // Adding a method to show the survey
    pub fn show_survey(&mut self, ui: &mut Ui) {
        for (index, question) in self.questions.iter().enumerate() {
            ui.group(|ui| {
                ui.label(question);

                // Here, we can use radio buttons for the answer options.
                let mut selected_option = self.answers.get(&index).cloned().unwrap_or(AnswerOption::A);

                ui.horizontal(|ui| {
                    ui.radio_value(&mut selected_option, AnswerOption::A, "A");
                    ui.radio_value(&mut selected_option, AnswerOption::B, "B");
                    ui.radio_value(&mut selected_option, AnswerOption::C, "C");
                    ui.radio_value(&mut selected_option, AnswerOption::D, "D");
                });

                // Update the selected option
                self.answers.insert(index, selected_option);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_percentage() {
        let mut survey = Survey::new(vec!["Q1".to_string(), "Q2".to_string()]);
        survey.answers.insert(0, AnswerOption::C);
        survey.answers.insert(1, AnswerOption::B);

        assert_eq!(calculate_answers(&survey), 100.0);
    }

    #[test]
    fn test_half_correct_percentage() {
        let mut survey = Survey::new(vec!["Q1".to_string(), "Q2".to_string()]);
        survey.answers.insert(0, AnswerOption::C);
        survey.answers.insert(1, AnswerOption::A);

        assert_eq!(calculate_answers(&survey), 50.0);
    }

    #[test]
    fn test_zero_correct_percentage() {
        let mut survey = Survey::new(vec!["Q1".to_string(), "Q2".to_string()]);
        survey.answers.insert(0, AnswerOption::D);
        survey.answers.insert(1, AnswerOption::A);

        assert_eq!(calculate_answers(&survey), 0.0);
    }
}