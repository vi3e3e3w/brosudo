pub fn usage_msg() -> &'static str {
    xlat!(
        "\
usage: sudo -h | -K | -k | -V
usage: sudo [-ABbknS] [-p prompt] [-D directory] [-g group] [-u user] [-i | -s] [command [arg ...]]
usage: sudo -v [-ABknS] [-p prompt] [-g group] [-u user]
usage: sudo -l [-ABknS] [-p prompt] [-U user] [-g group] [-u user] [command [arg ...]]
usage: sudo -e [-ABknS] [-p prompt] [-D directory] [-g group] [-u user] file ..."
    )
}

fn descriptor() -> &'static str {
    xlat!("Hi! I'm sudo. I'll run your command as another user, no cosplay required.")
}

fn help_msg() -> &'static str {
    xlat!(
        "Options:
  -A, --askpass                 use a helper for the password prompt, bro
  -b, --background              run the command in the background, low-key
  -B, --bell                    ring the bell when prompting
  -D, --chdir=directory         switch working directory before the command
  -e, --edit                    edit files instead of running a command
  -g, --group=group             run as the specified group name or ID
  -h, --help                    display this help and exit, no cap
  -i, --login                   run a login shell as the target user
  -K, --remove-timestamp        delete the timestamp file, respectfully
  -k, --reset-timestamp         invalidate the timestamp file
  -l, --list                    list privileges or check a command; use twice for the full lore
  -n, --non-interactive         non-interactive mode, no prompts
  -p, --prompt=prompt           use the specified password prompt
  -S, --stdin                   read the password from standard input
  -s, --shell                   run a shell as the target user
  -U, --other-user=user         in list mode, show another user's privileges
  -u, --user=user               run or edit as the specified user name or ID
  -V, --version                 display version information and exit
  -v, --validate                update the user's timestamp without a command
      --preserve-env=list       preserve selected environment variables
  --                            stop processing command-line arguments"
    )
}

pub fn long_help_message() -> String {
    format!("{}\n{}\n{}", descriptor(), usage_msg(), help_msg())
}
