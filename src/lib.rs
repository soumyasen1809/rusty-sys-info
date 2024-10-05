pub mod cpu;
pub mod disk;
pub mod memory;
pub mod socket;

pub trait Measurements: Send + Sync {
    fn print_info(&self) {}
}
