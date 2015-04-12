# Redacting Files

You should redact the secrets from your configuration files. It's
still up to you to identify those secrets, but hermit can assist you
in keeping them out of your git history and helping them stay secret.

When you run `hermit add` you can prefix any file or directory name
with either a `-r ` or a `--redact=` and hermit will allow you to
redact these files. This means that you get a chance to edit them, and
then hermit looks at the differences between the original and
"redacted" files.

If you've made changes, then hermit will offer to encrypt the secret
portion in a special "secrets" file. Then, the non-secret portion of
the file and the encrypted secret portion can both be safely committed
to your hermit profile and your configurations are that much more
portable.

It might also be helpful to read about [File Views][concepts].

[concepts]: https://github.com/RadicalZephyr/hermit/blob/master/doc/concepts.md#file-views
