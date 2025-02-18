import pytest
from keyringrs import Entry, CredentialType


def test_entry():
    entry = Entry("my-service", "my-name")

    # Set a password
    pass_str = "0Xl$$K6o2bBwDe"

    # The default linux credential is KeyUtil persistence, which require properly set up dbus
    with pytest.raises(Exception):
        entry.set_password(pass_str)


def test_entry_keyutil():
    entry = Entry("my-service", "my-name", credential_type=CredentialType.KeyUtil)

    # Set a password
    pass_str = "0Xl$$K6o2bBwDe"
    entry.set_password(pass_str)

    # Retrieve the password
    assert entry.get_password() == pass_str

    # Delete the credential
    entry.delete_credential()

    with pytest.raises(Exception):
        entry.get_password()
