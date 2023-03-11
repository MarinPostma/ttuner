use std::fmt::{self, Write};

use crate::pitch::{cents_between_freqs, find_note};

use super::Ui;

pub struct TunerUi(pub f32);

impl fmt::Display for TunerUi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let freq = self.0;
        if freq == 0.0 {
            return Ok(());
        }
        let (note, fnote) = find_note(freq);
        let cents = cents_between_freqs(fnote, freq);
        let steps = (cents / 2.0).abs().floor() as usize;

        let pick_color = |f: &mut fmt::Formatter| -> fmt::Result {
            let dist = cents.abs();
            if dist >= 0.0 && dist < 10.0 {
                write!(f, "{}", termion::color::Fg(termion::color::Green))?
            } else if dist >= 10.0 && dist < 30.0 {
                write!(f, "{}", termion::color::Fg(termion::color::Yellow))?
            } else {
                write!(f, "{}", termion::color::Fg(termion::color::Red))?
            }

            Ok(())
        };

        write!(f, "\r{}", termion::clear::CurrentLine)?;
        // draw left
        let bar = ".........................";
        if cents.is_sign_negative() {
            let (left, right) = bar.split_at(steps);
            f.write_str(right)?;
            pick_color(f)?;
            f.write_char('v')?;
            write!(f, "{}", termion::color::Fg(termion::color::Reset))?;
            f.write_str(left)?;
        } else {
            f.write_str(bar)?;
        }

        pick_color(f)?;
        write!(f, "{note:^5}")?;
        write!(f, "{}", termion::color::Fg(termion::color::Reset))?;

        // draw right
        if cents.is_sign_positive() {
            let (left, right) = bar.split_at(steps);
            f.write_str(left)?;
            pick_color(f)?;
            f.write_char('v')?;
            write!(f, "{}", termion::color::Fg(termion::color::Reset))?;
            f.write_str(right)?;
        } else {
            f.write_str(bar)?;
        };

        write!(f, " {cents:3.0} cents")?;

        Ok(())
    }
}

impl Ui for TunerUi {
    fn update(&mut self, freq: f32) {
        self.0 = freq;
    }
}
