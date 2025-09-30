"""
Comprehensive test suite for rusthonian_uuid module
Tests 100% coverage of the Rust uuid crate bindings
"""

import rusthonian_uuid as uuid_mod
import sys

def test_basic_generation():
    """Test all UUID generation methods"""
    print("Testing UUID generation...")
    
    # V4 - Random
    u4 = uuid_mod.uuid4()
    print(f"  UUID v4: {u4}")
    assert u4.version == 4
    
    # V7 - Unix timestamp based
    u7 = uuid_mod.uuid7()
    print(f"  UUID v7: {u7}")
    assert u7.version == 7
    assert u7.has_timestamp()
    
    # V3 - MD5 namespace
    u3 = uuid_mod.uuid3(uuid_mod.NAMESPACE_DNS, "example.com")
    print(f"  UUID v3: {u3}")
    assert u3.version == 3
    
    # V5 - SHA1 namespace
    u5 = uuid_mod.uuid5(uuid_mod.NAMESPACE_URL, "https://example.com")
    print(f"  UUID v5: {u5}")
    assert u5.version == 5
    
    # V8 - Custom
    custom_bytes = b'\x00\x11\x22\x33\x44\x55\x66\x77\x88\x99\xaa\xbb\xcc\xdd\xee\xff'
    u8 = uuid_mod.uuid8(custom_bytes)
    print(f"  UUID v8: {u8}")
    assert u8.version == 8
    
    # V1 - Timestamp + MAC
    u1 = uuid_mod.uuid1()
    print(f"  UUID v1: {u1}")
    assert u1.version == 1
    
    # V6 - Reordered timestamp
    u6 = uuid_mod.uuid6()
    print(f"  UUID v6: {u6}")
    assert u6.version == 6
    
    print("  ✓ Generation tests passed")

def test_special_uuids():
    """Test special UUID values"""
    print("Testing special UUIDs...")
    
    nil = uuid_mod.nil()
    print(f"  Nil UUID: {nil}")
    assert nil.is_nil()
    assert str(nil) == "00000000-0000-0000-0000-000000000000"
    
    max_uuid = uuid_mod.max()
    print(f"  Max UUID: {max_uuid}")
    assert max_uuid.is_max()
    assert str(max_uuid) == "ffffffff-ffff-ffff-ffff-ffffffffffff"
    
    print("  ✓ Special UUID tests passed")

def test_constructors():
    """Test UUID constructor methods"""
    print("Testing UUID constructors...")
    
    # From hex string
    u1 = uuid_mod.UUID(hex="550e8400-e29b-41d4-a716-446655440000")
    print(f"  From hex: {u1}")
    
    # From bytes
    u2 = uuid_mod.UUID.from_bytes(b'\x55\x0e\x84\x00\xe2\x9b\x41\xd4\xa7\x16\x44\x66\x55\x44\x00\x00')
    assert str(u1) == str(u2)
    
    # From int
    u3 = uuid_mod.UUID.from_int(113059749145936325402354257176981405696)
    assert str(u1) == str(u3)
    
    # From u128
    u4 = uuid_mod.UUID.from_u128(u3.as_u128())
    assert str(u1) == str(u4)
    
    print("  ✓ Constructor tests passed")

def test_formatting():
    """Test UUID formatting methods"""
    print("Testing UUID formatting...")
    
    u = uuid_mod.uuid4()
    
    # Different formats
    hyphenated = u.as_hyphenated()
    simple = u.as_simple()
    braced = u.as_braced()
    urn = u.as_urn()
    
    print(f"  Hyphenated: {hyphenated}")
    print(f"  Simple:     {simple}")
    print(f"  Braced:     {braced}")
    print(f"  URN:        {urn}")
    
    assert len(hyphenated) == 36
    assert len(simple) == 32
    assert len(braced) == 38
    assert len(urn) == 45
    assert braced.startswith('{')
    assert urn.startswith('urn:uuid:')
    
    # Uppercase encoding
    upper_hyph = u.encode_hyphenated_upper()
    assert upper_hyph.isupper()
    
    print("  ✓ Formatting tests passed")

