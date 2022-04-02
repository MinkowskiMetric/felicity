pub trait HeapOrder<T> {
    fn left_can_go_above(&self, left: &T, right: &T) -> bool;
}

impl<T, F: Fn(&T, &T) -> bool> HeapOrder<T> for F {
    fn left_can_go_above(&self, left: &T, right: &T) -> bool {
        self(left, right)
    }
}

#[derive(Debug, Clone)]
pub struct MinOrder<T: Ord>(std::marker::PhantomData<T>);

impl<T: Ord> Default for MinOrder<T> {
    fn default() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl<T: Ord> HeapOrder<T> for MinOrder<T> {
    fn left_can_go_above(&self, left: &T, right: &T) -> bool {
        left < right
    }
}

#[derive(Debug, Clone)]
pub struct MaxOrder<T: Ord>(std::marker::PhantomData<T>);

impl<T: Ord> Default for MaxOrder<T> {
    fn default() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl<T: Ord> HeapOrder<T> for MaxOrder<T> {
    fn left_can_go_above(&self, left: &T, right: &T) -> bool {
        left > right
    }
}

fn heapify_in_place<T>(data: &mut [T], order: &impl HeapOrder<T>) {
    for i in (0..(data.len() / 2)).rev() {
        heapify_down(data, i, order);
    }
}

fn heapify_down<T>(data: &mut [T], mut top_index: usize, order: &impl HeapOrder<T>) {
    loop {
        let mut highest_index = top_index;
        let right_child_index = 2 * (highest_index + 1);
        let left_child_index = right_child_index - 1;

        if left_child_index < data.len() && order.left_can_go_above(&data[left_child_index], &data[highest_index]) {
            highest_index = left_child_index;
        }

        if right_child_index < data.len() && order.left_can_go_above(&data[right_child_index], &data[highest_index]) {
            highest_index = right_child_index;
        }

        if highest_index != top_index {
            data.swap(top_index, highest_index);
            top_index = highest_index;
        } else {
            break;
        }
    }
}

fn heapify_up<T>(data: &mut [T], mut pos_index: usize, order: &impl HeapOrder<T>) {
    while pos_index > 0 {
        let parent_index = (pos_index - 1) / 2;
        if order.left_can_go_above(&data[parent_index], &data[pos_index]) {
            // The parent and node are in the correct order so we can stop
            break;
        } else {
            // Swap the parent and the node and walk back up towards the root
            data.swap(parent_index, pos_index);
            pos_index = parent_index;
        }
    }
}

fn is_heap<T>(values: &[T], order: &impl HeapOrder<T>) -> bool {
    // Iterate over the leaf nodes
    for leaf_index in ((values.len() + 1) / 2)..values.len() {
        let mut pos_index = leaf_index;
        while pos_index > 0 {
            let parent_index = (pos_index - 1) / 2;
            if !order.left_can_go_above(&values[parent_index], &values[pos_index]) {
                return false;
            }

            pos_index = parent_index;
        }
    }

    return true;
}

#[derive(Debug, Clone)]
pub struct Heap<T, Order: HeapOrder<T>> {
    data: Vec<T>,
    order: Order,
}

impl<T, Order: HeapOrder<T>> Heap<T, Order> {
    pub fn from_vec_and_cmp(mut data: Vec<T>, order: Order) -> Self {
        heapify_in_place(&mut data, &order);

        Self {
            data,
            order,
        }
    }

    pub fn with_capacity_and_cmp(capacity: usize, order: Order) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            order,
        }
    }

    pub unsafe fn unsafe_from_heap_and_cmp(data: Vec<T>, order: Order) -> Self {
        debug_assert!(is_heap(&data, &order), "Heap is not valid");

        Self {
            data,
            order,
        }
    }

    pub fn try_from_heap_and_cmp(data: Vec<T>, order: Order) -> Option<Self> {
        if is_heap(&data, &order) {
            Some(unsafe { Self::unsafe_from_heap_and_cmp(data, order) })
        } else {
            None
        }
    }
}

impl<T, Order: HeapOrder<T>> Extend<T> for Heap<T, Order> {
    fn extend<IntoIter: IntoIterator<Item = T>>(&mut self, iter: IntoIter) {
        for item in iter {
            self.insert(item)
        }
    }
}

