# GTK QQ (WIP)

[![CI](https://github.com/lomirus/gtk-qq/actions/workflows/ci.yaml/badge.svg)](https://github.com/lomirus/gtk-qq/actions/workflows/ci.yaml)
[![dependency status](https://deps.rs/repo/github/lomirus/gtk-qq/status.svg)](https://deps.rs/repo/github/lomirus/gtk-qq)

Unofficial [QQ](https://im.qq.com/) client, based on GTK4 and libadwaita, developed with Rust and [Relm4](https://relm4.org/).

## Develop & Build

### Linux

To develop or build this project, first you need to setup the environment by  [meson](https://mesonbuild.com/Quick-guide.html):

```bash
meson setup build
meson install -C build
```

Then as usual, just run:

```bash
cargo run
```

or

```bash
cargo build --release
```

depending on your purpose.

### Windows & MacOS

Gtk applications are harder to be compiled on Windows & MacOS and it will need more configurations to make it work. But onsidering some special reasons that you know, this project will not offer the release on Windows and MacOS. You can build it still if you like, but I would suggest you not to distribute the Windows/MacOS build of this project to the public in order to ensure the maintenance of this project.
