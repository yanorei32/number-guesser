use std::cmp::Ordering;

use num_traits::{Unsigned, PrimInt};
use min_max_traits::{Max, Min};
use rand::distributions::{Distribution, Standard};

use crate::{questioner::Questioner, answerer::Answerer};

#[derive(Copy, Clone)]
pub enum GameState<T> {
    Continue(T, Ordering),
    Correct(T),
}

pub struct Host<T> 
where
    T: Min + Max + Unsigned + PrimInt + Ord,
    Standard: Distribution<T>,
{
    questioner: Questioner<T>,
    answerer: Answerer<T>,
    state: Option<GameState<T>>,
}

impl<T> Host<T>
where
    T: Min + Max + Unsigned + PrimInt + Ord,
    Standard: Distribution<T>,
{
    pub fn new(questioner: Questioner<T>, answerer: Answerer<T>) -> Self {
        Self {
            questioner,
            answerer,
            state: None,
        }
    }
}

impl<T> Iterator for Host<T>
where
    T: Min + Max + Unsigned + PrimInt + Ord,
    Standard: Distribution<T>,
{
    type Item = GameState<T>;

    fn next(&mut self) -> Option<GameState<T>> {
        if let Some(GameState::Correct(_)) = self.state {
            return None;
        }

        let guess = self.answerer.guess();
        let judge = self.questioner.judge(guess);

        match judge {
            Ordering::Equal => {
                self.state = Some(GameState::Correct(guess));
            },
            _ => {
                self.answerer.accept(judge);
                self.state = Some(GameState::Continue(guess, judge));
            }
        }

        Some(self.state.unwrap())
    }
}
