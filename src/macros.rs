//! Useful macros for debugging and printing purposes.

/// Macro to get the current function name.
#[allow(unused_imports)]
#[macro_export]
macro_rules! current_function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // &name[..name.len() - 3]

        // Extract function name and the parent module/trait.
        match &name[..name.len() - 3].rfind("::") {
            Some(pos1) => &name[pos1 + 2..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

/// Override the default print! macro to disable printing when needed.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! print {
    () => {
        unsafe {
            if !$crate::debug::QUIET {
                std::print!("")
            }
        }
    };
    ($($arg:tt)*) => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            if !$crate::debug::QUIET {
                if $crate::debug::DEBUG {
                    let msg = std::fmt::format(std::format_args!($($arg)*));
                    std::println!("{}", msg);
                }
                else {
                    let mut msg = String::new();
                    let _ = write!(msg, $($arg)*);
                    std::print!("{}", msg);
                }
            }
        }
    }
}

/// Override the default println! macro to disable printing when needed.
#[allow(unused_imports, unused_unsafe)]
// #[allow_internal_unstable(format_args_nl)]
#[macro_export]
macro_rules! println {
    () => {
        unsafe {
            if !$crate::debug::QUIET {
                std::println!("")
            }
        }
    };
    ($($arg:tt)*) => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            if !$crate::debug::QUIET {
                if $crate::debug::DEBUG {
                    let msg = std::fmt::format(std::format_args!($($arg)*));
                    std::println!("{}", msg);
                }
                else {
                    // std::io::_print($crate::format_args_nl!($($arg)*));
                    let mut msg = String::new();
                    let _ = writeln!(msg, $($arg)*);
                    std::print!("{}", msg);
                }
            }
        }
    }
}

/// Print a long double line as a separator (=====).
#[allow(unused_imports)]
#[macro_export]
macro_rules! print_long_double_separator_line {
    () => {
        unsafe {
            if !$crate::debug::QUIET {
                let line = "=".repeat(55);
                std::println!("\n{}", line);
            }
        }
    };
}

/// Print a long dashed line as a separator (-----).
#[allow(unused_imports)]
#[macro_export]
macro_rules! print_long_dashed_separator_line {
    () => {
        unsafe {
            if !$crate::debug::QUIET {
                let line = "-".repeat(55);
                std::println!("\n{}", line);
            }
        }
    };
}

/// Print a short double line as a separator (=====).
#[allow(unused_imports)]
#[macro_export]
macro_rules! print_short_double_separator_line {
    () => {
        unsafe {
            if !$crate::debug::QUIET {
                let line = "=".repeat(30);
                std::println!("\n{}", line);
            }
        }
    };
}

/// Print a short dashed line as a separator (-----).
#[allow(unused_imports)]
#[macro_export]
macro_rules! print_short_dashed_separator_line {
    () => {
        unsafe {
            if !$crate::debug::QUIET {
                let line = "-".repeat(30);
                std::println!("\n{}", line);
            }
        }
    };
}

/// Core macro to print a debugging message.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! debug_core {
    ($marker:expr, $($arg:tt)*) => {
        use std::fmt::Write as FmtWrite;
        let mut msg = String::new();
        let _ = write!(msg, $($arg)*);
        std::println!("{}", msg);
    }
}

/// Macro to print a debugging message.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEBUG && !debug::QUIET {
                $crate::debug_core!("!! ", $($arg)*);
            }
        }
    }
}

/// Macro to print a deep-debugging message.
#[macro_export]
#[allow(unused_imports, unused_unsafe)]
macro_rules! ddebug {
    ($($arg:tt)*) => {
        unsafe {
            use $crate::debug;
            if debug::DEBUG_EXTRA && !debug::QUIET {
                $crate::debug_core!("!! ", $($arg)*);
            }
        }
    }
}

/// Override the default `todo!` macro to print backtrace.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! todo {
    () => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            use $crate::debug;
            if !debug::QUIET {
                let mut msg = String::from("TODO: Not yet implemented!\n");
                let _ = write!(msg, "\n> reported at: {}:{}:{}:{}",
                               std::file!(), $crate::current_function!(),
                               std::line!(), std::column!());
                std::println!("\n{}\n", msg);
            }
        }
    };
    ($($arg:tt)*) => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            use $crate::debug;
            if !$crate::debug::QUIET {
                let mut msg = String::from("TODO: ");
                let _ = write!(msg, $($arg)*);
                let _ = write!(msg, "\n> reported at: {}:{}:{}:{}",
                               std::file!(), $crate::current_function!(),
                               std::line!(), std::column!());
                std::println!("\n{}\n", msg);
            }
        }
    }
}

/// Macro to print a fixme message.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! fixme {
    () => {
        unsafe {
            if $crate::debug::DEBUG && !$crate::debug::QUIET {
                std::print!("\n")
            }
        }
    };
    ($($arg:tt)*) => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            use $crate::debug;
            if !debug::QUIET {
                let mut msg = String::from("FIXME: ");
                let _ = write!(msg, $($arg)*);
                let _ = write!(msg, "\n> reported at: {}:{}:{}:{}",
                               std::file!(), $crate::current_function!(),
                               std::line!(), std::column!());
                std::println!("\n{}\n", msg);
            }
        }
    }
}

/// Macro to print a warning message.
#[allow(unused_imports, unused_unsafe)]
#[macro_export]
macro_rules! warning {
    () => {
        unsafe {
            if !$crate::debug::QUIET {
                std::print!("\n")
            }
        }
    };
    ($($arg:tt)*) => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            use $crate::debug;
            if !debug::QUIET {
                let mut msg = String::from("Warning: ");
                let _ = write!(msg, $($arg)*);
                let _ = write!(msg, "\n\n> Reported at: {}:{}:{}:{}",
                               std::file!(), $crate::current_function!(),
                               std::line!(), std::column!());
                let _ = write!(msg, "\n\n> Run with `-D` to show backtrace if available");
                std::println!("\n{}\n", msg);
            }
        }
    }
}

/// Macro to print an error and exit the program.
#[allow(unused_imports)]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        unsafe {
            use std::fmt::Write as FmtWrite;
            use $crate::debug;
            if !debug::QUIET {
                let mut msg = String::from("Error: ");
                let _ = write!(msg, $($arg)*);
                let _ = write!(msg, "\n\n> Reported at: {}:{}:{}:{}",
                               std::file!(), $crate::current_function!(),
                               std::line!(), std::column!());
                let _ = write!(msg, "\n\n> Run with `-D` to show backtrace if available");
                std::println!("\n{}\n", msg);
            }
            std::process::exit(1)
        }
    }
}
