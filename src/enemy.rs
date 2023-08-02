use crate::fp_vec::FpVec;
use crate::player::Player;
use crate::{
    player::PlayerCard, Damage, DefenseProps, EffectCondition, EffectTrigger, EffectType,
    ElementType, GameEffect,
};

#[derive(Debug, Clone)]
pub struct Enemy {
    pub hit_points: i32,
    pub defense_props: DefenseProps,
    pub start_turn_effects: FpVec<GameEffect>,
    pub end_turn_effects: FpVec<GameEffect>,
}

impl Enemy {
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
        println!("Applying effect to enemy: {:?}", effect);
        match effect {
            EffectType::Damage(dmg) => self.take_damage(dmg),
            EffectType::LifeAdjust(amt) => Self {
                hit_points: self.hit_points + amt,
                ..self
            },
            _ => Self { ..self },
        }
    }

    fn take_damage(self, damage: Damage) -> Self {
        let raw_damage = match damage.element_type {
            ElementType::Wind => self.defense_props.wind.adjust_damage(damage.amount),
            ElementType::Land => self.defense_props.land.adjust_damage(damage.amount),
            ElementType::Water => self.defense_props.water.adjust_damage(damage.amount),
            ElementType::NoElement => self.defense_props.any.adjust_damage(damage.amount),
        };
        Enemy {
            hit_points: self.hit_points - raw_damage,
            ..self
        }
    }
}
