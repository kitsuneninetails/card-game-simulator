use crate::fp_vec::FpVec;
use crate::{
    enemy::Enemy, Damage, EffectTarget, EffectTrigger, EffectType, ElementType, Enchantment,
    GameEffect,
};
use std::cmp::min;

#[derive(Debug, Clone)]
pub struct Player {
    pub cards: FpVec<PlayerCard>,
    pub hit_points: i32,
    pub power_reserve: i32,
    pub current_activated_effects: FpVec<Enchantment>,
}

impl Player {
    pub fn player_enchantments(self) -> Self {
        let current_activated_effects =
            self.cards.inner.iter().fold(FpVec::new(), |eff_vec, card| {
                card.posession_effects
                    .inner
                    .iter()
                    .fold(eff_vec, |card_eff_vec, effect| {
                        if let EffectTarget::Player = effect.target {
                            match &effect.effect {
                                EffectTrigger::Always(eff) => match eff {
                                    EffectType::Enchantment(ench) => {
                                        card_eff_vec.push(ench.clone())
                                    }
                                    _ => card_eff_vec,
                                },
                                EffectTrigger::Condition(cond, eff) if cond.check_player(&self) => {
                                    match eff {
                                        EffectType::Enchantment(ench) => {
                                            card_eff_vec.push(ench.clone())
                                        }
                                        _ => card_eff_vec,
                                    }
                                }
                                _ => card_eff_vec,
                            }
                        } else {
                            card_eff_vec
                        }
                    })
            });
        Self {
            current_activated_effects,
            ..self
        }
    }

    pub fn player_play_card(&self, card: PlayerCard) -> FpVec<GameEffect> {
        let card = self
            .current_activated_effects
            .inner
            .iter()
            .fold(card, |card, eff| match eff {
                Enchantment::SpellCostAdjust(element, amt) if *element == card.element => {
                    PlayerCard {
                        power_cost: min(1, card.power_cost - amt),
                        ..card
                    }
                }
                Enchantment::SpellDamageAdjust(element, adj) if *element == card.element => {
                    PlayerCard {
                        play_card_effects: card.play_card_effects.push(GameEffect {
                            target: EffectTarget::Enemy,
                            effect: EffectTrigger::Always(EffectType::Damage(Damage {
                                element_type: element.clone(),
                                amount: *adj,
                            })),
                        }),
                        ..card
                    }
                }
                _ => card,
            });

        FpVec::from_vec(vec![GameEffect::player(EffectTrigger::Always(
            EffectType::PowerAdjust(-1 * (card.power_cost as i32)),
        ))])
        .extend(card.play_card_effects)
    }

    pub fn start_turn(&self) -> FpVec<GameEffect> {
        let base_effects = FpVec::from_vec(vec![GameEffect::player(EffectTrigger::Always(
            EffectType::PowerAdjust(3),
        ))]);
        self.cards.inner.iter().fold(base_effects, |effects, card| {
            effects.extend(card.posession_effects.clone())
        })
    }

    pub fn end_turn(&self) -> FpVec<GameEffect> {
        self.cards.inner.iter().fold(FpVec::new(), |effects, card| {
            effects.extend(card.posession_effects.clone())
        })
    }

    pub fn trigger_effect(self, trigger: EffectTrigger, enemy: &Enemy) -> Self {
        match trigger {
            EffectTrigger::Always(effect) => self.apply_effect(effect),
            EffectTrigger::Condition(cond, effect) => {
                if cond.check_player(&self) && cond.check_enemy(enemy) {
                    self.apply_effect(effect)
                } else {
                    Self { ..self }
                }
            }
        }
    }

    fn apply_effect(self, effect: EffectType) -> Self {
        println!("Applying effect to player: {:?}", effect);
        match effect {
            EffectType::Damage(dmg) => self.take_damage(dmg),
            EffectType::LifeAdjust(amt) => Self {
                hit_points: self.hit_points + amt,
                ..self
            },
            EffectType::PowerAdjust(amt) => Self {
                power_reserve: self.power_reserve + amt,
                ..self
            },
            _ => Self { ..self },
        }
    }

    fn take_damage(self, damage: Damage) -> Self {
        Self {
            hit_points: self.hit_points - damage.amount,
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerCard {
    pub power_cost: i32,
    pub element: ElementType,
    pub can_play: bool,
    pub name: String,
    pub posession_effects: FpVec<GameEffect>,
    pub play_card_effects: FpVec<GameEffect>,
}
