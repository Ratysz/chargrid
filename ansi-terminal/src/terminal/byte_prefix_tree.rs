const ENTRIES_PER_NODE: usize = 256;

#[derive(Clone)]
pub struct BytePrefixTree<T> {
    data: Option<T>,
    children: Vec<Option<BytePrefixTree<T>>>,
}

impl<T> Default for BytePrefixTree<T> {
    fn default() -> Self {
        Self {
            data: None,
            children: Vec::new(),
        }
    }
}

impl<T> BytePrefixTree<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, key: &[u8], data: T) {
        if let Some((head, tail)) = key.split_first() {
            if self.children.is_empty() {
                self.children.resize_with(ENTRIES_PER_NODE, || None);
            }
            self.children[*head as usize]
                .get_or_insert_with(Self::new)
                .insert(tail, data)
        } else {
            self.data = Some(data);
        }
    }

    pub fn get_longest<'a, 'b>(&'a self, key: &'b [u8]) -> Option<Found<'a, 'b, T>> {
        if let Some((head, tail)) = key.split_first() {
            if let Some(next) = self
                .children
                .get(*head as usize)
                .and_then(|o| o.as_ref())
                .as_ref()
            {
                let maybe_found = next.get_longest(tail);
                if maybe_found.is_some() {
                    // there's a longer key than us with data, so just return it
                    maybe_found
                } else {
                    // there's nothing down there, but maybe there's something here
                    self.data.as_ref().map(|d| Found::WithRemaining(d, key))
                }
            } else {
                // Either head was out of bounds, meaning the vector was empty, or it was in bounds
                // but with a value of None.
                // Thus we've reached a dead end, so return data if there's any here
                // along with the remaining slice.
                self.data.as_ref().map(|d| Found::WithRemaining(d, key))
            }
        } else {
            // we're out of bytes, so return data if there's any here
            self.data.as_ref().map(Found::Exact)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Found<'a, 'b, T: 'a> {
    Exact(&'a T),
    WithRemaining(&'a T, &'b [u8]),
}

#[cfg(test)]
mod tests {

    use super::{BytePrefixTree, Found};

    #[test]
    fn get_longest() {
        let mut tree = BytePrefixTree::new();
        tree.insert(b"helloworld", 0);
        tree.insert(b"hello", 1);
        tree.insert(b"world", 2);
        assert_eq!(tree.get_longest(b"helloworld"), Some(Found::Exact(&0)));
        assert_eq!(tree.get_longest(b"hello"), Some(Found::Exact(&1)));
        assert_eq!(
            tree.get_longest(b"hellowo"),
            Some(Found::WithRemaining(&1, b"wo"))
        );
        assert_eq!(
            tree.get_longest(b"helloworldblah"),
            Some(Found::WithRemaining(&0, b"blah"))
        );
        assert_eq!(tree.get_longest(b"world"), Some(Found::Exact(&2)));
        assert_eq!(tree.get_longest(b"worl"), None);
        assert_eq!(tree.get_longest(b""), None);
        assert_eq!(
            tree.get_longest(b"worlds"),
            Some(Found::WithRemaining(&2, b"s"))
        );
    }

    #[test]
    fn data_in_root() {
        let mut tree = BytePrefixTree::new();
        tree.insert(b"", 0);
        tree.insert(b"abc", 1);
        tree.insert(b"def", 2);
        assert_eq!(tree.get_longest(b""), Some(Found::Exact(&0)));
        assert_eq!(tree.get_longest(b"a"), Some(Found::WithRemaining(&0, b"a")));
        assert_eq!(
            tree.get_longest(b"ab"),
            Some(Found::WithRemaining(&0, b"ab"))
        );
        assert_eq!(tree.get_longest(b"abc"), Some(Found::Exact(&1)));
        assert_eq!(
            tree.get_longest(b"abcd"),
            Some(Found::WithRemaining(&1, b"d"))
        );
    }
}
