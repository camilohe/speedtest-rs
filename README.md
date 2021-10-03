# speedtest-rs

This is a lightweight backend written in Rust for [Librespeed](https://github.com/librespeed/speedtest).

## Compatibility
Supported by all Librespeed frontends, though some features are missing (see below).

## Features

- [x] Download
- [x] Upload
- [x] Ping
- [x] Jitter
- [x] IP Address, ISP
- [x] Multiple Points of Test (optional)
- [x] Compatible with PHP frontend predefined endpoints (with `.php` suffixes)
- [x] Distance from server (optional)
- [ ] Telemetry (optional)
- [ ] Results sharing (optional)
- [ ] [Proxy Protocol](https://www.haproxy.org/download/2.3/doc/proxy-protocol.txt)?

## Server requirements
* A Rust supported platform
* A fast! Internet connection

## Installation

You need Rust 1.55+ to compile the binary. 

1. Clone this repository:

```bash
git clone github.com/drobson03/speedtest-rs
# Change current working directory to the repository
cd speedtest-rs
```

2. Build before running
```bash
# Compile to target/release/speedtest-rs
cargo build --release
```

3. Copy the `assets` directory and the compiled `speedtest-rs` binary into a single directory along with a copy of `.env.example` named `.env` with your preferred port, listen address and [IPinfo.io](https://ipinfo.io/) API token.


5. Put `assets` folder under the same directory as your compiled binary.
    - Make sure font files and JavaScript files are in the `assets` directory
    - You can have multiple HTML pages under `assets` directory. They can be access directly under the server root
    (e.g. `/example-singleServer-full.html`)
    - It's possible to have a default page mapped to `/`, simply put a file named `index.html` under `assets`

6. Change `.env` according to your environment:

```sh
# your ipinfo.io API token
IPINFO_TOKEN=
# your server's latitude
SERVER_LATITUDE=1
# your server's longitude
SERVER_LONGITUDE=1
# the port to bind to
ROCKET_PORT=8000
# the bind address (0.0.0.0 is all interfaces)
ROCKET_ADDRESS=0.0.0.0
```

## Differences between Go and PHP implementation

- FAST (no garbage collector unlike Go)

## License
Copyright (C) 2016-2021 Federico Dossena

Copyright (C) 2021 Darcy Robson

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU Lesser General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/lgpl>.