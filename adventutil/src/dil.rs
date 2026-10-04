/// A doubly-linked list backed by a borrowed slice, with adjacency of elements
/// implemented by a separate `Vec` that maps each index to the indices of the
/// previous and next elements of the list.  An element can be removed from the
/// list in constant time by adjusting the previous & next adjacencies to skip
/// over the element.
///
/// The indices used by the various methods are increasing, but once elements
/// have been removed, they're no longer contiguous, and operating on the index
/// of a removed element produces unspecified results.  Thus, it is recommended
/// to interact with the list via a cursor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DoubleIndexList<'a, T> {
    data: &'a [T],
    adjacencies: Vec<Adjacent>,
    // Index of the first element; `None` if there are no elements
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

    /// Retrieve the value at the given index
    pub fn get(&self, index: usize) -> Option<&'a T> {
        self.data.get(index)
    }

    /// Retrieve the value after the value at the given index.
    ///
    /// If `index` was previously removed, the results are unspecified.
    pub fn get_next(&self, index: usize) -> Option<&'a T> {
        self.data.get(self.next_index(index)?)
    }

    /// Return the index of the element immediately after the given index.
    ///
    /// If `index` was previously removed, the results are unspecified.
    pub fn next_index(&self, index: usize) -> Option<usize> {
        self.adjacencies.get(index)?.next_index
    }

    /// Return the index of the element immediately before the given index.
    ///
    /// If `index` was previously removed, the results are unspecified.
    pub fn prev_index(&self, index: usize) -> Option<usize> {
        self.adjacencies.get(index)?.prev_index
    }

    /// Remove the value at the given index
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
