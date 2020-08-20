

// TypeMap

use std::collections::HashMap;
use std::any::{TypeId, Any};

// tuple struct hashmap for types, key is hashed id
struct TypeMap(HashMap<TypeId, Box<dyn Any>>);

impl TypeMap {
    pub fn set<T: Any + 'static>(&mut self, t: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(t));
    }
    pub fn has<T: 'static+Any>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }
    
    pub fn get_mut<T: 'static+Any>(&mut self) -> Option<&mut T> {
        self.0.get_mut(&TypeId::of::<T>()).map(|t| {
            t.downcast_mut::<T>().unwrap()
        })
    }
}

let mut map = TypeMap::new();
map.set::<i32>(1);
