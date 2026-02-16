//! Actor system: base for all objects in the world.

pub struct Actor {
    pub id: u64,
    pub name: String,
    pub components: Vec<Box<dyn Component>>,
}

pub trait Component {
    fn update(&mut self, delta_time: f32);
}

impl Actor {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            components: Vec::new(),
        }
    }
    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }
    pub fn update(&mut self, delta_time: f32) {
        for c in &mut self.components {
            c.update(delta_time);
        }
    }
}
