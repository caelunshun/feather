use std::convert::TryInto;
use std::mem::ManuallyDrop;
use std::num::NonZeroUsize;

/// A packed array of integers where each integer consumes
/// `bits_per_value` bits. Used to store block data in chunks.
#[derive(Debug, Clone, PartialEq)]
pub struct PackedArray {
    length: usize,
    bits_per_value: NonZeroUsize,
    bits: Vec<u64>,
}

impl PackedArray {
    /// Creates a new `PackedArray` with the given length
    /// and number of bits per value. Values are initialized
    /// to zero.
    ///
    /// # Panics
    /// Panics if `bits_per_value > 64`.
    pub fn new(length: usize, bits_per_value: NonZeroUsize) -> Self {
        let mut this = Self {
            length,
            bits_per_value,
            bits: Vec::new(),
        };
        let needed_u64s = this.needed_u64s();
        this.bits = vec![0u64; needed_u64s];

        this
    }

    /// Creates a `PackedArray` from raw `u64` data
    /// and a length.
    pub fn from_u64_vec(bits: Vec<u64>, length: usize) -> Self {
        assert!(!bits.is_empty());
        let bits_per_value = (bits.len() * u64::BITS as usize / length)
            .try_into()
            .unwrap();
        Self {
            length,
            bits_per_value,
            bits,
        }
    }

    /// Creates a `PackedArray` from raw `i64` data
    /// and a length.
    pub fn from_i64_vec(bits: Vec<i64>, length: usize) -> Self {
        // SAFETY: i64 and u64 have the same memory layout
        let mut bits = ManuallyDrop::new(bits);
        Self::from_u64_vec(
            unsafe {
                Vec::from_raw_parts(bits.as_mut_ptr() as *mut u64, bits.len(), bits.capacity())
            },
            length,
        )
    }

    /// Gets the value at the given index.
    #[inline]
    pub fn get(&self, index: usize) -> Option<u64> {
        if index >= self.len() {
            return None;
        }

        let (u64_index, bit_index) = self.indexes(index);

        let u64 = self.bits[u64_index];
        Some((u64 >> bit_index) & self.mask())
    }

    /// Sets the value at the given index.
    ///
    /// # Panics
    /// Panics if `index >= self.length()` or `value > self.max_value()`.
    #[inline]
    pub fn set(&mut self, index: usize, value: u64) {
        debug_assert!(
            index < self.len(),
            "index out of bounds: index is {}; length is {}",
            index,
            self.len()
        );

        let mask = self.mask();
        debug_assert!(value <= mask);

        let (u64_index, bit_index) = self.indexes(index);

        let u64 = &mut self.bits[u64_index];
        *u64 &= !(mask << bit_index);
        *u64 |= value << bit_index;
    }

    /// Sets all values is the packed array to `value`.
    ///
    /// # Panics
    /// Panics if `value > self.max_value()`.
    pub fn fill(&mut self, value: u64) {
        assert!(value <= self.max_value());
        let mut x = 0;
        for i in 0..self.values_per_u64() {
            x |= value << (i * self.bits_per_value.get());
        }

        self.bits.fill(x);
    }

    /// Returns an iterator over values in this array.
    pub fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        let values_per_u64 = self.values_per_u64();
        let bits_per_value = self.bits_per_value().get() as u64;
        let mask = self.mask();
        let length = self.len();

