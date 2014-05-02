#!/bin/bash

# Currently, this really doesn't do a whole lot. Eventually it will
# help you manage your dotfiles with a git repository.

# The main feature of attache is that it facilitates the good-idea of
# not keeping your dotfiles git repo directly in your home dir.
# Instead, it stores it in the DEFAULT_ATTACHE_DIR, and then symlinks
# all of the files and directories from there into your home
# directory.

PROGNAME=$(basename $0)

DEFAULT_ATTACHE_DIR=~/.home

sub_help(){
    echo "Usage: $PROGNAME <subcommand> [options]\n"
    echo "Subcommands:"
    echo "    status  Display the status of your attache"
    echo "    import  Import a file into your attache"
    echo ""
    echo "For help with each subcommand run:"
    echo "$PROGNAME <subcommand> -h|--help"
    echo ""
}

sub_status(){
    TEMP=$(getopt -o 'h' -l 'help' -n "$PROGNAME $subcommand" -- "$@")

    if [ $? != 0 ] ; then echo "Terminating..." >&2 ; exit 1 ; fi

    eval set -- "$TEMP"

    while true; do
        case "$1" in
            -h | --help ) echo "There is no help!"; shift; exit ;;
            -- ) shift; break ;;
        esac
    done

    pushd $DEFAULT_ATTACHE_DIR
    git status
    popd
}

sub_import() {
    echo "Do stuff"
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
    "" | "-h" | "--help")
        sub_help
        ;;
    *)
        shift
        sub_${subcommand} $@
        if [ $? = 127 ]; then
            echo "Error: '$subcommand' is not a known subcommand." >&2
            echo "       Run '$PROGNAME --help' for a list of known subcommands." >&2
            exit 1
        fi
        ;;
esac
