#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MaxTracker<T>(Option<T>);

impl<T> MaxTracker<T> {
    pub fn new() -> MaxTracker<T> {
        MaxTracker(None)
    }

    pub fn get_ref(&self) -> Option<&T> {
        self.0.as_ref()
    }
}

impl<T: Ord> MaxTracker<T> {
    pub fn add(&mut self, value: T) {
        if self.0.as_ref().is_none_or(|prev| *prev < value) {
            self.0 = Some(value);
        }
    }
}

impl<T: Copy> MaxTracker<T> {
    pub fn get(&self) -> Option<T> {
        self.0
    }
}

impl<T> Default for MaxTracker<T> {
    fn default() -> MaxTracker<T> {
        MaxTracker::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FirstMaxKeyTracker<K, V>(Option<(K, V)>);

impl<K, V> FirstMaxKeyTracker<K, V> {
    pub fn new() -> FirstMaxKeyTracker<K, V> {
        FirstMaxKeyTracker(None)
    }

    pub fn get_ref(&self) -> Option<&(K, V)> {
        self.0.as_ref()
    }

    pub fn get_key_ref(&self) -> Option<&K> {
        self.0.as_ref().map(|(key, _)| key)
    }

    pub fn get_value_ref(&self) -> Option<&V> {
        self.0.as_ref().map(|(_, value)| value)
    }
}

impl<K, V: Ord> FirstMaxKeyTracker<K, V> {
    pub fn add(&mut self, key: K, value: V) {
        if self.0.as_ref().is_none_or(|(_, prev)| *prev < value) {
            self.0 = Some((key, value));
        }
    }
}

impl<K: Copy, V: Copy> FirstMaxKeyTracker<K, V> {
    pub fn get(&self) -> Option<(K, V)> {
        self.0
    }
}

impl<K: Copy, V> FirstMaxKeyTracker<K, V> {
    pub fn get_key(&self) -> Option<K> {
        self.get_key_ref().copied()
    }
}

impl<K, V: Copy> FirstMaxKeyTracker<K, V> {
    pub fn get_value(&self) -> Option<V> {
        self.get_value_ref().copied()
    }
}

impl<K, V> Default for FirstMaxKeyTracker<K, V> {
    fn default() -> FirstMaxKeyTracker<K, V> {
        FirstMaxKeyTracker::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LastMaxKeyTracker<K, V>(Option<(K, V)>);

impl<K, V> LastMaxKeyTracker<K, V> {
    pub fn new() -> LastMaxKeyTracker<K, V> {
        LastMaxKeyTracker(None)
    }

    pub fn get_ref(&self) -> Option<&(K, V)> {
        self.0.as_ref()
    }

    pub fn get_key_ref(&self) -> Option<&K> {
        self.0.as_ref().map(|(key, _)| key)
    }

    pub fn get_value_ref(&self) -> Option<&V> {
        self.0.as_ref().map(|(_, value)| value)
    }
}

impl<K, V: Ord> LastMaxKeyTracker<K, V> {
    pub fn add(&mut self, key: K, value: V) {
        if self.0.as_ref().is_none_or(|(_, prev)| *prev <= value) {
            self.0 = Some((key, value));
        }
    }
}

impl<K: Copy, V: Copy> LastMaxKeyTracker<K, V> {
    pub fn get(&self) -> Option<(K, V)> {
        self.0
    }
}

impl<K: Copy, V> LastMaxKeyTracker<K, V> {
    pub fn get_key(&self) -> Option<K> {
        self.get_key_ref().copied()
    }
}

impl<K, V: Copy> LastMaxKeyTracker<K, V> {
    pub fn get_value(&self) -> Option<V> {
        self.get_value_ref().copied()
    }
}

impl<K, V> Default for LastMaxKeyTracker<K, V> {
    fn default() -> LastMaxKeyTracker<K, V> {
        LastMaxKeyTracker::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MinTracker<T>(Option<T>);

impl<T> MinTracker<T> {
    pub fn new() -> MinTracker<T> {
        MinTracker(None)
    }

    pub fn get_ref(&self) -> Option<&T> {
        self.0.as_ref()
    }
}

impl<T: Ord> MinTracker<T> {
    pub fn add(&mut self, value: T) {
        if self.0.as_ref().is_none_or(|prev| *prev > value) {
            self.0 = Some(value);
        }
    }
}

impl<T: Copy> MinTracker<T> {
    pub fn get(&self) -> Option<T> {
        self.0
    }
}

impl<T> Default for MinTracker<T> {
    fn default() -> MinTracker<T> {
        MinTracker::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FirstMinKeyTracker<K, V>(Option<(K, V)>);

impl<K, V> FirstMinKeyTracker<K, V> {
    pub fn new() -> FirstMinKeyTracker<K, V> {
        FirstMinKeyTracker(None)
    }

    pub fn get_ref(&self) -> Option<&(K, V)> {
        self.0.as_ref()
    }

    pub fn get_key_ref(&self) -> Option<&K> {
        self.0.as_ref().map(|(key, _)| key)
    }

    pub fn get_value_ref(&self) -> Option<&V> {
        self.0.as_ref().map(|(_, value)| value)
    }
}

impl<K, V: Ord> FirstMinKeyTracker<K, V> {
    pub fn add(&mut self, key: K, value: V) {
        if self.0.as_ref().is_none_or(|(_, prev)| *prev > value) {
            self.0 = Some((key, value));
        }
    }
}

impl<K: Copy, V: Copy> FirstMinKeyTracker<K, V> {
    pub fn get(&self) -> Option<(K, V)> {
        self.0
    }
}

impl<K: Copy, V> FirstMinKeyTracker<K, V> {
    pub fn get_key(&self) -> Option<K> {
        self.get_key_ref().copied()
    }
}

impl<K, V: Copy> FirstMinKeyTracker<K, V> {
    pub fn get_value(&self) -> Option<V> {
        self.get_value_ref().copied()
    }
}

impl<K, V> Default for FirstMinKeyTracker<K, V> {
    fn default() -> FirstMinKeyTracker<K, V> {
        FirstMinKeyTracker::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LastMinKeyTracker<K, V>(Option<(K, V)>);

impl<K, V> LastMinKeyTracker<K, V> {
    pub fn new() -> LastMinKeyTracker<K, V> {
        LastMinKeyTracker(None)
    }

    pub fn get_ref(&self) -> Option<&(K, V)> {
        self.0.as_ref()
    }

    pub fn get_key_ref(&self) -> Option<&K> {
        self.0.as_ref().map(|(key, _)| key)
    }

    pub fn get_value_ref(&self) -> Option<&V> {
        self.0.as_ref().map(|(_, value)| value)
    }
}

impl<K, V: Ord> LastMinKeyTracker<K, V> {
    pub fn add(&mut self, key: K, value: V) {
        if self.0.as_ref().is_none_or(|(_, prev)| *prev >= value) {
            self.0 = Some((key, value));
        }
    }
}

impl<K: Copy, V: Copy> LastMinKeyTracker<K, V> {
    pub fn get(&self) -> Option<(K, V)> {
        self.0
    }
}

impl<K: Copy, V> LastMinKeyTracker<K, V> {
    pub fn get_key(&self) -> Option<K> {
        self.get_key_ref().copied()
    }
}

impl<K, V: Copy> LastMinKeyTracker<K, V> {
    pub fn get_value(&self) -> Option<V> {
        self.get_value_ref().copied()
    }
}

impl<K, V> Default for LastMinKeyTracker<K, V> {
    fn default() -> LastMinKeyTracker<K, V> {
        LastMinKeyTracker::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_tracker() {
        let mut tracker = MaxTracker::new();
        assert_eq!(tracker.get(), None);
        tracker.add(23);
        assert_eq!(tracker.get(), Some(23));
        tracker.add(17);
        assert_eq!(tracker.get(), Some(23));
        tracker.add(42);
        assert_eq!(tracker.get(), Some(42));
    }

    #[test]
    fn first_max_key_tracker() {
        let mut tracker = FirstMaxKeyTracker::new();
        assert_eq!(tracker.get(), None);
        tracker.add("apple", 23);
        assert_eq!(tracker.get(), Some(("apple", 23)));
        tracker.add("banana", 17);
        assert_eq!(tracker.get(), Some(("apple", 23)));
        tracker.add("coconut", 23);
        assert_eq!(tracker.get(), Some(("apple", 23)));
        tracker.add("kumquat", 42);
        assert_eq!(tracker.get(), Some(("kumquat", 42)));
    }

    #[test]
    fn last_max_key_tracker() {
        let mut tracker = LastMaxKeyTracker::new();
        assert_eq!(tracker.get(), None);
        tracker.add("apple", 23);
        assert_eq!(tracker.get(), Some(("apple", 23)));
        tracker.add("banana", 17);
        assert_eq!(tracker.get(), Some(("apple", 23)));
        tracker.add("coconut", 23);
        assert_eq!(tracker.get(), Some(("coconut", 23)));
        tracker.add("kumquat", 42);
        assert_eq!(tracker.get(), Some(("kumquat", 42)));
    }

    #[test]
    fn min_tracker() {
        let mut tracker = MinTracker::new();
        assert_eq!(tracker.get(), None);
        tracker.add(23);
        assert_eq!(tracker.get(), Some(23));
        tracker.add(17);
        assert_eq!(tracker.get(), Some(17));
        tracker.add(42);
        assert_eq!(tracker.get(), Some(17));
    }

    #[test]
    fn first_min_key_tracker() {
        let mut tracker = FirstMinKeyTracker::new();
        assert_eq!(tracker.get(), None);
        tracker.add("apple", 23);
        assert_eq!(tracker.get(), Some(("apple", 23)));
        tracker.add("banana", 17);
        assert_eq!(tracker.get(), Some(("banana", 17)));
        tracker.add("coconut", 17);
        assert_eq!(tracker.get(), Some(("banana", 17)));
        tracker.add("kumquat", 42);
        assert_eq!(tracker.get(), Some(("banana", 17)));
    }

    #[test]
    fn last_min_key_tracker() {
        let mut tracker = LastMinKeyTracker::new();
        assert_eq!(tracker.get(), None);
        tracker.add("apple", 23);
        assert_eq!(tracker.get(), Some(("apple", 23)));
        tracker.add("banana", 17);
        assert_eq!(tracker.get(), Some(("banana", 17)));
        tracker.add("coconut", 17);
        assert_eq!(tracker.get(), Some(("coconut", 17)));
        tracker.add("kumquat", 42);
        assert_eq!(tracker.get(), Some(("coconut", 17)));
    }
}
