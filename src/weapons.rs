use crate::logic::TILE;

#[derive(Clone)]
pub struct Weapon {
    pub base_damage: f32,
    pub lenght: f32,
    pub cooldown: f32,
}

impl Weapon {
    pub fn rusty_sword() -> Self {
        Weapon {
            base_damage: 10.,
            lenght: TILE,
            cooldown: 0.3,
        }
    }

    pub fn black_sword() -> Self {
        Weapon {
            base_damage: 20.,
            lenght: TILE * 1.2,
            cooldown: 0.4,
        }
    }
}
