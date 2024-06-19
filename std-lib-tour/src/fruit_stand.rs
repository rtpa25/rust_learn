use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Fruit {
    Apple,
    Banana,
    Orange,
}

pub struct FruitStand {
    pub fruits: HashMap<Fruit, u32>,
}

impl IntoIterator for FruitStand {
    type Item = (Fruit, u32);
    type IntoIter = std::collections::hash_map::IntoIter<Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruits.into_iter() // <- use for moving values
    }
}

impl<'a> IntoIterator for &'a FruitStand {
    type Item = (&'a Fruit, &'a u32);
    type IntoIter = std::collections::hash_map::Iter<'a, Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruits.iter() // <- use for borrowing values
    }
}

impl<'a> IntoIterator for &'a mut FruitStand {
    type Item = (&'a Fruit, &'a mut u32);
    type IntoIter = std::collections::hash_map::IterMut<'a, Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruits.iter_mut() // <- use for borrowing values
    }
}
