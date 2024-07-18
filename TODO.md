# TODO list

## V2: 

- [x] Reimpl errors
- [x] Last typst version
- [x] More commands:
  - [x] Unlink
  - [x] List
- [x] Create `typst.toml` by asking questions
- [x] use semver
- [x] Use custom packages namespace (e.g "@custom/example:1.0.1")
- [x] Fix typo

## V2.1:

- [X] ""pre-export"" package by giving them what they need 
- [ ] Documentation for developpers
  - [ ] utils.rs
  - [ ] main.rs
  - [ ] commands.rs → remake it?
    - [ ] commands/create.rs
    - [ ] commands/link.rs
    - [ ] commands/list.rs
    - [ ] commands/unlink.rs
    - [ ] commands/install.rs
- [x] Download packages from unofficial repos, see #3
  - [x] git2-rs
  - [x] Dependencies managed
  - [x] use utpm namespace to use libs (or portable so without any links) → It wouldn't be as good as it sounds, typst can't use package outside the data folder
  - [x] Maybe a launchable version from utpm to link packages?
- [x] Portable version and only installable version
  - [x] Integrate install
  - [x] And all of todos from above with #3
- [ ] New commands for install:
  - [ ] Info.rs
  - [ ] Update, (using semver)
  - [x] Bulk delete
  - [ ] Clean?
- [ ] Maybe a listing dependencies system? -> Track every dependencies to delete when they aren't used -> Not for now
- [ ] Templates (first impl) -> Not now → V3
- [ ] JSON only mode 

## V3

This update will introduce documentations, a better handling error system, JSON and some commands. 

- [x] Better handling errors (json, string, toml maybe)
- [ ] Maybe a listing dependencies system? -> Track every dependencies to delete when they aren't used
- [ ] Create a global and local configuration instead of using typst.toml file. It can become harder to 
- [x] JSON only mode 
- [x] Remake clap
- [ ] Documentation for developpers and users
  - [ ] utils.rs
  - [ ] main.rs
  - [ ] commands.rs → remake it?
    - [ ] commands/create.rs
    - [ ] commands/link.rs
    - [ ] commands/list.rs
    - [ ] commands/unlink.rs
    - [ ] commands/install.rs
- [ ] New commands for install:
  - [ ] Info.rs -> Partial impl for now
  - [ ] Update, (using semver) → \w listing dependencies
  - [ ] Clean?


## V4 (2024.03.10)

> Last update: 05.06.2024

As of today (2024.03.10), a new version of typst has been released (`v0.11.0-rc1 (fe94bd85)`) with a new template system.
For now on, this version of utpm will focus on both adapting the new system and being compatible with the previous system.

If time isn't running out, I'll add quality of life improvements such as a `Context` system, new commands to go along with the `typst init` command and Dockerise everything (kubernetes included).

The main focus will be : 
- [ ] Add templates in `utpm` (transfer to [typst-project](https://github.com/tingerrr/typst-project))
  - [X] Struct
  - [ ] Implementation
- [ ] Compatibility with older version of typst
- [ ] Use `tracing-subscriber` as a logger (thanks @frozolotl)
- [ ] Improve `README`: add more example, an explanation, ... (thanks @Pachi)
- [ ] Add `utpm publish` command, like [this repository](https://github.com/tingerrr/alabaster) (thanks @tingerrr)

Optional, not needed in this PR but will be added in the future :
- [ ] Docker, Compose and Kubernetes files (and examples)
- [ ] ENV compatible.
- [ ] get along with `typst init`
- [x] tree and list for `list` commands
- [ ] Transform "portable" to "CI" binary

#### Appendix

- [Typst package](https://github.com/typst/packages/tree/0a5370faafd3b0662310255c4f827f9f2f1425cb)

If you have any ideas to improve utpm, I will gladly accept them into account <3