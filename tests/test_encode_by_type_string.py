from typing import Any, Dict, Tuple

import pytest

import bt_decode


TEST_TYPE_STRING_SCALE_INFO_DECODING: Dict[str, Tuple[str, Any]] = {
    "scale_info::2": ("01", 1),  # u8
    "scale_info::441": (
        "c40352ca71e26e83b6c86058fd4d3c9643ea5dc11f120a7c80f47ec5770b457d8853018ca894cb3d02aaf9b96741c831a3970cf250a58ec46e6a66f269be0b4b040400ba94330000000000c7020000e0aaf22c000000000000000000000000ad240404000000000000000000000000000000000000000000000000000000000000000000048853018ca894cb3d02aaf9b96741c831a3970cf250a58ec46e6a66f269be0b4b6220f458c056ce4900c0bc4276030000006e1e9b00000404feff0300009d03",
        {
            "hotkey": (
                (
                    196,
                    3,
                    82,
                    202,
                    113,
                    226,
                    110,
                    131,
                    182,
                    200,
                    96,
                    88,
                    253,
                    77,
                    60,
                    150,
                    67,
                    234,
                    93,
                    193,
                    31,
                    18,
                    10,
                    124,
                    128,
                    244,
                    126,
                    197,
                    119,
                    11,
                    69,
                    125,
                ),
            ),
            "coldkey": (
                (
                    136,
                    83,
                    1,
                    140,
                    168,
                    148,
                    203,
                    61,
                    2,
                    170,
                    249,
                    185,
                    103,
                    65,
                    200,
                    49,
                    163,
                    151,
                    12,
                    242,
                    80,
                    165,
                    142,
                    196,
                    110,
                    106,
                    102,
                    242,
                    105,
                    190,
                    11,
                    75,
                ),
            ),
            "uid": 1,
            "netuid": 1,
            "active": False,
            "axon_info": {
                "block": 3380410,
                "version": 711,
                "ip": 754100960,
                "port": 9389,
                "ip_type": 4,
                "protocol": 4,
                "placeholder1": 0,
                "placeholder2": 0,
            },
            "prometheus_info": {
                "block": 0,
                "version": 0,
                "ip": 0,
                "port": 0,
                "ip_type": 0,
            },
            "stake": (
                (
                    (
                        (
                            136,
                            83,
                            1,
                            140,
                            168,
                            148,
                            203,
                            61,
                            2,
                            170,
                            249,
                            185,
                            103,
                            65,
                            200,
                            49,
                            163,
                            151,
                            12,
                            242,
                            80,
                            165,
                            142,
                            196,
                            110,
                            106,
                            102,
                            242,
                            105,
                            190,
                            11,
                            75,
                        ),
                    ),
                    373098520,
                ),
            ),
            "rank": 48,
            "emission": 1209237,
            "incentive": 48,
            "consensus": 47,
            "trust": 56720,
            "validator_trust": 0,
            "dividends": 0,
            "last_update": 2541467,
            "validator_permit": False,
            "weights": ((1, 65535),),
            "bonds": (),
            "pruning_score": 231,
        },
    ),  # NeuronInfo
    "Option<scale_info::39> ": ("00", None),
    "Option<scale_info::39>": ("010100", 1),  # u16
}


TEST_TYPE_STRING_PLAIN_DECODING: Dict[str, Tuple[str, Any]] = {
    "bool": ("01", True),
    "bool ": ("00", False),
    "u8": ("01", 1),
    "u16": ("0100", 1),
    "u32": ("01000000", 1),
    "u64": ("0100000000000000", 1),
    "u128": ("01000000000000000000000000000000", 1),
    "Compact<u8>": ("00", 0),
    "Compact<u8> ": ("fd03", 2**8 - 1),
    "Compact<u16>": ("feff0300", 2**16 - 1),
    "Compact<u32>": ("03ffffffff", 2**32 - 1),
    "Compact<u64>": ("13ffffffffffffffff", 2**64 - 1),
    "Option<u8>": ("010c", 12),
    "Option<u8>": ("00", None),
    "Option<u32>": ("00", None),
    "Option<u32> ": ("0101000000", 1), 
    "()": ("", ()),
    "[u8; 4]": ("62616265", (98, 97, 98, 101)),
    "[u8; 4]": ("62616265", [98, 97, 98, 101]),
    "Vec<u8>": ("0c010203", (1, 2, 3)),
    "Vec<u8> ": ("00", []),
    "Vec<u8> ": ("00", ()),
    "(u8, u16) ": ("7bffff", (123, 2**16-1)),
    "str": ("0c666f6f", "foo"),
}

TEST_TYPES_JSON = "tests/test_types.json"


@pytest.mark.parametrize(
    "type_string,test_hex,test_value",
    [(x, y, z) for x, (y, z) in TEST_TYPE_STRING_PLAIN_DECODING.items()],
)
class TestEncodeByPlainTypeString:
    # Test combinations of human-readable type strings and hex-encoded values
    registry: bt_decode.PortableRegistry

    @classmethod
    def setup_class(cls) -> None:
        with open(TEST_TYPES_JSON, "r") as f:
            types_json_str = f.read()

        cls.registry = bt_decode.PortableRegistry.from_json(types_json_str)

    def test_encode_values(self, type_string: str, test_value: Any, test_hex: str):
        type_string = type_string.strip()

        test_bytes = bytes.fromhex(test_hex)
        actual: list[int] = bt_decode.encode(type_string, self.registry, test_value)
        assert bytes(actual) == test_bytes


@pytest.mark.parametrize(
    "type_string,test_hex,test_value",
    [(x, y, z) for x, (y, z) in TEST_TYPE_STRING_SCALE_INFO_DECODING.items()],
)
class TestEncodeByScaleInfoTypeString:
    # Test combinations of scale_info::NUM -formatted type strings and hex-encoded values
    registry: bt_decode.PortableRegistry

    @classmethod
    def setup_class(cls) -> None:
        with open(TEST_TYPES_JSON, "r") as f:
            types_json_str = f.read()

        cls.registry = bt_decode.PortableRegistry.from_json(types_json_str)

    def test_encode_values(self, type_string: str, test_value: Any, test_hex: str):
        type_string = type_string.strip()

        test_bytes = bytes.fromhex(test_hex)
        actual: list[int] = bt_decode.encode(type_string, self.registry, test_value)
        assert bytes(actual) == test_bytes
