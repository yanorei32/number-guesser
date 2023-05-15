use num_traits::{Unsigned, PrimInt};
use min_max_traits::{Max, Min};
use std::cmp::Ordering;

pub struct Answerer<T>
where
    T: Min + Max + Unsigned + PrimInt,
{
    min: T,
    max: T,
}

impl<T> Answerer<T>
where
    T: Min + Max + Unsigned + PrimInt,
{
    pub fn new() -> Self {
        Self {
            min: T::MIN,
            max: T::MAX,
        }
    }

    pub fn guess(&self) -> T {
        return self.min + (self.max - self.min).unsigned_shr(1);
    }

    pub fn accept(&mut self, o: Ordering) {
        match o {
            Ordering::Less => self.max = self.guess(),
            Ordering::Greater => self.min = self.guess(),
            Ordering::Equal => unreachable!(),
        }
    }
}
