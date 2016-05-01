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
///     fn run(&self) {}
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
/// #     fn run(&self) {}
/// # }
/// #
/// fn call_a_function(the_function: &Fn()) {
///     the_function();
/// }
///
/// let my_runnable = MyRunnable;
///
/// call_a_function(&|| my_runnable.run());
/// ```
///
pub trait Runnable {
    fn run(&self);
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
///     fn call(&self) -> i32 {
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
/// #     fn call(&self) -> i32 {
/// #         1337
/// #     }
/// # }
/// #
/// fn call_a_function(the_function: &Fn() -> i32) {
///     let the_value = the_function();
/// }
///
/// let my_callable = MyCallable;
///
/// call_a_function( &|| -> i32 { my_callable.call() } );
/// ```
///
pub trait Callable<T> {
    fn call(&self) -> T;
}

#[cfg(test)]
mod tests {
    use super::Runnable;
    use super::Callable;

    struct MyRunnable;
    impl Runnable for MyRunnable {
        fn run(&self) {}
    }

    struct MyCallable;
    impl Callable<i32> for MyCallable {
        fn call(&self) -> i32 {
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
