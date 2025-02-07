trait Process {
    fn process(&mut self);
}

pub struct Object {
    pub name: String,
}

impl Process for Object {
    fn process(&mut self) {}
}