impl<T: Ord> Heap<T, MaxOrder<T>> {
    pub fn new() -> Self {
        Self::max(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_cmp(capacity, MaxOrder::default())
    }

    pub fn max(data: Vec<T>) -> Self {
        Self::from_vec_and_cmp(data, MaxOrder::default())
    }

    pub unsafe fn unsafe_from_max_heap(data: Vec<T>) -> Self {
        Self::unsafe_from_heap_and_cmp(data, MaxOrder::default())
    }

    pub fn try_from_max_heap(data: Vec<T>) -> Option<Self> {
        Self::try_from_heap_and_cmp(data, MaxOrder::default())
    }
}

pub type MaxHeap<T> = Heap<T, MaxOrder<T>>;

impl<T: Ord> Default for MaxHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> From<Vec<T>> for MaxHeap<T> {
    fn from(f: Vec<T>) -> Self {
        Self::max(f)
    }
}

impl<T: Ord> FromIterator<T> for MaxHeap<T> {
    fn from_iter<IntoIter: IntoIterator<Item = T>>(iter: IntoIter) -> Self {
        let iter = iter.into_iter();
        let mut ret = Self::with_capacity(iter.size_hint().0);
        for item in iter {
            ret.insert(item);
        }
        
        ret
    }
}

impl<T: Ord> Heap<T, MinOrder<T>> {
    pub fn new() -> Self {
        Self::min(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_cmp(capacity, MinOrder::default())
    }

    pub fn min(data: Vec<T>) -> Self {
        Self::from_vec_and_cmp(data, MinOrder::default())
    }

    pub unsafe fn unsafe_from_min_heap(data: Vec<T>) -> Self {
        Self::unsafe_from_heap_and_cmp(data, MinOrder::default())
    }

    pub fn try_from_min_heap(data: Vec<T>) -> Option<Self> {
        Self::try_from_heap_and_cmp(data, MinOrder::default())
    }
}

pub type MinHeap<T> = Heap<T, MinOrder<T>>;

impl<T: Ord> Default for MinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> From<Vec<T>> for MinHeap<T> {
    fn from(f: Vec<T>) -> Self {
        Self::min(f)
    }
}

impl<T: Ord> FromIterator<T> for MinHeap<T> {
    fn from_iter<IntoIter: IntoIterator<Item = T>>(iter: IntoIter) -> Self {
        let iter = iter.into_iter();
        let mut ret = Self::with_capacity(iter.size_hint().0);
        for item in iter {
            ret.insert(item);
        }
        
        ret
    }
}

impl<T, Order: HeapOrder<T>> Heap<T, Order> {
    pub fn values(&self) -> &[T] {
        &self.data
    }

    pub fn order(&self) -> &Order {
        &self.order
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn to_values(self) -> Vec<T> {
        self.data
    }

    pub fn insert(&mut self, value: T) {
        // Insert the new item in the left most open slot. Which in practise just means "push it to the end"
        let new_node_index = self.data.len();
        self.data.push(value);

        // Technically, we can elide this operation if the heap property is already satisfied, but since
        // that is the first thing heapify_up checks for there is no point - we'd just be checking twice
        heapify_up(&mut self.data, new_node_index, &self.order);
    }

    pub fn remove(&mut self, index: usize) -> T {
        // Take out the element we are removing, and put the last element in it's
        // place. I've never understood what this function was for, but now I know.
        let ret = self.data.swap_remove(index);

        if !self.data.is_empty() {
            if self.order.left_can_go_above(&self.data[index], &ret) {
                // The new element can be higher up the tree than the original element, so we
                // do an up-heapify to make sure the heap property is maintained
                heapify_up(&mut self.data, index, &self.order);
            } else if self.order.left_can_go_above(&ret, &self.data[index]) {
                // The value we removed could go below the value we replaced it with, so down-heapify
                heapify_down(&mut self.data, index, &self.order);
            }
        }

        ret
    }
}

impl<T: std::fmt::Debug, Order: HeapOrder<T>> Heap<T, Order> {
    pub fn tree_format(&self) -> TreeFormatHeap<'_, T, Order> {
        TreeFormatHeap(self)
    }
}

#[repr(transparent)]
pub struct TreeFormatHeap<'a, T: std::fmt::Debug, Order: HeapOrder<T>>(&'a Heap<T, Order>);

impl<'a, T: std::fmt::Debug, Order: HeapOrder<T>> std::fmt::Debug for TreeFormatHeap<'a, T, Order> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row_width = 1;
        let mut row_start = 0;
        let values = self.0.values();

        for (idx, value) in values.iter().enumerate() {
            if idx == row_start + row_width {
                writeln!(f)?;
                row_start += row_width;
                row_width *= 2;
            }

            write!(f, "{}: {:?} ", idx, value)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    fn shuffle_vec<T>(mut vec: Vec<T>) -> Vec<T> {
        let mut rng = thread_rng();
        vec.as_mut_slice().shuffle(&mut rng);
        vec
    }

    fn check_heap<T, Order: HeapOrder<T>>(heap: &Heap<T, Order>) -> bool {
        is_heap(heap.values(), heap.order())
    }

    #[test]
    fn test_is_heap() {
        assert!(is_heap(&[4, 3, 1, 2], &MaxOrder(std::marker::PhantomData)));
        assert!(is_heap(&[4, 3, 2, 1], &MaxOrder(std::marker::PhantomData)));
        assert!(is_heap(&[4, 2, 3, 1], &MaxOrder(std::marker::PhantomData)));
        assert!(!is_heap(&[4, 2, 3, 7], &MaxOrder(std::marker::PhantomData)));
    }

    #[test]
    fn test_initialize() {
        let test_set: Vec<_> = shuffle_vec((0..1000).collect());
        let test_order = MaxOrder(std::marker::PhantomData);

        for i in 1..test_set.len() {
            let heap = Heap::from_vec_and_cmp(test_set[0..i].to_vec(), test_order.clone());
            assert!(check_heap(&heap));
        }
    }
}
