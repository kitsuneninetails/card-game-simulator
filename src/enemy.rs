use crate::fp_vec::FpVec;
use crate::player::Player;
use crate::{
    game_effects::GameEffect, Damage, DefenseProps, EffectTrigger, EffectType, ElementType,
};

#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: String,
    pub hit_points: i32,
    pub defense_props: DefenseProps,
    pub skip_next_turn: bool,
    pub start_turn_effects: FpVec<GameEffect>,
    pub end_turn_effects: FpVec<GameEffect>,
    pub temp_start_turn_effects: FpVec<GameEffect>,
    pub player_start_turn_effects: FpVec<GameEffect>,
    pub player_play_card_effects: FpVec<GameEffect>,
}

impl Enemy {
    pub fn new(name: &str, hit_points: i32, defense_props: DefenseProps, turn_damage: i32) -> Self {
        Self {
            name: name.to_string(),
            hit_points,
            defense_props,
            skip_next_turn: false,
            start_turn_effects: FpVec::new(),
            end_turn_effects: FpVec::from_vec(vec![GameEffect::enemy_attack(turn_damage)]),
            temp_start_turn_effects: FpVec::new(),
            player_start_turn_effects: FpVec::new(),
            player_play_card_effects: FpVec::new(),
        }
    }

    pub fn start_turn_effects(self, effect: GameEffect) -> Self {
        Self {
            start_turn_effects: self.start_turn_effects.push(effect),
            ..self
        }
    }

    pub fn end_turn_effects(self, effect: GameEffect) -> Self {
        Self {
            end_turn_effects: self.end_turn_effects.push(effect),
            ..self
        }
    }

    pub fn player_start_turn_effects(self, effect: GameEffect) -> Self {
        Self {
            player_start_turn_effects: self.player_start_turn_effects.push(effect),
            ..self
        }
    }

    pub fn player_play_card_effects(self, effect: GameEffect) -> Self {
        Self {
            player_play_card_effects: self.player_play_card_effects.push(effect),
            ..self
        }
    }

    pub fn description(&self) -> String {
        format!(
            "{} - HP [{}]\nStart Turn Effects [{}]\nEnd Turn Effects [{}]\nPlayer Start Turn Effects [{}]\nPlayer Play Card Effects [{}]\n",
            self.name,
            self.hit_points,
            self.start_turn_effects
                .inner
                .iter()
                .map(|eff| eff.description())
                .collect::<Vec<String>>()
                .join(", "),
            self.end_turn_effects
                .inner
                .iter()
                .map(|eff| eff.description())
                .collect::<Vec<String>>()
                .join(", "),
            self.player_start_turn_effects
                .inner
                .iter()
                .map(|eff| eff.description())
                .collect::<Vec<String>>()
                .join(", "),
            self.player_play_card_effects
                .inner
                .iter()
                .map(|eff| eff.description())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    pub fn start_turn(&self, player: &Player) -> FpVec<GameEffect> {
        self.start_turn_effects.clone()
    }

    pub fn end_turn(&self, player: &Player) -> FpVec<GameEffect> {
        self.end_turn_effects.clone()
    }

    pub fn trigger_effect(self, trigger: EffectTrigger, player: &Player) -> Self {
        match trigger {
            EffectTrigger::Always(effect) => self.apply_effect(effect),
            EffectTrigger::Condition(cond, effect) => {
                if cond.check_enemy(&self) && cond.check_player(player) {
                    self.apply_effect(effect)
                } else {
                    Self { ..self }
                }
            }
        }
    }

    fn apply_effect(self, effect: EffectType) -> Self {
        match effect {
            EffectType::Damage(dmg) => self.take_damage(dmg),
            EffectType::LifeAdjust(amt) => {
                println!("Emeny, {} HP", amt);
                Self {
                    hit_points: self.hit_points + amt,
                    ..self
                }
            }
            EffectType::SkipTurn => {
                println!("Enemy Has to Skip Next Turn");
                Self {
                    skip_next_turn: true,
                    temp_start_turn_effects: self.temp_start_turn_effects.push(GameEffect::enemy(
                        "Skip Turn",
                        EffectTrigger::Always(EffectType::SkipTurn),
                    )),
                    ..self
                }
            }
            _ => Self { ..self },
        }
    }

    pub fn skip_turn(self) -> Self {
        Self {
            skip_next_turn: false,
            ..self
        }
    }

    fn take_damage(self, damage: Damage) -> Self {
        let raw_damage = match damage.element_type {
            ElementType::Wind => self.defense_props.wind.adjust_damage(damage.amount),
            ElementType::Land => self.defense_props.land.adjust_damage(damage.amount),
            ElementType::Water => self.defense_props.water.adjust_damage(damage.amount),
            ElementType::NoElement => self.defense_props.any.adjust_damage(damage.amount),
        };
        println!(
            "Enemy takes damage: {}/{} ({} actual)",
            damage.element_type.description(),
            damage.amount,
            raw_damage
        );
        Enemy {
            hit_points: self.hit_points - raw_damage,
            ..self
        }
    }
}
