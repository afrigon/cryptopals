pub enum BinaryState {
    Off = 0,
    On = 1,
}

pub struct Binary<'a> {
    data: &'a [u8],
    len: usize,
}

impl<'a> Binary<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            len: data.len(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<'a> IntoIterator for Binary<'a> {
    type Item = BinaryState;
    type IntoIter = BinaryIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            index: 0,
            local_index: 0,
            binary: self,
        }
    }
}

pub struct BinaryIterator<'a> {
    index: usize,
    local_index: u8,
    binary: Binary<'a>,
}

impl<'a> Iterator for BinaryIterator<'a> {
    type Item = BinaryState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.local_index == 8 {
            self.local_index = 0;
            self.index += 1;
        }

        if self.index == self.binary.len() {
            return None;
        }

        let result = if self.binary.data[self.index] >> (7 - self.local_index) & 0b1 == 1 {
            BinaryState::On
        } else {
            BinaryState::Off
        };

        self.local_index += 1;

        Some(result)
    }
}
