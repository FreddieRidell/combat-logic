use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RPGErrorKind {
    DiceExpressionInvalid,
    TomeEntryInvalid,
    TomeEntryNotFound,
}

impl fmt::Display for RPGErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RPGErrorKind::*;

        write!(
            f,
            "{}",
            match self {
                DiceExpressionInvalid => "DiceExpressionInvalid",
                TomeEntryInvalid => "TomeEntryInvalid",
                TomeEntryNotFound => "TomeEntryNotFound",
            }
        )
    }
}

impl RPGError {
    pub fn into_serde_error<DeError: serde::de::Error>(self) -> DeError {
        DeError::custom(format!("{}", self))
    }
}

#[derive(Debug)]
pub struct RPGError {
    kind: RPGErrorKind,
    meta: Option<String>,
    source: Option<Box<dyn Error + 'static>>,
}

pub type RPGResult<T> = Result<T, RPGError>;

impl RPGError {
    pub fn new(kind: RPGErrorKind) -> Self {
        Self {
            kind,
            meta: None,
            source: None,
        }
    }

    pub fn msg(mut self, meta: &'static str) -> Self {
        self.meta = Some(meta.to_owned());
        self
    }

    pub fn with_msg<F: Fn() -> String>(mut self, meta_factory: F) -> Self {
        self.meta = Some(meta_factory());
        self
    }

    pub fn from<E: 'static + Error>(mut self, source: E) -> Self {
        self.source = Some(Box::new(source));
        self
    }
}

impl fmt::Display for RPGError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RPGError::{}", self.kind)?;

        if let Some(meta_text) = &self.meta {
            write!(f, ": {}", meta_text)?
        }

        if let Some(from_err) = &self.source {
            write!(f, " ({})", from_err)?
        }

        Ok(())
    }
}

impl Error for RPGError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(source_box) = &self.source {
            Some(source_box.as_ref())
        } else {
            None
        }
    }
}

impl From<RPGError> for String {
    fn from(error: RPGError) -> Self {
        format!("{}", error)
    }
}
