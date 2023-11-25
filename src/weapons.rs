use crate::logic::STANDARD_SQUARE;

#[derive(Clone)]
pub struct Weapon {
    pub base_damage: f32,
    pub lenght: f32,
    pub cooldown: f32,
    pub angle: f32,
}

impl Weapon {
    pub fn sword() -> Self {
        Weapon {
            base_damage: 10.,
            lenght: STANDARD_SQUARE,
            cooldown: 0.2,
            angle: 0.,
        }
    }
}
