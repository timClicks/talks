use circbuf::CircularBuffer;

#[cfg(not(kani))]
fn main() {
    let mut buffer = CircularBuffer::<3>::new();

    // Push some values
    assert!(buffer.push(1));
    assert!(buffer.push(2));
    assert!(buffer.push(3));
    assert!(!buffer.push(4)); // Buffer is full

    // Verify FIFO ordering
    assert_eq!(buffer.pop(), Some(1));
    assert_eq!(buffer.pop(), Some(2));
    assert_eq!(buffer.peek(), Some(&3));
    assert_eq!(buffer.pop(), Some(3));
    assert_eq!(buffer.pop(), None); // Buffer is empty
}