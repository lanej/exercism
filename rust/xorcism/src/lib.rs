/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    pos: usize,
}

/// For composability, it is important that `munge` returns an iterator compatible with its input.
///
/// However, `impl Trait` syntax can specify only a single non-auto trait.
/// Therefore, we define this output trait with generic implementations on all compatible types,
/// and return that instead.
pub trait MungeOutput: Iterator<Item = u8> + ExactSizeIterator {}
impl<T> MungeOutput for T where T: Iterator<Item = u8> + ExactSizeIterator {}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<T: AsRef<[u8]>>(key: &'a T) -> Xorcism<'a> {
        Self {
            key: key.as_ref(),
            pos: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        unimplemented!()
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<T: AsRef<[u8]>>(&mut self, data: T) -> impl MungeOutput {
        let plaintext = data.as_ref();

        self.pos = (self.pos + plaintext.len()) % self.key.len();

        plaintext
            .iter()
            .zip(self.key.iter().cycle().skip(self.pos))
            .map(|(p, d)| p ^ d)
            .collect::<Vec<u8>>()
            .into_iter()
    }
}
