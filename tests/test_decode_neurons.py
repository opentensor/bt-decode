from typing import Any, Callable, Dict, List, Optional

import dataclasses
import netaddr
import unittest

import bt_decode
import bittensor

from . import (
    get_file_bytes,
    fix_field as fix_field_fixes,
    py_getattr as py_getattr_fixes,
)

TEST_NEURON_INFO_LITE_HEX = {
    "normal": "c40352ca71e26e83b6c86058fd4d3c9643ea5dc11f120a7c80f47ec5770b457d8853018ca894cb3d02aaf9b96741c831a3970cf250a58ec46e6a66f269be0b4b040400ba94330000000000c7020000e0aaf22c000000000000000000000000ad240404000000000000000000000000000000000000000000000000000000000000000000048853018ca894cb3d02aaf9b96741c831a3970cf250a58ec46e6a66f269be0b4b1ee07d572901f6fc6f002901210166e7030000006e1e9b00007d05",
    "vec normal": lambda: get_file_bytes("tests/neurons_lite.hex"),
}

TEST_NEURON_INFO_HEX = {
    "normal": "c40352ca71e26e83b6c86058fd4d3c9643ea5dc11f120a7c80f47ec5770b457d8853018ca894cb3d02aaf9b96741c831a3970cf250a58ec46e6a66f269be0b4b040400ba94330000000000c7020000e0aaf22c000000000000000000000000ad240404000000000000000000000000000000000000000000000000000000000000000000048853018ca894cb3d02aaf9b96741c831a3970cf250a58ec46e6a66f269be0b4b6220f458c056ce4900c0bc4276030000006e1e9b00000404feff0300009d03",
    "vec normal": lambda: get_file_bytes("tests/neurons.hex"),
}

FIELD_FIXES: Dict[str, Callable] = {
    "axon_info": {
        "ip": lambda x: str(netaddr.IPAddress(x)),
    },
    "bonds": lambda x: [[e[0], e[1]] for e in x],
    "coldkey": lambda x: bittensor.u8_key_to_ss58(x),
    "hotkey": lambda x: bittensor.u8_key_to_ss58(x),
    "consensus": lambda x: bittensor.U16_NORMALIZED_FLOAT(x),
    "dividends": lambda x: bittensor.U16_NORMALIZED_FLOAT(x),
    "emission": lambda x: x / 1e9,
    "incentive": lambda x: bittensor.U16_NORMALIZED_FLOAT(x),
    "prometheus_info": {
        "ip": lambda x: str(netaddr.IPAddress(x)),
    },
    "rank": lambda x: bittensor.U16_NORMALIZED_FLOAT(x),
    "stake": lambda x: bittensor.Balance(sum([y[1] for y in x])),
    "trust": lambda x: bittensor.U16_NORMALIZED_FLOAT(x),
    "validator_trust": lambda x: bittensor.U16_NORMALIZED_FLOAT(x),
    "weights": lambda x: [[e[0], e[1]] for e in x],
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


class TestDecodeNeuronInfoLite(unittest.TestCase):
    def test_decode_no_errors(self):
        _ = bt_decode.NeuronInfoLite.decode(
            bytes.fromhex(TEST_NEURON_INFO_LITE_HEX["normal"])
        )

    def test_decode_matches_python_impl(self):
        neuron_info: bt_decode.NeuronInfoLite = bt_decode.NeuronInfoLite.decode(
            bytes.fromhex(TEST_NEURON_INFO_LITE_HEX["normal"])
        )

        neuron_info_py = bittensor.NeuronInfoLite.from_vec_u8(
            list(bytes.fromhex(TEST_NEURON_INFO_LITE_HEX["normal"]))
        )

        attr_count = 0
        for attr in dir(neuron_info):
            if not attr.startswith("__") and not callable(getattr(neuron_info, attr)):
                attr_count += 1

                attr_py = py_getattr(neuron_info_py, attr)
                if dataclasses.is_dataclass(attr_py):
                    attr_rs = getattr(neuron_info, attr)

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
                        fix_field(attr, getattr(neuron_info, attr)),
                        py_getattr(neuron_info_py, attr),
                        f"Attribute {attr} does not match",
                    )

        self.assertGreater(attr_count, 0, "No attributes found")

    def test_decode_vec_no_errors(self):
        _ = bt_decode.NeuronInfoLite.decode_vec(
            TEST_NEURON_INFO_LITE_HEX["vec normal"]()
        )

    def test_decode_vec_matches_python_impl(self):
        neurons_info: List[bt_decode.NeuronInfoLite] = (
            bt_decode.NeuronInfoLite.decode_vec(
                TEST_NEURON_INFO_LITE_HEX["vec normal"]()
            )
        )

        neurons_info_py: List[bittensor.NeuronInfoLite] = (
            bittensor.NeuronInfoLite.list_from_vec_u8(
                list(TEST_NEURON_INFO_LITE_HEX["vec normal"]())
            )
        )

        for neuron_info, neuron_info_py in zip(neurons_info, neurons_info_py):
            attr_count = 0
            for attr in dir(neuron_info):
                if not attr.startswith("__") and not callable(
                    getattr(neuron_info, attr)
                ):
                    attr_count += 1
                    attr_py = py_getattr(neuron_info_py, attr)
                    if dataclasses.is_dataclass(attr_py):
                        attr_rs = getattr(neuron_info, attr)

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
                            fix_field(attr, getattr(neuron_info, attr)),
                            py_getattr(neuron_info_py, attr),
                            f"Attribute {attr} does not match",
                        )

            self.assertGreater(attr_count, 0, "No attributes found")


