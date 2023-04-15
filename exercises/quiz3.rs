// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub trait Card{
    fn print(&self) -> String;
}

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl Card for ReportCard {
    fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, &self.grade)
    }
}

impl ReportCard{
    pub fn to_grade(&self) -> ReportGradeCard{
        let mut res = ReportGradeCard{student_name:self.student_name.clone(),student_age:self.student_age,grade:String::new()};
        if self.grade < 1.5{
            res.grade = String::from("A+")
        }
        res
    }
}

pub struct ReportGradeCard{
    pub grade: String,
    pub student_name: String,
    pub student_age: u8,
}

impl Card for ReportGradeCard {
    fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, &self.grade)
    }
}

fn print(card :&impl Card) -> String{
    card.print()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            print(&report_card),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: 1.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            print(&report_card.to_grade()),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
