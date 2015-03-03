#![crate_type = "lib"]
#![feature(io)]

//! A faster and leaner `Lines` adaptor.
//!
//! # What's wrong with the `Lines` adaptor provided by the stdlib?
//!
//! On *each* iteration, `std::io::Lines` creates a new empty `String` (with zero capacity) and
//! then fills it with a line of text. This approach can lead to several (re)allocations *per*
//! iteration.
//!
//! On the other hand, the `Lines` adaptor provided here, creates a `String` *once* in its
//! constructor, and reuses that buffer on each iteration - this greatly reduces the number of
//! (re)allocations.
//!
//! # Show me some numbers!
//!
//! Heap usage while reading wikipedia's main page line by line (see `src/bin`):
//!
//! ``` text
//! $ cargo build --release
//! $ wget 'http://en.wikipedia.org'
//! $ cat index.html | wc
//!     603    4030   64972
//! $ valgrind target/release/test-std
//! ==31389== HEAP SUMMARY:
//! ==31389==     in use at exit: 0 bytes in 0 blocks
//! ==31389==   total heap usage: 626 allocs, 626 frees, 167,000 bytes allocated
//! $ valgrind target/release/test-lines
//! ==31740== HEAP SUMMARY:
//! ==31740==     in use at exit: 0 bytes in 0 blocks
//! ==31740==   total heap usage: 30 allocs, 30 frees, 75,240 bytes allocated
//! ```
//!
//! One alloc/free per line read for stdlib's `Lines`.
//!
//! # What's the catch?
//!
//! The `Lines` adaptor doesn't implement `Iterator`. Instead it provides a `next()` method that
//! returns a `&str` which is a view into its internal buffer. This means that you can't use a for
//! loop with the adaptor, but you can use `while let`:
//!
//! ``` ignore
//! while let Some(Ok(line)) = lines.next() { .. }
//! ```

use std::io::{BufRead, self};

/// An "iterator" over the lines of a `BufRead`
pub struct Lines<B> where B: BufRead {
    stream: B,
    buffer: String,
}

impl<B> Lines<B> where B: BufRead {
    /// Lazily extract lines from `stream`
    pub fn from(stream: B) -> Lines<B> {
        Lines {
            stream: stream,
            buffer: String::new(),
        }
    }

    /// Yields next line
    pub fn next(&mut self) -> Option<io::Result<&str>> {
        self.buffer.clear();

        match self.stream.read_line(&mut self.buffer) {
            Ok(()) if self.buffer.len() == 0 => None,
            Ok(()) => {
                if self.buffer.ends_with("\n") {
                    self.buffer.pop();
                }
                Some(Ok(&self.buffer))
            },
            Err(e) => Some(Err(e)),
        }
    }
}
