use super::object::Object;

trait Process {
    fn process(&mut self);
}

pub struct Scene {
    pub objects: Vec<Object>,
}

impl Process for Scene {
    fn process(&mut self) {
        for object in self.objects.iter() {
            println!("this object name is : {}", object.name); // a test to make sure
        }
    }
}
