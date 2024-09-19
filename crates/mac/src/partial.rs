
#[macro_export]
macro_rules! partial {
    // exhausted macro arguments, create closure
    (@inner [(() $id:expr) ($($cl_arg:ident),*) ($($fn_arg:expr),*)] ()) => {
        |$($cl_arg),*| $id($($fn_arg),*);
    };
    // with move
    (@inner [(move $id:expr) ($($cl_arg:ident),*) ($($fn_arg:expr),*)] ()) => {
        move |$($cl_arg),*| $id($($fn_arg),*);
    };

    // process forwarder '_' ,
    (@inner [$pt:tt ($($cl_arg:ident),*) ($($fn_arg:expr),*)] (_ , $($m_arg:tt)*) ) => {
        partial!(
            @inner [$pt ($($cl_arg,)* a) ($($fn_arg,)* a)] ($($m_arg)*)
        )
    };
    // last forwarder (if no trailing comma)
    (@inner [$pt:tt ($($cl_arg:ident),*) ($($fn_arg:expr),*)] (_) ) => {
        partial!(
            @inner [$pt ($($cl_arg,)* a) ($($fn_arg,)* a)] ()
        )
    };

    // process given expr
    (@inner [$pt:tt $cl_args:tt ($($fn_arg:expr),*)] ($e:expr , $($m_arg:tt)*) ) => {
        partial!(
            @inner [$pt $cl_args ($($fn_arg,)* $e)] ($($m_arg)*)
        )
    };
    // last expr (if no trailing comma)
    (@inner [$pt:tt $cl_args:tt ($($fn_arg:expr),*)] ($e:expr) ) => {
        partial!(
            @inner [$pt $cl_args ($($fn_arg,)* $e)] ()
        )
    };

    // entry points
    // ordered to match eagerly
    // move
    (move $id:expr , $($args:tt)*) => {
        partial!(@inner [(move $id) () ()] ($($args)*))
    };
    (move $id:expr ; $($args:tt)*) => {
        partial!(@inner [(move $id) () ()] ($($args)*))
    };
    (move $id:expr => $($args:tt)*) => {
        partial!(@inner [(move $id) () ()] ($($args)*))
    };

    // no move
    ($id:expr , $($args:tt)*) => {
        partial!(@inner [(() $id) () ()] ($($args)*))
    };
    ($id:expr ; $($args:tt)*) => {
        partial!(@inner [(() $id) () ()] ($($args)*))
    };
    ($id:expr => $($args:tt)*) => {
        partial!(@inner [(() $id) () ()] ($($args)*))
    };
}

// use partial_application::partial;

fn foo(a: i32, b: i32, c: i32, d: i32, mul: i32, off: i32) -> i32 {
    (a + b * b + c.pow(3) + d.pow(4)) * mul - off
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_bar() {
        let bar = partial!(foo => _, _, 10, 42, 10, 10);
        assert_eq!(foo(15, 15, 10, 42, 10, 10), bar(15, 15));
    }
}