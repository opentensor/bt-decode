from typing import List, Optional, Tuple

class AxonInfo:
    #  Axon serving block.
    block: int
    #  Axon version
    version: int
    #  Axon u128 encoded ip address of type v6 or v4.
    ip: int
    #  Axon u16 encoded port.
    port: int
    #  Axon ip type, 4 for ipv4 and 6 for ipv6.
    ip_type: int
    #  Axon protocol. TCP, UDP, other.
    protocol: int
    #  Axon proto placeholder 1.
    placeholder1: int
    #  Axon proto placeholder 2.
    placeholder2: int

    @staticmethod
    def decode(encoded: bytes) -> "PrometheusInfo":
        pass
    @staticmethod
    def decode_vec(encoded: bytes) -> List["PrometheusInfo"]:
        pass

class PrometheusInfo:
    block: int
    # Prometheus version.
    version: int
    #  Prometheus u128 encoded ip address of type v6 or v4.
    ip: int
    # Prometheus u16 encoded port.
    port: int
    # Prometheus ip type, 4 for ipv4 and 6 for ipv6.
    ip_type: int

    @staticmethod
    def decode(encoded: bytes) -> "PrometheusInfo":
        pass
    @staticmethod
    def decode_vec(encoded: bytes) -> List["PrometheusInfo"]:
        pass

class NeuronInfoLite:
    hotkey: bytes
    coldkey: bytes
    uid: int
    netuid: int
    active: bool
    axon_info: AxonInfo
    prometheus_info: PrometheusInfo
    stake: List[
        Tuple[bytes, int]
    ]  # map of coldkey to stake on this neuron/hotkey (includes delegations)
    rank: int
    emission: int
    incentive: int
    consensus: int
    trust: int
    validator_trust: int
    dividends: int
    last_update: int
    validator_permit: bool
    # has no weights or bonds
    pruning_score: int

    @staticmethod
    def decode(encoded: bytes) -> "NeuronInfoLite":
        pass
    @staticmethod
    def decode_vec(encoded: bytes) -> List["NeuronInfoLite"]:
        pass

class SubnetIdentity:
    subnet_name: bytes  # TODO: or List[int] ??
    # The github repository associated with the chain identity
    github_repo: bytes
    # The subnet's contact
    subnet_contact: bytes

    @staticmethod
    def decode(encoded: bytes) -> "SubnetIdentity":
        pass
    @staticmethod
    def decode_vec(encoded: bytes) -> List["SubnetIdentity"]:
        pass

class SubnetInfo:
    netuid: int
    rho: int
    kappa: int
    difficulty: int
    immunity_period: int
    max_allowed_validators: int
    min_allowed_weights: int
    max_weights_limit: int
    scaling_law_power: int
    subnetwork_n: int
    max_allowed_uids: int
    blocks_since_last_step: int
    tempo: int
    network_modality: int
    network_connect: List[List[int]]  # List[[int, int]]
    emission_values: int
    burn: int
    owner: bytes

    @staticmethod
    def decode(encoded: bytes) -> "SubnetInfo":
        pass
    @staticmethod
    def decode_vec(encoded: bytes) -> List["SubnetInfo"]:
        pass

class SubnetInfoV2:
    netuid: int
    rho: int
    kappa: int
    difficulty: int
    immunity_period: int
    max_allowed_validators: int
    min_allowed_weights: int
    max_weights_limit: int
    scaling_law_power: int
    subnetwork_n: int
    max_allowed_uids: int
    blocks_since_last_step: int
    tempo: int
    network_modality: int
    network_connect: List[List[int]]  # List[[int, int]]
    emission_values: int
    burn: int
    owner: bytes
    identity: Optional[SubnetIdentity]

    @staticmethod
    def decode(encoded: bytes) -> "SubnetInfoV2":
        pass
    @staticmethod
    def decode_vec(encoded: bytes) -> List["SubnetInfoV2"]:
        pass

class SubnetHyperparameters:
    rho: int
    kappa: int
    immunity_period: int
    min_allowed_weights: int
    max_weights_limit: int
    tempo: int
    min_difficulty: int
    max_difficulty: int
    weights_version: int
    weights_rate_limit: int
    adjustment_interval: int
    activity_cutoff: int
    registration_allowed: bool
    target_regs_per_interval: int
    min_burn: int
    max_burn: int
    bonds_moving_avg: int
    max_regs_per_block: int
    serving_rate_limit: int
    max_validators: int
    adjustment_alpha: int
    difficulty: int
    commit_reveal_weights_interval: int
    commit_reveal_weights_enabled: bool
    alpha_high: int
    alpha_low: int
    liquid_alpha_enabled: bool

    @staticmethod
    def decode(encoded: bytes) -> "SubnetHyperparameters":
        pass
    @staticmethod
    def decode_vec(encoded: bytes) -> List["SubnetHyperparameters"]:
        pass

class StakeInfo:
    hotkey: bytes
    coldkey: bytes
    stake: int

    @staticmethod
    def decode(encoded: bytes) -> "StakeInfo":
        pass
    @staticmethod
    def decode_vec(encoded: bytes) -> List["StakeInfo"]:
        pass
    @staticmethod
    def decode_vec_tuple_vec(encoded: bytes) -> List[Tuple[str, List["StakeInfo"]]]:
        pass

class DelegateInfo:
    delegate_ss58: bytes
    take: int
    nominators: List[Tuple[bytes, int]]  # map of nominator_ss58 to stake amount
    owner_ss58: bytes
    registrations: List[int]  # Vec of netuid this delegate is registered on
    validator_permits: List[int]  # Vec of netuid this delegate has validator permit on
    return_per_1000: int  # Delegators current daily return per 1000 TAO staked minus take fee
    total_daily_return: int

    @staticmethod
    def decode(encoded: bytes) -> "DelegateInfo":
        pass
    @staticmethod
    def decode_vec(encoded: bytes) -> List["DelegateInfo"]:
        pass
    @staticmethod
    def decode_delegated(encoded: bytes) -> List[Tuple["DelegateInfo", int]]:
        pass
