// all gutter
// all one pin
// one spare
// one strike
// all strike

#[derive(Default)]
struct Game {
    rolls: [i32; 21],
    current_roll: usize,
}

impl Game {
    fn new() -> Self {
        Game { rolls: [0; 21], current_roll: 0 }
    }

    fn roll(&mut self, pins: i32) -> &mut Self {
        self.rolls[self.current_roll] = pins;
        self.current_roll += 1;
        self
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        let mut frame_index = 0;
        for _ in 0..10 {
            if self.is_spare(frame_index) {
                score += 10 + self.rolls[frame_index + 2];
                frame_index += 2;
            } else {
                score += self.rolls[frame_index] + self.rolls[frame_index + 1];
                frame_index += 2;
            }
        }
        score
    }

    fn is_spare(&self, i: usize) -> bool {
        self.rolls[i] + self.rolls[i + 1] == 10
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
        spare(&mut game);
        game.roll(3);
        many_rolls(&mut game, 0, 17);
        assert_eq!(16, game.score())
    }

    #[test]
    fn one_strike() {
        let mut game = start();
        game.roll(10);
        game.roll(4);
        game.roll(3);
        many_rolls(&mut game, 0, 16);
        assert_eq!(24, game.score());
    }

    fn start() -> Game {
        Game::new()
    }

    fn many_rolls(game: &mut Game, pins: i32, n: i32) {
        for _ in 0..n {
            game.roll(pins);
        }
    }

    fn spare(game: &mut Game) {
        game.roll(5);
        game.roll(5);
    }
}