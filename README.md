Hermit
======

Bring your home with you
------------------------

Hermit is a home directory configuration management tool that
facilitates moving between different computers without losing your
shell (configurations).

Hermit was originally inspired by [Briefcase] which solves the same
problem.  However, Briefcase is written in Ruby.  We wanted something
more trivially portable. Since your shell configuration is so basic,
it seemed like having minimal dependencies would be a good
thing. Currently Hermit is dependent on the bash shell, but we hope to
make it depend solely on POSIX shell capabilities to maximize
portability.

The code infrastructure is heavily borrowed from the awesome shell
tools [rbenv], and [nvm].

_Hermit is currently alpha software. I use it and we have a reasonable
number of tests, but there may (will!) be bugs._

[![License GPL 3][badge-license]](http://www.gnu.org/licenses/gpl-3.0.txt)
[![Build Status][badge-build]](https://travis-ci.org/RadicalZephyr/hermit)
[![Crabs harmed][badge-crabs]](http://shields.io/)

[badge-license]: https://img.shields.io/badge/license-GPL_3-green.svg
[badge-build]: https://travis-ci.org/RadicalZephyr/hermit.svg?branch=master
[badge-crabs]: http://img.shields.io/badge/crabs_harmed-0-blue.svg

[Briefcase]: https://github.com/jim/briefcase
[rbenv]: https://github.com/sstephenson/rbenv
[nvm]: https://github.com/creationix/nvm

The Problem
-----------

Hermit aims to alleviate three separate but related problems related
to keeping

- Keeping your

<sub>This section was inspired by David Nolen's talk given to
[Hacker School][HS] about "Solution Oriented Language."</sub>

[HS]: https://www.hackerschool.com/

Philosophy
----------

Hermit is dependent on Git for much of it's functionality. In theory
any version control system could be used, but there has been no
provision for making it easy to make such a switch.

Hermit aims to be an assistant; it's not trying to run the show. As a
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

Still working on this... Check out the [tutorial], or peruse the
[docs folder][hermit-docs] or run `hermit help`.

[tutorial]: https://github.com/RadicalZephyr/hermit/tree/master/doc/tutorial.md
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
