// all gutter
// all one pin
// one spare
// one strike
// all strike

#[derive(Default)]
struct Game {
    rolls: [i32; 20],
    current_roll: usize,
}

impl Game {
    fn new() -> Self {
        Game { rolls: [0; 20], current_roll: 0 }
    }

    fn roll(&mut self, pins: i32) -> &mut Self {
        self.rolls[self.current_roll] = pins;
        self.current_roll += 1;
        self
    }

    fn score(&self) -> i32 {
        self.rolls.iter().sum()
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
        assert_eq!(0, game.score())
    }

    #[test]
    fn all_one_pin() {
        let mut game = start();
        many_rolls(&mut game, 1, 20);
        assert_eq!(20, game.score())
    }

    #[test]
    fn one_spare() {
        let mut game = start();
        game.roll(5);
        game.roll(5);
        game.roll(3);
        many_rolls(&mut game, 0, 17);
        assert_eq!(16, game.score())
    }

    fn start() -> Game {
        Game::new()
    }

    fn many_rolls(game: &mut Game, pins: i32, n: i32) {
        for _ in 0..n {
            game.roll(pins);
        }
    }
}