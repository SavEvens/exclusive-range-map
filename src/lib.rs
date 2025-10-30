use std::ops::Range;

struct ExclusiveRangeMapInvalidInsertError {}
struct ExclusiveRangeMapInvalidIndexError {}

struct ExclusiveRangeMapNode<Idx> {
    key_range: Range<Idx>,
    index: Option<Idx>,
    child_left: Option<Idx>,
    child_right: Option<Idx>,
    parent: Option<Idx>,
}

impl<Idx> ExclusiveRangeMapNode<Idx> {
    pub fn new(key_range: Range<Idx>) -> Self {
        Self {
            key_range: key_range,
            index: None,
            child_left: None,
            child_right: None,
            parent: None,
        }
    }
}

struct ExclusiveRangeMap<Idx, T> {
    array: Vec<T>,
    head: Option<ExclusiveRangeMapNode<Idx>>,
    size: usize,
}

impl<Idx, T> ExclusiveRangeMap<Idx, T> {
    pub fn new() -> Self {
        Self {
            array: Vec::<T>::new(),
            head: None,
            size: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node() {
        let mut node = ExclusiveRangeMapNode::new(0..1);
        assert_eq!(node.key_range.start, 0);
        assert_eq!(node.key_range.end, 1);
    }

    #[test]
    fn test_create_map() {
        let mut map = ExclusiveRangeMap::<usize, usize>::new();
        assert_eq!(map.array.len(), 0);
        assert_eq!(map.size, 0);
        match map.head {
            Some(_) => {
                panic!("Head should be empty!")
            }
            None => {}
        }
    }
}
