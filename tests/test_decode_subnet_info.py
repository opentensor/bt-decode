from typing import Callable, Dict

import dataclasses
import unittest

import bt_decode
import bittensor

from . import (
    get_file_bytes,
    fix_field as fix_field_fixes,
    py_getattr as py_getattr_fixes,
)

TEST_SUBNET_INFO_HEX = {
    "normal": "0828feff010013ffffffffffffffff214e010104feff0300c8010401040d03a1050000c28ff4070398b6d54370c07a546ab0bab5ca9847eb5890ada1bda127633e607097ad4517dd2ca0f010",
    "option normal": "010828feff010013ffffffffffffffff214e010104feff0300c8010401040d03a1050000c28ff4070398b6d54370c07a546ab0bab5ca9847eb5890ada1bda127633e607097ad4517dd2ca0f010",
    "option none": "00",
    "vec option normal": lambda: get_file_bytes("tests/subnets_info.hex"),
}


FIELD_FIXES: Dict[str, Callable] = {
    "network_connect": lambda x: {
        str(int(netuid)): bittensor.U16_NORMALIZED_FLOAT(int(req)) for netuid, req in x
    },
    "owner": lambda x: bittensor.u8_key_to_ss58(x),
}
fix_field = lambda key, value, parent_key=None: fix_field_fixes(
    FIELD_FIXES, key, value, parent_key
)

ATTR_NAME_FIXES: Dict[str, str] = {
    "emission_values": "emission_value",
    "max_allowed_uids": "max_n",
    "max_weights_limit": "max_weight_limit",
    "network_connect": "connection_requirements",
    "network_modality": "modality",
    "owner": "owner_ss58",
}

py_getattr = lambda obj, attr, parent_name=None: py_getattr_fixes(
    ATTR_NAME_FIXES, obj, attr, parent_name
)


class TestDecodeSubnetInfo(unittest.TestCase):
    def test_decode_no_errors(self):
        _ = bt_decode.SubnetInfo.decode(bytes.fromhex(TEST_SUBNET_INFO_HEX["normal"]))

    def test_decode_matches_python_impl(self):
        subnet_info: bt_decode.SubnetInfo = bt_decode.SubnetInfo.decode(
            bytes.fromhex(TEST_SUBNET_INFO_HEX["normal"])
        )

        subnet_info_py = bittensor.SubnetInfo.from_vec_u8(
            list(bytes.fromhex(TEST_SUBNET_INFO_HEX["normal"]))
        )

        attr_count = 0
        for attr in dir(subnet_info):
            if not attr.startswith("__") and not callable(getattr(subnet_info, attr)):
                attr_count += 1

                attr_py = py_getattr(subnet_info_py, attr)
                if dataclasses.is_dataclass(attr_py):
                    attr_rs = getattr(subnet_info, attr)

                    for sub_attr in dir(attr_rs):
                        if not sub_attr.startswith("__") and not callable(
                            getattr(attr_rs, sub_attr)
                        ):
                            self.assertEqual(
                                fix_field(sub_attr, getattr(attr_rs, sub_attr), attr),
                                py_getattr(attr_py, sub_attr),
                                f"Attribute {attr}.{sub_attr} does not match",
                            )
                else:
                    self.assertEqual(
                        fix_field(attr, getattr(subnet_info, attr)),
                        py_getattr(subnet_info_py, attr),
                        f"Attribute {attr} does not match",
                    )

        self.assertGreater(attr_count, 0, "No attributes found")

    def test_decode_option_no_errors(self):
        _ = bt_decode.SubnetInfo.decode_option(
            bytes.fromhex(TEST_SUBNET_INFO_HEX["option normal"])
        )

    def test_decode_option_handles_none_some(self):
        should_be_some = bt_decode.SubnetInfo.decode_option(
            bytes.fromhex(TEST_SUBNET_INFO_HEX["option normal"])
        )
        self.assertIsNotNone(should_be_some)

        should_be_none = bt_decode.SubnetInfo.decode_option(
            bytes.fromhex(TEST_SUBNET_INFO_HEX["option none"])
        )
        self.assertIsNone(should_be_none)

    def test_decode_vec_option_no_errors(self):
        _ = bt_decode.SubnetInfo.decode_vec_option(
            TEST_SUBNET_INFO_HEX["vec option normal"]()
        )

    def test_decode_vec_option_matches_python_impl(self):
        subnet_info_list: bt_decode.SubnetInfo = bt_decode.SubnetInfo.decode_vec_option(
            TEST_SUBNET_INFO_HEX["vec option normal"]()
        )

        subnet_info_list_py = (
            bittensor.SubnetInfo.list_from_vec_u8(  # Option specified internally
                list(TEST_SUBNET_INFO_HEX["vec option normal"]())
            )
        )

        for subnet_info, subnet_info_py in zip(subnet_info_list, subnet_info_list_py):
            if subnet_info is None:
                self.assertIsNone(subnet_info_py, "None does not match")
                continue

            attr_count = 0
            for attr in dir(subnet_info):
                if not attr.startswith("__") and not callable(
                    getattr(subnet_info, attr)
                ):
                    attr_count += 1

                    attr_py = py_getattr(subnet_info_py, attr)
                    if dataclasses.is_dataclass(attr_py):
                        attr_rs = getattr(subnet_info, attr)

                        for sub_attr in dir(attr_rs):
                            if not sub_attr.startswith("__") and not callable(
                                getattr(attr_rs, sub_attr)
                            ):
                                self.assertEqual(
                                    fix_field(
                                        sub_attr, getattr(attr_rs, sub_attr), attr
                                    ),
                                    py_getattr(attr_py, sub_attr),
                                    f"Attribute {attr}.{sub_attr} does not match",
                                )
                    else:
                        self.assertEqual(
                            fix_field(attr, getattr(subnet_info, attr)),
                            py_getattr(subnet_info_py, attr),
                            f"Attribute {attr} does not match",
                        )

            self.assertGreater(attr_count, 0, "No attributes found")
