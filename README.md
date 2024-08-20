<div align="center">

![UTPM logo](./assets/logo.svg)

> _Unofficial typst package manager_

**UTPM** is a _package manager_ for **[local](https://github.com/typst/packages#local-packages)** and **remote** packages. Create quickly new _projects_ and _templates_ from a **singular tool**, and then **publish** it _directly_ to **Typst**!

[![Thumuss - utpm](https://img.shields.io/static/v1?label=Thumuss&message=utpm&color=blue&logo=github)](https://github.com/Thumuss/utpm "Go to GitHub repo")
[![stars - utpm](https://img.shields.io/github/stars/Thumuss/utpm?style=social)](https://github.com/Thumuss/utpm)
[![forks - utpm](https://img.shields.io/github/forks/Thumuss/utpm?style=social)](https://github.com/Thumuss/utpm)
<br/>
[![GitHub tag](https://img.shields.io/github/tag/Thumuss/utpm?include_prereleases=&sort=semver&color=blue)](https://github.com/Thumuss/utpm/releases/)
[![License](https://img.shields.io/badge/License-MIT-blue)](#license)
[![issues - utpm](https://img.shields.io/github/issues/Thumuss/utpm)](https://github.com/Thumuss/utpm/issues)


</div>

## 🔥 Features

- [x] ✨ Create packages automatically (`utpm ws create`)
  - [x] ⏯️ Interactive
  - [x] ⌨️ CLI version
- [x] 🛠 Put your package directly into your local packages (`utpm ws link`)
  - [x] 💻 Link without copying! (`utpm ws link --no-copy`)
- [x] 🌐 Dependencies outsite typst!
  - [x] 📦 Install directly from the tool
  - [x] 🔒 Portable installer (limited for now)
- [x] 📃 List all your packages
  - [x] 🗃️ As a list `utpm pkg list`
  - [x] 🌲 As a tree `utpm pkg tree`
- [x] 🔎 Cloning package to edit them in the way you want! `utpm ws clone`
- [ ] 💥 Customize your output (json or classic, `-j` in your commands)
- [x] 🗄️ Delete and bulk delete your packages (`utpm pkg unlink`, `utpm pkg bulk-delete`)
- [ ] 🚀 Publish it directly to Typst!

**_And many other features!_**

## 🔎 How to use it?

### The basic workflow

- _Firstly, you'll need to [create](#create) your `typst.toml` file!_
- _Then, edit your file! Like `index.typ` or `lib.typ`_
- _Finally, [link](#link) your new package to typst!_

### Commands

#### 🗄️ Bulk Delete

<!-- TODO: GIF -->

_A command to delete multiple packages at once!_

![bulk-delete.gif](./assets/gifs/bulk_delete.gif)

<div id="create">

#### ✨ Create
_Create a `typst.toml` to make a package_

Vous pouvez le faire de deux façon, soit de manière interactive soit seulement à travers une commande, montrée ci dessous:

![create_cli.gif](./assets/gifs/create_cli.gif)
<!-- TODO: GIF v2 -->



</div>
<div id="help">


#### ❓ Help

_Generate a help message_

Utilisable partout à l'aide de `--help`. Cela permet d'avoir des informations détaillés sur l'utilisation des commandes et des options.

![help.gif](./assets/gifs/help.gif)

</div>
<div id="install">

#### 📦 Install
Installe les dépendances ajoutés par la commande `utpm ws add`.  

![install.gif](./assets/gifs/install.gif)
<!-- TODO: GIF & text-->

</div>
<div id="link">

#### 🛠 Link
![link.gif](./assets/gifs/link.gif)
<!-- TODO: GIF & text-->

</div>
<div id="list">

#### 🗃️ List
<!-- TODO: text -->

![list.gif](./assets/gifs/list.gif)

</div>
<div id="package-path">
<!-- TODO: text -->

#### 🚦 Package Path

![packages-path.gif](./assets/gifs/packages-path.gif)

</div>
<div id="tree">

#### 🌲 Tree

_A simple command to show all packages installed in your local dir like a tree!_

![tree.gif](./assets/gifs/tree.gif)

</div>
<div id="unlink">
<!-- TODO: GIF -->

#### 🗄️ Unlink

![unlink.gif](./assets/gifs/unlink.gif)

</div>

## ⚡ Install

You will need Cargo and Rust .

The easiest way to install utpm using Cargo is:

```bash
$ cargo install --git https://github.com/Thumuss/utpm
```

## ❤️ Contribution

If you want to help me develop this package, simply make an issue or a PR!

By using this app, you contribute to it, thank you! <3
