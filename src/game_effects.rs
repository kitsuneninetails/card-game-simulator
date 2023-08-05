use crate::{
    Damage, EffectCondition, EffectTarget, EffectTrigger, EffectType, ElementType, Enchantment,
};

#[derive(Debug, Clone)]
pub struct GameEffect {
    pub name: String,
    pub target: EffectTarget,
    pub effect: EffectTrigger,
}

impl GameEffect {
    pub fn description(&self) -> String {
        format!(
            "{} - target [{}] effect [{}]",
            self.name,
            self.target.description(),
            self.effect.description()
        )
    }
    pub fn player(name: &str, effect: EffectTrigger) -> Self {
        Self {
            name: name.to_string(),
            target: EffectTarget::Player,
            effect,
        }
    }

    pub fn enemy(name: &str, effect: EffectTrigger) -> Self {
        Self {
            name: name.to_string(),
            target: EffectTarget::Enemy,
            effect,
        }
    }
}

pub struct EnemyEffects;
impl EnemyEffects {
    pub fn attack(amount: i32) -> GameEffect {
        GameEffect::player(
            "Enemy Attack",
            EffectTrigger::Always(EffectType::Damage(Damage {
                element_type: ElementType::NoElement,
                amount,
            })),
        )
    }
}

pub struct Enchantments;
impl Enchantments {
    pub fn player_take_damage_elem_card_present(
        element_type: ElementType,
        amount: i32,
    ) -> GameEffect {
        GameEffect::player(
            &format!(
                "Elemental Backlash Damage ({} Element)",
                element_type.description()
            ),
            EffectTrigger::Condition(
                EffectCondition::PlayerHasCardWithElement(element_type.clone()),
                EffectType::Damage(Damage {
                    element_type,
                    amount,
                }),
            ),
        )
    }

    pub fn player_elem_spell_damage_adj(element_type: ElementType, amount: i32) -> GameEffect {
        GameEffect::player(
            &format!(
                "{} Element Spells adjust damage by {}",
                element_type.description(),
                amount
            ),
            EffectTrigger::Always(EffectType::Enchantment(Enchantment::SpellDamageAdjust(
                element_type,
                amount,
            ))),
        )
    }

    pub fn player_elem_spells_forbidden(element_type: ElementType) -> GameEffect {
        GameEffect::player(
            &format!("{} Element Spells Forbidden", element_type.description()),
            EffectTrigger::Always(EffectType::Enchantment(Enchantment::SpellElementForbidden(
                element_type,
            ))),
        )
    }

    pub fn player_shield_from_elem(amount: i32) -> GameEffect {
        GameEffect::player(
            &format!("Global Shield {}", amount,),
            EffectTrigger::Always(EffectType::Enchantment(Enchantment::ShieldDamage(amount))),
        )
    }

    pub fn player_heal_per_turn(amount: i32) -> GameEffect {
        GameEffect::player(
            &format!("Global Shield {}", amount,),
            EffectTrigger::Always(EffectType::Enchantment(Enchantment::ShieldDamage(amount))),
        )
    }
}

pub struct OnCardPlayEffects;
impl OnCardPlayEffects {
    pub fn take_damage_on_play_elem(element_type: ElementType, amount: i32) -> GameEffect {
        GameEffect::player(
            &format!(
                "Player takes {} Damage for Playing {} Spell",
                amount,
                element_type.description()
            ),
            EffectTrigger::Condition(
                EffectCondition::PlayerPlaysCardWithElement(element_type.clone()),
                EffectType::Damage(Damage {
                    element_type,
                    amount,
                }),
            ),
        )
    }

    pub fn heal_on_play_elem(element_type: ElementType, amount: i32) -> GameEffect {
        GameEffect::player(
            &format!(
                "Player Heals {} for Playing {} Spell",
                amount,
                element_type.description()
            ),
            EffectTrigger::Condition(
                EffectCondition::PlayerPlaysCardWithElement(element_type.clone()),
                EffectType::LifeAdjust(amount),
            ),
        )
    }

    pub fn heal_enemy_on_play_elem(element_type: ElementType, amount: i32) -> GameEffect {
        GameEffect::enemy(
            &format!(
                "Enemy Heals {} for Player Playing {}",
                amount,
                element_type.description()
            ),
            EffectTrigger::Condition(
                EffectCondition::PlayerPlaysCardWithElement(element_type.clone()),
                EffectType::LifeAdjust(amount),
            ),
        )
    }

    pub fn discard_this_card() -> GameEffect {
        GameEffect::player(
            "Discard this card after playing",
            EffectTrigger::Discard(String::new()),
        )
    }
}

pub struct CardEffects;
impl CardEffects {
    //Damage
    pub fn do_element_damage(element_type: ElementType, amount: i32) -> GameEffect {
        GameEffect::enemy(
            &format!("{} Damage", element_type.description()),
            EffectTrigger::Always(EffectType::Damage(Damage {
                element_type,
                amount,
            })),
        )
    }

    pub fn do_physical_damage(amount: i32) -> GameEffect {
        GameEffect::enemy(
            &format!("Physical Damage"),
            EffectTrigger::Always(EffectType::Damage(Damage {
                element_type: ElementType::NoElement,
                amount,
            })),
        )
    }

    pub fn do_percent_damage(pct: f64) -> GameEffect {
        GameEffect::enemy(
            &format!("Physical Damage"),
            EffectTrigger::Always(EffectType::PercentDamage(pct)),
        )
    }

    //Heals
    pub fn heal(amount: i32) -> GameEffect {
        GameEffect::player(
            &format!("Heal"),
            EffectTrigger::Always(EffectType::LifeAdjust(amount)),
        )
    }

    //Utility
    pub fn skip_enemy_turn() -> GameEffect {
        GameEffect::enemy(
            &format!("Skip Turn"),
            EffectTrigger::Always(EffectType::SkipTurn),
        )
    }
}
