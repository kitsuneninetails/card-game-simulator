pub mod enemy;
pub mod fp_vec;
pub mod game;
pub mod game_effects;
pub mod player;

use crate::player::PlayerCard;
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
    pub fn is_player(&self) -> bool {
        match self {
            EffectTarget::Player => true,
            EffectTarget::Enemy => false,
        }
    }
    pub fn is_enemy(&self) -> bool {
        match self {
            EffectTarget::Player => false,
            EffectTarget::Enemy => true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum EffectTrigger {
    Always(EffectType),
    Condition(EffectCondition, EffectType),
    Discard(String),
}

impl EffectTrigger {
    pub fn description(&self) -> String {
        match self {
            EffectTrigger::Always(eff) => format!("[ALWAYS] {}", eff.description()),
            EffectTrigger::Condition(cond, eff) => {
                format!("[COND - {}] {}", cond.description(), eff.description())
            }
            EffectTrigger::Discard(id) => format!("Discard card id: {}", id),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EffectCondition {
    PlayerHasCardWithElement(ElementType),
    PlayerHasNoCardWithElement(ElementType),
    PlayerPlaysCardWithElement(ElementType),
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
            EffectCondition::PlayerPlaysCardWithElement(el) => {
                format!("PLAYS_SPELL_ELEMENT [{}]", el.description())
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
            _ => false,
        }
    }

    pub fn check_player_card(&self, card: &PlayerCard) -> bool {
        match self {
            EffectCondition::PlayerPlaysCardWithElement(el) => card.element == *el,
            _ => false,
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
    Enchantment(Enchantment),
    PercentDamage(f64),
    SkipTurn,
}

impl EffectType {
    pub fn description(&self) -> String {
        match self {
            EffectType::Damage(dmg) => {
                format!("Damage [{}/{}]", dmg.element_type.description(), dmg.amount)
            }
            EffectType::LifeAdjust(adj) => format!("Life {}", adj),
            EffectType::Enchantment(ench) => format!("Enchant [{}]", ench.description()),
            EffectType::PercentDamage(dmg) => format!("Damage {}%", dmg),
            EffectType::SkipTurn => format!("Skip Turn"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Enchantment {
    SpellDamageAdjust(ElementType, i32),
    ShieldDamage(i32),
    LifeAdjPerTurn(i32),
    SpellElementForbidden(ElementType),
}

impl Enchantment {
    pub fn description(&self) -> String {
        match self {
            Enchantment::SpellDamageAdjust(cond, amt) => {
                format!("Spell damage adjust {} [{}]", amt, cond.description())
            }
            Enchantment::ShieldDamage(amt) => {
                format!("Damage adjust {} on Attack", amt)
            }
            Enchantment::LifeAdjPerTurn(amt) => {
                format!("Each turn, adjust life by {}", amt)
            }
            Enchantment::SpellElementForbidden(elem) => {
                format!("{} Spells Forbidden", elem.description())
            }
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
