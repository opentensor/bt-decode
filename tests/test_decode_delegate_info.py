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

TEST_DELEGATE_INFO_HEX = {
    "delegated normal": lambda: get_file_bytes("tests/delegated_info.hex"),
    "vec normal": lambda: get_file_bytes("tests/delegates_info.hex"),
}


FIELD_FIXES: Dict[str, Callable] = {
    # None
    "delegate_ss58": lambda x: bittensor.u8_key_to_ss58(x),
    "owner_ss58": lambda x: bittensor.u8_key_to_ss58(x),
    "take": lambda x: bittensor.U16_NORMALIZED_FLOAT(x),
    "nominators": lambda x: [(bittensor.u8_key_to_ss58(e[0]), e[1]) for e in x],
}
fix_field = lambda key, value, parent_key=None: fix_field_fixes(
    FIELD_FIXES, key, value, parent_key
)

ATTR_NAME_FIXES: Dict[str, str] = {
    # None
    "delegate_ss58": "hotkey_ss58",
}

py_getattr = lambda obj, attr, parent_name=None: py_getattr_fixes(
    ATTR_NAME_FIXES, obj, attr, parent_name
)


class TestDecodeDelegateInfo(unittest.TestCase):
    def test_decode_delegated_no_errors(self):
        _ = bt_decode.DelegateInfo.decode_delegated(
            TEST_DELEGATE_INFO_HEX["delegated normal"]()
        )

    def test_decode_delegated_matches_python_impl(self):
        delegate_info_list: List[Tuple[bt_decode.DelegateInfo, int]] = (
            bt_decode.DelegateInfo.decode_delegated(
                TEST_DELEGATE_INFO_HEX["delegated normal"]()
            )
        )

        delegate_info_py_list = bittensor.DelegateInfo.delegated_list_from_vec_u8(
            list(TEST_DELEGATE_INFO_HEX["delegated normal"]())
        )

        for (delegate_info, balance), (delegate_info_py, balance_py) in zip(
            delegate_info_list, delegate_info_py_list
        ):
            self.assertEqual(balance, balance_py, "Balance does not match")

            attr_count = 0

            for attr in dir(delegate_info):
                if not attr.startswith("__") and not callable(
                    getattr(delegate_info, attr)
                ):
                    attr_count += 1
                    attr_py = py_getattr(delegate_info_py, attr)
                    if dataclasses.is_dataclass(attr_py):
                        attr_rs = getattr(delegate_info, attr)

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
                            fix_field(attr, getattr(delegate_info, attr)),
                            py_getattr(delegate_info_py, attr),
                            f"Attribute {attr} does not match",
                        )

            self.assertGreater(attr_count, 0, "No attributes found")

    def test_decode_vec_no_errors(self):
        _ = bt_decode.DelegateInfo.decode_vec(TEST_DELEGATE_INFO_HEX["vec normal"]())

    def test_decode_vec_matches_python_impl(self):
        delegates_info: List[bt_decode.DelegateInfo] = (
            bt_decode.DelegateInfo.decode_vec(TEST_DELEGATE_INFO_HEX["vec normal"]())
        )

        delegates_info_py: List[bittensor.DelegateInfo] = (
            bittensor.DelegateInfo.list_from_vec_u8(
                list(TEST_DELEGATE_INFO_HEX["vec normal"]())
            )
        )

        for delegate_info, delegate_info_py in zip(delegates_info, delegates_info_py):
            attr_count = 0
            for attr in dir(delegate_info):
                if not attr.startswith("__") and not callable(
                    getattr(delegate_info, attr)
                ):
                    attr_count += 1
                    attr_py = py_getattr(delegate_info_py, attr)
                    if dataclasses.is_dataclass(attr_py):
                        attr_rs = getattr(delegate_info, attr)

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
                            fix_field(attr, getattr(delegate_info, attr)),
                            py_getattr(delegate_info_py, attr),
                            f"Attribute {attr} does not match",
                        )

            self.assertGreater(attr_count, 0, "No attributes found")
