use std::{num::NonZeroU8, ops};

#[derive(Clone, Copy)]
pub struct Interest(pub(crate) NonZeroU8);

const READABLE: u8 = 0b001;
const WRITABLE: u8 = 0b010;
const PRIORITY: u8 = 0b100;

impl Interest {
    pub const READABLE: Interest = Interest(unsafe { NonZeroU8::new_unchecked(READABLE) });
    pub const WRITABLE: Interest = Interest(unsafe { NonZeroU8::new_unchecked(WRITABLE) });
    /// PRIORITY: Represents priority levels for I/O operations on the Linux operating system.
    /// These flags are used to specify the priority of asynchronous I/O operations.
    /// The constant is defined only on the Linux operating system, where it allows
    /// setting priority levels for asynchronous I/O operations.
    /// Prioritizing I/O operations can help optimize the order in which they are
    /// processed, potentially improving the performance and responsiveness of the program.
    pub const PRIORITY: Interest = Interest(unsafe { NonZeroU8::new_unchecked(PRIORITY) });

    pub fn add(self, other: Interest) -> Self {
        Interest(unsafe { NonZeroU8::new_unchecked(self.0.get() | other.0.get()) })
    }

    pub fn remove(self, other: Interest) -> Option<Interest> {
        NonZeroU8::new(self.0.get() & !other.0.get()).map(Interest)
    }

    pub const fn is_readable(self) -> bool {
        (self.0.get() & READABLE) != 0
    }

    pub const fn is_writable(self) -> bool {
        (self.0.get() & WRITABLE) != 0
    }

    pub const fn is_priority(self) -> bool {
        (self.0.get() & PRIORITY) != 0
    }
}

impl ops::BitOr for Interest {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        self.add(other)
    }
}

impl ops::BitOrAssign for Interest {
    fn bitor_assign(&mut self, other: Self) {
        self.0 = (*self | other).0;
    }
}
