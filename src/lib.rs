/// A Runnable is a single function trait with no return value
///
/// # Examples
///
/// Simple runnable that does nothing:
///
/// ```
/// use rust_runnables::Runnable;
///
/// struct MyRunnable;
///
/// impl Runnable for MyRunnable {
///     fn run(&mut self) {}
/// }
/// ```
///
/// Passing a Runnable to something that expects a function pointer:
///
/// ```
/// # use rust_runnables::Runnable;
/// #
/// # struct MyRunnable;
/// # impl Runnable for MyRunnable {
/// #     fn run(&mut self) {}
/// # }
/// #
/// fn call_a_function(the_function: &mut FnMut()) {
///     the_function();
/// }
///
/// let mut my_runnable = MyRunnable;
/// call_a_function( &mut || my_runnable.run());
/// ```
///
/// Or for a function that takes an immutable `Fn()`:
///
/// ```
/// # use rust_runnables::Runnable;
/// #
/// # struct MyRunnable;
/// # impl Runnable for MyRunnable {
/// #     fn run(&mut self) {}
/// # }
/// #
/// use std::cell::RefCell;
///
/// fn call_a_function(the_function: &Fn()) {
///     the_function();
/// }
///
/// let celled_runnable = RefCell::new(MyRunnable);
/// call_a_function( &|| celled_runnable.borrow_mut().run());
/// ```
///
pub trait Runnable {
    fn run(&mut self);
}

/// A Callable is a single function trait that returns a value
///
/// # Examples
///
/// ```
/// use rust_runnables::Callable;
///
/// struct MyCallable;
///
/// impl Callable<i32> for MyCallable {
///     fn call(&mut self) -> i32 {
///         1337
///     }
/// }
/// ```
///
/// Passing a Callable to something that expects a function pointer:
///
/// ```
/// # use rust_runnables::Callable;
/// #
/// # struct MyCallable;
/// # impl Callable<i32> for MyCallable {
/// #     fn call(&mut self) -> i32 {
/// #         1337
/// #     }
/// # }
/// #
/// fn call_a_function(the_function: &mut FnMut() -> i32) {
///     let the_value = the_function();
/// }
///
/// let mut my_callable = MyCallable;
/// call_a_function( &mut || -> i32 { my_callable.call() } );
/// ```
///
/// Or for a function that takes an immutable `Fn()`:
///
/// ```
/// # use rust_runnables::Callable;
/// #
/// # struct MyCallable;
/// # impl Callable<i32> for MyCallable {
/// #     fn call(&mut self) -> i32 {
/// #         1337
/// #     }
/// # }
/// #
/// use std::cell::RefCell;
///
/// fn call_a_function(the_function: &Fn() -> i32) {
///     let the_value = the_function();
/// }
///
/// let celled_callable = RefCell::new(MyCallable);
/// call_a_function( &|| -> i32 { celled_callable.borrow_mut().call() } );
/// ```
///
pub trait Callable<T> {
    fn call(&mut self) -> T;
}

#[cfg(test)]
mod tests {
    use super::Runnable;
    use super::Callable;

    struct MyRunnable;
    impl Runnable for MyRunnable {
        fn run(&mut self) {}
    }

    struct MyCallable;
    impl Callable<i32> for MyCallable {
        fn call(&mut self) -> i32 {
            1337
        }
    }

    struct StructWithClosure<'life> {
        closure_field: &'life Fn(),
    }

    struct StructTwoWithClosure<'life> {
        closure_field: &'life Fn() -> i32,
    }

    #[test]
    fn runnable_can_run() {
        MyRunnable.run();
    }

    #[test]
    fn runnable_passed_as_closure() {
        let my_struct = StructWithClosure { closure_field: &|| MyRunnable.run() };
        assert_eq!((my_struct.closure_field)(), ());
    }

    #[test]
    fn callable_can_be_called() {
        assert_eq!(MyCallable.call(), 1337);
    }

    #[test]
    fn callable_passed_as_closure() {
        let struct_two = StructTwoWithClosure { closure_field: &|| -> i32 { MyCallable.call() } };
        assert_eq!((struct_two.closure_field)(), 1337);
    }
}
