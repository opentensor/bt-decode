from typing import Callable, Dict, List

import dataclasses
import netaddr
import unittest

import bittensor
import bt_decode

from . import (
    get_file_bytes,
    fix_field as fix_field_fixes,
    py_getattr as py_getattr_fixes,
)

from .utils import chain_data

TEST_NEURON_INFO_LITE_HEX = {
    "normal": "fe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0008010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000041cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0bbb22f41921010bbb22f41921010bbb22f4192101000007989cc65f0100000000009801feff0300",
    "vec normal": lambda: get_file_bytes("tests/neurons_lite.hex"),
}

TEST_NEURON_INFO_HEX = {
    "normal": "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0408010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000041cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c00000000000000000000002d0101000000",
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


def fix_field(key, value, parent_key=None):
    return fix_field_fixes(FIELD_FIXES, key, value, parent_key)


ATTR_NAME_FIXES: Dict[str, str] = {
    # None
}


def py_getattr(obj, attr, parent_name=None):
    return py_getattr_fixes(ATTR_NAME_FIXES, obj, attr, parent_name)


class TestDecodeNeuronInfoLite(unittest.TestCase):
    def test_decode_no_errors(self):
        _ = bt_decode.NeuronInfoLite.decode(
            bytes.fromhex(TEST_NEURON_INFO_LITE_HEX["normal"])
        )

    def test_decode_matches_python_impl(self):
        neuron_info: bt_decode.NeuronInfoLite = bt_decode.NeuronInfoLite.decode(
            bytes.fromhex(TEST_NEURON_INFO_LITE_HEX["normal"])
        )

        neuron_info_py = chain_data.NeuronInfoLite.from_vec_u8(
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

        neurons_info_py: List[chain_data.NeuronInfoLite] = (
            chain_data.NeuronInfoLite.list_from_vec_u8(
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

        neuron_info_py = chain_data.NeuronInfo.from_vec_u8(
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

        neurons_info_py: List[chain_data.NeuronInfo] = (
            chain_data.NeuronInfo.list_from_vec_u8(
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
