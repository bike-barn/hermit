# `update` command

The idea behind updating is that you already have a file in your
hermit profile, but you've made changes to it and want to update
it. Thus, you can call `hermit update <filename>` and hermit will
check the file for changes and do the necessary things to move it into
your git repository.

This process is totally transparent for files that contain no secret
information, since the file itself is actually a symlink to a
git-controlled file.  Normal git commands will pick up the changes.

Things are a bit different when a file has been redacted. In this
case, the "complete" file which is what you would automatically edit
by doing something like `nano ~/.bashrc` is not git-controlled.

Instead of requiring you to directly edit the git-safe view of the
file, hermit does some `diff` magic to determine what's new, and
present you with a new version of the file for redacting. From there
the process is the same as that described for [redacting][redact]

[redact]: https://github.com/RadicalZephyr/hermit/blob/master/doc/commands/redact.md
