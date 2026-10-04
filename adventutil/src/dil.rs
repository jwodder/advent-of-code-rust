/// A doubly-linked list backed by a borrowed slice, with adjacency of elements
/// implemented by a separate `Vec` that maps each index to the indices of the
/// previous and next elements of the list.  An element can be removed from the
/// list in constant time by adjusting the previous & next adjacencies to skip
/// over the element.
///
/// Most `DoubleIndexList` methods act on *backing indices* that identify
/// elements by their index in the original unaltered slice (rather than by
/// their index in the sequence obtained by iterating over the
/// `DoubleIndexList`).  These indices are increasing, but once elements have
/// been removed, they're no longer contiguous, and operating on the index of a
/// removed element produces unspecified results.  Thus, it is recommended to
/// interact with a list via a cursor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DoubleIndexList<'a, T> {
    data: &'a [T],
    adjacencies: Vec<Adjacent>,
    // Backing index of the first element; `None` if there are no elements
    first: Option<usize>,
    len: usize,
}

impl<'a, T> DoubleIndexList<'a, T> {
    pub fn new(data: &'a [T]) -> Self {
        let qty = data.len();
        DoubleIndexList {
            data,
            adjacencies: (0..qty)
                .map(|i| Adjacent {
                    prev_index: i.checked_sub(1),
                    next_index: i.checked_add(1).filter(|&j| j < qty),
                })
                .collect(),
            first: (qty > 0).then_some(0),
            len: qty,
        }
    }

    /// Reset the adjacencies to restore the list to its initial state upon
    /// construction, without having to allocate a new list.
    pub fn reset(&mut self) {
        let qty = self.data.len();
        for (i, adj) in self.adjacencies.iter_mut().enumerate() {
            *adj = Adjacent {
                prev_index: i.checked_sub(1),
                next_index: i.checked_add(1).filter(|&j| j < qty),
            };
        }
        self.first = (qty > 0).then_some(0);
        self.len = self.data.len();
    }

