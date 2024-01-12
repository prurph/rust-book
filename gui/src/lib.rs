pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw> is the trait object. Users of the library can add their own components.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "I'm a {}x{} button, and I say: {}",
            self.width, self.height, self.label
        )
    }
}
