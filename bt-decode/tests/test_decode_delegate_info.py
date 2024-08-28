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
    # "vec normal": lambda : get_file_bytes("tests/delegate_info.hex"),
}


FIELD_FIXES: Dict[str, Callable] = {
    # None
}
fix_field = lambda key, value, parent_key=None: fix_field_fixes(
    FIELD_FIXES, key, value, parent_key
)

ATTR_NAME_FIXES: Dict[str, str] = {
   # None
}

py_getattr = lambda obj, attr, parent_name=None: py_getattr_fixes(
    ATTR_NAME_FIXES, obj, attr, parent_name
)


class TestDecodeDelegateInfo(unittest.TestCase):
    def test_decode_delegated_no_errors(self):
        _ = bt_decode.DelegateInfo.decode_delegated( TEST_DELEGATE_INFO_HEX["delegated normal"]() )

    def test_decode_delegated_matches_python_impl(self):
        delegate_info_list: List[ Tuple[bt_decode.DelegateInfo, int] ] = bt_decode.DelegateInfo.decode_delegated(
            TEST_DELEGATE_INFO_HEX["delegated normal"]()
        )

        delegate_info_py_list = bittensor.DelegateInfo.delegated_list_from_vec_u8 (
            list( TEST_DELEGATE_INFO_HEX["delegated normal"]() )
        )

        for (delegate_info, balance), (delegate_info_py, balance_py) in zip(delegate_info_list, delegate_info_py_list):
            self.assertEqual(
                balance,
                balance_py,
                "Balance does not match"
            )

            for attr in dir(delegate_info):
                if not attr.startswith("__") and not callable(getattr(delegate_info, attr)):
                    attr_py = py_getattr(delegate_info_py, attr)
                    if dataclasses.is_dataclass(attr_py):
                        attr_rs = getattr(delegate_info, attr)

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
                            fix_field(attr, getattr(delegate_info, attr)),
                            py_getattr(delegate_info_py, attr),
                            f"Attribute {attr} does not match",
                        )
