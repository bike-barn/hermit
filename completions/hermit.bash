_hermit() {
    COMPREPLY=()
    local word="${COMP_WORDS[COMP_CWORD]}"
    if [ "$COMP_CWORD" -eq 1 ]; then
        COMPREPLY=( $(compgen -W "$(hermit commands)" -- "$word") )
    else
        local words=("${COMP_WORDS[@]}")
        unset words[0] # Get rid of hermit in comp_words
        #unset words[$COMP_CWORD] # Get rid of last argument
        local completions=$(hermit completions "${words[@]}")
        COMPREPLY=( $(compgen -W "$completions" -- "$word") )
    fi
}

complete -o nospace -o default -F _hermit hermit
