# Contributing to Bevy

Hey, so you're interested in contributing, feel free to pitch in on whatever interests you and I'll be happy to help you contribute.

I use the Bevy [Code of Conduct](https://github.com/bevyengine/bevy/blob/main/CODE_OF_CONDUCT.md)

If you want to say hi, feel free to say hi on the Bevy [Discord].

## Getting oriented

The repo is pretty small.
Github workflows live in .github. 
Cargo settings are in .cargo. 
If you use vscode, .vscode has some settings specific to the project (I would prefer if you used vscode)

Other than that, each individual crate is linked in crates/

### Contributing code

Rantz Suite is actively open to code contributions from community members.

Here is the workflow:

1. Fork the `BobG1983/rantz_suite` repository on GitHub. You'll need to create a GitHub account if you don't have one already.
2. Make your changes in a local clone of your fork, typically in its own new branch.
3. Push your changes to your fork on Github and open a Pull Request.
4. Respond to any CI failures or review feedback. While CI failures must be fixed before we can merge your PR, you do not need to *agree* with all feedback from your reviews, merely acknowledge that it was given. If you cannot come to an agreement, leave the thread open and defer to a Maintainer or Project Lead's final judgement.
5. When your PR is ready to merge, a Maintainer or Project Lead will review it and suggest final changes. If those changes are minimal they may even apply them directly to speed up merging.

If you end up adding a new crate to the `rantz_suite` repo:

1. Add the new crate to the [./release-plz.toml](./release-plz.toml) file - make sure to add its name to the rantz_suite entry too.
2. Make sure your crate is an optional dependency
3. Add it to the `rantz_suite` crate in the prelude, with a feature flag

When contributing, please:

* Ensure commit messages for new features start with "feat:"
* Ensure commit message for bug fixes start with "fix:"
* Ensure commit messages for docs start with "doc:"
* Run Cargo fmt before you PR
* Run Cargo clippy before you PR
* Explain what you're doing and why.
* Document new code with doc comments - or cargo check/clippy will shout at you.

Thank you :D
