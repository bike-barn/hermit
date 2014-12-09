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

## File Views

There is a key concept to understand with the redaction
process. Whenever the user is editing a file, we want them to be
receiving a consistent view from that file.

Under this paradigm, the "redacted" file is the "git view."  Anything
in that file is fair game to be committed to git.

The "complete" file though is the "secret view."  It contains
everything, including all your secrets.

The "secrets" file is just the secrets.

Notice that given any two of these files we can construct the third.

Create "complete"
```
patch -u "redacted" "secrets"
```

Create "secrets"
```
diff -u "redacted" "complete"
```

Create "redacted"
```
patch -uR "complete" "secrets"
```
