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
    pub fn check_game_result(enemy: &Enemy, player: &Player, turn_number: u32) -> GameOutcome {
        match 0 {
            _ if player.hit_points <= 0 => GameOutcome::EnemyWins(turn_number),
            _ if enemy.hit_points <= 0 => GameOutcome::PlayerWins(turn_number),
            _ => GameOutcome::Undecided,
        }
    }

    pub fn check_enchantments(
        enchantments: &FpVec<GameEffect>,
        enemy: &Enemy,
        player: &Player,
    ) -> (FpVec<Enchantment>, FpVec<Enchantment>) {
        enchantments.inner.iter().fold(
            (FpVec::new(), FpVec::new()),
            |(enemy_vec, player_vec), eff| {
                if eff.target.is_enemy() {
                    match &eff.effect {
                        EffectTrigger::Always(EffectType::Enchantment(ench)) => {
                            (enemy_vec.push(ench.clone()), player_vec)
                        }
                        EffectTrigger::Condition(cond, EffectType::Enchantment(ench))
                            if cond.check_enemy(&enemy) =>
                        {
                            (enemy_vec.push(ench.clone()), player_vec)
                        }
                        _ => (enemy_vec, player_vec),
                    }
                } else {
                    match &eff.effect {
                        EffectTrigger::Always(EffectType::Enchantment(ench)) => {
                            (enemy_vec, player_vec.push(ench.clone()))
                        }
                        EffectTrigger::Condition(cond, EffectType::Enchantment(ench))
                            if cond.check_player(&player) =>
                        {
                            (enemy_vec, player_vec.push(ench.clone()))
                        }
                        _ => (enemy_vec, player_vec),
                    }
                }
            },
        )
    }

    pub fn start(enemy: Enemy, player: Player) -> Self {
        let (enemy_ench_from_enemy, player_ench_from_enemy) =
            Self::check_enchantments(&enemy.enchantments, &enemy, &player);
        let (enemy_ench_from_player, player_ench_from_player) = player.cards.inner.iter().fold(
            (FpVec::new(), FpVec::new()),
            |(enemy_eff_vec, player_eff_vec), card| {
                let (e_vec, p_vec) =
                    Self::check_enchantments(&card.game_start_effects, &enemy, &player);
                (enemy_eff_vec.extend(e_vec), player_eff_vec.extend(p_vec))
            },
        );

        let enemy_enchantments = enemy_ench_from_enemy.extend(enemy_ench_from_player);
        let player_enchantments = player_ench_from_enemy.extend(player_ench_from_player);
        println!(
            "Game Starting with global enchantments:\n  * Enemy [{}]\n  * Player [{}]",
            enemy_enchantments
                .inner
                .iter()
                .fold(String::new(), |desc, ench| format!(
                    "{}{}",
                    desc,
                    ench.description()
                )),
            player_enchantments
                .inner
                .iter()
                .fold(String::new(), |desc, ench| format!(
                    "{}{}",
                    desc,
                    ench.description()
                )),
        );

        Game {
            enemy: Enemy {
                current_activated_effects: enemy_enchantments,
                ..enemy
            },
            player: Player {
                current_activated_effects: player_enchantments,
                ..player
            },
            turn_number: 1,
            game_result: GameOutcome::Undecided,
        }
    }

    pub fn take_player_turn(self, card_play_list: FpVec<PlayerCard>) -> Self {
        let enemy = self.enemy;
        let player = self.player;

        let start_effects = player.start_turn();
        let play_effects = card_play_list
            .inner
            .into_iter()
            .fold(FpVec::new(), |effects, card| {
                effects.extend(player.player_play_card(&enemy, card))
            });
        let effects = start_effects.extend(play_effects).extend(player.end_turn());

        let (enemy, player) = effects
            .inner
            .into_iter()
            .fold((enemy, player), fold_effects);

        let game_result = Self::check_game_result(&enemy, &player, self.turn_number);

        let player = Player { ..player };
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
