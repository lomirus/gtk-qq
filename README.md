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
> 
> The two screenshots have been a little outdated. The UI now has been adjusted and improved compared to them.

## Run & Build

### Requirements

Make you have installed [`meson`](https://mesonbuild.com/Quick-guide.html), `ninja-build`, `libgtk-4-dev` and `libadwaita-1-dev` before building.

#### Ubuntu

```bash
sudo apt-get install meson ninja-build libgtk-4-dev libadwaita-1-dev
```

#### Windows & MacOS

Gtk4 based projects would be more complex to compile on Windows/MacOS platform. Nevertheless, considering some special reasons that you know, we shall not offer the Windows/MacOS release or even build scripts. 

> **Warning**
> 
> You can try to build it still if you are just for personal use. At the same time, you should also promise that you will not distribute the Windows/MacOS build to the public in order to ensure the maintenance of this project.
> 
> The user builds, uses or distributes this project at the user's own risk. This project and its contributors assume no responsibility.

### Setup

You only need to run the commands below once unless you change the related codes.

```bash
# In the root directory of project
meson setup builddir
meson compile -C builddir
```

### Build

```bash
cargo build --release
```

## Contributing

- You can feel free to use English or Chinese to open an issue or pull request.
- The commit message should follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
- If you want make changes to the UI part, read the [GNOME Human Interface Guidelines](https://developer.gnome.org/hig/index.html) before it.

## License

This repository is under the [AGPL-3.0 license ](https://github.com/lomirus/gtk-qq/blob/main/LICENSE).
