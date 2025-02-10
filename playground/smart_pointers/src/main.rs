struct MyRc<T> {
    ptr: NonNull<RcBox<T>>,
}

struct RcBox<T> {
    value: T,
    ref_count: usize,
}

impl<T> MyRc<T> {
    fn new(value: T) -> Self {
        Self {
            ptr: Box::leak(Box::new(RcBox {
                value: value,
                ref_count: 1,
            }))
            .into(),
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        self.ptr.as_ref().ref_count += 1;
        Self::from_ptr(self.ptr)
    }
}

fn main() {}
