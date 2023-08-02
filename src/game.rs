use crate::fp_vec::FpVec;
use crate::player::PlayerCard;
use crate::{enemy::Enemy, player::Player, EffectTarget, GameEffect};

#[derive(Debug, Clone, PartialEq)]
pub enum GameOutcome {
    Undecided,
    PlayerWins(u32),
    EnemyWins(u32),
}

fn fold_effects((enemy, player): (Enemy, Player), effect: GameEffect) -> (Enemy, Player) {
    match effect.target {
        EffectTarget::Player => {
            let player = player.trigger_effect(effect.effect, &enemy);
            (enemy, player)
        }
        EffectTarget::Enemy => {
            let enemy = enemy.trigger_effect(effect.effect, &player);
            (enemy, player)
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub enemy: Enemy,
    pub player: Player,
    pub turn_number: u32,
    pub game_result: GameOutcome,
}

impl Game {
    pub fn start(enemy: Enemy, player: Player) -> Self {
        Self {
            player,
            enemy,
            turn_number: 1,
            game_result: GameOutcome::Undecided,
        }
    }

    pub fn take_player_turn(self, card_play_list: FpVec<PlayerCard>) -> Self {
        let enemy = self.enemy;
        let player = self.player;

        let effects = player
            .start_turn()
            .extend(
                card_play_list
                    .inner
                    .into_iter()
                    .fold(FpVec::from_vec(vec![]), |effects, card| {
                        effects.extend(player.player_play_card(card))
                    }),
            )
            .extend(player.end_turn());

        let (enemy, player) = effects
            .inner
            .into_iter()
            .fold((enemy, player), fold_effects);

        let game_result = match 0 {
            _ if player.hit_points <= 0 => GameOutcome::EnemyWins(self.turn_number),
            _ if enemy.hit_points <= 0 => GameOutcome::PlayerWins(self.turn_number),
            _ => GameOutcome::Undecided,
        };

        Self {
            enemy,
            player,
            game_result,
            ..self
        }
    }

    pub fn take_enemy_turn(self) -> Self {
        let enemy = self.enemy;
        let player = self.player;

        let effects = enemy.start_turn(&player).extend(enemy.end_turn(&player));

        let (enemy, player) = effects
            .inner
            .into_iter()
            .fold((enemy, player), fold_effects);

        let game_result = match 0 {
            _ if player.hit_points <= 0 => GameOutcome::EnemyWins(self.turn_number),
            _ if enemy.hit_points <= 0 => GameOutcome::PlayerWins(self.turn_number),
            _ => GameOutcome::Undecided,
        };

        Self {
            enemy,
            player,
            game_result,
            turn_number: self.turn_number + 1,
        }
    }

    pub fn log_state(self) -> Self {
        println!("State of game: {:?}", &self);
        self
    }
}

pub fn game_turn(game: Game, card_plays: FpVec<PlayerCard>) -> Game {
    let new_game = if game.game_result == GameOutcome::Undecided {
        game.take_player_turn(card_plays)
    } else {
        game
    };
    if new_game.game_result == GameOutcome::Undecided {
        new_game.take_enemy_turn()
    } else {
        new_game
    }
    .log_state()
}
