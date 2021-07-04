use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

use crossbeam_epoch::{self as epoch, Atomic, Owned};

unsafe impl<T: Send> Sync for Stack<T> {}

struct Node<T: Send> {
    next: Atomic<Node<T>>,
    value: Option<T>,
}

impl<T: Send> Node<T> {
    fn new(v: T) -> Self {
        Self {
            next: Atomic::null(),
            value: Some(v),
        }
    }

    fn sentinel() -> Self {
        Self {
            next: Atomic::null(),
            value: None,
        }
    }
}

pub struct Stack<T: Send> {
    top: Atomic<Node<T>>,
}

impl<T: Send> Stack<T> {
    pub fn new() -> Self {
        let s = Stack {
            top: Atomic::null(),
        };

        let sentinel = Owned::new(Node::sentinel());
        let guard = unsafe { &epoch::unprotected() };

        let sentinel = sentinel.into_shared(guard);
        s.top.store(sentinel, Relaxed);
        s
    }

    pub fn push(&self, v: T) {
        unsafe { self.try_push(v) }
    }

    unsafe fn try_push(&self, v: T) {
        let guard = &epoch::pin();
        let node = Owned::new(Node::new(v)).into_shared(guard);

        loop {
            let top_ptr = self.top.load(Acquire, guard);
            (*node.as_raw()).next.store(top_ptr, Relaxed);

            if self
                .top
                .compare_exchange(top_ptr, node, Release, Relaxed, guard)
                .is_ok()
            {
                break;
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        unsafe { self.try_pop() }
    }

    unsafe fn try_pop(&self) -> Option<T> {
        let guard = &epoch::pin();

        loop {
            let top_ptr = self.top.load(Acquire, guard);
            let next_ptr = (*top_ptr.as_raw()).next.load(Acquire, guard);

            if next_ptr.is_null() {
                return None;
            }

            if self
                .top
                .compare_exchange(top_ptr, next_ptr, Release, Relaxed, guard)
                .is_ok()
            {
                let top_ptr = top_ptr.as_raw() as *mut Node<T>;
                return (*top_ptr).value.take();
            }
        }
    }
}
