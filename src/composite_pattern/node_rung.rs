use crate::composite_pattern::Component;

pub struct Rung {
    components: Vec<Box<dyn Component>>,
}

impl Rung {
    pub fn new(name: &'static str) -> Self {
        Self {
            components: vec![],
        }
    }

    pub fn add(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }

/*
    pub fn remove(&mut self, component: &dyn Component) {

    }*/

}

impl Component for Rung {
    fn logic(& self) {
        // Iterate through the leaf components and call their logic functions
        for component in self.components.iter() {
            component.logic();
        }
    }
}
