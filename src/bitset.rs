pub struct BitSet {
    capacity_in_bits: usize,
    words: Vec<u64>,
}

impl BitSet {
    const SHIFT_OFFSET: usize = 6;
    const BITS_IN_WORD: usize = 64;

    pub fn new(capacity_in_bits: usize) -> BitSet {
        let capacity_in_words = BitSet::word_index(capacity_in_bits - 1) + 1;
        BitSet {
            capacity_in_bits: capacity_in_words << BitSet::SHIFT_OFFSET,
            words: vec![0; capacity_in_words],
        }
    }

    pub fn set(&mut self, bit_index: usize, value: bool) -> bool {
        let word_index = BitSet::word_index(bit_index);
        let bit_mask: u64 = 1 << (bit_index % BitSet::BITS_IN_WORD);
        let old_value = self.words[word_index] & bit_mask != 0;
        if value {
            self.words[word_index] |= bit_mask;
        } else {
            self.words[word_index] &= !bit_mask;
        }
        old_value
    }

    pub fn get(&self, bit_index: usize) -> bool {
        let word_index = BitSet::word_index(bit_index);
        let bit_mask: u64 = 1 << (bit_index % BitSet::BITS_IN_WORD);
        self.words[word_index] & bit_mask != 0
    }

    pub fn capacity_in_bits(&self) -> usize {
        self.capacity_in_bits
    }

    fn word_index(bit_index: usize) -> usize {
        bit_index >> BitSet::SHIFT_OFFSET
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitset_capacity_rounds_to_the_closest_word() {
        assert_eq!(BitSet::new(16).capacity_in_bits(), 64);
        assert_eq!(BitSet::new(64).capacity_in_bits(), 64);
        assert_eq!(BitSet::new(65).capacity_in_bits(), 128);
        assert_eq!(BitSet::new(150).capacity_in_bits(), 192);
    }

    #[test]
    fn bitset_set_and_get() {
        let mut set = BitSet::new(128);
        for i in 0..(set.capacity_in_bits) {
            assert_eq!(set.get(i), false);
        }
        for i in 0..(set.capacity_in_bits) {
            assert_eq!(set.set(i, i % 2 == 0), false);
        }
        for i in 0..(set.capacity_in_bits) {
            assert_eq!(set.get(i), i % 2 == 0);
        }
    }
}
