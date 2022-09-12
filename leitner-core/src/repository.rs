use std::fmt;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::{Fact, scheduler};

pub trait FactRepository {
    fn get_created_date(&self) -> DateTime<Utc>;

    // store
    fn add_fact(&mut self, fact: Fact);

    // retrieve
    fn get_fact_count(&self) -> usize;
    fn get_all_facts(&self) -> Vec<Fact>;
    // fn get_ready_facts(&self) -> Vec<Fact>;
    // fn get_next_fact(&mut self) -> Fact;
    // fn resolve_fact(&mut self, fact: Fact, answer: bool);

    // delete
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InMemoryFactRepository {
    created_at: DateTime<Utc>,
    pub facts: Vec<Fact>,
}

impl InMemoryFactRepository {
    pub fn new() -> InMemoryFactRepository {
        InMemoryFactRepository {
            created_at: Utc::now(),
            facts: Vec::new(),
        }
    }
}

impl FactRepository for InMemoryFactRepository {
    fn get_created_date(&self) -> DateTime<Utc> {
        self.created_at
    }
    fn add_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }
    fn get_fact_count(&self) -> usize { self.facts.len() }
    fn get_all_facts(&self) -> Vec<Fact> {
        self.facts.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::{FactRepository, InMemoryFactRepository};

    #[test]
    fn empty_repository_is_empty() {
        let repository = InMemoryFactRepository::new();
        assert_eq!(repository.facts.len(), 0)
    }

    #[test]
    fn repository_stores_facts() {
        let mut repository = InMemoryFactRepository::new();
        repository.add_fact(Fact::new("What is the capital of France?".to_string(), "Paris".to_string()));
        assert_eq!(repository.facts.len(), 1)
    }

    #[test]
    fn repository_with_stored_facts_peeks_all_facts() {
        let mut repository = InMemoryFactRepository::new();
        repository.add_fact(Fact::new("Fact 1".to_string(), "Answer 1".to_string()));
        repository.add_fact(Fact::new("Fact 2".to_string(), "Answer 2".to_string()));
        repository.add_fact(Fact::new("Fact 3".to_string(), "Answer 2".to_string()));

        let all_facts = repository.get_all_facts();
        assert_eq!(all_facts[0].question, "Fact 1");
        assert_eq!(all_facts[1].question, "Fact 2");
        assert_eq!(all_facts[2].question, "Fact 3");
    }

}