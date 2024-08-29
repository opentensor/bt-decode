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

TEST_SUBNET_HYP_HEX = {
    "normal": "28feff0100214e04feff0300a10513ffffffffffffffff13ffffffffffffffff31119101a105214e010402286bee0700e876481782ee360004c8010113ea51b81e85eb51f813ffffffffffffffffa10f009a990300cecc020000",
    # "vec normal": lambda : get_file_bytes("tests/subnet_hyp.hex"),
}


FIELD_FIXES: Dict[str, Callable] = {
    # None
}
fix_field = lambda key, value, parent_key=None: fix_field_fixes(
    FIELD_FIXES, key, value, parent_key
)

ATTR_NAME_FIXES: Dict[str, str] = {"max_weights_limit": "max_weight_limit"}

py_getattr = lambda obj, attr, parent_name=None: py_getattr_fixes(
    ATTR_NAME_FIXES, obj, attr, parent_name
)


class TestDecodeSubnetHyperparameters(unittest.TestCase):
    def test_decode_no_errors(self):
        _ = bt_decode.SubnetHyperparameters.decode(
            bytes.fromhex(TEST_SUBNET_HYP_HEX["normal"])
        )

    def test_decode_matches_python_impl(self):
        subnet_hyp: bt_decode.SubnetHyperparameters = (
            bt_decode.SubnetHyperparameters.decode(
                bytes.fromhex(TEST_SUBNET_HYP_HEX["normal"])
            )
        )

        subnet_hyp_py = bittensor.SubnetHyperparameters.from_vec_u8(
            list(bytes.fromhex(TEST_SUBNET_HYP_HEX["normal"]))
        )

        attr_count = 0
        for attr in dir(subnet_hyp):
            if not attr.startswith("__") and not callable(getattr(subnet_hyp, attr)):
                attr_count += 1

                attr_py = py_getattr(subnet_hyp_py, attr)
                if dataclasses.is_dataclass(attr_py):
                    attr_rs = getattr(subnet_hyp, attr)

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
                        fix_field(attr, getattr(subnet_hyp, attr)),
                        py_getattr(subnet_hyp_py, attr),
                        f"Attribute {attr} does not match",
                    )

        self.assertGreater(attr_count, 0, "No attributes found")
