# PasswordGenerator-rs
![Static Badge](https://img.shields.io/badge/cargo-1.95.0-orange)
![Crates.io MSRV (version)](https://img.shields.io/crates/msrv/egui/0.34.1)
![GitHub Repo stars](https://img.shields.io/github/stars/BPowell76/passwordgenerator-rs)
![GitHub forks](https://img.shields.io/github/forks/BPowell76/passwordgenerator-rs)

<div style="text-align: center;">
    <img src="ui-example.png" alt="PasswordGenerator-rs UI" />
</div>

## Description
PasswordGenerator-rs is a simple rust application built with the **egui** GUI library and **eframe** framework.
This application aims to offer a local, offline password generation tool using standard ASCII character codes.
Passwords can be 8 to 24 characters in length and have the option to not use special characters.

> [!note]
> The 24-character limit for the generated password was just an arbitrary choice.
> The max value can be increased to 32 or 50 characters, if need be.
> The 8-character minimum is generally considered the bare minimum for making a password with some experts suggesting it should be at least 12 characters.

This project was created as a means of experimenting with creating a GUI application using Rust.
Rust was selected as a means of exploring creating a desktop application using a low-level language versus using C or C++.
Being new at low-level programming, rust was chosen due to its enforcement on safety.
As I get more comfortable with low-level programming, I could look at remaking this using C/C++.

The egui and eframe packages were chosen for developing the UI as they are written natively with rust instead of using some other language/syntax.

## Usage
This application has a simple user interface.
To generate a password, click on the **Generate Password** button.
To adjust the length of the password, the slider can be adjusted from **8** to **24**.
If you do not want to use special characters (`@`, `&`, `$`, etc.), click on the checkbox for **Use Special Characters**.

These same instructions can also be found on the application's UI.

## Dependency List
The following dependencies are used in this project:

| Dependency | Project Version | Dependency Status                                                                                            |                   Crates.io Latest Version                   |
|:----------:|:---------------:|:-------------------------------------------------------------------------------------------------------------|:------------------------------------------------------------:|
|   eframe   |     0.34.1      | [![dependency status](https://deps.rs/crate/eframe/0.34.1/status.svg)](https://deps.rs/crate/eframe/0.34.1)  | ![Crates.io Version](https://img.shields.io/crates/v/eframe) |
|    egui    |     0.34.1      | [![dependency status](https://deps.rs/crate/egui/0.34.1/status.svg)](https://deps.rs/crate/egui/0.34.1)      |  ![Crates.io Version](https://img.shields.io/crates/v/egui)  |
|    rand    |     0.10.1      | [![dependency status](https://deps.rs/crate/rand/0.10.1/status.svg)](https://deps.rs/crate/rand/0.10.1)      |  ![Crates.io Version](https://img.shields.io/crates/v/rand)  |

## Installation
### Building From Source
> [!important]
> To build from source you need at least cargo version **1.92.0** to support the egui and eframe crates.

To build this project from source, use the following steps:

1. Clone the repository using `git clone`.
2. Change into the project's root directory: `cd passwordgenerator-rs`.
3. Build the project as a release optimization using `cargo build -r`.

For the Windows build, this project is built using `cargo build -r --target x86_64-pc-windows-gnu`, but for building on your own device `cargo build -r` is sufficient as it will use your default toolchain.
The build output will be located in `target/release` with the name `passwordgenerator-rs.exe`.
If building on Windows, you can use the `x86_64-pc-windows-msvc` target as long as you've installed the Microsoft Visual C++ tools.

### Pre-Built Binaries
> [!warning]
> This code is currently **unsigned** and may cause issues if running on Windows.

The current stable release has two locally pre-compiled 64-bit versions of the project:

1. A Linux binary (`passwordgenerator-rs`)
2. A Windows executable (`passwordgenerator-rs.exe`)

> [!note]
> MacOS is not supported due to lack of cross-compile support on Linux.
> ARM support can be added if there is interest.

To check the integrity of the executables, a `passwordgenerator-rs.sha256` file is provided with SHA256 checksum values.
Checking the integrity of the binaries, can be performed using `sha256sum -c passwordgenerator-rs.sha256` on Linux and using `(Get-FileHash .\passwordgenerator-rs.exe).Hash` on Windows using PowerShell.

If on Linux, ensure that the checksum file and the binary are located in the same directory.
If the binary is good, the terminal output will say **OK** along with the package name.

On Windows, you will need to manually compare the checksum file about the returned hash value.

## License
The project is licensed under the **Apache License, Version 2.0**.