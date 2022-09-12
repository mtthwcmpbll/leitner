mod scheduler;
mod repository;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    pub question: String,
    pub answer: String,
    level: u8,
}

///
/// This trait can be assigned to anything that can be scheduled vie spaced repitition.  It presents
/// broadly a numeric level that can be increased, demoted back to the first level, and queries for
/// it's current level.
///
trait SpacedRepeatable {
    fn get_level(&self) -> u8;
    fn set_level(&mut self, level: u8);

    /// The default implementation calls the set_level method to increment by 1.
    fn increase_level(&mut self) {
        self.set_level(self.get_level() + 1);
    }

    /// The default implementation for demote is to set the level back to 1.
    fn demote(&mut self) {
        self.set_level(1);
    }
}

impl SpacedRepeatable for Fact {
    fn get_level(&self) -> u8 {
        self.level
    }

    fn set_level(&mut self, level: u8) {
        self.level = level;
    }
}

impl Fact {
    pub fn new(question: String, answer: String) -> Fact {
        Fact {
            question,
            answer,
            level: 1,
        }
    }

    fn with_level(mut self, level: u8) -> Fact {
        self.set_level(level);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}
