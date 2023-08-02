pub mod enemy;
pub mod fp_vec;
pub mod game;
pub mod player;

use enemy::Enemy;
use player::Player;

#[derive(Debug, Clone)]
pub struct GameEffect {
    target: EffectTarget,
    effect: EffectTrigger,
}

impl GameEffect {
    pub fn player(effect: EffectTrigger) -> Self {
        Self {
            target: EffectTarget::Player,
            effect,
        }
    }

    pub fn enemy(effect: EffectTrigger) -> Self {
        Self {
            target: EffectTarget::Enemy,
            effect,
        }
    }
}

#[derive(Debug, Clone)]
pub enum EffectTarget {
    Player,
    Enemy,
}

#[derive(Debug, Clone)]
pub enum EffectTrigger {
    Always(EffectType),
    Condition(EffectCondition, EffectType),
}

#[derive(Debug, Clone)]
pub enum EffectCondition {
    PlayerHasCardWithElement(ElementType),
    PlayerHasNoCardWithElement(ElementType),
}

impl EffectCondition {
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

#[derive(Debug, Clone)]
pub enum Enchantment {
    SpellCostAdjust(ElementType, i32),
    SpellDamageAdjust(ElementType, i32),
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
