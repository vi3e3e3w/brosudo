use crate::{pam::PamError, system::Hostname};
use std::{borrow::Cow, fmt, path::PathBuf};

use super::{SudoPath, SudoString};

#[derive(Debug)]
pub enum Error {
    Silent,
    NotAllowed {
        username: SudoString,
        command: Cow<'static, str>,
        hostname: Hostname,
        other_user: Option<SudoString>,
    },
    SelfCheckSetuid,
    SelfCheckNoNewPrivs,
    CommandNotFound(PathBuf),
    InvalidCommand(PathBuf),
    ChDirNotAllowed {
        chdir: SudoPath,
        command: PathBuf,
    },
    UserNotFound(String),
    GroupNotFound(String),
    Authorization(String),
    InteractionRequired,
    EnvironmentVar(Vec<String>),
    Configuration(String),
    Options(String),
    Pam(PamError),
    Io(Option<PathBuf>, std::io::Error),
    MaxAuthAttempts(u16),
    PathValidation(PathBuf),
    StringValidation(String),
    #[cfg(feature = "apparmor")]
    AppArmor(String, std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Silent => Ok(()),
            Error::NotAllowed {
                username,
                command,
                hostname,
                other_user,
            } => {
                if let Some(other_user) = other_user {
                    xlat_write!(
                        f,
                        "Oops, {user}, you can't run '{command}' as {other_user} on {hostname}, bro.",
                        user = username,
                        command = command,
                        other_user = other_user,
                        hostname = hostname,
                    )
                } else {
                    xlat_write!(
                        f,
                        "Oops, {user}, you can't run '{command}' on {hostname}, bro.",
                        user = username,
                        command = command,
                        hostname = hostname,
                    )
                }
            }

            Error::SelfCheckSetuid => {
                xlat_write!(
                    f,
                    "Nah, I can't cook without root privileges (UID 0) and the setuid bit set. \
        Without them, I may run, but I can't switch to the user you requested."
                )
            }
            Error::SelfCheckNoNewPrivs => {
                xlat_write!(
                    f,
                    "OH NO! The \"no new privileges\" flag is set, \
        so I can't gain the privileges I need, bro.\n\
        If I'm running in a container, check its security configuration \
        and the intended privilege model.\n\
        If I'm running on your main machine, review the process security \
        settings before making changes.\n\
        Bro, check your security first!"
                )
            }

            Error::CommandNotFound(p) => {
                xlat_write!(f, "'{path}': command not found, bro.", path = p.display())
            }
            Error::InvalidCommand(p) => {
                xlat_write!(
                    f,
                    "'{path}': that command doesn't look valid, bro.",
                    path = p.display()
                )
            }
            Error::UserNotFound(u) => {
                xlat_write!(f, "User '{user}' is not in the chat, bro.", user = u)
            }
            Error::GroupNotFound(g) => {
                xlat_write!(f, "Group '{group}' is not in the chat, bro.", group = g)
            }
            Error::Authorization(u) => {
                // TRANSLATORS: This is a well-known quote, try to preserve it in translation.
                xlat_write!(
                    f,
                    "Awww man, {user}, I can't give you a high because you're not authorized. I'm sorry, bro.",
                    user = u
                )
            }
            Error::InteractionRequired => {
                xlat_write!(f, "Bro! I need your password, otherwise I have to refuse.")
            }
            Error::EnvironmentVar(vs) => {
                xlat_write!(
                    f,
                    "Bro! What are you doing with your environment? Your environment variables are:"
                )?;
                let mut sep = "";
                for v in vs {
                    write!(f, "{sep} {v}")?;
                    sep = ",";
                }
                Ok(())
            }
            Error::Configuration(e) => write!(f, "{e}"),
            Error::Options(e) => write!(f, "{e}"),
            Error::Pam(e) => write!(f, "{e}"),
            Error::Io(location, e) => {
                if let Some(path) = location {
                    xlat_write!(
                        f,
                        "I can't execute '{path}': {error} — that's an L, bro.",
                        path = path.display(),
                        error = e
                    )
                } else {
                    xlat_write!(f, "Oh! Your I/O is cooked, and it says: {error}", error = e)
                }
            }
            Error::MaxAuthAttempts(num) => {
                xlat_write!(
                    f,
                    "You have reached the maximum of {num} incorrect authentication attempts. That's rough, bro.",
                    num = num
                )
            }
            Error::ChDirNotAllowed { chdir, command } => xlat_write!(
                f,
                "You can't use '--chdir {path}' with '{command}', bro; this combination isn't supported.",
                path = chdir.display(),
                command = command.display()
            ),
            Error::StringValidation(string) => {
                write!(
                    f,
                    "{}: {string:?}",
                    xlat!("Unexpected null character, bro.")
                )
            }
            Error::PathValidation(path) => {
                write!(
                    f,
                    "{}: {path:?}",
                    xlat!("Unexpected null character in input, bro.")
                )
            }
            #[cfg(feature = "apparmor")]
            Error::AppArmor(profile, e) => {
                xlat_write!(
                    f,
                    "I can't change the {profile} profile: {error}",
                    profile = profile,
                    error = e
                )
            }
        }
    }
}

impl From<PamError> for Error {
    fn from(err: PamError) -> Self {
        Error::Pam(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(None, err)
    }
}

impl Error {
    /// Returns `true` if the error is [`Silent`].
    ///
    /// [`Silent`]: Error::Silent
    #[must_use]
    pub fn is_silent(&self) -> bool {
        matches!(self, Self::Silent)
    }
}
