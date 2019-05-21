use super::bitset::BitSet;

pub struct RingBitSet {
    bitset: BitSet,
    capacity_in_bits: usize,
    next_index: usize,
    cardinality: usize,
    length: usize,
}

impl RingBitSet {
    pub fn new(capacity_in_bits: usize) -> RingBitSet {
        RingBitSet {
            bitset: BitSet::new(capacity_in_bits),
            capacity_in_bits: capacity_in_bits,
            next_index: 0,
            cardinality: 0,
            length: 0,
        }
    }

    pub fn set_next_bit(&mut self, value: bool) -> usize {
        if self.length < self.capacity_in_bits {
            self.length += 1
        }
        let old_value = self.bitset.set(self.next_index, value);
        self.cardinality = self.cardinality - (old_value as usize) + (value as usize);
        self.next_index = (self.next_index + 1) % self.capacity_in_bits;
        self.cardinality
    }

    pub fn cardinality(&self) -> usize {
        self.cardinality
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn capacity_in_bits(&self) -> usize {
        self.capacity_in_bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ringbitset() {
        let mut set = RingBitSet::new(4);
        assert_eq!(set.cardinality(), 0);
        assert_eq!(set.length(), 0);

        set.set_next_bit(true);
        assert_eq!(set.cardinality(), 1);
        assert_eq!(set.length(), 1);

        set.set_next_bit(false);
        assert_eq!(set.cardinality(), 1);
        assert_eq!(set.length(), 2);

        set.set_next_bit(true);
        assert_eq!(set.cardinality(), 2);
        assert_eq!(set.length(), 3);

        set.set_next_bit(true);
        assert_eq!(set.cardinality(), 3);
        assert_eq!(set.length(), 4);

        set.set_next_bit(false);
        assert_eq!(set.cardinality(), 2);
        assert_eq!(set.length(), 4);
    }
}