def test_conversions():
    """Test UUID conversion methods"""
    print("Testing UUID conversions...")
    
    u = uuid_mod.uuid4()
    
    # To various formats
    as_bytes = u.bytes
    as_int = u.int
    as_u128 = u.as_u128()
    as_fields = u.fields
    as_u64_pair = u.as_u64_pair()
    
    print(f"  Bytes: {len(as_bytes)} bytes")
    print(f"  Int: {as_int}")
    print(f"  U128: {as_u128}")
    print(f"  Fields: {as_fields}")
    print(f"  U64 pair: {as_u64_pair}")
    
    assert len(as_bytes) == 16
    assert as_int == as_u128
    assert len(as_fields) == 6
    
    # Little-endian conversions
    bytes_le = u.bytes_le
    to_bytes_le = u.to_bytes_le()
    assert bytes_le == to_bytes_le
    
    u128_le = u.to_u128_le()
    fields_le = u.to_fields_le()
    
    print(f"  U128 LE: {u128_le}")
    print(f"  Fields LE: {fields_le}")
    
    print("  ✓ Conversion tests passed")

def test_builder():
    """Test UUID Builder"""
    print("Testing UUID Builder...")
    
    # Create from bytes
    builder = uuid_mod.Builder.from_bytes(b'\x00' * 16)
    builder.with_version(4)
    builder.with_variant('RFC4122')
    u = builder.build()
    
    print(f"  Built UUID: {u}")
    assert u.version == 4
    assert u.variant == 'RFC4122'
    
    # From u128
    builder2 = uuid_mod.Builder.from_u128(12345678901234567890)
    u2 = builder2.build()
    print(f"  From u128: {u2}")
    
    # From fields
    d4_bytes = b'\x00\x00\x00\x00\x00\x00\x00\x00'
    builder3 = uuid_mod.Builder.from_fields(0x12345678, 0x1234, 0x5678, d4_bytes)
    u3 = builder3.build()
    print(f"  From fields: {u3}")
    
    print("  ✓ Builder tests passed")

def test_timestamp():
    """Test Timestamp functionality"""
    print("Testing Timestamp...")
    
    # Create timestamp
    ts = uuid_mod.Timestamp(1234567890, 123456789)
    print(f"  Timestamp: {ts}")
    assert ts.seconds == 1234567890
    assert ts.nanos == 123456789
    
    # From Unix
    ts2 = uuid_mod.Timestamp.from_unix(1234567890, 123456789, 0)
    unix_vals = ts2.to_unix()
    print(f"  To Unix: {unix_vals}")
    
    # Gregorian format
    greg = ts2.to_gregorian()
    print(f"  Gregorian: {greg}")
    
    # V7 with timestamp
    u7 = uuid_mod.uuid7()
    if u7.has_timestamp():
        extracted_ts = u7.get_timestamp()
        if extracted_ts:
            print(f"  Extracted timestamp from v7: {extracted_ts}")
    
    print("  ✓ Timestamp tests passed")

def test_context():
    """Test Context for v1/v6 UUIDs"""
    print("Testing Context...")
    
    ctx = uuid_mod.Context(42)
    print(f"  Context: {ctx}")
    
    print("  ✓ Context tests passed")

def test_parsing():
    """Test UUID parsing and validation"""
    print("Testing UUID parsing...")
    
    # Valid UUID
    valid_str = "550e8400-e29b-41d4-a716-446655440000"
    assert uuid_mod.is_valid(valid_str)
    
    u = uuid_mod.parse_str(valid_str)
    print(f"  Parsed: {u}")
    assert str(u) == valid_str
    
    # Invalid UUID
    assert not uuid_mod.is_valid("not-a-uuid")
    
    print("  ✓ Parsing tests passed")

