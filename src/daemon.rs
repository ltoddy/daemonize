use std::env::set_current_dir;
use std::fs::{File, OpenOptions};
use std::os::unix::io::IntoRawFd;
use std::path::PathBuf;

use crate::ffi_wrapper::{apply_fork, duplicate_file_descriptor2, get_pid, set_session_id, umask};
use crate::result::Result;
use std::io::Write;

pub struct Daemon {
    pidfile: Option<PathBuf>,
    home_dir: Option<PathBuf>,
    mask: Option<libc::mode_t>,
}

impl Daemon {
    pub fn new(pidfile: Option<PathBuf>, home_dir: Option<PathBuf>, mask: Option<libc::mode_t>) -> Self {
        Daemon {
            pidfile,
            home_dir,
            mask,
        }
    }

    pub fn daemonize(&mut self) -> Result<()> {
        apply_fork()?;
        set_current_dir(self.home_dir.as_ref().unwrap_or(&PathBuf::from("/")))?;
        set_session_id()?;
        umask(self.mask.unwrap_or(0));
        apply_fork()?; // double-fork, this is a magic, lol :)

        let devnull = File::open("/dev/null")?;
        let devnull = devnull.into_raw_fd();
        duplicate_file_descriptor2(devnull, libc::STDIN_FILENO)?;
        duplicate_file_descriptor2(devnull, libc::STDOUT_FILENO)?;
        duplicate_file_descriptor2(devnull, libc::STDOUT_FILENO)?;

        self.record_pid()?;
        Ok(())
    }

    fn record_pid(&mut self) -> Result<()> {
        if let Some(pidfile) = &self.pidfile {
            let mut file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(pidfile)?;

            let pid = get_pid()?;

            file.write_all(format!("{}", pid).as_bytes())?;
        }

        Ok(())
    }
}
