mod game;

fn main() {
    println!("Hello World");
}

mod tests {
    use super::game::{Game};

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
        strike(&mut game);
        game.roll(4);
        game.roll(3);
        many_rolls(&mut game, 0, 16);
        assert_eq!(24, game.score());
    }

    #[test]
    fn all_strike() {
        let mut game = start();
        many_rolls(&mut game, 10, 20);
        assert_eq!(300, game.score());
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

    fn strike(game: &mut Game) {
        game.roll(10);
    }
}