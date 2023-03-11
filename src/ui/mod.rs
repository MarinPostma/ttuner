use std::fmt;

pub mod pitch_test;
pub mod tuner;

pub trait Ui: fmt::Display {
    fn update(&mut self, freq: f32);
}
