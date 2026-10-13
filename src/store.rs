pub struct Store {
    pub items: Vec<String>,
}

impl Store {
    pub fn add(&mut self, item: String) {
        self.items.push(item);
    }
}
