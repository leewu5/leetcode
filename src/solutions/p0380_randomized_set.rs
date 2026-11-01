use std::collections::HashMap;

use rand::Rng;

pub struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
    size: usize,
}

impl RandomizedSet {

    pub fn new() -> Self {
        Self { 
            map: HashMap::new(), 
            list: vec![], 
            size: 0 
        }
    }
    
    pub fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) { return false; }
        if self.list.len() > self.size { self.list[self.size] = val; }
        else { self.list.push(val); }
        self.map.insert(val, self.size);
        self.size += 1;
        return true;
    }
    
    pub fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) { return false; }
        self.size -= 1;
        let index = self.map.get(&val).unwrap();
        self.list[*index] = self.list[self.size];
        self.map.insert(self.list[self.size], *index);
        self.map.remove(&val);
        return true;
    }
    
    pub fn get_random(&self) -> i32 {
        return self.list[rand::thread_rng().gen_range(0..self.size)];
    }
}