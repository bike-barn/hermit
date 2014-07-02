Hermit - Bring your home with you
==================================

Hermit is a home directory configuration management tool. Inspired
originally by [Briefcase] but also heavily borrows from awesome tools
like [rbenv], and [nvm]. It's (going to be) like having profiles for
your shell environments. Or, it just helps manage your home directory
being under version control.

[![Build Status](https://travis-ci.org/RadicalZephyr/hermit.svg?branch=master)](https://travis-ci.org/RadicalZephyr/hermit)

[Briefcase]: https://github.com/jim/briefcase
[rbenv]: https://github.com/sstephenson/rbenv
[nvm]: https://github.com/creationix/nvm


Install
-------

### Install script

To install you could use the [install script] using cURL:

    curl https://raw.githubusercontent.com/RadicalZephyr/hermit/master/install.sh | sh

or Wget:

    wget -qO- https://raw.githubusercontent.com/RadicalZephyr/hermit/master/install.sh | sh

<sub>[The script][install script] clones the Hermit repository to
`~/.hermit` and adds the source line to your profile
(`~/.bash_profile`, `~/.zshrc` or `~/.profile`).</sub>

You can customize the install source, directory and profile
respectively using the `HERMIT_SOURCE`, `HERMIT_DIR` and `PROFILE`
variables: Eg: `curl ... | HERMIT_DIR=/usr/local/hermit sh` for a
global install.


[install script]: https://raw.githubusercontent.com/RadicalZephyr/hermit/master/install.sh

### Manual Install

Just clone the hermit repository with `git`.

    git clone git@github.com:RadicalZephyr/hermit.git ~/.hermit

Now, you need to have `~/.hermit/bin` on your path, so add something
like this to your shell init file:

    export PATH="$PATH":~/.hermit/bin

Usage
-----

Still working on this... Try checking out `hermit help`.

Running the Tests
-----------------

Hermit now has a test suite! You can run it with `./runtests.sh`.

Feature Checklist/Roadmap
-------------------------

Checkout
[the issue](https://github.com/RadicalZephyr/hermit/issues/12) for
the latest status.


Thoughts n' stuff
-----------------

We've got a whole [themey-wemey] thing going on. Any suggestions or
problems with the names should take this into consideration ;)

[themey-wemey]: http://www.zephyrizing.net/images/timey-wimey.gif


License
-------

Copyright 2014, Geoff Shannon

This file is part of Hermit.

Hermit is free software: you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Hermit is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with Hermit. If not, see <http://www.gnu.org/licenses/>.
