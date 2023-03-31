# Software

Follow the steps below for a default installation of the ESP32-C3 platform tooling.

🔎 Should you desire a customized installation (e.g. building parts from source, or adding support for Xtensa targets), instructions for doing so can be found in the [Rust on ESP targets](https://esp-rs.github.io/book/installation/index.html) chapter of the *Rust on ESP* Book.

## Rust toolchain

✅ If you haven't got Rust on your computer, obtain it via <https://rustup.rs/>

Furthermore, for ESP32-C3, a [*nightly* version](https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust) of the Rust toolchain is currently required, for this training we will use `nightly-2023-02-28` version.

✅ Install *nightly* Rust and add support for the target architecture using the following command:

```console
rustup toolchain install nightly-2023-02-28 --component rust-src
```

🔎 Rust is capable of cross-compiling to any supported target (see `rustup target list`). By default, only the native architecture of your system is installed.
To build for the Xtensa architecture (*not* part of this material), a fork of the Rust compiler is required as of January 2022.

## Espressif toolchain

Several tools are required:
- [`cargo-espflash`](https://github.com/esp-rs/espflash/tree/main/cargo-espflash) - upload firmware to the microcontroller and open serial monitor with cargo integration
- [`espflash`](https://github.com/esp-rs/espflash/tree/main/espflash) - upload firmware to the microcontroller and open serial monitor
- [`ldproxy`](https://github.com/esp-rs/embuild/tree/master/ldproxy) - Espressif build toolchain dependency

✅ Install them with the following command:

```console
cargo install cargo-espflash espflash ldproxy
```

⚠️ The `espflash` and `cargo-espflash` commands listed in the book assume version is >= 2

## Toolchain dependencies

### Debian/Ubuntu

```console
sudo apt install llvm-dev libclang-dev clang
```
### macOS

When using the Homebrew package manager, which we recommend:
```console
brew install llvm
```

### Troubleshooting

- Python 3 is a required dependency. It comes preinstalled on stock macOS and typically on desktop Linux distributions. An existing **Python 2** installation with the `virtualenv` add-on pointing to it is known to potentially cause build problems.

- Error `failed to run custom build command for libudev-sys vX.X.X` or `esp-idf-sys vX.X.X`:

    At time of writing, this can be solved by:
    1. Running this line:

    `apt-get update \
    && apt-get install -y vim nano git curl gcc ninja-build cmake libudev-dev python3 python3-pip libusb-1.0-0 libssl-dev \
    pkg-config libtinfo5`

    2. Restarting the terminal.

    3. If this is not working, try `cargo clean`, remove the `~/.espressif` folder (`%USERPROFILE%\.espressif` in Windows) and rebuild your project.

    4. On Ubuntu, you might need to change your kernel to `5.19`. Run `uname -r` to obtain your kernel version.


## Docker

An alternative environment, is to use Docker. The repository contains a `Dockerfile`
with instructions to install the Rust toolchain, and all required packages. **This virtualized environment is designed
to compile the binaries for the Espressif target. Flashing binaries from containers is not possible**, hence there are two options:
- Execute flashing commands, e.g., `cargo-espflash`, on the host system.
- Use [`web-flash`](https://github.com/esp-rs/esp-web-flash-server) crate to flash the resulting binaries from the container. The container already includes `web-flash`. Here is how you would flash the build output of [`hardware-check` project](./02_4_hello_board.md):
   ```console
   web-flash --chip esp32c3 target/riscv32imc-esp-espidf/debug/hardware-check
   ```

✅ Install [`Docker`](https://docs.docker.com/get-docker/) for your operating system.

To build the Docker image, run the following command from the root folder:

```console
docker image build --tag esp --file .devcontainer/Dockerfile .
```

Building the image takes a while depending on the OS & hardware (20-30 minutes).

To start the new Docker container run:

```console
docker run --mount type=bind,source="$(pwd)",target=/workspace,consistency=cached -it esp /bin/bash
```

This starts an interactive shell in the Docker container. It also mounts the local repository to a folder
named `/workspace` inside the container. Changes to the project on the host system are reflected inside the container & vice versa.

Using this Docker setup requires certain commands to run inside the container, while others have to be executed on the host system.
It's recommended to keep two terminals open, one connected to the Docker container, one on the host system.

* in the container: compile the project
* on the host: use the `cargo-espflash` sub-command to flash the program onto the embedded hardware

## Additional Software

### VS Code

One editor with good Rust support is [VS Code](https://code.visualstudio.com/), which is available for most platforms.
When using VS Code, we recommend the following extensions to help during the development.

* [`Rust Analyzer`](https://rust-analyzer.github.io/) to provide code completion & navigation
* `Even Better TOML` for editing TOML based configuration files

There are a few more useful extensions for advanced usage

* [`lldb`](https://github.com/vadimcn/vscode-lldb) a native debugger extension based on LLDB
* [`crates`](https://github.com/serayuzgur/crates) to help manage Rust dependencies

### VS Code & Devcontainer

One extension for VS Code that might be helpful to develop inside a Docker container is [`Remote Containers`](https://github.com/Microsoft/vscode-remote-release).
It uses the same `Dockerfile` as the [Docker setup](#docker), but builds the image and connects to it from within VS Code.
Once the extension is installed, VS Code recognizes the configuration in the `.devcontainer` folder. Use the `Remote Containers - Reopen in Container` command to connect VS Code to the container.

## Gitpod

[Gitpod](https://www.gitpod.io) can provide fully initialized, perfectly set-up developer environments for this training with no installation required
on the host system, other than a [Gitpod-compatible browser](https://www.gitpod.io/docs/configure/user-settings/browser-settings).

✅ Open a Gitpod Workspace:
[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/esp-rs/std-training)

> It may take a few minutes to build the container and setup the environment.

> Note that flashing from the Gitpod workspace is only available using [`web-flash`](https://github.com/esp-rs/esp-web-flash-server). The Gitpod workspace already includes `web-flash`. Here is how you would flash the build output of [`hardware-check` project](./02_4_hello_board.md):
>   ```console
>   web-flash --chip esp32c3 target/riscv32imc-esp-espidf/debug/hardware-check
>   ```
