__version__ = "0.0.1"

import bt_decode
from bt_decode import NeuronInfo


def decode(type_string: str, metadata: bytes, encoded: bytes) -> str:
    return bt_decode.decode(type_string, metadata, encoded)



