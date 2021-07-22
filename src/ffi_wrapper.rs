use crate::error::{Error, ErrorKind};
use crate::result::Result;

pub(crate) fn apply_fork() -> Result<()> {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            let err = Error::errno();
            return Err(Error::custom(
                ErrorKind::Fork,
                format!("fork #1 failed: {} ({})", err, Error::strerror(err)),
            ));
        }

        if pid > 0 {
            std::process::exit(0);
        }
    }

    Ok(())
}

pub(crate) fn set_session_id() -> Result<()> {
    unsafe {
        let pid = libc::setsid();
        if pid < 0 {
            let err = Error::errno();
            return Err(Error::custom(
                ErrorKind::DetachSession,
                format!("setsid #1 failed: {} ({})", err, Error::strerror(err)),
            ));
        }
    }
    Ok(())
}

// This system call always succeeds and the previous value of the mask is returned.
pub(crate) fn umask(mask: libc::mode_t) {
    unsafe {
        libc::umask(mask);
    }
}

pub(crate) fn duplicate_file_descriptor2(src: libc::c_int, dst: libc::c_int) -> Result<()> {
    unsafe {
        let newfd = libc::dup2(src, dst);
        if newfd < 0 {
            let err = Error::errno();
            return Err(Error::custom(
                ErrorKind::DuplicateFileDescriptor,
                format!("dup2 #1 failed: {} ({})", err, Error::strerror(err)),
            ));
        }
    }

    Ok(())
}

pub(crate) fn get_pid() -> Result<libc::pid_t> {
    unsafe {
        let pid = libc::getpid();
        if pid < 0 {
            return Err(Error::custom(ErrorKind::Fork, format!("")));
        }

        Ok(pid)
    }
}
