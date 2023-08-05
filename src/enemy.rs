use crate::fp_vec::FpVec;
use crate::game_effects::{Enchantments, EnemyEffects, OnCardPlayEffects};
use crate::player::Player;
use crate::{
    game_effects::GameEffect, Damage, DamageAdjustment, DefenseProps, EffectTrigger, EffectType,
    ElementType, Enchantment,
};
use std::cmp::min;

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
    pub enchantments: FpVec<GameEffect>,
    pub current_activated_effects: FpVec<Enchantment>,
}

impl Enemy {
    pub fn new(name: &str, hit_points: i32, defense_props: DefenseProps, turn_damage: i32) -> Self {
        Self {
            name: name.to_string(),
            hit_points,
            defense_props,
            skip_next_turn: false,
            start_turn_effects: FpVec::new(),
            end_turn_effects: FpVec::from_vec(vec![EnemyEffects::attack(turn_damage)]),
            temp_start_turn_effects: FpVec::new(),
            player_start_turn_effects: FpVec::new(),
            player_play_card_effects: FpVec::new(),
            enchantments: FpVec::new(),
            current_activated_effects: FpVec::new(),
        }
    }

    pub fn start_turn_effect(self, effect: GameEffect) -> Self {
        self.enchantments
            .inner
            .iter()
            .fold((), |card, eff| match eff {
                _ => (),
            });
        Self {
            start_turn_effects: self.start_turn_effects.push(effect),
            ..self
        }
    }

    pub fn end_turn_effect(self, effect: GameEffect) -> Self {
        Self {
            end_turn_effects: self.end_turn_effects.push(effect),
            ..self
        }
    }

    pub fn player_start_turn_effect(self, effect: GameEffect) -> Self {
        Self {
            player_start_turn_effects: self.player_start_turn_effects.push(effect),
            ..self
        }
    }

    pub fn player_play_card_effect(self, effect: GameEffect) -> Self {
        Self {
            player_play_card_effects: self.player_play_card_effects.push(effect),
            ..self
        }
    }

    pub fn enchantment(self, effect: GameEffect) -> Self {
        Self {
            enchantments: self.enchantments.push(effect),
            ..self
        }
    }

    pub fn description(&self) -> String {
        format!(
            "{} - HP [{}]\n  * Start Turn Effects [{}]\n  * End Turn Effects [{}]\n  * Player Start Turn Effects [{}]\n  * Player Play Card Effects [{}]\n  * Current Enchantments [{}]",
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
            self.current_activated_effects
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
            EffectTrigger::Condition(cond, effect)
                if cond.check_enemy(&self) && cond.check_player(player) =>
            {
                self.apply_effect(effect)
            }
            _ => self,
        }
    }

