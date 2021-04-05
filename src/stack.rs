use std::sync::atomic::{AtomicPtr, Ordering};

unsafe impl<T: Send> Sync for Stack<T> {}

struct Node<T: Send> {
    next: AtomicPtr<Node<T>>,
    value: Option<T>,
}

impl<T: Send> Node<T> {
    fn new(x: T) -> Self {
        Self {
            next: AtomicPtr::default(),
            value: Some(x),
        }
    }

    fn sentry() -> Self {
        Self {
            next: AtomicPtr::default(),
            value: None,
        }
    }
}

pub struct Stack<T: Send> {
    top: AtomicPtr<Node<T>>,
}

impl<T: Send> Stack<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node::sentry()));

        Stack {
            top: AtomicPtr::new(dummy),
        }
    }

    pub fn push(&self, x: T) {
        unsafe { self.try_push(x) }
    }

    unsafe fn try_push(&self, x: T) {
        let node = Box::leak(Box::new(Node::new(x)));

        loop {
            let top_ptr = self.top.load(Ordering::Acquire);
            node.next.store(top_ptr, Ordering::Relaxed);

            if self
                .top
                .compare_exchange(top_ptr, node, Ordering::Release, Ordering::Relaxed)
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
        loop {
            let top_ptr = self.top.load(Ordering::Acquire);
            let next_ptr = (*top_ptr).next.load(Ordering::Acquire);

            if next_ptr.is_null() {
                return None;
            }

            if self
                .top
                .compare_exchange(top_ptr, next_ptr, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                let mut node = Box::from_raw(top_ptr);
                return node.value.take();
            }
        }
    }
}

impl<T: Send> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut p = self.top.load(Ordering::Relaxed);

        while !p.is_null() {
            unsafe {
                let next = (*p).next.load(Ordering::Relaxed);
                Box::from_raw(p);
                p = next;
            }
        }
    }
}
