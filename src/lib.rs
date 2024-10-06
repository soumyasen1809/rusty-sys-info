use std::any::Any;
use std::fmt::Debug;
use std::fmt::Display;

pub mod sys_stats;
pub mod ui;

pub trait Measurements: Send + Sync + Debug + Display {
    fn print_info(&self);
    fn as_any(&self) -> &dyn Any; // The downcast_ref method is available for Any,
                                  // but Box<dyn Measurements> doesn’t directly support it.
                                  // Instead, you can use the Any trait to perform type checking and downcasting.
}
