use std::env::set_current_dir;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, stderr, stdin, stdout};
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;

macro_rules! sys_call {
    ($f: ident ( $($args:expr),* $(,)? )) => {
        {
            let res = unsafe { libc::$f($($args),*) };

            match res.cmp(&0) {
                std::cmp::Ordering::Less => Err(std::io::Error::last_os_error()),
                _ => Ok(res)
            }
        }
    };
}

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

    pub fn daemonize(&mut self) -> io::Result<()> {
        sys_call!(fork())?;
        set_current_dir(self.home_dir.as_ref().unwrap_or(&PathBuf::from("/")))?;
        sys_call!(setsid())?;
        sys_call!(umask(self.mask.unwrap_or(0)))?;
        sys_call!(fork())?; // double-fork, this is a magic, lol :)

        self.record_pid()?;

        let devnull = File::open("/dev/null")?;
        let devnull = devnull.as_raw_fd();
        sys_call!(dup2(devnull, stdin().as_raw_fd()))?;
        sys_call!(dup2(devnull, stdout().as_raw_fd()))?;
        sys_call!(dup2(devnull, stderr().as_raw_fd()))?;

        Ok(())
    }

    fn record_pid(&mut self) -> io::Result<()> {
        if let Some(pidfile) = &self.pidfile {
            let mut file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(pidfile)?;

            let pid = sys_call!(getpid())?;
            println!("PID: {}", pid);

            file.write_all(format!("{}", pid).as_bytes())?;
        }

        Ok(())
    }
}
