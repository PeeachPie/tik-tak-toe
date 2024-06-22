use crate::api::game;

pub struct Settings {
    pub mode: game::Mode,
    pub bot_player: u8,
    pub bot_lvl: u8,
}

impl Settings {
    pub fn bot_game(player_order: u8, bot_lvl: u8) -> Settings {
        if player_order > 2 || player_order < 1 {
            panic!("Player order should be in range from 1 to 2, got {player_order}");
        }
        if bot_lvl >= 10 {
            panic!("Bot lvl should be in range from 0 to 9, got {bot_lvl}");
        }
        Settings {
            mode: game::Mode::OnePlayer,
            bot_player: player_order % 2 + 1,
            bot_lvl: bot_lvl,
        }
    }

    pub fn two_player_game() -> Settings {
        Settings {
            mode: game::Mode::TwoPlayers,
            ..Self::default()
        }
    }

    pub fn default() -> Settings {
        Settings {
            mode: game::Mode::OnePlayer,
            bot_player: 2,
            bot_lvl: 0,
        }
    }
}
