mod games;
use games::guessing_game::run::run as guessing_game_runner;

fn main() {
    guessing_game_runner(3, false);
}
