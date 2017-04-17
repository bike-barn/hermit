[![Stories in Ready](https://badge.waffle.io/RadicalZephyr/hermit.png?label=ready&title=Ready)](https://waffle.io/RadicalZephyr/hermit)
Hermit
======

Hermit is changing!
------------------

We feel that we've reached the limits of what seems reasonable to
accomplish in shell. So we're switching to Rust!

Stay tuned, exciting things will be happening soon.

Bring your home with you
------------------------

<img src="http://www.zephyrizing.net/hermit/images/hermit-crab.png"
 alt="Hermit Logo" title="The Crab" align="right" />

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
[![Build Status][badge-build]](https://travis-ci.org/bike-barn/hermit)
[![Stories Ready to Work On][badge-todo]](https://waffle.io/bike-barn/hermit)
[![Crabs harmed][badge-crabs]](http://shields.io/)

[badge-license]: https://img.shields.io/badge/license-GPL_3-green.svg
[badge-build]: https://travis-ci.org/bike-barn/hermit.svg?branch=master
[badge-todo]: https://badge.waffle.io/bike-barn/hermit.svg?label=to-do&title=To-Do
[badge-crabs]: http://img.shields.io/badge/crabs_harmed-0-blue.svg

[Briefcase]: https://github.com/jim/briefcase
[rbenv]: https://github.com/sstephenson/rbenv
[nvm]: https://github.com/creationix/nvm

The Problem
-----------

Hermit aims to alleviate three separate but related problems related
to keeping your dotfiles under source control.

1. Not having the source control directory in your home root
2. Facilitating sym-link management because of #1
3. Having secret information in your dotfiles that is NOT committed to
   git

<sub>This section was inspired by David Nolen's talk given to
[The Recurse Center][RC] about "Solution Oriented Language."</sub>

[RC]: https://www.recursecenter.com/

Philosophy
----------

Hermit is dependent on Git for much of it's functionality. In fact, if
you don't know how to use Git, Hermit won't be very useful for
you. This is because Hermit as a tool fundamentally collaborates with
Git.

Hermit commands are mostly wrappers around the corresponding Git
commands. But Hermit always adds some extra behavior on top of what
Git normally does. As a convenience, Hermit also provides a way to run
arbitrary Git commands against your dotfiles repository without
`cd`ing to that directory.

Hermit aims to be an assistant; it's not trying to run the show. As a
consequence, Hermit will never actually commit anything to your
profile repository. Instead - like the perfect office assistant -
Hermit will prepare everything for you, and then let you decide
whether or not to commit it. This gives you the option of reviewing
exactly the changes that Hermit is proposing to make to your profile
and helps you avoid accidentally committing any secrets.

Install
-------

### Use `cargo` with `git`

Since we're using Rust we've decided to package our project for installation
with Rust's package manager, [cargo][install-cargo].

You're going to need `git` to make use of `hermit`, so just make sure
you have `git` [installed][install-git] and then install through `cargo`.

    cargo install --git https://github.com/bike-barn/hermit.git

[install-cargo]: http://doc.crates.io/
[install-git]: https://git-scm.com/book/en/v1/Getting-Started-Installing-Git

### Dependencies

Additional dependencies can be installed with your package manager.

#### Ubuntu & Debian
    sudo apt-get install cmake

#### Fedora
    sudo dnf install cmake

#### OS X
    brew install cmake


Uninstall
---------

We're sorry to see you go, but it's straightforward to ditch `hermit`.

    cargo uninstall hermit

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

Where We're Headed
------------------

Checkout our [Waffle.io][waffle] board for a more organized view of
what's going on in the project.

[waffle]: https://waffle.io/bike-barn/hermit

Contributing
------------

Please note that Hermit is released with
a [Contributor Code of Conduct][covenant]. By participating in this
project you agree to abide by its terms.

If you believe someone is violating the code of conduct, we ask that
you report it by emailing hermit@zephyrizing.net. For more details please
see our [Reporting Guide].

First, please take a look at our
[Contribution Guidelines][contributing].  Next, probably check out
our [Waffle.io][waffle] board, and look at the size 1 issues in the
To-Do column. From there, the standard "[fork], [branch], [code],
[pull request]" workflow works well.

[covenant]: https://github.com/bike-barn/hermit/blob/master/CODE_OF_CONDUCT.md
[contributing]: https://github.com/bike-barn/hermit/blob/master/CONTRIBUTING.md
[fork]: https://help.github.com/articles/fork-a-repo/
[branch]: https://help.github.com/articles/creating-and-deleting-branches-within-your-repository/
[code]: http://stackoverflow.com/questions/tagged/rust
[pull request]: https://help.github.com/articles/creating-a-pull-request/

As a bonus, if you use the waffle.io
[branch naming scheme][waffle-flow] for the issue you're working on,
it will automatically update our Waffle board.

[waffle-flow]: https://github.com/waffleio/waffle.io/wiki/FAQs#branch-moving

License
-------

Copyright 2014-2016, Geoff Shannon and contributors.

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
