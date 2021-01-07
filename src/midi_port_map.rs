use crate::prelude::*;

pub struct MIDIPortMapIterator<'a, T: MIDIPort> {
    inner: std::collections::hash_map::Iter<'a, u32, T>,
}

impl<'a, T: MIDIPort> Iterator for MIDIPortMapIterator<'a, T> {
    type Item = (&'a u32, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct MIDIPortMap<T: MIDIPort> {
    inner: std::collections::HashMap<u32, T>,
}

impl<T: MIDIPort> MIDIPortMap<T> {
    pub fn iter(&self) -> MIDIPortMapIterator<T> {
        MIDIPortMapIterator { inner: self.inner.iter() }
    }
}

impl MIDIPortMap<MIDIInput> {
    pub fn new() -> Self {
        // use std::collections::hash_map::Iter;
        todo!()
    }
}

impl MIDIPortMap<MIDIOutput> {
    pub fn new() -> Self {
        todo!()
    }
}
