use core::fmt;
use std::{error::Error};

#[derive(Debug)]
pub(crate) enum ErrorType{
    NotFound,
    PermissionDenied,
    AlreadyExists,
    WouldBlock,
    NotADirectory,
    IsADirectory,
    DirectoryNotEmpty,
    ReadOnlyFilesystem,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    StorageFull,
    FileTooLarge,
    ResourceBusy,
    Deadlock,
    ArgumentListTooLong,
    Interrupted,
    Unsupported,
    UnexpectedEof,
    OutOfMemory,
    Other
}

impl From<std::io::ErrorKind> for ErrorType{
    fn from(value: std::io::ErrorKind) -> Self {
        match value{
            std::io::ErrorKind::NotFound => return ErrorType::NotFound,
            std::io::ErrorKind::PermissionDenied => return ErrorType::PermissionDenied,
            std::io::ErrorKind::AlreadyExists => return ErrorType::AlreadyExists,
            std::io::ErrorKind::WouldBlock => return ErrorType::WouldBlock,
            std::io::ErrorKind::NotADirectory => return ErrorType::NotADirectory,
            std::io::ErrorKind::IsADirectory => return ErrorType::IsADirectory,
            std::io::ErrorKind::DirectoryNotEmpty => return ErrorType::DirectoryNotEmpty,
            std::io::ErrorKind::ReadOnlyFilesystem => return ErrorType::ReadOnlyFilesystem,
            std::io::ErrorKind::InvalidInput => return ErrorType::InvalidInput,
            std::io::ErrorKind::InvalidData => return ErrorType::InvalidData,
            std::io::ErrorKind::TimedOut => return ErrorType::TimedOut,
            std::io::ErrorKind::WriteZero => return ErrorType::WriteZero,
            std::io::ErrorKind::StorageFull => return ErrorType::StorageFull,
            std::io::ErrorKind::FileTooLarge => return ErrorType::FileTooLarge,
            std::io::ErrorKind::ResourceBusy => return ErrorType::ResourceBusy,
            std::io::ErrorKind::Deadlock => return ErrorType::Deadlock,
            std::io::ErrorKind::ArgumentListTooLong => return ErrorType::ArgumentListTooLong,
            std::io::ErrorKind::Interrupted => return ErrorType::Interrupted,
            std::io::ErrorKind::Unsupported => return ErrorType::Unsupported,
            std::io::ErrorKind::UnexpectedEof => return ErrorType::UnexpectedEof,
            std::io::ErrorKind::OutOfMemory => return ErrorType::OutOfMemory,
            _ => return ErrorType::Other,
        }
    }
}




#[derive(Debug)]
pub(crate) struct EditorIoError{
    message: String,
    error_type: ErrorType
}
impl EditorIoError{
    pub(crate) fn new(msg: &str, etype: ErrorType) -> EditorIoError{
        EditorIoError { 
            message: msg.to_string(), 
            error_type: etype 
        }
    }
}

impl Error for EditorIoError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl fmt::Display for EditorIoError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERROR: {:?} msg: {}", self.error_type, self.message)
    }
}

impl From<std::io::Error> for EditorIoError{
    fn from(value: std::io::Error) -> Self {
        let msg = value.to_string();
        let etype = value.kind();

        return EditorIoError { 
            message: msg, 
            error_type: etype.into() 
        }
    }
}