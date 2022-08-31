use std::time::SystemTime;
use serde::{Serialize, Deserialize};


pub trait FactRepository {
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
    pub facts: Vec<Fact>,
}

impl InMemoryFactRepository {
    pub fn new() -> InMemoryFactRepository {
        InMemoryFactRepository {
            facts: Vec::new(),
        }
    }
}

impl FactRepository for InMemoryFactRepository {
    fn add_fact(&mut self, fact: Fact) {
        self.facts.push(fact);
    }
    fn get_fact_count(&self) -> usize { self.facts.len() }
    fn get_all_facts(&self) -> Vec<Fact> {
        self.facts.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    pub question: String,
    pub answer: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl Fact {
    pub fn new(question: String, answer: String) -> Fact {
        Fact {
            question,
            answer,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

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
