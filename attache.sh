#!/usr/bin/env bash

# Currently, this really doesn't do a whole lot. Eventually it will
# help you manage your dotfiles with a git repository.

# The main feature of attache is that it facilitates the good-idea of
# not keeping your dotfiles git repo directly in your home dir.
# Instead, it stores it in the DEFAULT_ATTACHE_DIR, and then symlinks
# all of the files and directories from there into your home
# directory.

PROGNAME=$(basename $0)
VERSION="0.1.0"

DEFAULT_ATTACHE_DIR=${DEFAULT_ATTACHE_DIR:-~/.attache}
SECRETS_SUFFIX=.secrets

attache_add_file() {

    ATTACHE_FILE=${1#$DEFAULT_ATTACHE_DIR}
    [ -f $ATTACHE_FILE ] || echo "File $1 does not exist in $DEFAULT_ATTACHE_DIR" ; exit 1

    git add -vf $ATTACHE_FILE
}

sub_help() {
    echo -e "\nAttaché $VERSION\n"
    echo    "Usage:"
    echo    "    help    Show help for Attaché or a specific subcommand"
    echo    "    init    Create a new attaché"
    echo    "    fetch   Create a local attaché from an existing remote attaché"
    echo    "    status  Display the status of your attaché"
    echo    "    open    Go to your attaché directory"
    echo    "    add     Add a file to your attaché"
    echo    "    link    Symlink the contents of your attaché to \$HOME"
    echo    "    redact  Remove sensitive information from a file"
    echo    ""
    echo    "See \`$PROGNAME help <command>' for information on a specific subcommand."
    echo    "For full documentation, see: https://github.com/RadicalZephyr/attache"
    echo    ""
}

sub_init() {

    [ -d $DEFAULT_ATTACHE_DIR ] && echo "The folder \
$DEFAULT_ATTACHE_DIR already exists!
Maybe you should change your DEFAULT_ATTACHE_DIR environment variable..." && exit 1

    mkdir -p $DEFAULT_ATTACHE_DIR
    pushd $DEFAULT_ATTACHE_DIR >/dev/null 2>/dev/null
    git init
    echo "*$SECRETS_SUFFIX" > .gitignore
    git add .gitignore
    git commit -m "Initial commit"
    popd >/dev/null 2>/dev/null

    echo "Congratulations! You now have a brand new attache located at
$DEFAULT_ATTACHE_DIR"
}

sub_fetch() {
    echo $UNIMPLEMENTED
}

sub_status() {
    TEMP=$(getopt -o 'h' -l 'help' -n "$PROGNAME $subcommand" -- "$@")

    if [ $? != 0 ] ; then echo "Terminating..." >&2 ; exit 1 ; fi

    eval set -- "$TEMP"

    while true; do
        case "$1" in
            -h | --help ) echo "Usage: $PROGNAME $subcommand\n";
                          echo "Display the git status of your attache";
                          shift; exit ;;
            -- ) shift; break ;;
        esac
    done

    pushd $DEFAULT_ATTACHE_DIR >/dev/null 2>/dev/null
    git status
    popd >/dev/null 2>/dev/null
}

UNIMPLEMENTED="Unimplemented! You should probably contribute it if you really want it done!"

sub_add() {
    OLDFILE=$1
    NEWFILE=$DEFAULT_ATTACHE_DIR/${OLDFILE#~/}

    [ -f $NEWFILE ] && echo "$OLDFILE is already in your attache!"; exit 1

    mkdir -vp $(basename $NEWFILE)
    mv -vn $OLDFILE $NEWFILE
    ln -vs $NEWFILE $OLDFILE
    attache_add_file $NEWFILE
}

sub_link() {
    echo $UNIMPLEMENTED
}

sub_redact() {
    echo $UNIMPLEMENTED
}

# Keep this snippet for use with subcommands

# TEMP=$(getopt -o '' -n $(basename $0) -- "$@")

# if [ $? != 0 ] ; then echo "Terminating..." >&2 ; exit 1 ; fi

# eval set -- "$TEMP"

# while true; do
#     case "$1" in
#         -- ) shift; break ;;
#     esac
# done

subcommand=$1
case $subcommand in
    "" | "-h" | "--help" )
        sub_help
        ;;
    "open" | "go" )
        cd $DEFAULT_ATTACHE_DIR ;;
    * )
        shift
        sub_${subcommand} $@
        if [ $? = 127 ]; then
            echo "Error: '$subcommand' is not a known subcommand." >&2
            echo "       Run '$PROGNAME --help' for a list of known subcommands." >&2
            exit 1
        fi
        ;;
esac
