from typing import Callable, Dict, List, Tuple

import dataclasses
import unittest

import bt_decode
import bittensor

from . import (
    get_file_bytes,
    fix_field as fix_field_fixes,
    py_getattr as py_getattr_fixes,
)

TEST_STAKE_INFO_HEX = {
    "vec normal": "08e4df2c7397e1443378b4cec0f2fca9dac1d0923d020e7aab11dd41428014ab595c40bc195cb2fd36b8b0e2397087c73b555b81e0bfe2975a40b9f78e039d44420759e02b6017ae4f8eac06ab73ff50aa97c0aafd27cd5c311e2fbbe5628f24901f4e3e1b06695c40bc195cb2fd36b8b0e2397087c73b555b81e0bfe2975a40b9f78e039d4442e25c4a0c",
    "vec vec normal": "085c40bc195cb2fd36b8b0e2397087c73b555b81e0bfe2975a40b9f78e039d444208e4df2c7397e1443378b4cec0f2fca9dac1d0923d020e7aab11dd41428014ab595c40bc195cb2fd36b8b0e2397087c73b555b81e0bfe2975a40b9f78e039d44420759e02b6017ae4f8eac06ab73ff50aa97c0aafd27cd5c311e2fbbe5628f24901f4e3e1b06695c40bc195cb2fd36b8b0e2397087c73b555b81e0bfe2975a40b9f78e039d4442e25c4a0ce2a8d08674697d6ee2ccec36f0e207653788275b619392256e509be04d32950d0484d83d08ca89f8e60424ffa286f165c16dd8752e4faa4d8977221e6720678d28e2a8d08674697d6ee2ccec36f0e207653788275b619392256e509be04d32950d0ba85af01a4234",
}


FIELD_FIXES: Dict[str, Callable] = {
    "coldkey": lambda x: bittensor.u8_key_to_ss58(x),
    "hotkey": lambda x: bittensor.u8_key_to_ss58(x),
}
fix_field = lambda key, value, parent_key=None: fix_field_fixes(
    FIELD_FIXES, key, value, parent_key
)

ATTR_NAME_FIXES: Dict[str, str] = {
    # None
    "coldkey": "coldkey_ss58",
    "hotkey": "hotkey_ss58",
}

py_getattr = lambda obj, attr, parent_name=None: py_getattr_fixes(
    ATTR_NAME_FIXES, obj, attr, parent_name
)


class TestDecodeStakeInfo(unittest.TestCase):
    def test_decode_vec_no_errors(self):
        _ = bt_decode.StakeInfo.decode_vec(
            bytes.fromhex(TEST_STAKE_INFO_HEX["vec normal"])
        )

    def test_decode_vec_matches_python_impl(self):
        stake_info_list: List[bt_decode.StakeInfo] = bt_decode.StakeInfo.decode_vec(
            bytes.fromhex(TEST_STAKE_INFO_HEX["vec normal"])
        )

        stake_info_py_list: List[bittensor.StakeInfo] = (
            bittensor.StakeInfo.list_from_vec_u8(
                list(bytes.fromhex(TEST_STAKE_INFO_HEX["vec normal"]))
            )
        )

        for stake_info, stake_info_py in zip(stake_info_list, stake_info_py_list):
            attr_count = 0
            for attr in dir(stake_info):
                if not attr.startswith("__") and not callable(
                    getattr(stake_info, attr)
                ):
                    attr_count += 1

                    attr_py = py_getattr(stake_info_py, attr)
                    if dataclasses.is_dataclass(attr_py):
                        attr_rs = getattr(stake_info, attr)

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
                            fix_field(attr, getattr(stake_info, attr)),
                            py_getattr(stake_info_py, attr),
                            f"Attribute {attr} does not match",
                        )

            self.assertGreater(attr_count, 0, "No attributes found")

    def test_decode_vec_vec_matches_python_impl(self):
        stake_info_list: List[Tuple[bytes, List[bt_decode.StakeInfo]]] = (
            bt_decode.StakeInfo.decode_vec_tuple_vec(
                bytes.fromhex(TEST_STAKE_INFO_HEX["vec vec normal"])
            )
        )

        # Poor method name, should be dict_of_list_from_vec_u8
        stake_info_py_dict: Dict[str, List[bittensor.StakeInfo]] = (
            bittensor.StakeInfo.list_of_tuple_from_vec_u8(
                list(bytes.fromhex(TEST_STAKE_INFO_HEX["vec vec normal"]))
            )
        )

        for stake_info_tuple, (coldkey, stake_info_py_list) in zip(
            stake_info_list, stake_info_py_dict.items()
        ):
            coldkey_rs, stake_info_list = stake_info_tuple
            self.assertEqual(fix_field("coldkey", coldkey_rs), coldkey)

            for stake_info, stake_info_py in zip(stake_info_list, stake_info_py_list):
                attr_count = 0
                for attr in dir(stake_info):
                    if not attr.startswith("__") and not callable(
                        getattr(stake_info, attr)
                    ):
                        attr_count += 1

                        attr_py = py_getattr(stake_info_py, attr)
                        if dataclasses.is_dataclass(attr_py):
                            attr_rs = getattr(stake_info, attr)

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
                                fix_field(attr, getattr(stake_info, attr)),
                                py_getattr(stake_info_py, attr),
                                f"Attribute {attr} does not match",
                            )

                self.assertGreater(attr_count, 0, "No attributes found")
