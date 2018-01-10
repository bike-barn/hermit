# Contributing to Hermit
These are the guidelines for contributing to Hermit. Exceptions can always be
made, but these guidelines exist for a reason and ought to keep workflow, code,
and documentation in order. Please review them thoroughly before attempting to
contribute. If it's obvious that you haven't read any of these guidelines it's
unlikely your contributions will be accepted.

## Repository Management
Repository management is a tricky thing to get right. The preferred model for
Hermit is to have a [central repository][repo] that all contributors fork. No
changes should reach the `master` branch except by way of a [pull request].

### Submitting Issues
Issues can be for **anything**, not just bugs. Find some incorrect
documentation?  Submit an issue! Is there a function that's logically sound,
but butchers the code style conventions? Submit an issue! Do you want the
software to support a new feature? Submit an issue! Better yet, fix the problem
yourself and submit a [pull request].

#### Bug Reports
A bug is a _demonstrable problem_ caused by code in the repository.

When submitting a bug report:
1. **Search for the issue** &mdash; check if the issue has already been
   reported.

2. **Check if the issue has been fixed** &mdash; try to reproduce it using the
   latest `master` or development branch in the repository.

3. **Isolate the problem** &mdash; try to narrow down the cause of the bug.
   Include an example in your report.

Example:

> Short and descriptive example bug report title
>
> A summary of the issue and the browser/OS environment in which it occurs. If
> suitable, include the steps required to reproduce the bug.
>
> 1. This is the first step
> 2. This is the second step
> 3. Further steps, etc.
>
> Any other information you want to share that is relevant to the issue being
> reported. This might include the lines of code that you have identified as
> causing the bug, and potential solutions (and your opinions on their
> merits).

#### Feature Requests
Feature requests are welcome, but please consider whether your idea fits within
the project scope. Provide as much detail as possible. For example, a feature
request like
> Use YAML for configuration because it's better than JSON.

is unlikely to be given any thought. An alternative like
> Use YAML for configuration in place of JSON since YAML supports references
> and multiline strings which could be useful for deployment in multiple
> environments and better formatting of SQL statements.

is **much** more likely to be considered.

#### Pull Requests
Pull requests are _wonderful_ help. They should remain focused, in scope, and
avoid containing unrelated commits (see [topic branch] model). Make sure your
pull request contains a clear title and description.

It's a good idea to **ask** before undertaking any work on a significant pull
request, otherwise you risk doing a lot of work for something the rest of the
developers might not want to merge.

Also, we use a [Waffle board][waffle-board] to track "In Progress"
work. You can get Waffle to automatically move an issue/card into the
"In Progress" column for you by following
it's [branch naming conventions][waffle-branch-naming].

[waffle-board]: https://waffle.io/bike-barn/hermit
[waffle-branch-naming]: https://help.waffle.io/faq/waffle-workflow/why-are-my-issues-in-backlog-being-automatically-moved-to-in-progress

Be sure to follow the guidelines on [writing code](#writing-code) if you want
your work considered for inclusion.

### Handling Pull Requests
- Merging your own pull request defeats the purpose. You might as well
  `git push --force` to `master`. Except in rare circumstances no one should
  accept their own pull request.

- When possible there should be more than one reviewer with eyes on the code
  before a pull request is actually accepted.

- For all but the most trivial merges involving code a reviewer should actually
  pull down the modified code and verify that it works before accepting.

### Issue and Pull Request Conventions
It's common to voice approval or disapproval by using :+1: or :-1:. Show the
submitter you either approve or disapprove of their suggestions. It's also
common to see :heart: and :shipit: (for "ship it!") as emojis. Obviously these
emojis should go hand in hand with concrete feedback.

## Writing Code
So you want to add (or perhaps remove) some code? Great! Please adhere to these
guidelines to ensure coherence throughout the application.

### Coding Style
- If you haven't read the [Rust book] you should stop what you're doing and
read that right now.

- Rust's coding style is still evolving as the community grows, but for now
we've decided to use [`rustfmt`][rustfmt]. We recommend using the `cargo fmt`
plugin for `cargo`. If we pull down your proposed pull request and we change
the code by running `cargo fmt` it's likely you'll hear about it before we
accept that pull request.

- Familiarize yourself with general Rust patterns and anti-patterns. If you're
looking for examples of good, clean Rust code check out [Rust by Example]. The
[Rust subreddit] is also a great place to ask questions if you're unfamiliar
best practices or simply want to bounce questions off someone else that's a
great place to do so.

### Version Control
- [Fork][forking] the [central repository][repo] and work from a clone of your
own fork.

- Follow the [topic branch] model and submit pull requests from branches named
according to their purpose.

- Review the [GitHub Flow] documentation and, in general, try to stick to the
principles outlined there.

## Shout-outs
- Much thanks to [Nicolas Gallagher] for his generic [issue guidelines].

[repo]: https://github.com/bike-barn/Hermit
[pull request]: https://help.github.com/articles/using-pull-requests/
[forking]: https://help.github.com/articles/fork-a-repo/
[topic branch]: https://git-scm.com/book/en/v2/Git-Branching-Branching-Workflows#Topic-Branches
[rustfmt]: https://github.com/rust-lang-nursery/rustfmt
[Rust book]: https://doc.rust-lang.org/book/
[Rust by Example]: http://rustbyexample.com/
[Rust subreddit]: https://www.reddit.com/r/rust/
[GitHub Flow]: https://guides.github.com/introduction/flow/
[Nicolas Gallagher]: http://nicolasgallagher.com/
[issue guidelines]: https://github.com/necolas/issue-guidelines

## Code of Conduct

As a contributor, you can help us keep the Hermit community open and
inclusive. Please read and follow our [Code of Conduct].

If you believe someone is violating the code of conduct, we ask that
you report it by emailing hermit@zephyrizing.net. For more details
please see our [Reporting Guide].

[Code of Conduct]: /CODE_OF_CONDUCT.md
[Reporting Guide]: /doc/conduct/reporting-guide.md
