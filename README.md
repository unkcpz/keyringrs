# keyringrs

A Python binding for [keyring-rs](https://github.com/hwchen/keyring-rs).

While [keyring](https://github.com/jaraco/keyring) is a great tool, it doesn't fully meet our general requirements due to the following reasons:

- **Native Linux Keyutils Support**: We need seamless integration with keyutils on Linux.  
- **Proper Memory Management on macOS**: We perform numerous key queries, and [keyring](https://github.com/jaraco/keyring) has known memory management issues on macOS (see [issue jaraco/keyring #670](https://github.com/jaraco/keyring/issues/670)).

This library aims to maintain API compatibility with `keyring-rs` as closely as possible.

It therefore provide built-in support following platform-specific credential stores inherited from keyring-rs:

* _Linux_: The DBus-based Secret Service, the kernel keyutils, and a combo of the two.
* _FreeBSD_, _OpenBSD_: The DBus-based Secret Service.
* _macOS_, _iOS_: The local keychain.
* _Windows_: The Windows Credential Manager.

## Installation

To install the package, simply run:

```bash
pip install keyringrs
```

## Usage

Here's a quick example of how to use `keyringrs`:

```python
from keyringrs import Entry

# Create an entry for the given service and username
entry = Entry("my-service", "my-name")

# Set a password
entry.set_password("0Xl$$K6o2bBwDe")

# Retrieve the password
password = entry.get_password()
print(f"My password is '{password}'")

# Delete the credential
entry.delete_credential()
```

This interface follows the same logic as `keyring-rs` to ensure consistency and ease of use.

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
