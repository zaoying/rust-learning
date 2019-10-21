mod sort;
mod guessing_game;

use sort::sort::sort;
use guessing_game::guessing_game::exec;

fn main() {
    sort();
    exec();
}