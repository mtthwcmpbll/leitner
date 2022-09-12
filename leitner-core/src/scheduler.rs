use std::fmt;
use chrono::{DateTime, Utc};

use crate::repository::InMemoryFactRepository;

pub trait SpacedSchedule {
    fn get_start_date(&self) -> DateTime<Utc>;

    fn get_day_of_schedule(&self, day: DateTime<Utc>) -> u8 {
        let day = day.signed_duration_since(self.get_start_date()).num_days() as u8;
        day % self.len()
    }

    /// the number of days in this schedule
    fn len(&self) -> u8;

    /// How many levels are used in this schedule
    fn num_levels(&self) -> u8;

    /// Returns the levels for the given day, from highest to lowest.
    fn get_levels_for_day(&self, day: u8) -> Vec<u8>;

    fn get_levels_for_date(&self, day: DateTime<Utc>) -> Vec<u8> {
        self.get_levels_for_day(self.get_day_of_schedule(day) % self.len())
    }
}

pub struct LeitnerSchedule {
    start_date: DateTime<Utc>,
    schedule: Vec<Vec<u8>>,
}

impl LeitnerSchedule {
    pub fn new() -> LeitnerSchedule {
        LeitnerSchedule {
            start_date: Utc::now(),
            schedule: vec![
                vec![2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![4, 1],
                vec![2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1],
                vec![2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![5, 1],
                vec![4, 2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1],
                vec![2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![4, 1],
                vec![2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![6, 1],
                vec![2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![5, 1],
                vec![4, 2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1],
                vec![2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![4, 1],
                vec![2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1],
                vec![2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![5, 1],
                vec![4, 2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1],
                vec![2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![4, 1],
                vec![2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![7, 1],
                vec![2, 1],
                vec![3, 1],
                vec![6, 2, 1],
                vec![5, 1],
                vec![4, 2, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1],
            ],
        }
    }
}

impl SpacedSchedule for LeitnerSchedule {
    fn get_start_date(&self) -> DateTime<Utc> {
        self.start_date
    }

    fn len(&self) -> u8 {
        self.schedule.len() as u8
    }

    fn num_levels(&self) -> u8 {
        7
    }

    fn get_levels_for_day(&self, day: u8) -> Vec<u8> {
        return self.schedule[ (day % self.len()) as usize ]
    }
}

impl fmt::Display for LeitnerSchedule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " day | levels\n")?;
        write!(f, "-----|--------\n")?;
        for day in 0..=63 {
            // let facts = self.facts.iter().filter(|fact| fact.day == day);
            // let count = facts.count();
            write!(f, "{:>4} | ", day)?;
            let levels_for_day = self.get_levels_for_day(day);
            for level in 1..=7 {
                write!(f, "{} ", if levels_for_day.contains(&level) { level.to_string() } else { " ".to_string() } )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use crate::Fact;
    use crate::repository::FactRepository;

    #[test]
    fn schedule_with_one_card_shows_full_schedule() {
        let mut schedule = LeitnerSchedule::new();

        // given
        let start_date = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11); // `2014-07-08T09:10:11Z`
        schedule.start_date = start_date;

        // day 1
        assert_eq!(schedule.get_levels_for_date(start_date + chrono::Duration::days(0)),
                   vec![2, 1]);

        // day 2
        assert_eq!(schedule.get_levels_for_date(start_date + chrono::Duration::days(1)),
                   vec![3, 1]);

        // day 13
        assert_eq!(schedule.get_levels_for_date(start_date + chrono::Duration::days(12)),
                   vec![4, 2, 1]);

        // day 28
        assert_eq!(schedule.get_levels_for_date(start_date + chrono::Duration::days(27)),
                   vec![5, 1]);
    }

    #[test]
    fn get_todays_cards() {
        let mut repository = InMemoryFactRepository::new();
        repository.add_fact(Fact::new("Fact 1".to_string(), "Answer 1".to_string()).with_level(1));
        repository.add_fact(Fact::new("Fact 2".to_string(), "Answer 2".to_string()).with_level(2));
        repository.add_fact(Fact::new("Fact 3".to_string(), "Answer 2".to_string()).with_level(3));
    }

}