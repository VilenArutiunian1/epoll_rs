use libc::epoll_event;

use crate::token::Token;

#[repr(transparent)]
pub struct Event {
    inner: epoll_event,
}

impl Event {
    pub fn token(&self) -> Token {
        Token(self.inner.u64 as usize)
    }

    pub fn is_readable(&self) -> bool {
        (self.inner.events as libc::c_int & libc::EPOLLIN) != 0
            || (self.inner.events as libc::c_int & libc::EPOLLPRI) != 0
    }

    pub fn is_writable(&self) -> bool {
        (self.inner.events as libc::c_int & libc::EPOLLOUT) != 0
    }

    pub fn is_error(&self) -> bool {
        (self.inner.events as libc::c_int & libc::EPOLLERR) != 0
    }

    pub fn is_read_closed(&self) -> bool {
        self.inner.events as libc::c_int & libc::EPOLLHUP != 0
            || (self.inner.events as libc::c_int & libc::EPOLLIN != 0
                && self.inner.events as libc::c_int & libc::EPOLLRDHUP != 0)
    }

    pub fn is_write_closed(&self) -> bool {
        self.inner.events as libc::c_int & libc::EPOLLHUP != 0
            || (self.inner.events as libc::c_int & libc::EPOLLOUT != 0
                && self.inner.events as libc::c_int & libc::EPOLLERR != 0)
            || self.inner.events as libc::c_int == libc::EPOLLERR
    }

    pub fn is_priority(&self) -> bool {
        (self.inner.events as libc::c_int & libc::EPOLLPRI) != 0
    }
}
