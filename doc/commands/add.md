# `add` command

Add simplifies the process of adding a new file to your current hermit
profile. Pass a series of relative or absolute paths to the files you
want to add.  Hermit will then move the original files into your
hermit profile directory and create symbolic links in their place that
point to the new locations.

The end result is that everything should function as before, but the
real content of your file is now inside your hermit profile and can be
tracked in git.
