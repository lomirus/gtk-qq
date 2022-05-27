# GTK QQ (WIP)

[![check-badge]][check-link]
[![build-badge]][build-link]
[![dependency-badge]][dependency-link]

[check-badge]: https://github.com/lomirus/gtk-qq/workflows/check/badge.svg
[check-link]: https://github.com/lomirus/gtk-qq/actions/workflows/check.yaml
[build-badge]: https://github.com/lomirus/gtk-qq/workflows/build/badge.svg
[build-link]: https://github.com/lomirus/gtk-qq/actions/workflows/build.yaml
[dependency-badge]: https://deps.rs/repo/github/lomirus/gtk-qq/status.svg
[dependency-link]: https://deps.rs/repo/github/lomirus/gtk-qq

Unofficial Linux [QQ](https://im.qq.com/) client, based on GTK4 and libadwaita, developed with Rust and [Relm4](https://relm4.org/).

## Screenshots

| Light                                      | Dark                                     |
| ------------------------------------------ | ---------------------------------------- |
| ![Light Mode Screenshot](./docs/light.png) | ![Dark Mode Screenshot](./docs/dark.png) |

> **Note**
> - These screenshots do not represent the final UI.
> - The chatting function is not implemented yet.

## Run & Build

### Linux

If you want join the development or just to build this project from source, you need first to setup the environment by [meson](https://mesonbuild.com/Quick-guide.html) in the root directory of this project:

```bash
meson setup builddir
meson compile -C builddir
```

Then as usual, just run:

```bash
cargo run
```

or

```bash
cargo build --release
```

up to your purpose.

### Windows & MacOS

Gtk4 based projects would be more complex to compile on Windows/MacOS platform. Nevertheless, considering some special reasons that you know, we shall not offer the Windows/MacOS release or even build scripts. 

> **Warning**
> 
> You can try to build it still if you are just for personal use. At the same time, you should also promise that you will not distribute the Windows/MacOS build to the public in order to ensure the maintenance of this project.
>
> The user builds, uses or distributes this project at the user's own risk. This project and its contributors assume no responsibility.

## Contributing

- You can feel free to use English or Chinese to open an issue or pull request.
- The commit message should follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
- If you want make changes to the UI part, read the [GNOME Human Interface Guidelines](https://developer.gnome.org/hig/index.html) before it.

## License

This repository is under the [AGPL-3.0 license ](https://github.com/lomirus/gtk-qq/blob/main/LICENSE).
