use crate::character::create::Character;
use crate::npcs::enemy::Enemy;

pub trait HasEffects {
    fn effects_mut(&mut self) -> &mut Vec<String>;
    fn hp_mut(&mut self) -> &mut u32;
    fn name(&self) -> &str;
}

impl HasEffects for Character {
    fn effects_mut(&mut self) -> &mut Vec<String> {
        &mut self.effects // or a real `effects` field if u want
    }
    fn hp_mut(&mut self) -> &mut u32 {
        &mut self.hp
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl HasEffects for Enemy {
    fn effects_mut(&mut self) -> &mut Vec<String> {
        &mut self.effects
    }
    fn hp_mut(&mut self) -> &mut u32 {
        &mut self.hp
    }
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct Effect {
    pub name: &'static str,
    pub desc: &'static str,
    pub apply: fn(target: &mut dyn HasEffects),
}

fn burn(target: &mut dyn HasEffects) {
    let hp = target.hp_mut();
    *hp = hp.saturating_sub(3);
    println!("{} is burning and takes 3 damage!", target.name());
}

fn poison(target: &mut dyn HasEffects) {
    let hp = target.hp_mut();
    *hp = hp.saturating_sub(2);
    println!("{} is poisoned and loses 2 HP.", target.name());
}

fn freeze(_: &mut dyn HasEffects) {
    println!("Frozen — no effect this turn (yet)");
}

pub fn all_effects() -> Vec<Effect> {
    vec![
        Effect {
            name: "burn",
            desc: "Deals 3 damage per turn.",
            apply: burn,
        },
        Effect {
            name: "poison",
            desc: "Deals 2 damage per turn.",
            apply: poison,
        },
        Effect {
            name: "freeze",
            desc: "Prevents some actions (not implemented).",
            apply: freeze,
        },
    ]
}

pub fn apply_reoccurring_effects<T: HasEffects>(target: &mut T) {
    let effects = all_effects();

    for active in target.effects_mut().clone() {
        if let Some(effect) = effects.iter().find(|e| e.name == active) {
            (effect.apply)(target);
        }
    }
}

