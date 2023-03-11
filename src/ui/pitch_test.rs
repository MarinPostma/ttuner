use std::fmt;

use rand::{seq::SliceRandom, thread_rng};

use crate::pitch::find_note;

use super::Ui;

const NOTES: &[&'static str] = &[
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "D#", "G", "G#",
];

const WIN_DEBUF_FRAME_COUNT: usize = 60;

pub struct PitchTestUi {
    current: &'static str,
    target_note: &'static str,
    score: usize,
    win_timeout: usize,
}

fn pick_random_note() -> &'static str {
    let mut rng = thread_rng();
    NOTES.choose(&mut rng).unwrap()
}

impl PitchTestUi {
    pub fn new() -> Self {
        Self {
            current: "",
            target_note: pick_random_note(),
            score: 0,
            win_timeout: 0,
        }
    }

    fn is_win(&self) -> bool {
        self.current.starts_with(self.target_note)
    }
}

impl Ui for PitchTestUi {
    fn update(&mut self, freq: f32) {
        if self.is_win() {
            if self.win_timeout < WIN_DEBUF_FRAME_COUNT {
                self.win_timeout += 1;
                return;
            } else {
                self.score += 1;
                self.win_timeout = 0;
                self.target_note = pick_random_note();
                return;
            }
        }

        self.current = find_note(freq).0;
    }
}

impl fmt::Display for PitchTestUi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\r{}", termion::clear::CurrentLine)?;
        write!(
            f,
            "target_note: {:^4} | current: {:^4} | score: {:<4}",
            self.target_note, self.current, self.score
        )?;

        Ok(())
    }
}
