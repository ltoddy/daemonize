#![allow(dead_code, clippy::new_without_default)]

pub mod error;
pub mod result;

use std::cmp::Ordering;
use std::env::set_current_dir;
use std::ffi::CString;
use std::fs::{File, OpenOptions, Permissions};
use std::io;
use std::io::prelude::*;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use std::process;

use libc::{
    c_int, chown, chroot, flock, fork, getpid, gid_t, mode_t, setsid, setuid, uid_t, umask, LOCK_NB, LOCK_SH, LOCK_UN,
};

use crate::error::Error;
use crate::result::Result;

pub struct Daemonize {
    current_dir: PathBuf,
    pid_file: Option<PathBuf>,
    chown_pid_file: bool,
    user: Option<uid_t>,
    group: Option<gid_t>,
    umask: mode_t,
    root: Option<PathBuf>,
}

impl Daemonize {
    pub fn new() -> Self {
        Daemonize {
            current_dir: PathBuf::from("/Users/ltoddy/github/ltoddy/daemonize-rs"),
            pid_file: None,
            chown_pid_file: false,
            user: None,
            group: None,
            umask: 0o027,
            root: None,
        }
    }
}

impl Daemonize {
    pub fn start(self) -> Result<()> {
        unsafe {
            perform_fork()?;

            set_current_dir(self.current_dir).map_err(|_| Error::ChangeDirectory)?;
            // set_sid()?;
            umask(self.umask);

            create_pid_file(PathBuf::from("daemonize.pid"))?;
            write_pid_file(PathBuf::from("daemonize.pid"))?;
        }

        Ok(())
    }
}

unsafe fn set_sid() -> Result<()> {
    if setsid() == 0 {
        return Ok(());
    }
    Err(Error::DetachSession(errno()))
}

unsafe fn perform_fork() -> Result<()> {
    let pid = fork();
    match pid.cmp(&0) {
        Ordering::Less => Err(Error::Fork),
        Ordering::Equal => Ok(()),
        Ordering::Greater => process::exit(9),
    }
}

unsafe fn set_user(user: uid_t) -> Result<()> {
    if setuid(user) != 0 {
        return Err(Error::SetUser(errno()));
    }
    Ok(())
}

// change file owner and group
unsafe fn change_file_owner_and_group(filename: PathBuf, uid: uid_t, gid: gid_t) -> Result<()> {
    let path = pathbuf_into_cstring(filename)?;
    if chown(path.as_ptr(), uid, gid) == -1 {
        return Err(Error::ChownPidfile(errno()));
    }

    Ok(())
}

fn create_pid_file(filename: PathBuf) -> Result<()> {
    if !filename.exists() {
        File::create(filename).map_err(|_| Error::OpenPidfile)?;
    }
    Ok(())
}

unsafe fn write_pid_file(filename: PathBuf) -> Result<()> {
    let pid = getpid();

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)
        .map_err(|_| Error::OpenPidfile)?;

    let perm = Permissions::from_mode(0o644);
    file.set_permissions(perm).map_err(|_| Error::LackPermission)?;

    let fd = file.as_raw_fd();
    if flock(fd, LOCK_SH | LOCK_NB) == -1 {
        return Err(Error::LockPidfile(errno()));
    }

    let content = format!("{}", pid);
    file.write(content.as_bytes()).map_err(|_| Error::WritePid)?;

    if flock(fd, LOCK_UN) == -1 {
        return Err(Error::UnlockPidfile(errno()));
    }

    Ok(())
}

unsafe fn change_root(path: PathBuf) -> Result<()> {
    let path = pathbuf_into_cstring(path)?;

    if chroot(path.as_ptr()) == 0 {
        Ok(())
    } else {
        Err(Error::Chroot(errno()))
    }
}

#[inline]
fn pathbuf_into_cstring(path: PathBuf) -> Result<CString> {
    CString::new(path.into_os_string().into_vec()).map_err(|_| Error::PathContainsNulError)
}

#[inline]
fn errno() -> c_int {
    io::Error::last_os_error().raw_os_error().expect("errno")
}