class TestDecodeNeuronInfo(unittest.TestCase):
    def test_decode_no_errors(self):
        _ = bt_decode.NeuronInfo.decode(bytes.fromhex(TEST_NEURON_INFO_HEX["normal"]))

    def test_decode_matches_python_impl(self):
        neuron_info: bt_decode.NeuronInfo = bt_decode.NeuronInfo.decode(
            bytes.fromhex(TEST_NEURON_INFO_HEX["normal"])
        )

        neuron_info_py = bittensor.NeuronInfo.from_vec_u8(
            list(bytes.fromhex(TEST_NEURON_INFO_HEX["normal"]))
        )

        attr_count = 0
        for attr in dir(neuron_info):
            if not attr.startswith("__") and not callable(getattr(neuron_info, attr)):
                attr_count += 1

                attr_py = py_getattr(neuron_info_py, attr)
                if dataclasses.is_dataclass(attr_py):
                    attr_rs = getattr(neuron_info, attr)

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
                        fix_field(attr, getattr(neuron_info, attr)),
                        py_getattr(neuron_info_py, attr),
                        f"Attribute {attr} does not match",
                    )

        self.assertGreater(attr_count, 0, "No attributes found")

    def test_decode_vec_no_errors(self):
        _ = bt_decode.NeuronInfo.decode_vec(TEST_NEURON_INFO_HEX["vec normal"]())

    def test_decode_vec_matches_python_impl(self):
        neurons_info: List[bt_decode.NeuronInfo] = bt_decode.NeuronInfo.decode_vec(
            TEST_NEURON_INFO_HEX["vec normal"]()
        )

        neurons_info_py: List[bittensor.NeuronInfo] = (
            bittensor.NeuronInfo.list_from_vec_u8(
                list(TEST_NEURON_INFO_HEX["vec normal"]())
            )
        )

        for neuron_info, neuron_info_py in zip(neurons_info, neurons_info_py):
            attr_count = 0

            for attr in dir(neuron_info):
                if not attr.startswith("__") and not callable(
                    getattr(neuron_info, attr)
                ):
                    attr_count += 1

                    attr_py = py_getattr(neuron_info_py, attr)
                    if dataclasses.is_dataclass(attr_py):
                        attr_rs = getattr(neuron_info, attr)

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
                            fix_field(attr, getattr(neuron_info, attr)),
                            py_getattr(neuron_info_py, attr),
                            f"Attribute {attr} does not match",
                        )

            self.assertGreater(attr_count, 0, "No attributes found")