    fn apply_effect(self, effect: EffectType) -> Self {
        match effect {
            EffectType::Damage(dmg) => self.take_damage(dmg),
            EffectType::PercentDamage(pct) => {
                let amount = ((self.hit_points as f64) * pct).floor() as i32;
                self.take_damage(Damage {
                    element_type: ElementType::NoElement,
                    amount,
                })
            }
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
        let raw_damage1 = self.defense_props.any.adjust_damage(damage.amount);
        let raw_damage2 = match damage.element_type {
            ElementType::Wind => self.defense_props.wind.adjust_damage(damage.amount),
            ElementType::Land => self.defense_props.land.adjust_damage(damage.amount),
            ElementType::Water => self.defense_props.water.adjust_damage(damage.amount),
            ElementType::NoElement => damage.amount,
        };
        println!(
            "Enemy takes damage: {}/{} ({} actual)",
            damage.element_type.description(),
            damage.amount,
            min(raw_damage1, raw_damage2)
        );
        Enemy {
            hit_points: self.hit_points - min(raw_damage1, raw_damage2),
            ..self
        }
    }

    pub fn oil_spill() -> Self {
        Self::new(
            "Oil Spill",
            14,
            DefenseProps {
                wind: DamageAdjustment::Normal,
                water: DamageAdjustment::Absolute(1),
                land: DamageAdjustment::Absolute(-1),
                any: DamageAdjustment::Normal,
            },
            3,
        )
    }

    pub fn typhoon() -> Self {
        Self::new(
            "Typhoon",
            12,
            DefenseProps {
                wind: DamageAdjustment::Percent(0.0),
                water: DamageAdjustment::Normal,
                land: DamageAdjustment::Absolute(-2),
                any: DamageAdjustment::Normal,
            },
            4,
        )
    }

    pub fn forest_fire() -> Self {
        Self::new(
            "Forest Fire",
            8,
            DefenseProps {
                wind: DamageAdjustment::Normal,
                water: DamageAdjustment::Absolute(1),
                land: DamageAdjustment::Percent(0.0),
                any: DamageAdjustment::Normal,
            },
            8,
        )
        .player_play_card_effect(OnCardPlayEffects::heal_enemy_on_play_elem(
            ElementType::Wind,
            1,
        ))
    }

    pub fn landslide() -> Self {
        Self::new(
            "Landslide",
            20,
            DefenseProps {
                wind: DamageAdjustment::Normal,
                water: DamageAdjustment::Normal,
                land: DamageAdjustment::Normal,
                any: DamageAdjustment::Absolute(-1),
            },
            2,
        )
    }

    pub fn avalanche() -> Self {
        Self::new(
            "Avalanche",
            10,
            DefenseProps {
                wind: DamageAdjustment::Normal,
                water: DamageAdjustment::Normal,
                land: DamageAdjustment::Normal,
                any: DamageAdjustment::Normal,
            },
            7,
        )
        .enchantment(Enchantments::player_elem_spells_forbidden(
            ElementType::Water,
        ))
    }

    pub fn famine() -> Self {
        Self::new(
            "Famine",
            16,
            DefenseProps {
                wind: DamageAdjustment::Normal,
                water: DamageAdjustment::Absolute(2),
                land: DamageAdjustment::Normal,
                any: DamageAdjustment::Normal,
            },
            5,
        )
        .player_play_card_effect(OnCardPlayEffects::take_damage_on_play_elem(
            ElementType::Wind,
            1,
        ))
    }

    pub fn earthquake() -> Self {
        Self::new(
            "Earthquake",
            10,
            DefenseProps {
                wind: DamageAdjustment::Normal,
                water: DamageAdjustment::Normal,
                land: DamageAdjustment::Normal,
                any: DamageAdjustment::Normal,
            },
            5,
        )
        .enchantment(Enchantments::player_elem_spells_forbidden(
            ElementType::Land,
        ))
    }

    pub fn volcano() -> Self {
        Self::new(
            "Volcano Eruption",
            5,
            DefenseProps {
                wind: DamageAdjustment::Normal,
                water: DamageAdjustment::Normal,
                land: DamageAdjustment::Normal,
                any: DamageAdjustment::Absolute(-1),
            },
            8,
        )
        .player_play_card_effect(OnCardPlayEffects::heal_on_play_elem(ElementType::Water, 5))
    }

    pub fn floods() -> Self {
        Self::new(
            "Floods Eruption",
            12,
            DefenseProps {
                wind: DamageAdjustment::Normal,
                water: DamageAdjustment::Normal,
                land: DamageAdjustment::Normal,
                any: DamageAdjustment::Normal,
            },
            5,
        )
    }

    pub fn drought() -> Self {
        Self::new(
            "Drought",
            25,
            DefenseProps {
                wind: DamageAdjustment::Normal,
                water: DamageAdjustment::Absolute(1),
                land: DamageAdjustment::Percent(0.0),
                any: DamageAdjustment::Normal,
            },
            2,
        )
    }

    pub fn tornado() -> Self {
        Self::new(
            "Tornado",
            10,
            DefenseProps {
                wind: DamageAdjustment::Percent(0.0),
                water: DamageAdjustment::Normal,
                land: DamageAdjustment::Normal,
                any: DamageAdjustment::Normal,
            },
            6,
        )
    }

    pub fn meltdown() -> Self {
        Self::new(
            "Nuclear Meltdown",
            35,
            DefenseProps {
                wind: DamageAdjustment::Normal,
                water: DamageAdjustment::Normal,
                land: DamageAdjustment::Normal,
                any: DamageAdjustment::Normal,
            },
            2,
        )
        .player_play_card_effect(OnCardPlayEffects::take_damage_on_play_elem(
            ElementType::Wind,
            2,
        ))
        .player_play_card_effect(OnCardPlayEffects::heal_enemy_on_play_elem(
            ElementType::Wind,
            1,
        ))
    }

    pub fn blackout() -> Self {
        Self::new(
            "Electricity Blackout",
            20,
            DefenseProps {
                wind: DamageAdjustment::Absolute(1),
                water: DamageAdjustment::Normal,
                land: DamageAdjustment::Normal,
                any: DamageAdjustment::Normal,
            },
            3,
        )
        .player_play_card_effect(OnCardPlayEffects::take_damage_on_play_elem(
            ElementType::Water,
            1,
        ))
    }
}
