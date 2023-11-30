use rand::Rng;
use crate::game_rules::entities::{game::Game, player::Player, tetromino::Tetromino};


pub fn game_over(current_game: Game, player: Player) -> (Player, Game)
{

    let mut player_info_updated = Player {
        high_score: player.high_score,
    };
    
    let mut current_game_status = current_game;


    current_game_status.game_over = true;
    player_info_updated = update_score(player, &current_game);

    (player_info_updated, current_game_status)
}

pub fn update_score(mut player: Player, current_game: &Game) -> Player
{
    if current_game.current_score > player.high_score {
        player.high_score = current_game.current_score;
    }
    player
}

pub fn form_line(mut current_game: Game) -> Game
{
    current_game.current_score += 120;

    current_game
}


pub fn generate_random_tetromino() -> Tetromino {
    let mut rng = rand::thread_rng();
    let random_8_bit_number: u8 = rng.gen();
    let tetromino = Tetromino {
        tetromino_type: random_8_bit_number,
    };
    tetromino
}

#[inline(always)]
pub fn new_session() -> (Player, Game)
{
    let player = Player{
        high_score : 0
    };

    let game = Game{
        game_over : false,
        next_piece : generate_random_tetromino(),
        current_piece : generate_random_tetromino(),
        current_score : 0
    };

    (player, game)
}

