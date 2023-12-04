
mod tests {
    use crate::game_rules::{tetrix_rules::tetrix_rules::{new_session, game_over, form_line, generate_random_tetromino}, entities::{player::Player, game::Game}};
    fn tetris_core_test_startup() -> (Player, Game)
    {
        let new_session = new_session();
        new_session
    }

    #[test]
    fn new_session_test() {
        let new_session = tetris_core_test_startup();
        let expected_session = 
        (Player{
            high_score: 0,
        }, Game{
            game_over: false,
            next_piece: new_session.1.next_piece,
            current_piece: new_session.1.current_piece,
            current_score: 0,
        });
        assert_eq!(new_session, expected_session);
        
    }
    #[test]
    fn game_over_test() {
        let mut session = tetris_core_test_startup();

        (session.0, session.1) = game_over(session.1, session.0);
        
        assert_eq!(session.1.game_over, true)
    }

    #[test]
    fn line_formed(){
        let mut session = tetris_core_test_startup();
        
        session.1 = form_line(session.1);
        
        let expected_score = 120;
        assert_eq!(session.1.current_score, expected_score);

    }
    #[test]
    fn new_high_score() {
        let mut session = tetris_core_test_startup();
        session.1 = form_line(session.1);
        session.1 = form_line(session.1);

        (session.0, session.1) = game_over(session.1, session.0);

        let expected_high_score = 240;
        assert_eq!(session.0.high_score, expected_high_score);
        
    }
    #[test]
    fn generate_tetromino_test()
    {
        let tetromino = generate_random_tetromino();

        let tetromino_expected_integers = vec![1, 2, 3, 4, 5, 6, 7];

        let tetromino_contains_in_expected = tetromino_expected_integers.contains(&tetromino.tetromino_type);

        assert_eq!(tetromino_contains_in_expected, true);
        
    }
}