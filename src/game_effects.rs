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

    // ENEMY EFFECTS
    // Damage
    pub fn enemy_attack(amount: i32) -> Self {
        Self::player(
            "Enemy Attack",
            EffectTrigger::Always(EffectType::Damage(Damage {
                element_type: ElementType::NoElement,
                amount,
            })),
        )
    }

    pub fn enemy_cond_attack_element_present(element_type: ElementType, amount: i32) -> Self {
        Self::player(
            &format!(
                "Enemy Backlash Attack ({} Element)",
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

    pub fn enemy_cond_thorns_element_played(element_type: ElementType, amount: i32) -> Self {
        Self::player(
            &format!(
                "Enemy Thorns Attack ({} Element)",
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

    // PLAYER CARD EFFECTS
    // Damage
    pub fn element_damage(element_type: ElementType, amount: i32) -> Self {
        Self::enemy(
            &format!("{} Damage", element_type.description()),
            EffectTrigger::Always(EffectType::Damage(Damage {
                element_type,
                amount,
            })),
        )
    }

    pub fn physical_damage(amount: i32) -> Self {
        Self::enemy(
            &format!("Physical Damage"),
            EffectTrigger::Always(EffectType::Damage(Damage {
                element_type: ElementType::NoElement,
                amount,
            })),
        )
    }

    pub fn gravity(pct: f64) -> Self {
        Self::enemy(
            &format!("Physical Damage"),
            EffectTrigger::Always(EffectType::PercentDamage(pct)),
        )
    }

    //Heals
    pub fn heal(amount: i32) -> Self {
        Self::player(
            &format!("Heal"),
            EffectTrigger::Always(EffectType::LifeAdjust(amount)),
        )
    }

    //Utility
    pub fn spell_cost_down_element(element_type: ElementType, amt: i32) -> Self {
        Self::player(
            &format!("{} Spells Cost {}", element_type.description(), amt),
            EffectTrigger::Always(EffectType::Enchantment(Enchantment::SpellCostAdjust(
                element_type,
                amt,
            ))),
        )
    }

    pub fn spell_cost_down_all(amt: i32) -> Self {
        Self::player(
            &format!("All Spells Cost {}", amt),
            EffectTrigger::Always(EffectType::Enchantment(Enchantment::SpellCostAdjust(
                ElementType::NoElement,
                amt,
            ))),
        )
    }

    pub fn power_add_per_turn(amt: i32) -> Self {
        Self::player(
            &format!("Power {} Each Turn", amt),
            EffectTrigger::Always(EffectType::Enchantment(Enchantment::PowerAddPerTurn(amt))),
        )
    }

    pub fn enemy_skip_turn() -> Self {
        Self::enemy(
            &format!("Skip Turn"),
            EffectTrigger::Always(EffectType::SkipTurn),
        )
    }
}
