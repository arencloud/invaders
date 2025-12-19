use crate::frame::Frame;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::Write;

pub fn render<W: Write>(stdout: &mut W, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Green)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    let mut current_fg: Option<Color> = None;
    let mut wrote = false;
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                let desired = Some(color_for_cell(s));
                if desired != current_fg {
                    if let Some(color) = desired {
                        stdout.queue(SetForegroundColor(color)).unwrap();
                    }
                    current_fg = desired;
                }
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                write!(stdout, "{}", *s).unwrap();
                wrote = true;
            }
        }
    }
    if wrote {
        stdout.queue(ResetColor).unwrap();
    }
    stdout.flush().unwrap();
}

fn color_for_cell(cell: &str) -> Color {
    let ch = cell.chars().next().unwrap_or(' ');
    match ch {
        'M' | 'W' => Color::Green,
        '|' | '!' | '*' => Color::Red,
        '🪃' => Color::Cyan,
        'X' | '+' => Color::Yellow,
        'S' | 'c' | 'o' | 'r' | 'e' | 'H' | 'i' | 'g' | 'h' | '0'..='9' => Color::Yellow,
        '.' | '\'' => Color::DarkGrey,
        '[' | ']' | '-' | '=' => Color::Grey,
        _ if ch.is_ascii_alphabetic() || ch.is_ascii_digit() => Color::Yellow,
        _ => Color::Grey,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::new_frame;
    use std::io::Cursor;

    #[test]
    fn render_writes_differences() {
        let last = new_frame();
        let mut curr = new_frame();
        curr[0][0] = "X";

        let mut buf = Cursor::new(Vec::new());
        render(&mut buf, &last, &curr, true);

        let output = String::from_utf8(buf.into_inner()).unwrap();
        assert!(
            output.contains('X'),
            "render output missing expected glyph: {output:?}"
        );
    }

    #[test]
    fn render_no_changes_no_output() {
        let frame = new_frame();
        let mut buf = Cursor::new(Vec::new());
        render(&mut buf, &frame, &frame, false);
        assert!(
            buf.get_ref().is_empty(),
            "render without changes should not write"
        );
    }
}
