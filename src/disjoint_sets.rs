use std::cell::Cell;

pub struct DisjointSets<T> {
    values: Vec<T>,
    parents: Vec<Cell<Option<usize>>>
}

#[derive(Clone, Copy, Debug)]
pub struct Set {
    index: usize
}

impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T> DisjointSets<T> {
    pub fn new() -> Self {
        DisjointSets {
            values: Vec::new(),
            parents: Vec::new()
        }
    }

    pub fn create_set(&mut self, value: T) -> Set {
        let index = self.values.len();
        let set = Set {
            index: index
        };

        self.values.push(value);
        self.parents.push(Cell::new(None));

        set
    }

    pub fn find_root(&self, set: &Set) -> Set {
        match self.parents[set.index].get() {
            Some(parent) => Set { index: parent },
            None => *set
        }
    }

    pub fn get_value(&self, set: &Set) -> &T {
        &self.values[set.index]
    }

    pub fn union(&mut self, first: &Set, second: &Set) -> Set {
        let first_parent = self.find_compress_parent_index(first.index);
        let second_parent = self.find_compress_parent_index(second.index);

        if first_parent != second_parent {
            self.parents[second_parent].set(Some(first_parent))
        }

        Set { index: first_parent }
    }

    fn find_compress_parent_index(&mut self, set_index: usize) -> usize {
        let mut result = set_index;
        while let Some(parent) = self.parents[result].get() {
            result = parent
        }

        if result != set_index {
            self.parents[set_index].set(Some(result))
        }

        result
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn singleton_is_its_root() {
        let mut sets = DisjointSets::new();
        let a = sets.create_set("a");
        let b = sets.create_set("b");

        assert_eq!("a", *sets.get_value(&sets.find_root(&a)));
        assert_eq!("b", *sets.get_value(&sets.find_root(&b)));
    }

    #[test]
    fn union_finds_the_same_parent() {
        let mut sets = DisjointSets::new();
        let a = sets.create_set("a");
        let b = sets.create_set("b");
        let c = sets.union(&a, &b);

        assert_eq!(c, sets.find_root(&a));
        assert_eq!(c, sets.find_root(&b));
    }
}