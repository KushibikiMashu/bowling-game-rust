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
        let mut game = Game::new();
        for _ in 0..20 {
            game.roll(0);
        }
        assert_eq!(0, game.score)
    }
}