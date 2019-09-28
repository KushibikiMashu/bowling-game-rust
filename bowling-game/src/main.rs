// all gutter
// all one pin
// one spare
// one strike
// all strike

#[derive(Default)]
struct Game {
    score: u32
}

impl Game {
    fn new() -> Self {
        Game { score: 0 }
    }

    fn roll(&mut self, pins: u32) -> &mut Self {
        self.score += pins;
        self
    }
}

fn main() {
    println!("Hello World");
}

mod tests {
    use super::Game;

    #[test]
    fn all_gutter() {
        let mut game = start();
        many_rolls(&mut game, 0, 20);
        assert_eq!(0, game.score)
    }

    #[test]
    fn all_one_pin() {
        let mut game = start();
        many_rolls(&mut game, 1, 20);
        assert_eq!(20, game.score);
    }

    fn start() -> Game {
        Game::new()
    }

    fn many_rolls(game: &mut Game, pins: u32, n: u32) {
        for _ in 0..n {
            game.roll(pins);
        }
    }
}