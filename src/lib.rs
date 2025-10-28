// TODO: Define the LendingIterator trait
// Requirements:
// - Associated type Item with lifetime parameter
// - Proper where clause
// - next() method signature

pub trait LendingIterator {
    type Item<'a> where Self: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>>;
}

pub struct WindowIterator<'data, T> {
    data: &'data [T],      // Borrows the data
    position: usize,        // Current window start
    window_size: usize,     // How big each window is
}

impl<'data, T> LendingIterator for WindowIterator<'data, T> {
    type Item<'a> = &'a [T] where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        // TODO: Implement sliding window logic
        // 
        // Requirements:
        // 1. Check if we have enough elements left for a window
        // 2. Return a slice of size `window_size` starting at `position`
        // 3. Advance `position` by 1 (sliding window, not jumping)
        // 4. Return None when we can't make a full window anymore
        //
        // Hint: Think about slice operations and when to return None
        let end = self.position + self.window_size;
        if end > self.data.len() || self.window_size == 0 {
            return None;
        }

        let res = &self.data[self.position..end];
        self.position += 1;
        Some(res)
    }
}

impl<'data, T> WindowIterator<'data, T> {
    pub fn new(data: &'data [T], window_size: usize) -> Self {
        // TODO: Create a new WindowIterator
        Self {
            data,
            position: 0,
            window_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_compiles() {
        // This test just checks the trait compiles
        // We'll add real tests after implementing WindowIterator
        let data = vec![1, 3, 5, 7, 9];
        let mut win_iter = WindowIterator::new(&data, 3);

        let first = win_iter.next(); 
        let first_expected = Some(&data[0..3]);
        assert_eq!(first, first_expected);

        let first_pos = win_iter.position; 
        assert_eq!(first_pos, 1);

        assert_eq!(win_iter.next(), Some(&data[1..4]));
        assert_eq!(win_iter.position, 2);

        assert_eq!(win_iter.next(), Some(&data[2..5]));
        assert_eq!(win_iter.position, 3);

        assert_eq!(win_iter.next(), None);
        assert_eq!(win_iter.position, 3);

    }

    #[test]
    fn empty_data() {
        let data: Vec<u32> = vec![];
        let mut win_iter: WindowIterator<'_, u32> = WindowIterator::new(&data, 2);
        assert_eq!(win_iter.next(), None);
    }

    #[test]
    fn win_size_greater_than_data_len() {
        let data = vec![1, 2];
        let mut win_iter = WindowIterator::new(&data, 3);
        assert_eq!(win_iter.next(), None);
    }

    #[test]
    fn single_elem_window() {
        // This test just checks the trait compiles
        // We'll add real tests after implementing WindowIterator
        let data = vec![1, 3, 5, 7, 9];
        let mut win_iter = WindowIterator::new(&data, 1);

        assert_eq!(win_iter.next(), Some(&data[0..1])); 
        assert_eq!(win_iter.next(), Some(&data[1..2]));
        assert_eq!(win_iter.next(), Some(&data[2..3]));
        assert_eq!(win_iter.next(), Some(&data[3..4]));
        assert_eq!(win_iter.next(), Some(&data[4..5]));

        assert_eq!(win_iter.next(), None); 
    }

    #[test]
    fn win_size_zero() {
        let data = vec![1, 2];
        let mut win_iter = WindowIterator::new(&data, 0);
        assert_eq!(win_iter.next(), None);
    }
} 
