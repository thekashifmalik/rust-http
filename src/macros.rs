#![macro_escape]


/// Like try! but for Option<T> - returns None
macro_rules! optional(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
)


/// Like try! but for Option<T> - returns Err(())
macro_rules! optional_try(
    ($e:expr) => (match $e { Some(e) => e, None => return Err(()) })
)
