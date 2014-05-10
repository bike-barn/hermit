Attaché - Bring your home with you
==================================

A home directory configuration management tool. Inspired originally by
[Briefcase](https://github.com/jim/briefcase) but also heavily borrows
from awesome tools like [rbenv], and [nvm]. It's (going to be) like
having profiles for your shell environments. Or, it just helps manage
your home directory being under version control.

[rbenv]: https://github.com/sstephenson/rbenv
[nvm]: https://github.com/creationix/nvm

Feature Checklist/Roadmap
-------------------------

- [x] init   - Create a new attache
- [x] add    - Add a file/folder to the attache
- [ ] redact - Add a file with sensitive information, do complicated
  stuff to make it safe(r)
- [x] status - Currently, just does `git status` in the attache dir,
  should give info about other stuff like what files haven't been linked
- [ ] ~~git  - Run arbitrary git commands in the attache dir~~ (this
  seemed like a bad idea, see [6cfa1d5])
- [ ] open   - (?) Move user into the attache dir for down and dirty work
- [ ] link   - Generate symlinks for all files in your attache dir
  into home
- [ ] unlink - Remove all the symlinks for the current profile
- [ ] fetch  - Get a remote git repository and set it up as the
  contents of your attache dir
- [ ] use    - Switch to using a particular profile

[6cfa1d5]: https://github.com/RadicalZephyr/attache/commit/6cfa1d5b27e7fab24fa8e8a24a9d759ff6ec81cea

### Thoughts n' stuff:

We've got a whole [themey-wemey] thing going on. Any suggestions or
problems with the names should take this into consideration ;)

[themey-wemey]: http://www.zephyrizing.net/images/timey-wimey.gif
