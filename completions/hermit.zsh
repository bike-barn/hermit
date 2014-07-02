if [[ ! -o interactive ]]; then
    return
fi

compctl -K _hermit hermit

_hermit() {
    local words completions
    read -cA words

    if [ "${#words}" -eq 2 ]; then
        completions="$(hermit commands)"
    else
        completions="$(hermit completions ${words[2,-2]})"
    fi

    reply=("${(ps:\n:)completions}")
}
