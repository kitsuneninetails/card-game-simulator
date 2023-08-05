use crate::fp_vec::FpVec;
use crate::game_effects::{CardEffects, Enchantments, OnCardPlayEffects};
use crate::{
    enemy::Enemy, game_effects::GameEffect, Damage, EffectCondition, EffectTrigger, EffectType,
    ElementType, Enchantment,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Player {
    pub cards: FpVec<PlayerCard>,
    pub hit_points: i32,
    pub current_activated_effects: FpVec<Enchantment>,
}

impl Player {
    pub fn new(hit_points: i32, cards: FpVec<PlayerCard>) -> Self {
        Self {
            cards,
            hit_points,
            current_activated_effects: FpVec::new(),
        }
    }

    pub fn description(&self) -> String {
        format!(
            "HP [{}]\n  * Enchantment Effects [{}]",
            self.hit_points,
            self.current_activated_effects
                .inner
                .iter()
                .map(|eff| eff.description())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    pub fn player_play_card(&self, enemy: &Enemy, card: PlayerCard) -> FpVec<GameEffect> {
        // Check global enchantments
        let card = self
            .current_activated_effects
            .inner
            .iter()
            .fold(card, |card, eff| match eff {
                Enchantment::SpellDamageAdjust(element, adj) if *element == card.element => {
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
                Enchantment::SpellElementForbidden(elem) if *elem == card.element => PlayerCard {
                    can_play: false,
                    ..card
                },
                _ => card,
            });

        if !card.can_play {
            println!("Cannot play card: {}", card.name);
            FpVec::new()
        } else {
            println!("Play card: {}", card.name,);
            let enemy_thorns_effects: FpVec<GameEffect> = enemy
                .player_play_card_effects
                .inner
                .iter()
                .fold(FpVec::new(), |effects, eff| {
                    match &eff.effect {
                        EffectTrigger::Condition(cond, triggered_effect)
                        if *cond == EffectCondition::PlayerPlaysCardWithElement(card.element.clone()) => {
                            if eff.target.is_player()
                            {
                                println!(
                                    "Player causes counter effect to self due to casting spell of element {}: {}",
                                    card.element.description(),
                                    triggered_effect.description()
                                );
                                effects.push(GameEffect::player(
                                    &eff.name,
                                    EffectTrigger::Always(triggered_effect.clone()),
                                ))
                            } else {
                                println!(
                                    "Player causes counter effect on enemy due to casting spell of element {}: {}",
                                    card.element.description(),
                                    triggered_effect.description()
                                );
                                effects.push(GameEffect::enemy(
                                    &eff.name,
                                    EffectTrigger::Always(triggered_effect.clone()),
                                ))
                            }
                        }
                        _ => effects,
                    }
                });
            FpVec::from_vec(
                card.play_card_effects
                    .inner
                    .into_iter()
                    .map(|eff| match eff.effect {
                        EffectTrigger::Discard(_) => {
                            GameEffect::player("Discard", EffectTrigger::Discard(card.id.clone()))
                        }
                        _ => eff,
                    })
                    .collect(),
            )
            .extend(enemy_thorns_effects)
        }
    }

    pub fn start_turn(&self) -> FpVec<GameEffect> {
        self.cards.inner.iter().fold(FpVec::new(), |effects, card| {
            effects.extend(card.start_turn_effects.clone())
        })
    }

    pub fn end_turn(&self) -> FpVec<GameEffect> {
        self.current_activated_effects
            .inner
            .iter()
            .fold(FpVec::new(), |effects, eff| match eff {
                Enchantment::LifeAdjPerTurn(amt) => effects.push(CardEffects::heal(*amt)),
                _ => effects,
            })
    }

    pub fn trigger_effect(self, trigger: EffectTrigger, enemy: &Enemy) -> Self {
        match trigger {
            EffectTrigger::Always(effect) => self.apply_effect(effect),
            EffectTrigger::Condition(cond, effect) => {
                if cond.check_player(&self) && cond.check_enemy(enemy) {
                    self.apply_effect(effect)
                } else {
                    self
                }
            }
            EffectTrigger::Discard(id) => {
                println!("Discarding {}", id);
                Self {
                    cards: FpVec::from_vec(
                        self.cards
                            .inner
                            .into_iter()
                            .filter(|card| card.id != id)
                            .collect(),
                    ),
                    ..self
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
            _ => Self { ..self },
        }
    }

    fn take_damage(self, damage: Damage) -> Self {
        let amount = self
            .current_activated_effects
            .inner
            .iter()
            .fold(damage.amount, |dmg, eff| match eff {
                Enchantment::ShieldDamage(amt) => {
                    if dmg - amt < 0 {
                        0
                    } else {
                        dmg - amt
                    }
                }
                _ => dmg,
            });

        println!(
            "Player takes damage: {}/{}",
            damage.element_type.description(),
            amount
        );

        Self {
            hit_points: self.hit_points - amount,
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerCard {
    pub id: String,
    pub element: ElementType,
    pub can_play: bool,
    pub name: String,
    pub description: String,
    pub game_start_effects: FpVec<GameEffect>,
    pub start_turn_effects: FpVec<GameEffect>,
    pub play_card_effects: FpVec<GameEffect>,
}

impl PlayerCard {
    pub fn new(name: &str, description: &str, element: ElementType) -> Self {
        Self {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            name: name.to_string(),
            description: description.to_string(),
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

    pub fn play_card_effect(self, effect: GameEffect) -> Self {
        Self {
            play_card_effects: self.play_card_effects.push(effect),
            ..self
        }
    }
}

pub struct SpecialCards;
impl SpecialCards {
    pub fn env_suit() -> PlayerCard {
        PlayerCard::new(
            "Environmental Suit",
            "If you have this card in your hand, take 2 less damage",
            ElementType::Water,
        )
        .game_start_effect(Enchantments::player_shield_from_elem(2))
    }
    pub fn power_amp() -> PlayerCard {
        PlayerCard::new(
            "Power Amplifier",
            "If you have this card in your hand, all spells do 2 more damage",
            ElementType::Wind,
        )
        .game_start_effect(Enchantments::player_elem_spell_damage_adj(
            ElementType::NoElement,
            2,
        ))
    }
    pub fn helis() -> PlayerCard {
        PlayerCard::new(
            "Hospital Helicopters",
            "If you have this card in your hand, heal 3 per turn",
            ElementType::Wind,
        )
        .game_start_effect(Enchantments::player_heal_per_turn(3))
    }
    pub fn hydro_power() -> PlayerCard {
        PlayerCard::new(
            "Hydroelectric Power",
            "If you have this card in your hand, water spells do 3 more damage",
            ElementType::Water,
        )
        .game_start_effect(Enchantments::player_elem_spell_damage_adj(
            ElementType::Water,
            3,
        ))
    }
    pub fn bulldozers() -> PlayerCard {
        PlayerCard::new(
            "heavy Bulldozers",
            "If you have this card in your hand, land spells do 3 more damage",
            ElementType::Land,
        )
        .game_start_effect(Enchantments::player_elem_spell_damage_adj(
            ElementType::Land,
            3,
        ))
    }
    pub fn wind_turbines() -> PlayerCard {
        PlayerCard::new(
            "Wind Turbines",
            "If you have this card in your hand, wind spells do 3 more damage",
            ElementType::Wind,
        )
        .game_start_effect(Enchantments::player_elem_spell_damage_adj(
            ElementType::Wind,
            3,
        ))
    }
    pub fn military_aid() -> PlayerCard {
        PlayerCard::new(
            "Military Aid",
            "If you have this card in your hand, add 3 to any physical damage",
            ElementType::Land,
        )
        .game_start_effect(Enchantments::player_elem_spell_damage_adj(
            ElementType::NoElement,
            2,
        ))
    }
    pub fn time_slip() -> PlayerCard {
        PlayerCard::new(
            "Time Slip",
            "Discard this card and Skip Enemy Turn",
            ElementType::Wind,
        )
        .play_card_effect(CardEffects::skip_enemy_turn())
        .play_card_effect(OnCardPlayEffects::discard_this_card())
    }
    pub fn fire_breaks() -> PlayerCard {
        PlayerCard::new(
            "Fire Breaks",
            "Discard this card and Deal 6 Land Damage",
            ElementType::Land,
        )
        .play_card_effect(CardEffects::do_element_damage(ElementType::Land, 6))
        .play_card_effect(OnCardPlayEffects::discard_this_card())
    }
    pub fn fire_hose() -> PlayerCard {
        PlayerCard::new(
            "Fire Hoses",
            "Discard this card and Deal 6 Water Damage",
            ElementType::Water,
        )
        .play_card_effect(CardEffects::do_element_damage(ElementType::Water, 6))
        .play_card_effect(OnCardPlayEffects::discard_this_card())
    }
    pub fn jet_blast() -> PlayerCard {
        PlayerCard::new(
            "Jet Blast",
            "Discard this card and Deal 6 Wind Damage",
            ElementType::Wind,
        )
        .play_card_effect(CardEffects::do_element_damage(ElementType::Wind, 6))
        .play_card_effect(OnCardPlayEffects::discard_this_card())
    }
    pub fn logistics() -> PlayerCard {
        PlayerCard::new(
            "Supply Chains",
            "Discard this card and Deal 4 Physical Damage",
            ElementType::Wind,
        )
        .play_card_effect(CardEffects::do_physical_damage(4))
        .play_card_effect(OnCardPlayEffects::discard_this_card())
    }
    pub fn inside_help() -> PlayerCard {
        PlayerCard::new(
            "Inside Help",
            "Discard this card and Cut Enemy Health in Half",
            ElementType::Wind,
        )
        .play_card_effect(CardEffects::do_percent_damage(0.5))
        .play_card_effect(OnCardPlayEffects::discard_this_card())
    }
    pub fn tbd() -> PlayerCard {
        PlayerCard::new(
            "Time Slip",
            "Discard this card and Skip Enemy Turn",
            ElementType::Wind,
        )
        .play_card_effect(CardEffects::skip_enemy_turn())
        .play_card_effect(OnCardPlayEffects::discard_this_card())
    }
}
