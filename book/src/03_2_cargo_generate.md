# Generating New Projects

We're now going to use [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) (a generic project wizard) to set up our first application.

More information on generating projects can be found in the [Writing Your Own Application](https://esp-rs.github.io/book/writing-your-own-application/index.html) chapter of The Rust on ESP Book.

> Most other exercises in this workshop already provide a project skeleton and don't require using `cargo-generate`.
>
✅ Install `cargo-generate`:

```console
cargo install cargo-generate
```

✅ Change to the `intro` directory and run `cargo generate` with the [`esp-idf` template](https://github.com/esp-rs/esp-idf-template):

```console
cd intro
cargo generate esp-rs/esp-idf-template cargo
```

[You'll be prompted for details regarding your new project](https://github.com/esp-rs/esp-idf-template#generate-the-project). When given a choice between several options, navigate using cursor up/down and select with the Return key.

The first message you see will be:
`⚠️Unable to load config file: /home/$USER/.cargo/cargo-generate.toml`. You see this error because you don't have a favorite config file, but you don't need one and you can ignore this warning.

🔎 You can create a [favorite config file](https://cargo-generate.github.io/cargo-generate/favorites.html) that will be placed in `$CARGO_HOME/cargo-generate`, and override it with `-c, --config <config-file>`.

> If you make a mistake, hit `Ctrl+C` and start anew.

✅ Configure your project:

(These items may appear in a different order)

* Project Name: `hello-world`
* MCU: `esp32c3`
* Configure advanced template options?: `false`

🔎 `.cargo/config.toml` contains local settings ([list of all settings](https://doc.rust-lang.org/cargo/reference/config.html)) for your package.
`Cargo.toml` contains dependencies and `Cargo.lock` will [import all your dependencies](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html).

Optional, but recommended: To save disk space and download time, set the [toolchain directory to global](https://github.com/esp-rs/esp-idf-sys/blob/master/BUILD-OPTIONS.md#esp_idf_tools_install_dir-esp_idf_tools_install_dir). Otherwise, each new project/workspace will have its own instance of the toolchain installed on your computer:


✅ Open `hello-world/.cargo/config.toml` and add the following line to the bottom of the `[env]` section. Leave everything else unchanged.

```toml
[env]
# ...
ESP_IDF_TOOLS_INSTALL_DIR = { value = "global" } # add this line
```

✅ Open `hello-world/rust-toolchain.toml` and change the file to look like this:

```toml
[toolchain]
channel = "nightly-2025-01-01" # change this line
```

✅ Run your project by using the following command out of the `hello-world` directory.

```console
cd hello-world
cargo run
```

✅ The last lines of your output should look like this:

```console
(...)
I (268) cpu_start: Starting scheduler.
Hello, world!
```

## Extra Tasks
- If your main function exits, you have to reset the microcontroller to start it again. What happens when you put an infinite loop at the end instead? Test your theory by flashing a looping program.
- Can you think of a way to prevent what you're now seeing? (click for hint:[^hint])

## Troubleshooting
- If `cargo run` is stuck on `Connecting...`, you might have another monitor process still running (e.g. from the initial `hardware-check` test). Try finding and terminating it. If this doesn't help, disconnect and reconnect the board's USB cable.
- `⛔ Git Error: authentication required`: your git configuration is probably set to override `https` GitHub URLs to `ssh`. Check your global `~/.git/config` for `insteadOf` sections and disable them.

[^hint]: yield control back to the underlying operating system by `sleep`ing in a loop instead of busy waiting. (use `use std::thread::sleep`)
