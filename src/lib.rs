use std::{any::Any, panic::panic_any};

pub fn throw<T: Any + Send>(exception: T) -> ! {
    panic_any(exception)
}

#[macro_export]
macro_rules! catch {
    ($block:block $(except $($name:ident:$ty:ty $catch:block)? $(_ $catch_:block)? $(;)?)*) => {
        {
            let hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let result = std::panic::catch_unwind(|| $block);
            std::panic::set_hook(hook);
            let re = match result {
                Ok(value) => value,
                Err(exception) => {
                    if false { unreachable!() } $(
                    else
                        $(if let Some($name) = exception.downcast_ref::<$ty>() {
                            $catch
                        })?
                        $(if true {
                            $catch_
                        })?
                    )* else {
                        if let Some(s) = exception.downcast_ref::<&str>() {
                            eprintln!("uncaught exception: {}", s);
                        } else if let Some(s) = exception.downcast_ref::<String>() {
                            eprintln!("uncaught exception: {}", s);
                        } else {
                            eprintln!("uncaught exception: {:?}", exception);
                        }
                        std::panic::resume_unwind(exception)
                    }
                }
            };
            re
        }
    };
}
