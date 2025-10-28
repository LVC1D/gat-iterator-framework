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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_compiles() {
        // This test just checks the trait compiles
        // We'll add real tests after implementing WindowIterator
    }
} 
