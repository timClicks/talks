use super::*;

#[kani::proof]
fn verify_empty_buffer() {
    let mut buffer = CircularBuffer::<4>::new();
    kani::assert(buffer.is_empty(), "New buffer should be empty");
    kani::assert(!buffer.is_full(), "New buffer should not be full");
    kani::assert(
        buffer.pop().is_none(),
        "Pop from empty buffer should return None",
    );
    kani::assert(
        buffer.peek().is_none(),
        "Peek at empty buffer should return None",
    );
}

#[kani::proof]
fn verify_push_pop_safety() {
    let mut buffer = CircularBuffer::<4>::new();

    // Push some symbolic values
    let a: u32 = kani::any();
    let b: u32 = kani::any();

    // Verify push behavior
    kani::assert(buffer.push(a));
    kani::assert(buffer.push(b));

    // Verify FIFO ordering
    assert_eq!(buffer.pop(), Some(a));
    assert_eq!(buffer.pop(), Some(b));
}

#[kani::proof]
fn verify_full_buffer() {
    let mut buffer = CircularBuffer::<2>::new();

    // Fill the buffer
    assert!(buffer.push(1));
    assert!(buffer.push(2));

    // Verify full state
    kani::assert(buffer.is_full(), "Buffer should be full");
    kani::assert(!buffer.push(3), "Push to full buffer should fail");

    // Verify we can still peek and pop
    assert_eq!(buffer.peek(), Some(&1));
    assert_eq!(buffer.pop(), Some(1));

    kani::assert(!buffer.is_full(), "Buffer should be full");
}

#[kani::proof]
fn verify_wraparound() {
    let mut buffer = CircularBuffer::<3>::new();

    // Fill and empty the buffer multiple times to test wraparound
    for _ in 0..4 {
        // Fill
        assert!(buffer.push(1));
        assert!(buffer.push(2));
        assert!(buffer.push(3));
        kani::assert(
            buffer.is_full(),
            "Buffer of length 3 should be full after 3 pushes",
        );

        // Empty
        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
        kani::assert(
            buffer.is_empty(),
            "Buffer of length 3 should be empty after starting full, then performing 3 pops",
        );
    }
}

#[kani::proof]
fn verify_arbitrary_operations() {
    let mut buffer = CircularBuffer::<3>::new();

    // Perform arbitrary sequence of operations
    for _iter in 0..10 {
        let op: u8 = kani::any();
        let value: u32 = kani::any();

        match op % 3 {
            0 => {
                buffer.push(value);
            }
            1 => {
                buffer.pop();
            }
            _ => {
                buffer.peek();
            }
        }

        // Verify invariants
        kani::assert(buffer.size <= 3, "Size never exceeds capacity");
        kani::assert(buffer.head < 3, "Head index in bounds");
        kani::assert(buffer.tail < 3, "Tail index in bounds");
    }
}
