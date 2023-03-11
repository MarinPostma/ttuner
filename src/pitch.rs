use crate::notes::NOTES;

pub fn find_note(freq: f32) -> (&'static str, f32) {
    let mut best_dist = f32::MAX;
    for (i, (_, f)) in NOTES.iter().enumerate() {
        let dist = (*f - freq).powi(2);
        if dist < best_dist {
            best_dist = dist;
        } else if i != 0 {
            return NOTES[i - 1];
        }
    }

    panic!("unknown note! freq: {freq}Hz");
}

pub fn cents_between_freqs(f1: f32, f2: f32) -> f32 {
    1200.0 * (f2 / f1).log2()
}
