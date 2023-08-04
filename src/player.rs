use crate::fp_vec::FpVec;
use crate::{
    enemy::Enemy, game_effects::GameEffect, Damage, EffectCondition, EffectTrigger, EffectType,
    ElementType, Enchantment,
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
    pub fn new(hit_points: i32, cards: FpVec<PlayerCard>) -> Self {
        Self {
            cards,
            hit_points,
            power_reserve: 0,
            current_activated_effects: FpVec::new(),
        }
    }

    pub fn description(&self) -> String {
        format!(
            "HP [{}]  Current Power [{}]  Enchantment Effects [{}]",
            self.hit_points,
            self.power_reserve,
            self.current_activated_effects
                .inner
                .iter()
                .map(|eff| eff.description())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    pub fn player_enchantments(self) -> Self {
        let current_activated_effects =
            self.cards.inner.iter().fold(FpVec::new(), |eff_vec, card| {
                card.game_start_effects
                    .inner
                    .iter()
                    .fold(eff_vec, |card_eff_vec, effect| {
                        if effect.target.is_player() {
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

    pub fn player_play_card(
        &self,
        enemy: &Enemy,
        card: PlayerCard,
        current_power_pool: i32,
    ) -> (FpVec<GameEffect>, i32) {
        if !card.can_play {
            (FpVec::new(), 0)
        } else {
            let card =
                self.current_activated_effects
                    .inner
                    .iter()
                    .fold(card, |card, eff| match eff {
                        Enchantment::SpellCostAdjust(element, amt)
                            if *element == card.element || *element == ElementType::NoElement =>
                        {
                            PlayerCard {
                                power_cost: if card.power_cost + amt < 1 {
                                    1
                                } else {
                                    card.power_cost + amt
                                },
                                ..card
                            }
                        }
                        Enchantment::SpellDamageAdjust(element, adj)
                            if *element == card.element =>
                        {
                            PlayerCard {
                                play_card_effects: card.play_card_effects.push(GameEffect::enemy(
                                    "Spell Damage Adjust Effect",
                                    EffectTrigger::Always(EffectType::Damage(Damage {
                                        element_type: element.clone(),
                                        amount: *adj,
                                    })),
                                )),
                                ..card
                            }
                        }
                        _ => card,
                    });

            if current_power_pool >= card.power_cost {
                println!(
                    "Play card: {} for {} cost ({} power available before play)",
                    card.name, card.power_cost, current_power_pool
                );
                let enemy_thorns_effects: FpVec<GameEffect> = enemy
                    .player_play_card_effects
                    .inner
                    .iter()
                    .fold(FpVec::new(), |effects, eff| match &eff.effect {
                        EffectTrigger::Condition(cond, triggered_effect)
                            if eff.target.is_player()
                                && *cond
                                    == EffectCondition::PlayerPlaysCardWithElement(
                                        card.element.clone(),
                                    ) =>
                        {
                            println!(
                                "Player causes effect due to casting spell of element {}: {}",
                                card.element.description(),
                                triggered_effect.description()
                            );
                            effects.push(GameEffect::player(
                                &eff.name,
                                EffectTrigger::Always(triggered_effect.clone()),
                            ))
                        }
                        _ => effects,
                    });
                (
                    FpVec::from_vec(vec![GameEffect::player(
                        &format!("Spell Cost for {}", card.name),
                        EffectTrigger::Always(EffectType::PowerAdjust(
                            -1 * (card.power_cost as i32),
                        )),
                    )])
                    .extend(card.play_card_effects)
                    .extend(enemy_thorns_effects),
                    card.power_cost,
                )
            } else {
                println!(
                    "Can't play {} (cost: {}), insufficient power ({} power left)",
                    card.name, card.power_cost, current_power_pool
                );
                (FpVec::new(), 0)
            }
        }
    }

    pub fn start_turn(&self) -> FpVec<GameEffect> {
        self.cards.inner.iter().fold(FpVec::new(), |effects, card| {
            effects.extend(card.start_turn_effects.clone())
        })
    }

    pub fn end_turn(&self) -> FpVec<GameEffect> {
        FpVec::new()
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
        match effect {
            EffectType::Damage(dmg) => self.take_damage(dmg),
            EffectType::LifeAdjust(amt) => {
                println!("Player, {} HP", amt);
                Self {
                    hit_points: self.hit_points + amt,
                    ..self
                }
            }
            EffectType::PowerAdjust(amt) => {
                println!("Player, {} Power", amt);
                Self {
                    power_reserve: self.power_reserve + amt,
                    ..self
                }
            }
            _ => Self { ..self },
        }
    }

    fn take_damage(self, damage: Damage) -> Self {
        println!(
            "Player takes damage: {}/{}",
            damage.element_type.description(),
            damage.amount
        );

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
    pub description: String,
    pub game_start_effects: FpVec<GameEffect>,
    pub start_turn_effects: FpVec<GameEffect>,
    pub play_card_effects: FpVec<GameEffect>,
}

impl PlayerCard {
    pub fn new(name: &str, description: &str, power_cost: i32, element: ElementType) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            power_cost,
            element,
            can_play: true,
            game_start_effects: FpVec::new(),
            start_turn_effects: FpVec::new(),
            play_card_effects: FpVec::new(),
        }
    }
    pub fn cant_play(self) -> Self {
        Self {
            can_play: false,
            ..self
        }
    }

    pub fn game_start_effect(self, effect: GameEffect) -> Self {
        Self {
            game_start_effects: self.game_start_effects.push(effect),
            ..self
        }
    }

    pub fn start_turn_effects(self, effect: GameEffect) -> Self {
        Self {
            start_turn_effects: self.start_turn_effects.push(effect),
            ..self
        }
    }

    pub fn play_card_effects(self, effect: GameEffect) -> Self {
        Self {
            play_card_effects: self.play_card_effects.push(effect),
            ..self
        }
    }
}
