# bt-decode
A python wrapper around the rust scale-codec crate for fast scale-decoding of Bittensor data structures.

## Usage

### DelegateInfo
#### get_delegates
```python
import bittensor
from bt_decode import DelegateInfo

# Setup subtensor connection
subtensor = bittensor.subtensor()
# Grab result from RuntimeAPI
hex_bytes_result = sub.query_runtime_api(
    runtime_api="DelegateInfoRuntimeApi",
    method="get_delegates",
    params=[ ]
)
# Decode scale-encoded DelegateInfo
delegates_info: List[DelegateInfo] = DelegateInfo.decode_vec(
    bytes.fromhex(
        hex_bytes_result
))
```
#### get_delegated
```python
import bittensor
from bt_decode import DelegateInfo

validator_key = bittensor.Keypair(ss58_address="5E9fVY1jexCNVMjd2rdBsAxeamFGEMfzHcyTn2fHgdHeYc5p")

# Setup subtensor connection
subtensor = bittensor.subtensor()
# Grab result from RuntimeAPI
hex_bytes_result = sub.query_runtime_api(
    runtime_api="DelegateInfoRuntimeApi",
    method="get_delegated",
    params=[list( validator_key.public_key )]
)
# Decode scale-encoded (DelegateInfo, take)
delegated_info: List[Tuple[DelegateInfo, int]] = DelegateInfo.decode_delegated(
    bytes.fromhex(
        hex_bytes_result
))
```

### StakeInfo
#### get_stake_info_for_coldkey
```python
import bittensor
from bt_decode import StakeInfo

validator_key = bittensor.Keypair(ss58_address="5HBtpwxuGNL1gwzwomwR7sjwUt8WXYSuWcLYN6f9KpTZkP4k")

# Setup subtensor connection
subtensor = bittensor.subtensor()
encoded_coldkey = list( validator_key.public_key )
# Grab result from RuntimeAPI
hex_bytes_result = sub.query_runtime_api(
    runtime_api="StakeInfoRuntimeApi",
    method="get_stake_info_for_coldkey",
    params=[encoded_coldkey]
)
# Decode scale-encoded StakeInfo
stake_info: List[StakeInfo] = StakeInfo.decode_vec(
    bytes.fromhex(
        hex_bytes_result
))
```

#### get_stake_info_for_coldkeys
```python
import bittensor
from bt_decode import StakeInfo

validator_key_0 = bittensor.Keypair(ss58_address="5GcCZ2BPXBjgG88tXJCEtkbdg2hNrPbL4EFfbiVRvBZdSQDC")
validator_key_1 = bittensor.Keypair(ss58_address="5HBtpwxuGNL1gwzwomwR7sjwUt8WXYSuWcLYN6f9KpTZkP4k")

encoded_coldkeys = [
    list( validator_key_0.public_key ),
    list( validator_key_1.public_key )
]

# Setup subtensor connection
subtensor = bittensor.subtensor()
# Grab result from RuntimeAPI
hex_bytes_result = sub.query_runtime_api(
    runtime_api="StakeInfoRuntimeApi",
    method="get_stake_info_for_coldkeys",
    params=[encoded_coldkeys]
)
# Decode scale-encoded (AccountId, StakeInfo)
stake_info: List[Tuple[bytes, List["StakeInfo"]]] = StakeInfo.decode_vec_tuple_vec(
    bytes.fromhex(
        hex_bytes_result
))
```