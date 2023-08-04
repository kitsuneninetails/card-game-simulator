use crate::fp_vec::FpVec;
use crate::player::PlayerCard;
use crate::{
    enemy::Enemy, game_effects::GameEffect, player::Player, EffectTarget, EffectTrigger,
    EffectType, Enchantment,
};

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
        let player = player.player_enchantments();
        Self {
            player,
            enemy,
            turn_number: 1,
            game_result: GameOutcome::Undecided,
        }
    }

    pub fn check_game_result(enemy: &Enemy, player: &Player, turn_number: u32) -> GameOutcome {
        match 0 {
            _ if player.hit_points <= 0 => GameOutcome::EnemyWins(turn_number),
            _ if enemy.hit_points <= 0 => GameOutcome::PlayerWins(turn_number),
            _ => GameOutcome::Undecided,
        }
    }

    pub fn take_player_turn(self, card_play_list: FpVec<PlayerCard>) -> Self {
        let enemy = self.enemy;
        let player = self.player;

        let start_effects = player.start_turn();
        let player =
            player.trigger_effect(EffectTrigger::Always(EffectType::PowerAdjust(3)), &enemy);
        let curr_power_pool = player.power_reserve
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
                let (new_effects, power_cost) = player.player_play_card(&enemy, card, current_pool);
                (current_pool - power_cost, effects.extend(new_effects))
            },
        );
        let effects = start_effects.extend(play_effects).extend(player.end_turn());

        let (enemy, player) = effects
            .inner
            .into_iter()
            .fold((enemy, player), fold_effects);

        let game_result = Self::check_game_result(&enemy, &player, self.turn_number);

        let player = Player {
            power_reserve: res_pool,
            ..player
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

        let (enemy, player) = enemy
            .temp_start_turn_effects
            .clone()
            .inner
            .into_iter()
            .fold((enemy, player), fold_effects);
        let (enemy, player) = if !enemy.skip_next_turn {
            let effects = enemy.start_turn(&player).extend(enemy.end_turn(&player));

            effects
                .inner
                .into_iter()
                .fold((enemy, player), fold_effects)
        } else {
            (enemy, player)
        };

        let game_result = Self::check_game_result(&enemy, &player, self.turn_number);

        Self {
            enemy: Enemy {
                skip_next_turn: false,
                temp_start_turn_effects: FpVec::new(),
                ..enemy
            },
            player,
            game_result,
            turn_number: self.turn_number + 1,
            ..self
        }
    }
}