        self.bits
            .iter()
            .flat_map(move |&u64| {
                (0..values_per_u64).map(move |i| (u64 >> (i as u64 * bits_per_value)) & mask)
            })
            .take(length)
    }

    /// Resizes this packed array to a new bits per value.
    #[must_use = "method returns a new array and does not mutate the original value"]
    pub fn resized(&self, new_bits_per_value: NonZeroUsize) -> PackedArray {
        Self::from_iter(self.iter(), new_bits_per_value)
    }

    /// Collects an iterator into a `PackedArray`.
    pub fn from_iter(iter: impl IntoIterator<Item = u64>, bits_per_value: NonZeroUsize) -> Self {
        assert!(bits_per_value.get() <= 64);
        let iter = iter.into_iter();
        let mut bits = Vec::with_capacity(iter.size_hint().0);

        let mut current_u64 = 0u64;
        let mut current_offset = 0;
        let mut length = 0;

        for value in iter {
            debug_assert!(value < 1 << bits_per_value.get());
            current_u64 |= value << current_offset;

            current_offset += bits_per_value.get();
            if current_offset > 64 - bits_per_value.get() {
                bits.push(current_u64);
                current_offset = 0;
                current_u64 = 0;
            }

            length += 1;
        }

        if current_offset != 0 {
            bits.push(current_u64);
        }

        Self {
            length,
            bits_per_value,
            bits,
        }
    }

    /// Returns the maximum value of an integer in this packed array.
    #[inline]
    pub fn max_value(&self) -> u64 {
        self.mask()
    }

    /// Returns the length of this packed array.
    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    /// Determines whether the length of this array is 0.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of bits used to represent each value.
    #[inline]
    pub fn bits_per_value(&self) -> NonZeroUsize {
        self.bits_per_value
    }

    /// Sets the number of bits used to represent each value.
    pub fn set_bits_per_value(&mut self, new_value: NonZeroUsize) {
        self.bits_per_value = new_value;
    }

    /// Gets the raw `u64` data.
    pub fn as_u64_slice(&self) -> &[u64] {
        &self.bits
    }

    /// Gets the raw mutable `u64` data.
    pub fn as_u64_mut_vec(&mut self) -> &mut Vec<u64> {
        &mut self.bits
    }

    fn mask(&self) -> u64 {
        (1 << self.bits_per_value.get()) - 1
    }

    fn needed_u64s(&self) -> usize {
        (self.length + self.values_per_u64() - 1) / self.values_per_u64()
    }

    fn values_per_u64(&self) -> usize {
        64 / self.bits_per_value.get()
    }

    fn indexes(&self, index: usize) -> (usize, usize) {
        let vales_per_u64 = self.values_per_u64();
        let u64_index = index / vales_per_u64;
        let index = index % vales_per_u64;
        (u64_index, index * self.bits_per_value.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn smoke() {
        let length = 100;
        let mut array = PackedArray::new(length, 10.try_into().unwrap());
        assert_eq!(array.len(), length);
        assert_eq!(array.bits_per_value().get(), 10);
        assert_eq!(array.bits.len(), 17);

        for i in 0..length {
            assert_eq!(array.get(i), Some(0));
            array.set(i, (i * 10) as u64);
            assert_eq!(array.get(i), Some((i * 10) as u64));
        }
    }

    #[test]
    fn out_of_bounds() {
        let array = PackedArray::new(97, 10.try_into().unwrap());
        assert_eq!(array.bits.len(), 17);
        assert_eq!(array.get(96), Some(0));
        assert_eq!(array.get(97), None);
    }

    #[test]
    fn iter() {
        let mut array = PackedArray::new(10_000, 10.try_into().unwrap());
        let mut oracle = Vec::new();

        for i in 0..array.len() {
            let value = i as u64;
            oracle.push(value);
            array.set(i, value);
            assert_eq!(array.get(i), Some(value));
        }

        for (i, &oracle_value) in oracle.iter().enumerate() {
            assert_eq!(array.get(i), Some(oracle_value));
        }

        for (value, &oracle_value) in array.iter().zip(oracle.iter()) {
            assert_eq!(value, oracle_value);
        }
    }

    #[test]
    fn resize() {
        let length = 1024;
        let mut array = PackedArray::new(length, 1.try_into().unwrap());

        let mut oracle = Vec::new();
        for new_bits_per_value in 2..=16 {
            for i in 0..array.len() {
                let value = i as u64;
                array.set(i, value);
                oracle.push(value);
            }

            for (i, &oracle_value) in oracle.iter().enumerate() {
                assert_eq!(array.get(i), Some(oracle_value));
            }

            array = array.resized(new_bits_per_value.try_into().unwrap());

            for (i, &oracle_value) in oracle.iter().enumerate() {
                assert_eq!(array.get(i), Some(oracle_value));
            }

            oracle.clear();
        }
    }

    #[test]
    fn fill() {
        let mut array = PackedArray::new(1024, 10.try_into().unwrap());
        array.fill(102);
        assert!(array.iter().all(|x| x == 102));

        array.fill(256);
        assert!(array.iter().all(|x| x == 256));
    }

    #[test]
    #[should_panic]
    fn fill_too_large() {
        let mut array = PackedArray::new(100, 10.try_into().unwrap());
        array.fill(1024); // 1024 == 2^10
    }
}
