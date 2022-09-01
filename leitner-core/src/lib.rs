use std::fmt;
use std::fmt::Display;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

fn get_leitner_levels_for_dates(start: DateTime<Utc>, day: DateTime<Utc>) -> Vec<u8> {
    let day = day.signed_duration_since(start).num_days() as u8;
    get_leitner_levels_for_day(day % 64)
}

fn get_leitner_levels_for_day(day: u8) -> Vec<u8> {
    return match day % 64 {
        0 => vec![2, 1],
        1 => vec![3, 1],
        2 => vec![2, 1],
        3 => vec![4, 1],
        4 => vec![2, 1],
        5 => vec![3, 1],
        6 => vec![2, 1],
        7 => vec![1],
        8 => vec![2, 1],
        9 => vec![3, 1],
        10 => vec![2, 1],
        11 => vec![5, 1],
        12 => vec![4, 2, 1],
        13 => vec![3, 1],
        14 => vec![2, 1],
        15 => vec![1],
        16 => vec![2, 1],
        17 => vec![3, 1],
        18 => vec![2, 1],
        19 => vec![4, 1],
        20 => vec![2, 1],
        21 => vec![3, 1],
        22 => vec![2, 1],
        23 => vec![6, 1],
        24 => vec![2, 1],
        25 => vec![3, 1],
        26 => vec![2, 1],
        27 => vec![5, 1],
        28 => vec![4, 2, 1],
        29 => vec![3, 1],
        30 => vec![2, 1],
        31 => vec![1],
        32 => vec![2, 1],
        33 => vec![3, 1],
        34 => vec![2, 1],
        35 => vec![4, 1],
        36 => vec![2, 1],
        37 => vec![3, 1],
        38 => vec![2, 1],
        39 => vec![1],
        40 => vec![2, 1],
        41 => vec![3, 1],
        42 => vec![2, 1],
        43 => vec![5, 1],
        44 => vec![4, 2, 1],
        45 => vec![3, 1],
        46 => vec![2, 1],
        47 => vec![1],
        48 => vec![2, 1],
        49 => vec![3, 1],
        50 => vec![2, 1],
        51 => vec![4, 1],
        52 => vec![2, 1],
        53 => vec![3, 1],
        54 => vec![2, 1],
        55 => vec![7, 1],
        56 => vec![2, 1],
        57 => vec![3, 1],
        58 => vec![6, 2, 1],
        59 => vec![5, 1],
        60 => vec![4, 2, 1],
        61 => vec![3, 1],
        62 => vec![2, 1],
        63 => vec![1],
        _ => unreachable!(),
    }
}

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

impl fmt::Display for InMemoryFactRepository {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " day | levels\n")?;
        write!(f, "-----|--------\n")?;
        for day in 0..=63 {
            // let facts = self.facts.iter().filter(|fact| fact.day == day);
            // let count = facts.count();
            write!(f, "{:>4} | ", day)?;
            let levels_for_day = get_leitner_levels_for_day(day);
            for level in 1..=7 {
                write!(f, "{} ", if levels_for_day.contains(&level) { level.to_string() } else { " ".to_string() } )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    pub question: String,
    pub answer: String,
    level: u8,
}

trait SpacedRepeatable {
    fn get_level(&self) -> u8;
    fn set_level(&mut self, level: u8);

    fn increase_level(&mut self) {
        self.set_level(self.get_level() + 1);
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

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

    #[test]
    fn schedule_with_one_card_shows_full_schedule() {
        // given
        let start_date = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11); // `2014-07-08T09:10:11Z`

        // day 1
        assert_eq!(get_leitner_levels_for_dates(start_date, start_date + chrono::Duration::days(0)),
                   vec![2, 1]);

        // day 2
        assert_eq!(get_leitner_levels_for_dates(start_date, start_date + chrono::Duration::days(1)),
                   vec![3, 1]);

        // day 13
        assert_eq!(get_leitner_levels_for_dates(start_date, start_date + chrono::Duration::days(12)),
                   vec![4, 2, 1]);

        // day 28
        assert_eq!(get_leitner_levels_for_dates(start_date, start_date + chrono::Duration::days(27)),
                   vec![5, 1]);
    }
}
