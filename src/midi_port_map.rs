use crate::{
    prelude::*,
    MIDIClient,
    MIDIEndpoint,
};

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
        MIDIPortMapIterator {
            inner: self.inner.iter(),
        }
    }

    fn port_for(&self, endpoint: &MIDIEndpoint) -> Option<&T> {
        let id = endpoint.id();
        self.iter().find(|x| x.1.id() == id).map(|x| x.1)
    }
}

impl MIDIPortMap<MIDIInput> {
    pub(crate) fn new(client: &MIDIClient) -> Self {
        let mut inner = std::collections::HashMap::new();
        for e in coremidi::Sources {
            let input = MIDIInput::new(e);
            inner.insert(input.id(), input);
        }
        Self { inner }
    }
}

impl MIDIPortMap<MIDIOutput> {
    pub(crate) fn new(client: &MIDIClient) -> Self {
        let mut inner = std::collections::HashMap::new();
        for e in coremidi::Destinations {
            let output = MIDIOutput::new(e);
            inner.insert(output.id(), output);
        }
        Self { inner }
    }
}

impl<T: MIDIPort> std::ops::Index<&u32> for MIDIPortMap<T> {
    type Output = T;
    fn index(&self, index: &u32) -> &Self::Output {
        &self.inner[index]
    }
}
