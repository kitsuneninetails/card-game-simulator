use crate::{EffectTarget, EffectTrigger};

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
