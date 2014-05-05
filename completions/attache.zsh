if [[ ! -o interactive ]]; then
    return
fi

compctl -K _attache attache

_attache() {
    local words completions
    read -cA words

    if [ "${#words}" -eq 2 ]; then
        completions="$(attache commands)"
    else
        completions="$(attache completions ${words[2,-2]})"
    fi

    reply=("${(ps:\n:)completions}")
}
