mod escape;

use crate::note::{AbsNote, Note};
use crate::ui::escape::{Color, EscapeCode};

/// Stores the state of the terminal's UI, such as number of lines scrolled, terminal width, etc.
#[derive(Default)]
pub struct Ui {}

const DOT_NORMAL: char = '•';
const DOT_EQUAL: char = '•';

impl Ui {
    /// Initializes the UI
    pub fn new() -> Ui {
        print!("{}", EscapeCode::HideCursor);
        Ui {}
    }

    pub fn show(&mut self, note: Option<Note>) {
        print!("\r");
        match note {
            Some(note) => {
                let note = note.abs_note();
                let (left, right) = self.get_dots(&note);

                print!("{}{}{}", left, self.get_note_text(&note), right);
            }
            None => print!("• • • • • ---- • • • • •"),
        }
    }

    /// Returns the color used for displaying the note
    fn get_note_color(&self, note: &AbsNote) -> Color {
        let dot = get_highlighted_dot(note);

        if dot == 10 {
            Color::Green
        } else if dot >= 5 && dot <= 15 {
            Color::Yellow
        } else {
            Color::Red
        }
    }

    /// Returns the text for the tuning interface, for example, "»E «" for an in-tune E note.
    /// This already has the color codes.
    fn get_note_text(&self, note: &AbsNote) -> String {
        let color = self.get_note_color(note);
        let dot = get_highlighted_dot(note);

        let left_arr = if dot <= 10 { '»' } else { ' ' };
        let right_arr = if dot >= 10 { '«' } else { ' ' };

        format!(
            "{}{}{:<2}{}{}",
            left_arr,
            color.fg(),
            note.name(),
            EscapeCode::Reset,
            right_arr,
        )
    }

    fn get_dots(&self, note: &AbsNote) -> (String, String) {
        let mut left_str = String::new();
        let mut right_str = String::new();
        let dot = get_highlighted_dot(note);
        let color = self.get_note_color(note);

        // Ugly messy code please look away
        for i in 0..21 {
            if i < 10 {
                if dot != i {
                    if i % 2 == 0 {
                        left_str.push(DOT_NORMAL);
                    } else {
                        left_str.push(' ');
                    }
                } else {
                    left_str.push_str(&format!("{}{}{}", color.fg(), DOT_EQUAL, EscapeCode::Reset));
                }
            } else if i > 10 {
                if dot != i {
                    if i % 2 == 0 {
                        right_str.push(DOT_NORMAL);
                    } else {
                        right_str.push(' ');
                    }
                } else {
                    right_str.push_str(&format!(
                        "{}{}{}",
                        color.fg(),
                        DOT_EQUAL,
                        EscapeCode::Reset
                    ));
                }
            }
        }

        (left_str, right_str)
    }

    pub fn enable_cursor() {
        print!("\n{}", EscapeCode::ShowCursor);
    }
}

/// Gets the dot to highlight. The result will be a number from 0 to 20,
/// which will indicate the position in which to draw the dot.
///
/// 0 is too flat, 10 is just right and 20 is too sharp.
fn get_highlighted_dot(note: &AbsNote) -> i32 {
    ((note.cents + 50) as f32 / 5.0).round() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note::AbsNote as AN;

    #[test]
    fn check_highlighted_dot() {
        let note = AN::new(2, 1, 0, 440.0);
        assert_eq!(get_highlighted_dot(&note), 10);

        let note = AN::new(2, 1, -50, 440.0);
        assert_eq!(get_highlighted_dot(&note), 0);

        let note = AN::new(2, 1, 50, 440.0);
        assert_eq!(get_highlighted_dot(&note), 20);

        let note = AN::new(2, 1, -2, 440.0);
        assert_eq!(get_highlighted_dot(&note), 10);
    }
}
