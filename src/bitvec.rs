const BITS: usize = 64;
const SHIFT_WIDTH: usize = 6;

pub struct BitVec {
    data: Vec<u64>,
    length: usize,
}

impl BitVec {
    /// converts idx to pos
    /// idx => index into bitvec
    /// pos => index into Vec<T> which is used to store the bitvec
    #[inline(always)]
    fn idx_to_pos(idx: usize) -> usize {
        idx >> SHIFT_WIDTH
    }

    #[inline(always)]
    fn get_mask(idx: usize) -> u64 {
        (1 as u64) << (idx & (BITS - 1))
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(Self::idx_to_pos(cap + BITS - 1)),
            length: 0,
        }
    }

    pub fn push_true(&mut self) {
        let idx = Self::idx_to_pos(self.length);

        if idx >= self.data.len() {
            self.data.push(0);
        }

        let mask = Self::get_mask(self.length);
        self.data[idx] |= mask;
        self.length += 1;
    }

    pub fn push_false(&mut self) {
        let idx = Self::idx_to_pos(self.length);

        if idx >= self.data.len() {
            self.data.push(0);
        }

        self.length += 1;
    }

    pub fn push(&mut self, val: bool) {
        if val {
            self.push_true();
        } else {
            self.push_false();
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        assert!(idx < self.length);
        let pos = Self::idx_to_pos(idx);
        let mask = Self::get_mask(idx);
        (self.data[pos] & mask) != 0
    }
}


#[cfg(test)]
mod tests {
    use crate::bitvec::BitVec;

    #[test]
    fn test_idx_to_pos() {
        assert_eq!(BitVec::idx_to_pos(0), 0);
        assert_eq!(BitVec::idx_to_pos(63), 0);
        assert_eq!(BitVec::idx_to_pos(64), 1);
    }

    #[test]
    fn test_push() {
        let mut bv = BitVec::with_capacity(10);
        bv.push_true();
        bv.push_false();
        bv.push_true();
        assert_eq!(bv.get(0), true);
        assert_eq!(bv.get(1), false);
        assert_eq!(bv.get(2), true);
    }

    #[test]
    #[should_panic]
    fn test_panic_oob() {
        let mut bv = BitVec::with_capacity(10);
        bv.push_true();
        assert_eq!(bv.get(1), true);
    }

    #[test]
    fn test_across_edge() {
        let mut x = BitVec::with_capacity(0);
        for j in 0..2 {
            for i in 0..64 {
                x.push((i+j)%2 == 0);
            }
        }
        for j in 0..2 {
            for i in 0..64 {
                assert_eq!(x.get(i+j*64), (i+j)%2 == 0);
            }
        }
    }

}
