pub mod enemy;
pub mod fp_vec;
pub mod game;
pub mod game_effects;
pub mod player;

use enemy::Enemy;
use player::Player;

#[derive(Debug, Clone)]
pub enum EffectTarget {
    Player,
    Enemy,
}

impl EffectTarget {
    pub fn description(&self) -> String {
        match self {
            EffectTarget::Player => "PLAYER".to_string(),
            EffectTarget::Enemy => "ENEMY".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EffectTrigger {
    Always(EffectType),
    Condition(EffectCondition, EffectType),
}

impl EffectTrigger {
    pub fn description(&self) -> String {
        match self {
            EffectTrigger::Always(eff) => format!("[ALWAYS] {}", eff.description()),
            EffectTrigger::Condition(cond, eff) => {
                format!("[COND - {}] {}", cond.description(), eff.description())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum EffectCondition {
    PlayerHasCardWithElement(ElementType),
    PlayerHasNoCardWithElement(ElementType),
}

impl EffectCondition {
    pub fn description(&self) -> String {
        match self {
            EffectCondition::PlayerHasCardWithElement(el) => {
                format!("HAS_ELEMENT [{}]", el.description())
            }
            EffectCondition::PlayerHasNoCardWithElement(el) => {
                format!("HAS_NO_ELEMENT [{}]", el.description())
            }
        }
    }

    pub fn check_player(&self, player: &Player) -> bool {
        match self {
            EffectCondition::PlayerHasCardWithElement(el) => {
                player.cards.inner.iter().any(|card| card.element == *el)
            }
            EffectCondition::PlayerHasNoCardWithElement(el) => {
                player.cards.inner.iter().all(|card| card.element != *el)
            }
        }
    }
    pub fn check_enemy(&self, enemy: &Enemy) -> bool {
        match self {
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum EffectType {
    Damage(Damage),
    LifeAdjust(i32),
    PowerAdjust(i32),
    Enchantment(Enchantment),
    PercentDamage(f64),
}

impl EffectType {
    pub fn description(&self) -> String {
        match self {
            EffectType::Damage(dmg) => {
                format!("Damage [{}/{}]", dmg.element_type.description(), dmg.amount)
            }
            EffectType::LifeAdjust(adj) => format!("Life {}", adj),
            EffectType::PowerAdjust(adj) => format!("Power {}", adj),
            EffectType::Enchantment(ench) => format!("Enchant [{}]", ench.description()),
            EffectType::PercentDamage(dmg) => format!("Damage {}%", dmg),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Enchantment {
    SpellCostAdjust(ElementType, i32),
    SpellDamageAdjust(ElementType, i32),
    PowerAddPerTurn(i32),
}

impl Enchantment {
    pub fn description(&self) -> String {
        match self {
            Enchantment::SpellCostAdjust(cond, amt) => {
                format!("Spell cost {} [{}]", amt, cond.description())
            }
            Enchantment::SpellDamageAdjust(cond, amt) => {
                format!("Spell damage {} [{}]", amt, cond.description())
            }
            Enchantment::PowerAddPerTurn(amt) => format!("Add {} power each turn", amt),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Damage {
    pub amount: i32,
    pub element_type: ElementType,
}

impl Damage {
    pub fn raw(amount: i32) -> Self {
        Self {
            element_type: ElementType::NoElement,
            amount,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DefenseProps {
    pub wind: DamageAdjustment,
    pub water: DamageAdjustment,
    pub land: DamageAdjustment,
    pub any: DamageAdjustment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementType {
    Wind,
    Land,
    Water,
    NoElement,
}

impl ElementType {
    pub fn description(&self) -> String {
        match self {
            ElementType::Wind => "Wind".to_string(),
            ElementType::Land => "Land".to_string(),
            ElementType::Water => "Water".to_string(),
            ElementType::NoElement => "No Elem".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DamageAdjustment {
    Percent(f64),
    Absolute(i32),
    Normal,
}

impl DamageAdjustment {
    pub fn adjust_damage(&self, damage: i32) -> i32 {
        match self {
            DamageAdjustment::Absolute(val) => damage + val,
            DamageAdjustment::Percent(val) => (damage as f64 * val).floor() as i32,
            DamageAdjustment::Normal => damage,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PowerCostAdjust {
    pub card_type: ElementType,
    pub amount: i32,
}
