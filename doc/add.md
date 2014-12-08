# `add` command

Add simplifies the process of adding a new file to your current hermit
profile. Call it with a relative or path to the file you want to add.
Hermit will then move the original file into your hermit profile
directory and create a symbolic link in it's place that points to it
in the new location.

The end result is that everything should function as before, but the
real content of your file is now inside your hermit profile and can be
tracked in git.
