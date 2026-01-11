# PasswordGenerator-rs
![Static Badge](https://img.shields.io/badge/cargo-1.92.0-orange)

<div style="text-align: center;">
    <img src="ui-example.png" alt="PasswordGenerator-rs UI" align="middle"/>
</div>

## Description
PasswordGenerator-rs is a simple rust application built with the **egui** GUI library and **eframe** framework.
This application aims to offer a local, offline password generation tool using standard ASCII character codes.
Passwords can 8 to 24 characters in length and have the option to not use special characters.

## Dependency List
The following dependencies are used in this project:

| Dependency |   Version   | Dependency Status                                                                                                 |
|:----------:|:-----------:|:------------------------------------------------------------------------------------------------------------------|
|   eframe   |   0.33.3    | [![dependency status](https://deps.rs/crate/eframe/0.33.3/status.svg)](https://deps.rs/crate/eframe/0.33.3)       |
|    egui    |   0.33.3    | [![dependency status](https://deps.rs/crate/egui/0.33.3/status.svg)](https://deps.rs/crate/egui/0.33.3)           |
|    rand    | 0.10.0-rc.0 | [![dependency status](https://deps.rs/crate/rand/0.10.0-rc.0/status.svg)](https://deps.rs/crate/rand/0.10.0-rc.0) |

## Installation
> [!warning]
> This code is currently **unsigned** and may cause issues if running on Windows.

The current stable release has two pre-compiled versions of the project:

1. A Linux binary (`passwordgenerator-rs`)
2. A Windows executable (`passwordgenerator-rs.exe`)

To check the integrity of the executables, a `passwordgenerator-rs.sha256` file is provided with SHA256 checksum values.

## License
The project is licensed under the **Apache License, Version 2.0**.