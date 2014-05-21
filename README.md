Attaché - Bring your home with you
==================================

Attaché is a home directory configuration management tool. Inspired
originally by [Briefcase] but also heavily borrows from awesome tools
like [rbenv], and [nvm]. It's (going to be) like having profiles for
your shell environments. Or, it just helps manage your home directory
being under version control.

[Briefcase]: https://github.com/jim/briefcase
[rbenv]: https://github.com/sstephenson/rbenv
[nvm]: https://github.com/creationix/nvm


Install
-------

### Install script

To install you could use the [install script] using cURL:

    curl https://raw.githubusercontent.com/RadicalZephyr/attache/master/install.sh | sh

or Wget:

    wget -qO- https://raw.githubusercontent.com/RadicalZephyr/attache/master/install.sh | sh

<sub>[The script][install script] clones the Attaché repository to
`~/.attache` and adds the source line to your profile
(`~/.bash_profile`, `~/.zshrc` or `~/.profile`).</sub>

You can customize the install source, directory and profile using the
`ATTACHE_SOURCE` and `ATTACHE_DIR` variables: Eg:
`curl ... | ATTACHE_DIR=/usr/local/attache sh` for a global install.


[install script]: https://raw.githubusercontent.com/RadicalZephyr/attache/master/install.sh

### Manual Install

Just clone the attache repository with `git`.

    git clone git@github.com:RadicalZephyr/attache.git ~/.attache

Now, you need to have `~/.attache/bin` on your path, so add something
like this to your shell init file:

    export PATH="$PATH":~/.attache/bin

Usage
-----

Still working on this... Try checking out `attache help`.


Feature Checklist/Roadmap
-------------------------

Checkout
[the issue](https://github.com/RadicalZephyr/attache/issues/12) for
the latest status.


Thoughts n' stuff
-----------------

We've got a whole [themey-wemey] thing going on. Any suggestions or
problems with the names should take this into consideration ;)

[themey-wemey]: http://www.zephyrizing.net/images/timey-wimey.gif


License
-------

Copyright 2014, Geoff Shannon

This file is part of Attaché.

Attaché is free software: you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Attaché is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with Attaché. If not, see <http://www.gnu.org/licenses/>.
