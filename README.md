# GTK QQ (WIP)

[![CI](https://github.com/lomirus/gtk-qq/actions/workflows/ci.yaml/badge.svg)](https://github.com/lomirus/gtk-qq/actions/workflows/ci.yaml)
[![dependency status](https://deps.rs/repo/github/lomirus/gtk-qq/status.svg)](https://deps.rs/repo/github/lomirus/gtk-qq)

Unofficial Linux [QQ](https://im.qq.com/) client, based on GTK4 and libadwaita, developed with Rust and [Relm4](https://relm4.org/).

## Screenshots

| Light                                      | Dark                                     |
| ------------------------------------------ | ---------------------------------------- |
| ![Light Mode Screenshot](./docs/light.png) | ![Dark Mode Screenshot](./docs/dark.png) |

Note: 
- These screenshots do not represent the final UI.
- The chatting function is not implemented yet.

## Run & Build

### Linux

If you want join the development or just to build this project, you need first to setup the environment by [meson](https://mesonbuild.com/Quick-guide.html) in the root directory of this project:

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

Gtk applications are harder to be compiled on Windows & MacOS and it will need more configurations to make it work. But considering some special reasons that you know, this project will not offer the release on Windows and MacOS. You can build it still if you like, but I hope you not to distribute the Windows/MacOS build of the project to the public in order to ensure the maintenance of this project.

## Contributing

- You can feel free to use English or Chinese to open an issue or pull request.
- The commit message should follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
- If you want make changes to the UI part, read the [GNOME Human Interface Guidelines](https://developer.gnome.org/hig/index.html) before it.

## License

This repository is under the [AGPL-3.0 license ](https://github.com/lomirus/gtk-qq/blob/main/LICENSE).
