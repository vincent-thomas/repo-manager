pub mod editor;
pub mod search;
pub mod srcs;

#[derive(Debug)]
pub enum HealthSeverity {
  Warn,
  Error,
}

#[derive(Debug)]
pub struct HealthError {
  pub severity: HealthSeverity,
  pub messages: String,
}

pub trait CheckHealth {
  fn checkhealth(&self) -> Result<(), HealthError>;
}
