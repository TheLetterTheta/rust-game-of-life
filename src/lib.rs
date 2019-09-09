pub mod game;

#[cfg(test)]
mod tests {
    use super::game::{Game, Cell};
    
    #[test]
    #[should_panic]
    fn it_does_not_generate_1_by_x_boards() {
        let _ = Game::new(1, 1000);
    }

    #[test]
    #[should_panic]
    fn it_does_not_generate_x_by_1_boards() {
        let _ = Game::new(1000, 1);
    }

    #[test]
    fn it_generates_gameboard() {
        let g = Game::new(1000,1000);   
        assert_eq!(1000, g.board.len());
        assert_eq!(1000, g.board[0].len());
        assert_eq!(1000, g.board[999].len());
    }

    #[test]
    fn it_initializes_gameboard() {
        let _ = Game::from_board(vec![
            vec![Cell::Alive, Cell::Alive],
            vec![Cell::Dead, Cell::Alive]
        ]);
    }

    #[test]
    fn it_calculates_rule_one() {
        let mut g = Game::from_board(vec![
            vec![Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Dead]
        ]);

        g.next();

        assert_eq!(g.board, vec![
            vec![Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead]
        ]);

        let mut g = Game::from_board(vec![
            vec![Cell::Alive, Cell::Alive],
            vec![Cell::Dead, Cell::Dead]
        ]);

        g.next();

        assert_eq!(g.board, vec![
            vec![Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead]
        ]);
    }

    #[test]
    fn it_calculates_rule_two() {
        let mut g = Game::from_board(vec![
            vec![Cell::Alive, Cell::Alive],
            vec![Cell::Alive, Cell::Alive]
        ]);

        g.next();

        assert_eq!(g.board, vec![
            vec![Cell::Alive, Cell::Alive],
            vec![Cell::Alive, Cell::Alive]
        ]);

        let mut g = Game::from_board(vec![
            vec![Cell::Dead, Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Alive, Cell::Dead, Cell::Dead, Cell::Alive],
            vec![Cell::Dead, Cell::Alive, Cell::Alive, Cell::Dead]
        ]);
        
        g.next();

        assert_eq!(g.board, vec![
            vec![Cell::Dead, Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Alive, Cell::Dead, Cell::Dead, Cell::Alive],
            vec![Cell::Dead, Cell::Alive, Cell::Alive, Cell::Dead]
        ]);
    }

    #[test]
    fn it_calculates_rule_three() {
        let mut g = Game::from_board(vec![
            vec![Cell::Alive, Cell::Dead, Cell::Alive],
            vec![Cell::Alive, Cell::Alive, Cell::Alive],
            vec![Cell::Dead, Cell::Alive, Cell::Dead]
        ]);

        g.next();

        assert_eq!(g.board, vec![
            vec![Cell::Alive, Cell::Dead, Cell::Alive],
            vec![Cell::Alive, Cell::Dead, Cell::Alive],
            vec![Cell::Alive, Cell::Alive, Cell::Alive]
        ]);
    }    

    #[test]
    fn it_iterates_gameboard() {
        let mut g = Game::new(10, 10);
        for _ in 0..1000 {
            g.next();
        }
    }

    #[test]
    fn it_prints_gameboards() {
        let g = Game::new(100, 100);
        print!("{}", g);
    }
}