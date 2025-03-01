# keyringrs

A Python binding for [keyring-rs](https://github.com/hwchen/keyring-rs).

While [keyring](https://github.com/jaraco/keyring) is a great tool, it doesn't fully meet our general requirements due to the following reasons:

- **Native Linux Keyutils Support**: We need seamless integration with keyutils on Linux.  
- **Proper Memory Management on macOS**: We perform numerous key queries, and [keyring](https://github.com/jaraco/keyring) has known memory management issues on macOS (see [issue jaraco/keyring #670](https://github.com/jaraco/keyring/issues/670)).
- **Headless Linux without dbus-x11**: In CI, we test the secret-service on a headless linux (GitHub CI), the workaround starts the Gnome keyring unlocked with a known password from [kering-rs CI](https://github.com/hwchen/keyring-rs/blob/master/.github/workflows/ci.yaml) works without `dbus-x11` required.  

The `keyringrs` is **not** plan as the alternative of [keyring](https://github.com/jaraco/keyring).
For CLI tool and more generic use cases I recommend to go to `keyring`.

Aside from our specific requirements with macOS and Linux keyutils, this library can serve as a backup solution if there are unresolved issues. 
While it doesn't guarantee a fix for the problems encountered, it can be a helpful tool to distinguish whether the issue stems from the library itself or from system configuration.

This library aims to maintain API compatibility with `keyring-rs` as closely as possible.
It therefore provide built-in support following platform-specific credential stores inherited from keyring-rs:

* _Linux_: The DBus-based Secret Service, the kernel keyutils, and a combo of the two (see below for example on how to control to use only `keyutils`).
* _FreeBSD_, _OpenBSD_: The DBus-based Secret Service.
* _macOS_, _iOS_: The local keychain.
* _Windows_: The Windows Credential Manager.

## Installation

To install the package, simply run:

```bash
pip install keyringrs
```

## Usage

Here's a quick example of how to use `keyringrs`.
For all platforms `service-name` and `key-name` are required fields to create a key entry to distinguish from other keys.
The `target` field is optional and depends on target platforms, please check docs from `keyring-rs` for particular cases:

- [keyutils](https://docs.rs/keyring/latest/keyring/keyutils/index.html) 
- [keyutils_persistent](https://docs.rs/keyring/latest/keyring/keyutils_persistent/index.html)
- [Linux Secret Service](https://docs.rs/keyring/latest/keyring/secret_service/index.html)
- [iOS](https://docs.rs/keyring/latest/aarch64-apple-ios/keyring/ios/index.html)
- [macOS](https://docs.rs/keyring/latest/aarch64-apple-darwin/keyring/macos/index.html)
- [Windows](https://docs.rs/keyring/latest/x86_64-pc-windows-msvc/keyring/windows/index.html)

```python
from keyringrs import Entry

# Create an entry for the given service and username
# Or `entry = Entry("my-service-name", "my-key-name", target="my-target-name")`
entry = Entry("my-service-name", "my-key-name")

# Set a password
entry.set_password("0Xl$$K6o2bBwDe")

# Retrieve the password
password = entry.get_password()
print(f"My password is '{password}'")

# Delete the credential
entry.delete_credential()
```

In linux, by default it assume it used in a desktop distro with `libdbus` installed.
For the fallback support to use only `keyutils`, the key stored will disappear after reboot.

```python
from keyringrs import Entry, CredentialType

# Create an entry use only keyutils in linux
entry = Entry("my-service-name", "my-key-name", credential_type=CredentialType.KeyUtils)
```

This interface follows the same logic as `keyring-rs` to ensure consistency and ease of use.

### Checking keys on different platforms



## tree layout

The repo organized using [mixed rust/python layout](https://www.maturin.rs/project_layout.html#mixed-rustpython-project).

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── keyringrs
│   └── __init__.py
├── pyproject.toml
├── src
│   └── lib.rs
├── tests
│   └── test_base.py
└── uv.lock
```

### Ack

There was an effort five years ago with [pyrust-keyring](https://github.com/dk26/pyrust-keyring). 
However, over the years, `keyring-rs` has gained more features, become more robust, and slightly diverged from the original `keyring` library. 
It would be beneficial to have a proper wrapper for it. 

Additionally, `pyo3` has significantly improved, making it much easier to create wrappers and distribute the library on PyPI.

There is also a python wrapper [python-linux-keyutils](https://github.com/thorgate/python-linux-keyutils) for `linux-keyutils` crate, which may provide more keyutils related APIs and error messages.

## License

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the 
Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

