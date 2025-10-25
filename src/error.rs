use std::{backtrace::Backtrace, error::Error, fmt::Display};

pub type Resultx<T> = Result<T, Errx>;

#[derive(Debug)]
pub struct Errx {
    src: Option<Box<dyn Error + 'static>>,
    bkt: Option<Backtrace>,
    knd: Kndx,
}

impl Errx {
    pub fn new(src: Option<Box<dyn Error + 'static>>, knd: Kndx) -> Self {
        if let Some(src) = src {
            match src.downcast::<Self>() {
                Ok(mut e) => {
                    let bkt = e.bkt.take();
                    Self {
                        src: Some(e),
                        bkt,
                        knd,
                    }
                }
                Err(e) => Self {
                    src: Some(e),
                    bkt: Some(Backtrace::force_capture()),
                    knd,
                },
            }
        } else {
            Self {
                src: None,
                bkt: Some(Backtrace::force_capture()),
                knd,
            }
        }
    }

    pub fn g(msg: impl Into<String>) -> Self {
        Self::new(None, Kndx::G(msg.into()))
    }

    pub fn e_g(src: impl Error + 'static, msg: impl Into<String>) -> Self {
        Self::new(Some(Box::new(src)), Kndx::G(msg.into()))
    }

    pub fn io(msg: impl Into<String>) -> Self {
        Self::new(None, Kndx::Io(msg.into()))
    }

    pub fn e_io(src: impl Error + 'static, msg: impl Into<String>) -> Self {
        Self::new(Some(Box::new(src)), Kndx::Io(msg.into()))
    }

    pub fn chain(&self) -> Vec<String> {
        let mut chain: Vec<String> = Vec::new();
        let mut source = Some(self as &dyn Error);
        while let Some(err) = source {
            chain.push(err.to_string());
            source = err.source();
        }
        chain
    }

    pub fn log(&self) {
        let msg = self.to_string();
        let error_chain = self.chain();
        let backtrace = self.bkt.as_ref().map(collect_std_backtrace);
        eprintln!("{msg}; chain: {error_chain:?}; backtrace: {backtrace:?}");
    }
}

impl Display for Errx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.knd)
    }
}

impl Error for Errx {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.src.as_deref()
    }
}

#[derive(Debug)]
pub enum Kndx {
    G(String),
    Io(String),
}

impl Display for Kndx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kndx::G(msg) => write!(f, "generic error: {}", msg),
            Kndx::Io(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

fn collect_std_backtrace(backtrace: &Backtrace) -> Vec<String> {
    fn is_dependency_code(symbol_line: &str, at_line: Option<&str>) -> bool {
        let project = "pyrope";
        if symbol_line.contains(project) || at_line.is_some_and(|x| x.contains(project)) {
            return false;
        }

        const SYM_PREFIXES: &[&str] = &[
            "std::",
            "core::",
            "backtrace::backtrace::",
            "_rust_begin_unwind",
            "__rust_",
            "___rust_",
            "__pthread",
            "_main",
            "__scrt_common_main_seh",
            "BaseThreadInitThunk",
            "_start",
            "__libc_start_main",
            "start_thread",
        ];

        if SYM_PREFIXES.iter().any(|x| symbol_line.contains(x)) {
            return true;
        }

        let Some(at_line) = at_line else {
            return false;
        };

        const FILE_PREFIXES: &[&str] = &[
            "/rust/",
            "/rustc/",
            "rustup/toolchains",
            "src/libstd/",
            "src/libpanic_unwind/",
            "src/libtest/",
            "cargo/registry/src/",
            "cargo/git/checkouts/",
        ];

        if FILE_PREFIXES.iter().any(|x| at_line.contains(x)) {
            return true;
        }

        false
    }

    let formatted_backtrace = format!("{backtrace}");
    let mut last_symbol: Option<&str> = None;
    let mut frames: Vec<(&str, Option<&str>)> = Vec::new();
    for partial_frame in formatted_backtrace.lines() {
        let trimmed = partial_frame.trim();
        let is_symbol_line = !trimmed.starts_with("at ");
        match (is_symbol_line, last_symbol) {
            (true, None) => {
                last_symbol = Some(trimmed);
            }
            (true, Some(sl)) => {
                frames.push((sl, None));
                last_symbol = Some(trimmed);
            }
            (false, None) => {
                frames.push(("", Some(trimmed)));
            }
            (false, Some(sl)) => {
                frames.push((sl, Some(trimmed)));
                last_symbol = None;
            }
        }
    }

    frames
        .into_iter()
        .filter(|(symbol_line, at_line)| !is_dependency_code(symbol_line, *at_line))
        .map(|(symbol_line, at_line)| {
            if let Some(at_line) = at_line {
                format!("{} {}", symbol_line, at_line.trim())
            } else {
                symbol_line.to_string()
            }
        })
        .collect()
}
