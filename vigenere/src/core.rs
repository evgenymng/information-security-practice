pub(crate) fn validate_key(key: &str, alphabet: &[char]) -> bool {
    !key.is_empty() && key.chars().all(|c| alphabet.contains(&c))
}

/// This is the looping key implementation (as I call it).
/// It must be made up of characters from the alphabet, and you
/// can pop `char`s from its iterator infinitely. Once the
/// internal counter reaches the end of the string, it loops back
/// to the start.
pub(crate) struct LoopingKey {
    inner: String,
}

impl LoopingKey {
    pub(crate) fn new(s: &str) -> Self {
        Self {
            inner: s.to_owned(),
        }
    }
}

impl IntoIterator for LoopingKey {
    type Item = char;
    type IntoIter = LoopingKeyIterator;

    fn into_iter(self) -> Self::IntoIter {
        LoopingKeyIterator {
            max_idx: self.inner.chars().count(),
            idx: 0,
            inner: self.inner,
        }
    }
}

/// Iterator that loops over its string content.
pub(crate) struct LoopingKeyIterator {
    max_idx: usize,
    idx: usize,
    inner: String,
}

impl Iterator for LoopingKeyIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.chars().nth(self.idx).map(|c| {
            self.idx = (self.idx + 1) % self.max_idx;
            c
        })
    }
}
