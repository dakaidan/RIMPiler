pub struct NameGenerator {
    name: String,
    counter: usize,
}

impl NameGenerator {
    pub fn new(base: String) -> Self {
        let mut name = String::from("generated_name_");
        name.push_str(&base);
        Self { name, counter: 0 }
    }

    pub fn generate(&mut self) -> String {
        let mut name = self.name.clone();
        name.push_str(&self.counter.to_string());
        self.counter += 1;
        name
    }

    fn reset(&mut self) {
        self.counter = 0;
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_counter(&self) -> usize {
        self.counter
    }

    fn set_counter(&mut self, counter: usize) {
        self.counter = counter;
    }
}
