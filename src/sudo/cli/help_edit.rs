pub fn usage_msg() -> &'static str {
    xlat!(
        "\
usage: sudoedit -h | -V
usage: sudoedit [-BknS] [-p prompt] [-g group] [-u user] file ..."
    )
}

fn descriptor() -> &'static str {
    xlat!("Hi! I'm sudoedit. I'll edit the file as another user, no cosplay required.")
}

fn help_msg() -> &'static str {
    xlat!(
        "Options:
  -B, --bell                    ring the bell when prompting
  -g, --group=group             run as the specified group name or ID
  -h, --help                    display this help and exit, no cap
  -k, --reset-timestamp         invalidate the timestamp file
  -n, --non-interactive         non-interactive mode, no prompts
  -p, --prompt=prompt           use the specified password prompt
  -S, --stdin                   read the password from standard input
  -u, --user=user               edit as the specified user name or ID
  -V, --version                 display version information and exit
  --                            stop processing command-line arguments"
    )
}

pub fn long_help_message() -> String {
    format!("{}\n{}\n{}", descriptor(), usage_msg(), help_msg())
}
