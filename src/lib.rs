use std::fmt::Debug;
use std::fmt::Display;

pub mod sys_stats;
pub mod ui;

pub trait Measurements: Send + Sync + Debug + Display {
    fn print_info(&self) {}
}
