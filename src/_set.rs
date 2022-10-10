use super::*;

use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap)]
pub struct Set<T>(Vec<T>) where T: MortonOrd;

impl Set {
    pub fn get(&self, x: u16, y: u16) -> Option<&u32> {
        self.binary_search(&encode_2d(x, y))
            .map(|i| &self.0[i])
            .ok()
    }
    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut u32> {
        self.binary_search(&encode_2d(x, y))
            .map(move |i| &mut self.0[i])
            .ok()
    }
    pub fn remove(&mut self, x: u16, y: u16) -> Option<u32> {
        self.binary_search(&encode_2d(x, y))
            .map(|i| self.0.remove(i))
            .ok()
        
    }
    pub fn insert(&mut self, x: u16, y: u16) -> Option<&u32> {
        let mort_index = encode_2d(x, y);
        match self.binary_search(&mort_index) {
            Ok(i) => Some(&self.0[i]),
            Err(i) if i < self.len() => { self.0.insert(i, mort_index); None },
            Err(_) => { self.0.push(mort_index); None },
        }
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut u32> + '_ {
        SortOnDrop(&mut self.0, 0)
    }
}

impl From<&[u32]> for Set {
    fn from(entries: &[u32]) -> Self {
        let mut out = entries.to_vec();
        out.sort();
        Self(out)
    }
}
impl From<Vec<u32>> for Set {
    fn from(mut entries: Vec<u32>) -> Self {
        entries.sort();
        Self(entries)
    }
}
use std::iter::FromIterator;
impl FromIterator<u32> for Set {
    fn from_iter<I: IntoIterator<Item=u32>>(iter: I) -> Self {
        let mut mv = Set(vec![]);
        for el in iter {
            mv.0.push(el)
        }
        mv.0.sort();
        mv
    }
}

impl FromIterator<(u16, u16)> for Set {
    fn from_iter<I: IntoIterator<Item=(u16, u16)>>(iter: I) -> Self {
        let mut mv = Set(vec![]);
        for (x, y) in iter {
            mv.0.push(encode_2d(x, y));
        }
        mv.0.sort();
        mv
    }
}


struct SortOnDrop<'a, T: Ord + Copy>(&'a mut [T], usize);

impl<'a, T: Ord + Copy> Iterator for SortOnDrop<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.1 += 1;
        if self.1 - 1  < self.0.len() {
            unsafe {Some(&mut *self.0.as_mut_ptr().add(self.1-1))}
        } else {
            None
        }
    }
}

impl<'a, T: Ord + Copy> Drop for SortOnDrop<'a, T> {
    fn drop(&mut self) {
        self.0.sort();
    }
}
