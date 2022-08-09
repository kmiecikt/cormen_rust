use std::ptr;

pub struct DisjointSet<T> {
    sets: Vec<Box<Set<T>>>
}

pub struct Set<T> {
    pub value: T,
    parent: *mut Set<T>
}

impl<T> DisjointSet<T> {
    pub fn new() -> DisjointSet<T> {
        DisjointSet { sets: Vec::new() }
    }   
    
    pub fn create_set(&mut self, value: T) -> &mut Set<T> {
       let set = Box::new(Set {
            value: value,
            parent: ptr::null_mut()
       });

       self.sets.push(set);
       let index = self.sets.len() - 1;
       
       &mut (*self.sets[index])
    }
    
}

impl<T> Set<T> {
    pub fn find_root(&mut self) -> &Set<T> {
        unsafe {
            &(*self.find_compress_parent())
        }
    }
    
    pub fn union<'a>(first: &'a mut Set<T>, second: &mut Set<T>) -> &'a Set<T> {
        unsafe {
            let first_root = Set::find_compress_parent(first);
            let mut second_root = Set::find_compress_parent(second);
            
            if first_root != second_root {
                (*second_root).parent = first_root;
            }

            &(*first_root)
        }
    }
    
    unsafe fn find_compress_parent(&mut self) -> *mut Set<T> {
        let mut self_ptr: *mut Set<T> = self;
        let mut root = self_ptr;
        while !(*root).parent.is_null() {
            root = (*root).parent;
        }
        
        if self_ptr != root {
            (*self_ptr).parent = root;
        }
        
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_root_for_new_set_returns_self() {
        let mut sets = DisjointSet::new();
        let mut a = sets.create_set("a");
    }
}