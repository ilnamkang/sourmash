use std::mem;
use std::panic;
use std::thread;
use std::cell::RefCell;

use backtrace::Backtrace;

use sourmash::errors::{ErrorKind, Error, Result};
use sourmash::core::SourmashErrorCode;

thread_local! {
    pub static LAST_ERROR: RefCell<Option<Error>> = RefCell::new(None);
    pub static LAST_PANIC: RefCell<Option<(String, Backtrace)>> = RefCell::new(None);
    pub static LAST_BACKTRACE: RefCell<Option<(Option<String>, Backtrace)>> = RefCell::new(None);
}

fn notify_err(err: Error) {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = Some(err);
    });
}

/// Clears the last error.
#[no_mangle]
pub unsafe extern "C" fn sourmash_err_clear() {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = None;
    });
    LAST_BACKTRACE.with(|e| {
        *e.borrow_mut() = None;
    });
}

/// Initializes the library
#[no_mangle]
pub unsafe extern "C" fn sourmash_init() {
    set_panic_hook();
}

/// Returns the last error code.
///
/// If there is no error, 0 is returned.
#[no_mangle]
pub unsafe extern "C" fn sourmash_err_get_last_code() -> SourmashErrorCode {
    LAST_ERROR.with(|e| {
        if let Some(ref err) = *e.borrow() {
            SourmashErrorCode::from_kind(err.kind())
        } else {
            SourmashErrorCode::NoError
        }
    })
}

pub unsafe fn set_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let backtrace = Backtrace::new();
        let thread = thread::current();
        let thread = thread.name().unwrap_or("unnamed");

        let msg = match info.payload().downcast_ref::<&str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &**s,
                    None => "Box<Any>",
                }
            }
        };

        let panic_info = match info.location() {
            Some(location) => {
                format!("thread '{}' panicked with '{}' at {}:{}",
                                     thread, msg, location.file(),
                                     location.line())
            }
            None => {
                format!("thread '{}' panicked with '{}'", thread, msg)
            }
        };

        LAST_PANIC.with(|e| {
            *e.borrow_mut() = Some((panic_info, backtrace));
        });
    }));
}

pub unsafe fn landingpad<F: FnOnce() -> Result<T> + panic::UnwindSafe, T>(
    f: F) -> T
{
    match panic::catch_unwind(f) {
        Ok(rv) => rv.map_err(|err| notify_err(err)).unwrap_or(mem::zeroed()),
        Err(err) => {
            use std::any::Any;
            let err = &*err as &Any;
            let msg = match err.downcast_ref::<&str>() {
                Some(s) => *s,
                None => {
                    match err.downcast_ref::<String>() {
                        Some(s) => &**s,
                        None => "Box<Any>",
                    }
                }
            };
            notify_err(ErrorKind::Panic(msg.to_string()).into());
            mem::zeroed()
        }
    }
}

macro_rules! ffi_fn (
    // a function that catches panics and returns a result (err goes to tls)
    (
        $(#[$attr:meta])*
        unsafe fn $name:ident($($aname:ident: $aty:ty),* $(,)*) -> Result<$rv:ty> $body:block
    ) => (
        #[no_mangle]
        $(#[$attr])*
        pub unsafe extern "C" fn $name($($aname: $aty,)*) -> $rv
        {
            $crate::utils::landingpad(|| $body)
        }
    );

    // a function that catches panics and returns nothing (err goes to tls)
    (
        $(#[$attr:meta])*
        unsafe fn $name:ident($($aname:ident: $aty:ty),* $(,)*) $body:block
    ) => {
        #[no_mangle]
        $(#[$attr])*
        pub unsafe extern "C" fn $name($($aname: $aty,)*)
        {
            // this silences panics and stuff
            $crate::utils::landingpad(|| { $body; Ok(0 as ::std::os::raw::c_int) });
        }
    }
);
