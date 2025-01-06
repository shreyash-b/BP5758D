#![macro_use]
#![allow(unused_macros)]

macro_rules! trace {
     ($s:literal $(, $x:expr)* $(,)?) => {
        {
            #[cfg(feature = "log")]
            log::trace!($s $(, $x)*);
            #[cfg(not(feature = "log"))]
            let _ = ($( & $x ),*);
        }
    };
}

macro_rules! debug {
     ($s:literal $(, $x:expr)* $(,)?) => {
        {
            #[cfg(feature = "log")]
            log::debug!($s $(, $x)*);
            #[cfg(not(feature = "log"))]
            let _ = ($( & $x ),*);
        }
    };
}

macro_rules! info {
     ($s:literal $(, $x:expr)* $(,)?) => {
        {
            #[cfg(feature = "log")]
            log::info!($s $(, $x)*);
            #[cfg(not(feature = "log"))]
            let _ = ($( & $x ),*);
        }
    };
}

macro_rules! error {
     ($s:literal $(, $x:expr)* $(,)?) => {
        {
            #[cfg(feature = "log")]
            log::error!($s $(, $x)*);
            #[cfg(not(feature = "log"))]
            let _ = ($( & $x ),*);
        }
    };
}