    /// Retrieve the value at the given backing index
    pub fn get(&self, index: usize) -> Option<&'a T> {
        self.data.get(index)
    }

    /// Retrieve the value after the value at the given backing index.
    ///
    /// If the element at `index` was previously removed from the list and
    /// there has not been an intervening reset, the results are unspecified.
    pub fn get_next(&self, index: usize) -> Option<&'a T> {
        self.data.get(self.next_index(index)?)
    }

    /// Return the backing index of the element immediately after the given
    /// backing index.
    ///
    /// If the element at `index` was previously removed from the list and
    /// there has not been an intervening reset, the results are unspecified.
    pub fn next_index(&self, index: usize) -> Option<usize> {
        self.adjacencies.get(index)?.next_index
    }

    /// Return the backing index of the element immediately before the given
    /// backing index.
    ///
    /// If the element at `index` was previously removed from the list and
    /// there has not been an intervening reset, the results are unspecified.
    pub fn prev_index(&self, index: usize) -> Option<usize> {
        self.adjacencies.get(index)?.prev_index
    }

    /// Remove the value at the given backing index.
    ///
    /// If the element at `index` was previously removed from the list and
    /// there has not been an intervening reset, the results are unspecified.
    pub fn remove(&mut self, index: usize) {
        let Some(adj) = self.adjacencies.get(index).copied() else {
            return;
        };
        let prev_index = adj.prev_index;
        let next_index = adj.next_index;
        if let Some(i) = prev_index {
            self.adjacencies[i].next_index = next_index;
        } else {
            debug_assert!(Some(index) == self.first);
            self.first = next_index;
        }
        if let Some(i) = next_index {
            self.adjacencies[i].prev_index = prev_index;
        }
        self.len -= 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn cursor<'c>(&'c mut self) -> Cursor<'a, 'c, T>
    where
        'a: 'c,
    {
        Cursor::new(self)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cursor<'a, 'c, T> {
    list: &'c mut DoubleIndexList<'a, T>,
    // Backing index of the current location in `list`
    index: Option<usize>,
}

impl<'a, 'c, T> Cursor<'a, 'c, T> {
    fn new(list: &'c mut DoubleIndexList<'a, T>) -> Self {
        let index = list.first;
        Cursor { list, index }
    }

    pub fn current(&self) -> Option<&'a T> {
        self.list.get(self.index?)
    }

    pub fn peek_next(&self) -> Option<&'a T> {
        self.list.get_next(self.index?)
    }

    pub fn move_next(&mut self) {
        if let Some(i) = self.index {
            self.index = self.list.next_index(i);
        } else {
            self.index = self.list.first;
        }
    }

    pub fn remove_current(&mut self) {
        let Some(i) = self.index else {
            return;
        };
        self.list.remove(i);
        self.move_next();
    }

    /// Remove the element the cursor is currently pointing to and the one
    /// after that, and then point the cursor at the element before the ones
    /// just removed (or at the new start of the list if the removed elements
    /// were at the start of the list).
    pub fn remove_two_and_back(&mut self) {
        let Some(i) = self.index else {
            return;
        };
        let Some(j) = self.list.next_index(i) else {
            return;
        };
        let new_index = self.list.prev_index(i);
        self.list.remove(i);
        self.list.remove(j);
        self.index = new_index.or(self.list.first);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Adjacent {
    prev_index: Option<usize>,
    next_index: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let dil = DoubleIndexList::<usize>::new(&[]);
        assert_eq!(dil.len(), 0);
        assert!(dil.is_empty());
        assert_eq!(dil.get(0), None);
        assert_eq!(dil.get_next(0), None);
        assert_eq!(dil.next_index(0), None);
        assert_eq!(dil.prev_index(0), None);
    }

    #[test]
    fn empty_cursor() {
        let mut dil = DoubleIndexList::<usize>::new(&[]);
        let mut cursor = dil.cursor();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), None);
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), None);
    }

    #[test]
    fn remove_from_empty() {
        let dil = DoubleIndexList::<usize>::new(&[]);
        let mut dil2 = dil.clone();
        dil2.remove(0);
        assert_eq!(dil, dil2);
    }

    #[test]
    fn three() {
        let dil = DoubleIndexList::new(&[1usize, 2, 3]);
        assert_eq!(dil.len(), 3);
        assert!(!dil.is_empty());
        assert_eq!(dil.get(0), Some(&1));
        assert_eq!(dil.get_next(0), Some(&2));
        assert_eq!(dil.next_index(0), Some(1));
        assert_eq!(dil.prev_index(0), None);
        assert_eq!(dil.next_index(1), Some(2));
        assert_eq!(dil.prev_index(1), Some(0));
        assert_eq!(dil.next_index(2), None);
        assert_eq!(dil.prev_index(2), Some(1));
    }

    #[test]
    fn three_cursor() {
        let mut dil = DoubleIndexList::new(&[1usize, 2, 3]);
        let mut cursor = dil.cursor();
        assert_eq!(cursor.current(), Some(&1));
        assert_eq!(cursor.peek_next(), Some(&2));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&2));
        assert_eq!(cursor.peek_next(), Some(&3));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&3));
        assert_eq!(cursor.peek_next(), None);
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), None);
    }

    #[test]
    fn remove_head() {
        let mut dil = DoubleIndexList::new(&[1usize, 2, 3]);
        dil.remove(0);
        let mut cursor = dil.cursor();
        assert_eq!(cursor.current(), Some(&2));
        assert_eq!(cursor.peek_next(), Some(&3));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&3));
        assert_eq!(cursor.peek_next(), None);
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), None);
    }

    #[test]
    fn remove_middle() {
        let mut dil = DoubleIndexList::new(&[1usize, 2, 3]);
        dil.remove(1);
        let mut cursor = dil.cursor();
        assert_eq!(cursor.current(), Some(&1));
        assert_eq!(cursor.peek_next(), Some(&3));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&3));
        assert_eq!(cursor.peek_next(), None);
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), None);
    }

    #[test]
    fn remove_end() {
        let mut dil = DoubleIndexList::new(&[1usize, 2, 3]);
        dil.remove(2);
        let mut cursor = dil.cursor();
        assert_eq!(cursor.current(), Some(&1));
        assert_eq!(cursor.peek_next(), Some(&2));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&2));
        assert_eq!(cursor.peek_next(), None);
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), None);
    }

    #[test]
    fn remove_current_head() {
        let mut dil = DoubleIndexList::new(&[1usize, 2, 3]);
        let mut cursor = dil.cursor();
        assert_eq!(cursor.current(), Some(&1));
        assert_eq!(cursor.peek_next(), Some(&2));
        cursor.remove_current();
        assert_eq!(cursor.current(), Some(&2));
        assert_eq!(cursor.peek_next(), Some(&3));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&3));
        assert_eq!(cursor.peek_next(), None);
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), None);
    }

    #[test]
    fn remove_current_middle() {
        let mut dil = DoubleIndexList::new(&[1usize, 2, 3]);
        let mut cursor = dil.cursor();
        assert_eq!(cursor.current(), Some(&1));
        assert_eq!(cursor.peek_next(), Some(&2));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&2));
        assert_eq!(cursor.peek_next(), Some(&3));
        cursor.remove_current();
        assert_eq!(cursor.current(), Some(&3));
        assert_eq!(cursor.peek_next(), None);
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), None);
    }

    #[test]
    fn remove_current_end() {
        let mut dil = DoubleIndexList::new(&[1usize, 2, 3]);
        let mut cursor = dil.cursor();
        assert_eq!(cursor.current(), Some(&1));
        assert_eq!(cursor.peek_next(), Some(&2));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&2));
        assert_eq!(cursor.peek_next(), Some(&3));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&3));
        assert_eq!(cursor.peek_next(), None);
        cursor.remove_current();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), None);
    }

    #[test]
    fn remove_two() {
        let mut dil = DoubleIndexList::new(&[1usize, 2, 3, 4, 5]);

        let mut cursor = dil.cursor();
        assert_eq!(cursor.current(), Some(&1));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&2));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&3));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&4));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&5));
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        cursor.move_next();

        dil.remove(1);
        let mut cursor = dil.cursor();
        assert_eq!(cursor.current(), Some(&1));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&3));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&4));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&5));
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        cursor.move_next();

        dil.remove(2);
        let mut cursor = dil.cursor();
        assert_eq!(cursor.current(), Some(&1));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&4));
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&5));
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        cursor.move_next();
    }
}
