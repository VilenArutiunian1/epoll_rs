use std::os::fd::AsRawFd;
use std::{os::fd::RawFd, io, time::Duration};
use libc::{EPOLLPRI, EPOLLET, EPOLLIN, EPOLLRDHUP, EPOLLOUT, epoll_event};

use crate::event::Event;
use crate::interest::Interest;
use crate::token::Token;
use crate::syscall;

pub struct Epoll {
    epfd: RawFd
}

impl Epoll {
    pub fn new() -> io::Result<Self> {
        let epfd = syscall!(epoll_create1(libc::EPOLL_CLOEXEC))?;
        Ok(Self {
            epfd: epfd as RawFd
        })
    }

    pub fn wait(&self, events: &mut Vec<Event>) -> io::Result<()> {
        events.clear();
        let n = syscall!(epoll_wait(
            self.epfd,
            events.as_mut_ptr() as *mut epoll_event,
            events.capacity() as libc::c_int,
            -1,
        ))?;
        unsafe { 
            events.set_len(n as usize);
        }
        Ok(())
    }

    pub fn wait_timeout(&self, events: &mut Vec<epoll_event>, timeout: Option<Duration>) -> io::Result<()> {
        let timeout = timeout
            .map(|to| {
                to
                    .checked_add(Duration::from_nanos(999_999))
                    .unwrap_or(to)
                    .as_millis() as libc::c_int
            })
            .unwrap_or(-1);
        events.clear();
        let n = syscall!(epoll_wait(
            self.epfd,
            events.as_mut_ptr(),
            events.capacity() as libc::c_int,
            timeout,
        ))?;
        unsafe { 
            events.set_len(n as usize);
        }
        Ok(())
    }

    pub fn register(&self, fd: RawFd, token: Token, interests: Interest) -> io::Result<()> {
        let mut event = libc::epoll_event {
            events: interests_to_epoll(interests),
            u64: usize::from(token) as u64,
        };
        syscall!(epoll_ctl(self.epfd, libc::EPOLL_CTL_ADD, fd, &mut event))?;
        Ok(())
    }

    pub fn deregister(&self, fd: RawFd) -> io::Result<()> {
        syscall!(epoll_ctl(self.epfd, libc::EPOLL_CTL_DEL, fd, std::ptr::null_mut()))?;
        Ok(())
    }
}

// epoll close
impl Drop for Epoll {
    fn drop(&mut self) {
        if let Err(err) = syscall!(close(self.epfd)) {
            panic!("error closing epoll: {}", err);
        }
    }
}

impl AsRawFd for Epoll {
    fn as_raw_fd(&self) -> RawFd {
        self.epfd as RawFd
    }
}

fn interests_to_epoll(interests: Interest) -> u32 {
    let mut kind = EPOLLET; // Edge-Triggered mechanism

    if interests.is_readable() {
        kind = kind | EPOLLIN | EPOLLRDHUP;
    }

    if interests.is_writable() {
        kind |= EPOLLOUT;
    }

    if interests.is_priority() {
        kind |= EPOLLPRI;
    }

    kind as u32
}
