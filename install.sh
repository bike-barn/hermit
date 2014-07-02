#!/usr/bin/env bash

# Copied shamelessly from nvm, copyright Tim Caswell
# See LICENSE.nvm for details

has() {
    type "$1" > /dev/null 2>&1
    return $?
}

if [ -z "$HERMIT_DIR" ]; then
    HERMIT_DIR="$HOME/.hermit"
fi

install() {
    if [ -z "$HERMIT_SOURCE" ]; then
        HERMIT_SOURCE="https://github.com/RadicalZephyr/hermit.git"
    fi

    if [ -d "$HERMIT_DIR/.git" ]; then
        echo "=> Hermit is already installed in $HERMIT_DIR, trying to update"
        echo -e "\r=> \c"
        cd "$HERMIT_DIR" && git pull 2> /dev/null || {
            echo >&2 "Failed to update Hermit, run 'git pull' in $HERMIT_DIR yourself..."
       }
    else
        # Cloning into $HERMIT_DIR
        echo "=> Downloading Hermit from GitHub to '$HERMIT_DIR'"
        echo -e "\r=> \c"
        mkdir -p "$HERMIT_DIR"
        git clone "$HERMIT_SOURCE" "$HERMIT_DIR"
    fi
}

if has "git"; then
    install
else
    echo >&2 "You need git in order to install Hermit"
    exit 1
fi

# Detect profile file if not specified as environment variable (eg: PROFILE=~/.myprofile).
PROFILES=".bash_profile .bashrc .zshrc .profile"
if [ -z "$PROFILE" ]; then
    for i in $PROFILES; do
        if [ -f "$HOME/$i" ]; then
            PROFILE="$HOME/$i"
            break
        fi
    done
fi

#UNBREAK: This still doesn't load Hermit. The SOURCE_STR is based on rbenv
#         and may require an equivalent of rbenv's eval "$(rbenv init -).
SOURCE_STR="export PATH=\"\$HOME/.hermit/bin:\$PATH\" # This loads Hermit"

if [ -z "$PROFILE" ] || [ ! -f "$PROFILE" ] ; then
  if [ -z $PROFILE ]; then
    echo "=> Profile not found. Tried ~/.bash_profile ~/.zshrc and ~/.profile."
    echo "=> Create one of them and run this script again"
  else
    echo "=> Profile $PROFILE not found"
    echo "=> Create it (touch $PROFILE) and run this script again"
  fi
  echo "   OR"
  echo "=> Append the following line to the correct file yourself:"
  echo
  echo "   $SOURCE_STR"
  echo
else
  if ! grep -qc '.hermit/bin' $PROFILE; then
    echo "=> Appending source string to $PROFILE"
    echo "" >> "$PROFILE"
    echo $SOURCE_STR >> "$PROFILE"
  else
    echo "=> Source string already in $PROFILE"
  fi
fi

echo "=> Close and reopen your terminal to start using Hermit"
echo "=> Alternatively, simply 'source $PROFILE'"
