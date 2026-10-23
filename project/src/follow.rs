//! Milestone m18 (Chapter 18): reading a file that is still being written.
//!
//! `loglens grep --follow app.log` keeps printing new matching lines as a
//! service appends them, like `tail -f`. A [`Follower`] owns the reader and
//! remembers a *partial* last line: a writer may have written half a line
//! when we read, and that half must wait for its `\n`.

use crate::error::ParseError;
use crate::filter::Filter;
use crate::pipeline::Tally;
use crate::sink::Sink;
use std::io::{self, BufRead};

/// Incremental line processing over a growing input.
#[derive(Debug)]
pub struct Follower<R> {
    reader: R,
    partial: Vec<u8>,
    line_no: u64,
}

impl<R: BufRead> Follower<R> {
    /// Follow `reader` from its current position.
    pub fn new(reader: R) -> Self {
        Follower {
            reader,
            partial: Vec::new(),
            line_no: 0,
        }
    }

    /// m18 B. Process every *complete* line available now, then return.
    ///
    /// Read with `read_until(b'\n', &mut self.partial)`: it appends, so a
    /// partial line from the previous poll is completed by this one. A line
    /// without its `\n` at the current end of input stays in `partial` and is
    /// not processed yet. Line numbers continue across polls. Each complete
    /// line is handled exactly as in [`crate::pipeline::process_reader`].
    /// Returns the tally of this poll only; a poll that finds no new complete
    /// line returns an all-zero tally.
    ///
    /// Reading a `File` that has reached its end returns 0 bytes; once more
    /// data is appended, the next read returns it. That is all `tail -f`
    /// needs (rotation and truncation are out of scope).
    pub fn poll<S: Sink + ?Sized>(
        &mut self,
        filter: &Filter,
        sink: &mut S,
        on_error: &mut dyn FnMut(ParseError),
    ) -> io::Result<Tally> {
        todo!("m18 B: complete lines only")
    }

    /// m18 C. At shutdown: process the pending partial line, if any, as a
    /// final line. Returns its tally (all zero without one).
    pub fn finish<S: Sink + ?Sized>(
        &mut self,
        filter: &Filter,
        sink: &mut S,
        on_error: &mut dyn FnMut(ParseError),
    ) -> io::Result<Tally> {
        todo!("m18 C: flush the partial line")
    }

    /// The bytes of the line still waiting for its `\n`.
    pub fn pending(&self) -> &[u8] {
        &self.partial
    }
}
