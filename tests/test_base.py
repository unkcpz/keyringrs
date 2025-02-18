import pytest
from keyringrs import Entry


def test_entry():
    entry = Entry("my-service", "my-name")

    # Set a password
    pass_str = "0Xl$$K6o2bBwDe"
    entry.set_password(pass_str)

    # Retrieve the password
    assert entry.get_password() == pass_str

    # Delete the credential
    entry.delete_credential()

    with pytest.raises(Exception):
        _ = entry.get_password()
