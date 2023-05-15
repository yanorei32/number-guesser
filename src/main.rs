use crate::{
    answerer::Answerer,
    host::{GameState, Host},
    questioner::Questioner,
};

mod answerer;
mod host;
mod questioner;

fn main() {
    let host: Host<u128>  = Host::new(Questioner::new(), Answerer::new());

    for (n, state) in host.enumerate() {
        let n = n + 1;

        match state {
            GameState::Continue(guess, judge) => {
                println!("Step {n}: Guess: {guess} Judge: {judge:?}")
            }
            GameState::Correct(correct) => println!("Step {n}: Game Set! Answer is {correct}"),
        }
    }
}
