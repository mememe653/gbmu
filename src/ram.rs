use std::collections::HashMap;

#[derive(Clone)]
pub struct RAM {
    hash_table: HashMap<u16, u8>,
}

impl RAM {
    pub fn new() -> Self {
        Self {
            hash_table: HashMap::new(),
        }
    }

    pub fn load(&self, address: u16) -> u8 {
        *self.hash_table.get(&address).unwrap()
    }

    pub fn store(&mut self, address: u16, val: u8) {
        self.hash_table.insert(address, val);
    }
}
