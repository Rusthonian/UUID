# Rusthonian UUID

This is the UUID submodule repository for Rusthonian. It provides 100% complete Python bindings for the Rust UUID crate.

## Repository Structure

This repository contains the UUID-specific bindings that are imported and wrapped by the main Rusthonian package.

## Usage

This module is typically used through the main Rusthonian package:

```python
import Rusthonian
from Rusthonian import UUID

# Generate a random v4 UUID
uuid4 = UUID.new_v4()
print(uuid4)  # e.g., "550e8400-e29b-41d4-a716-446655440000"

# Create UUID from string
my_uuid = UUID.UUID("550e8400-e29b-41d4-a716-446655440000")
```

## Development

This repository is designed to be a submodule of the main Rusthonian package. The main Rusthonian package imports and wraps this functionality.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
