# Key Concepts

Hermit has a number of key concepts that will probably be helpful to
understand.

## File Views

In understanding how Hermit's redaction facilities work it's important
to be aware of the concept of file views. Whenever the user is editing
a file, we want them to be receiving a consistent view from that file.

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
