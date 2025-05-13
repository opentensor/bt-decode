from typing import Any, Dict, Tuple

import pytest

import bt_decode


TEST_TYPE_STRING_SCALE_INFO_DECODING_LEGACY_ACCOUNT_ID: Dict[str, Tuple[str, Any]] = {
    # AccountId32
    "scale_info::0":  ("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d", ((212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125),)),
    # Just 32 bytes
    "scale_info::1": ("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d", (212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125)),
}

TEST_TYPE_STRING_SCALE_INFO_DECODING_NO_LEGACY_ACCOUNT_ID: Dict[str, Tuple[str, Any]] = {
    # AccountId32
    "scale_info::0":  ("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d", "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"),
    # Just 32 bytes
    "scale_info::1": ("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d", (212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125)),
}

TEST_TYPES_JSON = "tests/test_types.json"


@pytest.mark.parametrize(
    "type_string,test_hex,expected",
    [(x, y, z) for x, (y, z) in TEST_TYPE_STRING_SCALE_INFO_DECODING_LEGACY_ACCOUNT_ID.items()],
)
class TestDecodeByScaleInfoTypeString:
    # Test combinations of scale_info::NUM -formatted type strings and hex-encoded values
    registry: bt_decode.PortableRegistry

    @classmethod
    def setup_class(cls) -> None:
        with open(TEST_TYPES_JSON, "r") as f:
            types_json_str = f.read()

        cls.registry = bt_decode.PortableRegistry.from_json(types_json_str)

    def test_decode_values_legacy_account_id(self, type_string: str, test_hex: str, expected: Any):
        type_string = type_string.strip()

        test_bytes = bytes.fromhex(test_hex)
        # Legacy account id is True by default
        actual = bt_decode.decode(type_string, self.registry, test_bytes)
        print(actual)
        assert actual == expected


@pytest.mark.parametrize(
    "type_string,test_hex,expected",
    [(x, y, z) for x, (y, z) in TEST_TYPE_STRING_SCALE_INFO_DECODING_NO_LEGACY_ACCOUNT_ID.items()],
)
class TestDecodeByScaleInfoTypeStringUnwrapAccountId:
    # Test combinations of scale_info::NUM -formatted type strings and hex-encoded values
    registry: bt_decode.PortableRegistry

    @classmethod
    def setup_class(cls) -> None:
        with open(TEST_TYPES_JSON, "r") as f:
            types_json_str = f.read()

        cls.registry = bt_decode.PortableRegistry.from_json(types_json_str)

    def test_decode_values_no_legacy_account_id(self, type_string: str, test_hex: str, expected: Any):
        type_string = type_string.strip()

        test_bytes = bytes.fromhex(test_hex)
        actual = bt_decode.decode(type_string, self.registry, test_bytes, legacy_account_id=False)
        print(actual)
        assert actual == expected
