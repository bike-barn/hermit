#!/usr/bin/env bash

has() {
    type "$1" > /dev/null 2>&1
    return $?
}

if [ -z "$ATTACHE_DIR" ]; then
    ATTACHE_DIR="$HOME/.attache"
fi

install() {
    if [ -z "$ATTACHE_SOURCE" ]; then
        ATTACHE_SOURCE="https://github.com/RadicalZephyr/attache.git"
    fi

    if [ -d "$ATTACHE_DIR/.git" ]; then
        echo "=> Attaché is already installed in $ATTACHE_DIR, trying to update"
        echo -e "\r=> \c"
        cd "$ATTACHE_DIR" && git pull 2> /dev/null || {
            echo >&2 "Failed to update Attaché, run 'git pull' in $ATTACHE_DIR yourself..."
       }
    else
        # Cloning into $ATTACHE_DIR
        echo "=> Downloading Attaché from GitHub to '$ATTACHE_DIR'"
        echo -e "\r=> \c"
        mkdir -p "$ATTACHE_DIR"
        git clone "$ATTACHE_SOURCE" "$ATTACHE_DIR"
    fi
}

if has "git"; then
    install
else
    echo >&2 "You need git in order to install Attaché"
    exit 1
fi

# Detect profile file if not specified as environment variable (eg: PROFILE=~/.myprofile).
PROFILES=".bash_profile .zshrc .profile"
if [ -z "$PROFILE" ]; then
    for i in $PROFILES; do
        if [ -f "$HOME/$i" ]; then
            PROFILE="$HOME/$i"
            break
        fi
    done
fi

#UNBREAK: This still doesn't load Attaché. The SOURCE_STR is based on rbenv
#         and may require an equivalent of rbenv's eval "$(rbenv init -).
SOURCE_STR="export PATH=\"\$HOME/.attache/bin:\$PATH\" # This loads Attaché"

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
  if ! grep -qc '.attache/bin' $PROFILE; then
    echo "=> Appending source string to $PROFILE"
    echo "" >> "$PROFILE"
    echo $SOURCE_STR >> "$PROFILE"
  else
    echo "=> Source string already in $PROFILE"
  fi
fi

echo "=> Close and reopen your terminal to start using Attaché"
echo "=> Alternatively, simply `source $HOME/$PROFILE`"
