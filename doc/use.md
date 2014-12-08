# `use` command

Use is the main entry point to dealing with your hermit
profiles. Under the covers, `use` calls `link` and `unlink` to handle
the changing of symbolic links in your home directory.

In git terminology, `use` is a porcelain command, `link` and `unlink`
are the plumbing that make it work.
