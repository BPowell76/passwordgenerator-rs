# PasswordGenerator-rs
![Static Badge](https://img.shields.io/badge/cargo-1.95.0-orange)

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

## Usage
This application has a simple user interface.
To generate a password, click on the **Generate Password** button.
To adjust the length of the password, the slider can be adjusted from **8** to **24**.
If you do not want to use special characters (`@`, `&`, `$`, etc.), click on the checkbox for **Use Special Characters**.

These same instructions can also be found on the application's UI.

## Dependency List
The following dependencies are used in this project:

| Dependency | Version | Dependency Status                                                                                           |
|:----------:|:-------:|:------------------------------------------------------------------------------------------------------------|
|   eframe   | 0.34.1  | [![dependency status](https://deps.rs/crate/eframe/0.34.1/status.svg)](https://deps.rs/crate/eframe/0.34.1) |
|    egui    | 0.34.1  | [![dependency status](https://deps.rs/crate/egui/0.34.1/status.svg)](https://deps.rs/crate/egui/0.34.1)     |
|    rand    | 0.10.1  | [![dependency status](https://deps.rs/crate/rand/0.10.1/status.svg)](https://deps.rs/crate/rand/0.10.1)     |

## Installation
### Building From Source
To build this project from source, use the following steps:

1. Clone the repository using `git clone`.
2. Change into the project's root directory: `cd passwordgenerator-rs`.
3. Build the project as a release optimization using `cargo build -r`.

For the Windows build, this project is built using `cargo build -r --target x86_64-pc-windows-gnu`, but for building on your own device `cargo build -r` is sufficient.
The build output will be located in `target/release` with the name `passwordgenerator-rs`.

### Pre-Built Binaries
> [!warning]
> This code is currently **unsigned** and may cause issues if running on Windows.

The current stable release has two locally pre-compiled versions of the project:

1. A Linux binary (`passwordgenerator-rs`)
2. A Windows executable (`passwordgenerator-rs.exe`)

To check the integrity of the executables, a `passwordgenerator-rs.sha256` file is provided with SHA256 checksum values.

## License
The project is licensed under the **Apache License, Version 2.0**.