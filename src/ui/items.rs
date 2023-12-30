use std::rc::Rc;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Item {
    // Because a guy named Logan Smith told me so
    name: Rc<str>,
    description: Rc<str>,
    value: u32,
}

impl Item {
    pub fn slime() -> Self {
        Item {
            name: "Slime".into(),
            value: 5,
            description: "It's quite slimy".into(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_lowercase()
    }
}
