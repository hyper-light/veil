//! Veil: just-in-time secrets for people and their agents. For now this is the `veil` binary saying what it is; Veil itself
//! comes with CP-T8.1 (docs/design/README.md, STATUS.md).

/// What `veil` prints for its arguments, where, and the exit status.
#[derive(Debug, PartialEq, Eq)]
pub struct Reply {
    pub text: String,
    pub to_stderr: bool,
    pub status: u8,
}

pub const USAGE: &str = "usage: veil [--version | --help]\n";

const HELP: &str = "veil: Veil, just-in-time secrets for people and their agents.\n";

/// The name, the version and the commit it was built from: `VEIL_COMMIT` at build time, which
/// kiln CI sets, else "unknown commit".
#[must_use]
pub fn version_line() -> String {
    let commit = option_env!("VEIL_COMMIT").unwrap_or("unknown commit");
    format!("veil {} ({commit})\n", env!("CARGO_PKG_VERSION"))
}

/// The reply to the binary's arguments; `None` stands for an argument that isn't Unicode.
#[must_use]
pub fn reply(args: &[Option<String>]) -> Reply {
    let refuse = |why: String| Reply {
        text: format!("veil: {why}\n{USAGE}"),
        to_stderr: true,
        status: 2,
    };
    match args {
        [Some(a)] if a == "--version" || a == "-V" => Reply {
            text: version_line(),
            to_stderr: false,
            status: 0,
        },
        [Some(a)] if a == "--help" || a == "-h" => Reply {
            text: format!("{HELP}{USAGE}"),
            to_stderr: false,
            status: 0,
        },
        [] => Reply {
            text: USAGE.to_owned(),
            to_stderr: true,
            status: 2,
        },
        _ if args.iter().any(Option::is_none) => refuse("an argument isn't Unicode".to_owned()),
        _ => refuse(format!(
            "unknown arguments: {}",
            args.iter().flatten().cloned().collect::<Vec<_>>().join(" ")
        )),
    }
}
