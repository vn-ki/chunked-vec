use crate::bitvec;

pub struct ChunkedVecInt {
   pub bitmap: bitvec::BitVec,
   pub data: Vec<usize>
}

impl ChunkedVecInt {
   pub fn new() -> Self {
      Self {
         bitmap: bitvec::BitVec::with_capacity(0),
         data: Vec::new(),
      }
   }

   pub fn append(&mut self, val: Option<usize>) {
      if let Some(x) = val {
         self.bitmap.push_true();
         self.data.push(x);
      } else {
         self.bitmap.push_false();
         self.data.push(0);
      }
   }

   pub fn get(&self, idx: usize) -> Option<&usize> {
      if self.bitmap.get(idx) {
         Some(&self.data[idx])
      } else {
         None
      }
   }
}

#[cfg(test)]
mod tests {
   use crate::chunked_vec::ChunkedVecInt;

   #[test]
   fn append_and_get() {
      let mut c = ChunkedVecInt::new();
      c.append(Some(1));
      c.append(None);
      c.append(None);
      c.append(Some(4));

      assert_eq!(c.get(0), Some(&1));
      assert_eq!(c.get(1), None);
      assert_eq!(c.get(2), None);
      assert_eq!(c.get(3), Some(&4));
   }

   #[test]
   fn append_and_get_over_edge() {
      let mut c = ChunkedVecInt::new();
      for _ in 0..2 {
         for i in 0..64 {
            if i % 2 == 0 {
               c.append(Some(i));
            } else {
               c.append(None);
            }
         }
      }
      for j in 0..2 {
         for i in 0..64 {
            let curr = c.get(i + j*64);
            if i % 2 == 0 {
               assert_eq!(curr.unwrap(), &i);
            } else {
               assert!(curr.is_none());
            }
         }
      }
   }
}
