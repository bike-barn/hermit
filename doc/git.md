# Hermit's relationship with Git

So here's the guideline I'm going to throw down.  If it only has to do
with git, then it doesn't get wrapped.  To use it you must use `hermit
git`

If there's some extra functionality that hermit adds, then we wrap it

Thus, `hermit clone` makes sense, because we put profiles in a
particular place so we know where they are

but `remote`, `push` and `fetch` etc. are all strictly git related.
Hermit doesn't need to do anything for them, so to use them you have
to say `hermit git <command>`
