
#[cfg(kani)] mod verification;

/// A fixed-size circular buffer using const generics and raw pointers
pub struct CircularBuffer<const N: usize> {
    data: [u32; N],
    head: usize,
    tail: usize,
    size: usize,
}

impl<const N: usize> CircularBuffer<N> {
    /// Creates a new circular buffer with compile-time size N
    pub fn new() -> Self {
        assert!(N > 0, "Buffer size must be positive");
        CircularBuffer {
            data: [0; N],
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    /// Returns true if the buffer is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns true if the buffer is full
    #[inline]
    pub fn is_full(&self) -> bool {
        self.size == N
    }

    /// Pushes a value into the buffer using unsafe pointer arithmetic.
    /// Returns false if buffer is full.
    pub fn push(&mut self, value: u32) -> bool {
        if self.is_full() {
            return false;
        }

        // SAFETY: We've verified that:
        // 1. The buffer isn't full, so tail is a valid index
        // 2. tail is always maintained to be within bounds
        unsafe {
            let ptr = self.data.as_mut_ptr().add(self.tail);
            *ptr = value;
        }

        self.tail = (self.tail + 1) % N;
        self.size += 1;
        true
    }

    /// Pops a value from the buffer using unsafe pointer arithmetic.
    /// Returns None if buffer is empty.
    pub fn pop(&mut self) -> Option<u32> {
        if self.is_empty() {
            return None;
        }

        // SAFETY: We've verified that:
        // 1. The buffer isn't empty, so head is a valid index
        // 2. head is always maintained to be within bounds
        let value = unsafe {
            let ptr = self.data.as_ptr().add(self.head);
            *ptr
        };

        self.head = (self.head + 1) % N;
        self.size -= 1;
        Some(value)
    }

    /// Get a reference to the value at the head without removing it
    pub fn peek(&self) -> Option<&u32> {
        if self.is_empty() {
            return None;
        }

        // SAFETY: We've verified the buffer isn't empty and head is in bounds
        unsafe {
            Some(&*self.data.as_ptr().add(self.head))
        }
    }
}

// Verification code
// #[cfg(kani)]
// mod verification {
//     use super::*;

//     #[kani::proof]
//     fn verify_empty_buffer() {
//         let buffer = CircularBuffer::<4>::new();
//         kani::assert(buffer.is_empty(), "New buffer should be empty");
//         kani::assert(!buffer.is_full(), "New buffer should not be full");
//         kani::assert(buffer.pop().is_none(), "Pop from empty buffer should return None");
//         kani::assert(buffer.peek().is_none(), "Peek at empty buffer should return None");
//     }

//     #[kani::proof]
//     fn verify_push_pop_safety() {
//         let mut buffer = CircularBuffer::<4>::new();

//         // Push some symbolic values
//         let value1: u32 = kani::any();
//         let value2: u32 = kani::any();

//         // Verify push behavior
//         assert!(buffer.push(value1));
//         assert!(buffer.push(value2));

//         // Verify FIFO ordering
//         assert_eq!(buffer.pop(), Some(value1));
//         assert_eq!(buffer.pop(), Some(value2));
//     }

//     #[kani::proof]
//     fn verify_full_buffer() {
//         let mut buffer = CircularBuffer::<2>::new();

//         // Fill the buffer
//         assert!(buffer.push(1));
//         assert!(buffer.push(2));

//         // Verify full state
//         kani::assert(buffer.is_full(), "Buffer should be full");
//         kani::assert!(!buffer.push(3), "Push to full buffer should fail");

//         // Verify we can still peek and pop
//         kani::assert_eq!(buffer.peek(), Some(&1));
//         kani::assert_eq!(buffer.pop(), Some(1));
//     }

//     #[kani::proof]
//     fn verify_wraparound() {
//         let mut buffer = CircularBuffer::<3>::new();

//         // Fill and empty the buffer multiple times to test wraparound
//         for _ in 0..4 {
//             // Fill
//             assert!(buffer.push(1));
//             assert!(buffer.push(2));
//             assert!(buffer.push(3));
//             kani::assert!(buffer.is_full());

//             // Empty
//             assert_eq!(buffer.pop(), Some(1));
//             assert_eq!(buffer.pop(), Some(2));
//             assert_eq!(buffer.pop(), Some(3));
//             kani::assert!(buffer.is_empty());
//         }
//     }

//     #[kani::proof]
//     fn verify_arbitrary_operations() {
//         let mut buffer = CircularBuffer::<3>::new();

//         // Perform arbitrary sequence of operations
//         for _ in 0..3 {
//             let op: u8 = kani::any();
//             let value: u32 = kani::any();

//             match op % 3 {
//                 0 => { buffer.push(value); }
//                 1 => { buffer.pop(); }
//                 _ => { buffer.peek(); }
//             }

//             // Verify invariants hold
//             kani::assert!(buffer.size <= 3, "Size never exceeds capacity");
//             kani::assert!(buffer.head < 3, "Head index in bounds");
//             kani::assert!(buffer.tail < 3, "Tail index in bounds");
//         }
//     }
// }

// Example usage
