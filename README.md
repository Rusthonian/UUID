# Rusthonian UUID - Complete Python Bindings for Rust's UUID Crate

**100% Coverage** of the Rust `uuid` crate exposed to Python through high-performance PyO3 bindings.

## Features

This module provides **complete** Python bindings for the Rust [`uuid` crate](https://docs.rs/uuid/latest/uuid/), including:

### ✅ UUID Generation (All Versions)
- **v1**: Time-based UUIDs (MAC address + timestamp)
- **v3**: Name-based UUIDs using MD5 hashing
- **v4**: Random UUIDs
- **v5**: Name-based UUIDs using SHA-1 hashing
- **v6**: Reordered time-based UUIDs
- **v7**: Unix timestamp-based UUIDs
- **v8**: Custom UUIDs

### ✅ Complete Construction Methods
- From hex strings
- From bytes (big-endian and little-endian)
- From integer (u128)
- From fields
- From slices
- From u64 pairs

### ✅ Advanced Features
- **Builder Pattern**: Construct custom UUIDs with fine-grained control
- **Timestamp Extraction**: Extract timestamps from time-based UUIDs (v1, v6, v7)
- **Context Management**: For deterministic v1/v6 generation
- **All Formatting Options**:
  - Hyphenated (8-4-4-4-12)
  - Simple (32 hex digits)
  - Braced ({...})
  - URN (urn:uuid:...)
  - Uppercase/lowercase encoding

### ✅ Full Python Integration
- All Python special methods (`__str__`, `__repr__`, `__int__`, `__bytes__`, `__hash__`, `__eq__`, etc.)
- Namespace constants (DNS, URL, OID, X500)
- Comparison and ordering
- Hashing support for use in sets/dicts

### ✅ Comprehensive Conversions
- To/from bytes (big-endian and little-endian)
- To/from u128 (big-endian and little-endian)
- To/from fields
- Field extraction (time_low, time_mid, time_hi_version, clock_seq, node)

## Installation

```bash
# Using maturin (recommended for development)
maturin develop --release

# Or build and install with pip
pip install .
```

## Quick Start

```python
import rusthonian_uuid as uuid

# Generate UUIDs
u4 = uuid.uuid4()  # Random UUID
u7 = uuid.uuid7()  # Timestamp-based UUID
print(f"Random: {u4}")
print(f"Timestamp: {u7}")

# Name-based UUIDs
u3 = uuid.uuid3(uuid.NAMESPACE_DNS, "example.com")
u5 = uuid.uuid5(uuid.NAMESPACE_URL, "https://example.com")

# Parse UUIDs
u = uuid.UUID(hex="550e8400-e29b-41d4-a716-446655440000")
print(f"Parsed: {u}")
```

## Complete API Reference

### UUID Generation Functions

```python
# Python stdlib-style API
uuid.uuid1(node=None, clock_seq=None)  # v1: MAC + timestamp
uuid.uuid3(namespace, name)             # v3: MD5 namespace
uuid.uuid4()                            # v4: Random
uuid.uuid5(namespace, name)             # v5: SHA1 namespace
uuid.uuid6(node=None, clock_seq=None)  # v6: Reordered timestamp
uuid.uuid7()                            # v7: Unix timestamp
uuid.uuid8(bytes)                       # v8: Custom

# Rust-style API
uuid.new_v1(seconds, nanos, node)
uuid.new_v3(namespace, name)
uuid.new_v4()
uuid.new_v5(namespace, name)
uuid.new_v6(seconds, nanos, node)
uuid.new_v8(bytes)
uuid.now_v7()

# Advanced: With explicit timestamps
uuid.new_v1_from_timestamp(timestamp, node)
uuid.new_v6_from_timestamp(timestamp, node)
uuid.new_v7_from_timestamp(timestamp)

# Special UUIDs
uuid.nil()   # 00000000-0000-0000-0000-000000000000
uuid.max()   # ffffffff-ffff-ffff-ffff-ffffffffffff
```

### UUID Class

```python
# Constructor
u = uuid.UUID(
    hex=None,       # Hex string
    bytes=None,     # 16 bytes
    bytes_le=None,  # 16 bytes little-endian
    fields=None,    # (time_low, time_mid, time_hi, clk_hi, clk_low, node)
    int=None,       # 128-bit integer
    version=None    # Optional version override
)

# Static constructors
uuid.UUID.from_bytes(bytes)
uuid.UUID.from_bytes_le(bytes)
uuid.UUID.from_u128(value)
uuid.UUID.from_u128_le(value)
uuid.UUID.from_u64_pair(high, low)
uuid.UUID.from_int(value)
uuid.UUID.from_slice(bytes)
uuid.UUID.from_slice_le(bytes)
uuid.UUID.from_fields(time_low, time_mid, time_hi, clk_hi, clk_low, node)
uuid.UUID.from_fields_le(...)

# Properties
u.version              # UUID version (1-8) or None
u.variant              # UUID variant ("RFC4122", "NCS", "Microsoft", "Future")
u.bytes                # 16 bytes
u.bytes_le             # 16 bytes little-endian
u.int                  # 128-bit integer
u.hex                  # Hex string without hyphens
u.urn                  # URN format
u.fields               # (time_low, time_mid, time_hi, clk_hi, clk_low, node)

# Methods
u.as_u128()            # Get as u128
u.as_u64_pair()        # Get as (high, low) u64 pair
u.as_hyphenated()      # "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
u.as_simple()          # "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
u.as_braced()          # "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}"
u.as_urn()             # "urn:uuid:xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"

# Encoding methods
u.encode_hyphenated()       # Lowercase hyphenated
u.encode_simple()           # Lowercase simple
u.encode_braced()           # Lowercase braced
u.encode_urn()              # Lowercase URN
u.encode_hyphenated_upper() # Uppercase hyphenated
u.encode_simple_upper()     # Uppercase simple
u.encode_braced_upper()     # Uppercase braced
u.encode_urn_upper()        # Uppercase URN

# Conversions
u.to_bytes_le()        # Convert to little-endian bytes
u.to_u128_le()         # Convert to little-endian u128
u.to_fields_le()       # Convert to little-endian fields

# Field extraction
u.time_low()           # 32-bit time_low field
u.time_mid()           # 16-bit time_mid field
u.time_hi_version()    # 16-bit time_hi_and_version field
u.clock_seq()          # 14-bit clock sequence
u.node()               # 48-bit node ID

# Timestamp methods (for v1, v6, v7)
u.has_timestamp()      # Check if UUID has timestamp
u.get_timestamp()      # Extract Timestamp object (or None)

# Checks
u.is_nil()             # Check if nil UUID
u.is_max()             # Check if max UUID

# Python special methods
str(u)                 # String representation
repr(u)                # Python repr
int(u)                 # Convert to int
bytes(u)               # Convert to bytes
hash(u)                # Hash value
u == other             # Equality
u < other              # Comparison
```

### Builder Class

```python
# Create a Builder
builder = uuid.Builder()
builder = uuid.Builder.from_bytes(bytes)
builder = uuid.Builder.from_u128(value)
builder = uuid.Builder.from_u128_le(value)
builder = uuid.Builder.from_fields(d1, d2, d3, d4)
builder = uuid.Builder.from_fields_le(d1, d2, d3, d4)
builder = uuid.Builder.from_random_bytes(bytes)
builder = uuid.Builder.from_unix_timestamp(seconds, nanos, random_bytes)

# Modify the builder
builder.with_version(4)                  # Set version
builder.with_variant("RFC4122")          # Set variant
builder.set_byte(index, value)           # Set specific byte
byte = builder.get_byte(index)           # Get specific byte

# Build the UUID
u = builder.build()
```

### Timestamp Class

```python
# Create a Timestamp
ts = uuid.Timestamp(seconds, nanos, counter=0)
ts = uuid.Timestamp.from_unix(seconds, nanos, counter)
ts = uuid.Timestamp.from_gregorian(ticks, counter)

# Properties
ts.seconds             # Unix seconds
ts.nanos               # Nanoseconds
ts.counter             # Counter bits

# Methods
ts.to_unix()           # Convert to (seconds, nanos)
ts.to_gregorian()      # Convert to (ticks, counter)
```

### Context Class

```python
# Create a Context for v1/v6 generation
ctx = uuid.Context(counter)
```

### Namespace Constants

```python
uuid.NAMESPACE_DNS     # 6ba7b810-9dad-11d1-80b4-00c04fd430c8
uuid.NAMESPACE_URL     # 6ba7b811-9dad-11d1-80b4-00c04fd430c8
uuid.NAMESPACE_OID     # 6ba7b812-9dad-11d1-80b4-00c04fd430c8
uuid.NAMESPACE_X500    # 6ba7b814-9dad-11d1-80b4-00c04fd430c8
```

### Parsing Functions

```python
# Parse UUID from string
u = uuid.parse_str("550e8400-e29b-41d4-a716-446655440000")

# Validate UUID string
is_valid = uuid.is_valid("550e8400-e29b-41d4-a716-446655440000")  # True
is_valid = uuid.is_valid("not-a-uuid")  # False
```

## Examples

### Generate All UUID Versions

```python
import rusthonian_uuid as uuid

# v1 - MAC address + timestamp
u1 = uuid.uuid1()
print(f"v1: {u1}")

# v3 - MD5 namespace
u3 = uuid.uuid3(uuid.NAMESPACE_DNS, "example.com")
print(f"v3: {u3}")

# v4 - Random
u4 = uuid.uuid4()
print(f"v4: {u4}")

# v5 - SHA1 namespace
u5 = uuid.uuid5(uuid.NAMESPACE_URL, "https://example.com")
print(f"v5: {u5}")

# v6 - Reordered timestamp
u6 = uuid.uuid6()
print(f"v6: {u6}")

# v7 - Unix timestamp
u7 = uuid.uuid7()
print(f"v7: {u7}")

# v8 - Custom
custom_bytes = b'\x00\x11\x22\x33\x44\x55\x66\x77\x88\x99\xaa\xbb\xcc\xdd\xee\xff'
u8 = uuid.uuid8(custom_bytes)
print(f"v8: {u8}")
```

### Extract Timestamp from v7 UUID

```python
import rusthonian_uuid as uuid

u7 = uuid.uuid7()
if u7.has_timestamp():
    ts = u7.get_timestamp()
    if ts:
        print(f"Seconds: {ts.seconds}")
        print(f"Nanos: {ts.nanos}")
```

### Build Custom UUID

```python
import rusthonian_uuid as uuid

builder = uuid.Builder()
builder.with_version(4)
builder.with_variant("RFC4122")
builder.set_byte(0, 0xFF)
u = builder.build()
print(f"Custom UUID: {u}")
```

### Format UUID in Different Ways

```python
import rusthonian_uuid as uuid

u = uuid.uuid4()
print(f"Hyphenated: {u.as_hyphenated()}")
print(f"Simple:     {u.as_simple()}")
print(f"Braced:     {u.as_braced()}")
print(f"URN:        {u.as_urn()}")
print(f"Uppercase:  {u.encode_simple_upper()}")
```

### Convert Between Formats

```python
import rusthonian_uuid as uuid

u = uuid.uuid4()

# To various formats
as_bytes = u.bytes
as_int = u.int
as_u128 = u.as_u128()
as_fields = u.fields

# Reconstruct from formats
u2 = uuid.UUID.from_bytes(as_bytes)
u3 = uuid.UUID.from_int(as_int)
u4 = uuid.UUID.from_u128(as_u128)

assert u == u2 == u3 == u4
```

## Performance

This library leverages Rust's high-performance `uuid` crate and PyO3 for zero-cost abstractions. It's significantly faster than pure Python UUID implementations.

## Testing

Run the comprehensive test suite:

```bash
python test_comprehensive.py
```

All tests pass, confirming 100% coverage of the uuid crate functionality.

## License

This project is dual-licensed under MIT OR Apache-2.0, matching the Rust `uuid` crate.

## Contributing

Contributions are welcome! Please ensure all tests pass before submitting a PR.

## Credits

- Built with [PyO3](https://pyo3.rs/)
- Wraps the Rust [uuid crate](https://docs.rs/uuid/latest/uuid/)
- Part of the [Rusthonian](https://github.com/Rusthonian/Rusthonian) project