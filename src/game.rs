use crate::fp_vec::FpVec;
use crate::player::PlayerCard;
use crate::{enemy::Enemy, player::Player, EffectTarget, Enchantment, GameEffect};

#[derive(Debug, Clone, PartialEq)]
pub enum GameOutcome {
    Undecided,
    PlayerWins(u32),
    EnemyWins(u32),
}

fn fold_effects((enemy, player): (Enemy, Player), effect: GameEffect) -> (Enemy, Player) {
    println!("Checking game effect: {}", effect.name);
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
    pub power_pool: i32,
    pub turn_number: u32,
    pub game_result: GameOutcome,
}

impl Game {
    pub fn start(enemy: Enemy, player: Player) -> Self {
        let player = player.player_enchantments();
        Self {
            player,
            enemy,
            turn_number: 1,
            power_pool: 0,
            game_result: GameOutcome::Undecided,
        }
    }

    pub fn take_player_turn(self, card_play_list: FpVec<PlayerCard>) -> Self {
        let enemy = self.enemy;
        let player = self.player;

        let start_effects = player.start_turn();
        let curr_power_pool = self.power_pool
            + 3 // standard power up
            + player
                .current_activated_effects
                .inner
                .iter()
                .fold(0, |power_add, card| {
                    if let Enchantment::PowerAddPerTurn(amt) = card {
                        power_add + amt
                    } else {
                        power_add
                    }
                });

        let (res_pool, play_effects) = card_play_list.inner.into_iter().fold(
            (curr_power_pool, FpVec::new()),
            |(current_pool, effects), card| {
                let (new_effects, power_cost) = player.player_play_card(card, current_pool);
                (current_pool - power_cost, effects.extend(new_effects))
            },
        );
        let effects = start_effects.extend(play_effects).extend(player.end_turn());

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
            power_pool: res_pool,
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
            ..self
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