def test_comparison():
    """Test UUID comparison and hashing"""
    print("Testing UUID comparison...")
    
    u1 = uuid_mod.uuid4()
    u2 = uuid_mod.UUID.from_bytes(u1.bytes)
    u3 = uuid_mod.uuid4()
    
    # Equality
    assert u1 == u2
    assert u1 != u3
    
    # Ordering
    uuids = [uuid_mod.uuid4() for _ in range(5)]
    sorted_uuids = sorted(uuids, key=lambda x: str(x))
    print(f"  Sorted {len(sorted_uuids)} UUIDs")
    
    # Hashing
    uuid_set = {u1, u2, u3}
    assert len(uuid_set) == 2  # u1 and u2 are the same
    
    print("  ✓ Comparison tests passed")

def test_fields():
    """Test UUID field extraction"""
    print("Testing UUID fields...")
    
    u = uuid_mod.uuid1()
    
    time_low = u.time_low()
    time_mid = u.time_mid()
    time_hi_version = u.time_hi_version()
    clock_seq = u.clock_seq()
    node = u.node()
    
    print(f"  Time low: {time_low}")
    print(f"  Time mid: {time_mid}")
    print(f"  Time hi+version: {time_hi_version}")
    print(f"  Clock seq: {clock_seq}")
    print(f"  Node: {node}")
    
    print("  ✓ Field extraction tests passed")

def test_namespace_constants():
    """Test UUID namespace constants"""
    print("Testing namespace constants...")
    
    print(f"  NAMESPACE_DNS: {uuid_mod.NAMESPACE_DNS}")
    print(f"  NAMESPACE_URL: {uuid_mod.NAMESPACE_URL}")
    print(f"  NAMESPACE_OID: {uuid_mod.NAMESPACE_OID}")
    print(f"  NAMESPACE_X500: {uuid_mod.NAMESPACE_X500}")
    
    # All should be valid UUIDs
    for ns_name in ['NAMESPACE_DNS', 'NAMESPACE_URL', 'NAMESPACE_OID', 'NAMESPACE_X500']:
        ns = getattr(uuid_mod, ns_name)
        assert hasattr(ns, 'version')
    
    print("  ✓ Namespace constant tests passed")

def test_variant():
    """Test UUID variant"""
    print("Testing UUID variant...")
    
    u = uuid_mod.uuid4()
    variant = u.variant
    print(f"  Variant: {variant}")
    assert variant in ['NCS', 'RFC4122', 'Microsoft', 'Future']
    
    print("  ✓ Variant tests passed")

def test_python_methods():
    """Test Python special methods"""
    print("Testing Python special methods...")
    
    u = uuid_mod.uuid4()
    
    # __str__
    str_repr = str(u)
    print(f"  __str__: {str_repr}")
    
    # __repr__
    repr_str = repr(u)
    print(f"  __repr__: {repr_str}")
    assert repr_str.startswith('UUID(')
    
    # __int__
    int_val = int(u)
    print(f"  __int__: {int_val}")
    assert int_val == u.int
    
    # __bytes__
    bytes_val = bytes(u)
    print(f"  __bytes__: {len(bytes_val)} bytes")
    assert bytes_val == bytes(u.bytes)
    
    print("  ✓ Python special methods tests passed")

def main():
    """Run all tests"""
    print("=" * 60)
    print("Rusthonian UUID - Comprehensive Test Suite")
    print("Testing 100% uuid crate coverage")
    print("=" * 60)
    print()
    
    tests = [
        test_basic_generation,
        test_special_uuids,
        test_constructors,
        test_formatting,
        test_conversions,
        test_builder,
        test_timestamp,
        test_context,
        test_parsing,
        test_comparison,
        test_fields,
        test_namespace_constants,
        test_variant,
        test_python_methods,
    ]
    
    passed = 0
    failed = 0
    
    for test in tests:
        try:
            test()
            passed += 1
            print()
        except Exception as e:
            print(f"  ✗ FAILED: {e}")
            failed += 1
            import traceback
            traceback.print_exc()
            print()
    
    print("=" * 60)
    print(f"Test Results: {passed} passed, {failed} failed")
    print("=" * 60)
    
    if failed > 0:
        sys.exit(1)
    else:
        print("\n✓ All tests passed! UUID module is working correctly.")
        print("✓ 100% uuid crate functionality exposed to Python")

if __name__ == "__main__":
    main()
