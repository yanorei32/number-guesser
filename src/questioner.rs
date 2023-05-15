use std::cmp::Ordering;

use rand::distributions::{Distribution, Standard};

pub struct Questioner<T>
where
    T: Ord,
    Standard: Distribution<T>,
{
    answer: T,
}

impl<T> Questioner<T>
where
    T: Ord,
    Standard: Distribution<T>,
{
    pub fn new() -> Self {
        Self {
            answer: rand::random(),
        }
    }

    pub fn judge(&self, value: T) -> Ordering {
        self.answer.cmp(&value)
    }
}
