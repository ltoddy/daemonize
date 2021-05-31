use libc::c_int;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Error {
    /// Unable to fork
    Fork,
    /// Unable to create new session
    DetachSession(c_int),
    /// Unable to resolve group name to group id
    GroupNotFound,
    /// Group option contains NUL
    GroupContainsNul,
    /// Unable to set group
    SetGroup(c_int),
    /// Unable to resolve user name to user id
    UserNotFound,
    /// User option contains NUL
    UserContainsNul,
    /// Unable to set user
    SetUser(c_int),
    /// Unable to change directory
    ChangeDirectory,
    /// pid_file option contains NUL
    PathContainsNulError,
    /// Unable to open pid file
    OpenPidfile,
    /// Unable to lock pid file
    LockPidfile(c_int),
    /// Unable to unlock pid file
    UnlockPidfile(c_int),
    /// Unable to chown pid file
    ChownPidfile(c_int),
    /// Unable to redirect standard streams to /dev/null
    RedirectStreams(c_int),
    /// Unable to write self pid to pid file
    WritePid,
    /// Unable to chroot
    Chroot(c_int),
    /// user lacks permission change attributes on the underlying file
    LackPermission,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Fork => write!(f, "unable to fork"),
            Error::DetachSession(code) => write!(f, "unable to create new session, code: {}", code),
            Error::GroupNotFound => write!(f, "unable to resolve group name to group id"),
            Error::GroupContainsNul => write!(f, "group option contains NUL"),
            Error::SetGroup(code) => write!(f, "unable to set group, code: {}", code),
            Error::UserNotFound => write!(f, "unable to resolve user name to user id"),
            Error::UserContainsNul => write!(f, "user option contains NUL"),
            Error::SetUser(code) => write!(f, "unable to set user, code: {}", code),
            Error::ChangeDirectory => write!(f, "unable to change directory"),
            Error::PathContainsNulError => write!(f, "nul byte found in path"),
            Error::OpenPidfile => write!(f, "unable to open pid file"),
            Error::LockPidfile(code) => write!(f, "unable to lock pid file: {}", code),
            Error::UnlockPidfile(code) => write!(f, "unable to unlock pid file, code: {}", code),
            Error::ChownPidfile(code) => write!(f, "unable to chown pid file, code: {}", code),
            Error::RedirectStreams(code) => {
                write!(f, "unable to redirect standard streams to /dev/null, code: {}", code)
            }
            Error::WritePid => write!(f, "unable to write self pid to pid file"),
            Error::Chroot(code) => write!(f, "unable to change root into directory, code: {}", code),
            Error::LackPermission => write!(f, "user lacks permission change attributes on the underlying file"),
        }
    }
}

impl std::error::Error for Error {}
