use egui::*;
use std::collections::HashMap;
use log::set_max_level;

static CORRECT_ANSWERS: [AnswerOption; 5] = [
    AnswerOption::C,
    AnswerOption::B,
    AnswerOption::A,
    AnswerOption::C,
    AnswerOption::A,
];

// The `AnswerOption` and `Survey` structs have already been defined, so we don't need to redefine them.
#[derive(PartialEq, Debug, Clone)]
pub enum AnswerOption {
    A, B, C, D,
}

pub struct Survey {
    questions: Vec<String>,
    answers: Vec<AnswerOption>,
}

impl Survey {
    pub fn new(questions: Vec<String>) -> Self {
        let len = questions.len();
        Survey {
            questions,
            answers: vec![AnswerOption::A; len],
        }
    }
}

pub fn calculate_answers(survey: &Survey) -> f32 {
    let min_len = std::cmp::min(survey.answers.len(), CORRECT_ANSWERS.len());
    let count = survey
        .answers
        .iter()
        .zip(CORRECT_ANSWERS.iter())
        .take(min_len)
        .filter(|(a, b)| a == b)
        .count();

    count as f32 / survey.answers.len() as f32 * 100.0
}

impl Survey {
    // Adding a method to show the survey
    pub fn show_survey(&mut self, ui: &mut Ui) {
        for (index, question) in self.questions.iter().enumerate() {
            ui.group(|ui| {
                ui.label(question);

                if let Some(selected_option) = self.answers.get_mut(index) {
                    ui.horizontal(|ui| {
                        ui.radio_value(selected_option, AnswerOption::A, "A");
                        ui.radio_value(selected_option, AnswerOption::B, "B");
                        ui.radio_value(selected_option, AnswerOption::C, "C");
                        ui.radio_value(selected_option, AnswerOption::D, "D");
                    });
                }
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
