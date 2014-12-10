Hermit
======

Bring your home with you
------------------------

Hermit is a home directory configuration management tool. Inspired
originally by [Briefcase] but also heavily borrows from awesome tools
like [rbenv], and [nvm]. It's (going to be) like having profiles for
your shell environments. Or, it just helps manage your home directory
being under version control.

_Hermit is currently alpha software. I use it and we have some
tests, but there may be bugs._

[![Build Status](https://travis-ci.org/RadicalZephyr/hermit.svg?branch=master)](https://travis-ci.org/RadicalZephyr/hermit)
[![Crabs harmed](http://img.shields.io/badge/crabs_harmed-0-blue.svg)](http://shields.io/)

[Briefcase]: https://github.com/jim/briefcase
[rbenv]: https://github.com/sstephenson/rbenv
[nvm]: https://github.com/creationix/nvm


Philosophy
----------

Hermit aims to be an assistant, it's not trying to run the show. As a
consequence, Hermit will never actually commit anything to your
profile repository. Instead - like the perfect office assistant -
Hermit will prepare everything for you, and then let you decide
whether or not to commit it. This gives you the option of reviewing
exactly the changes that Hermit is proposing to make to your profile
and helps you avoid accidentally committing any secrets.

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

Still working on this... Try checking out the
[docs folder][hermit-docs] or running `hermit help`:

[hermit-docs]: https://github.com/RadicalZephyr/hermit/tree/master/doc


```
Usage: hermit <command> [<args>]

Some useful hermit commands are:
   commands    List all available hermit commands
   init        Start a new hermit profile
   clone       Create a local hermit from an existing remote hermit
   status      Display the status of your hermit
   use         Switch to using a different profile
   add         Add files to your hermit directory
   update      Update redacted files already stored in git

See `hermit help <command>' for information on a specific command.
```

Running the Tests
-----------------

Hermit now has a test suite! You can run it with `./runtests.sh`.

Feature Checklist/Roadmap
-------------------------

Checkout
[the feature checklist](https://github.com/RadicalZephyr/hermit/issues/12) for
the latest status.


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
