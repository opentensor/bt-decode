use codec::{Decode, Encode};
use custom_derive::pydecode;

use pyo3::prelude::*;

// Implements ToPyObject for Compact<T> where T is an unsigned integer.
macro_rules! impl_UnsignedCompactIntoPy {
    ( $($type:ty),* $(,)? ) => {
        $(
            impl IntoPy<PyObject> for Compact<$type> {
                fn into_py(self, py: Python<'_>) -> PyObject {
                    let value: $type = self.0.into();

                    value.into_py(py)
                }
            }
        )*
    };
}

#[derive(Clone, Encode, Decode, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Compact<T>(pub codec::Compact<T>);
impl_UnsignedCompactIntoPy!(u8, u16, u32, u64, u128);

type AccountId = [u8; 32];

#[pymodule(name = "bt_decode")]
mod bt_decode {
    use super::*;

    #[pyclass(name = "AxonInfo", get_all)]
    #[derive(Clone, Encode, Decode)]
    struct AxonInfo {
        ///  Axon serving block.
        pub block: u64,
        ///  Axon version
        pub version: u32,
        ///  Axon u128 encoded ip address of type v6 or v4.
        pub ip: u128,
        ///  Axon u16 encoded port.
        pub port: u16,
        ///  Axon ip type, 4 for ipv4 and 6 for ipv6.
        pub ip_type: u8,
        ///  Axon protocol. TCP, UDP, other.
        pub protocol: u8,
        ///  Axon proto placeholder 1.
        pub placeholder1: u8,
        ///  Axon proto placeholder 2.
        pub placeholder2: u8,
    }

    // #[pydecode]
    #[pymethods]
    impl AxonInfo {
        #[pyo3(name = "decode")]
        #[staticmethod]
        fn py_decode(encoded: &[u8]) -> Self {
            AxonInfo::decode(&mut &encoded[..]).expect("Failed to decode AxonInfo")
        }

        #[pyo3(name = "decode_vec")]
        #[staticmethod]
        fn py_decode_vec(encoded: &[u8]) -> Vec<AxonInfo> {
            Vec::<AxonInfo>::decode(&mut &encoded[..]).expect("Failed to decode Vec<AxonInfo>")
        }
    }

    #[pyclass(name = "PrometheusInfo", get_all)]
    #[derive(Clone, Encode, Decode)]
    struct PrometheusInfo {
        /// Prometheus serving block.
        pub block: u64,
        /// Prometheus version.
        pub version: u32,
        ///  Prometheus u128 encoded ip address of type v6 or v4.
        pub ip: u128,
        ///  Prometheus u16 encoded port.
        pub port: u16,
        /// Prometheus ip type, 4 for ipv4 and 6 for ipv6.
        pub ip_type: u8,
    }

    // #[pydecode]
    #[pymethods]
    impl PrometheusInfo {
        #[pyo3(name = "decode")]
        #[staticmethod]
        fn py_decode(encoded: &[u8]) -> Self {
            PrometheusInfo::decode(&mut &encoded[..]).expect("Failed to decode PrometheusInfo")
        }

        #[pyo3(name = "decode_vec")]
        #[staticmethod]
        fn py_decode_vec(encoded: &[u8]) -> Vec<PrometheusInfo> {
            Vec::<PrometheusInfo>::decode(&mut &encoded[..])
                .expect("Failed to decode Vec<PrometheusInfo>")
        }
    }

    #[pyclass(name = "NeuronInfo", get_all)]
    #[derive(Clone, Encode, Decode)]
    struct NeuronInfo {
        hotkey: AccountId,
        coldkey: AccountId,
        uid: Compact<u16>,
        netuid: Compact<u16>,
        active: bool,
        axon_info: AxonInfo,
        prometheus_info: PrometheusInfo,
        stake: Vec<(AccountId, Compact<u64>)>, // map of coldkey to stake on this neuron/hotkey (includes delegations)
        rank: Compact<u16>,
        emission: Compact<u64>,
        incentive: Compact<u16>,
        consensus: Compact<u16>,
        trust: Compact<u16>,
        validator_trust: Compact<u16>,
        dividends: Compact<u16>,
        last_update: Compact<u64>,
        validator_permit: bool,
        weights: Vec<(Compact<u16>, Compact<u16>)>, // Vec of (uid, weight)
        bonds: Vec<(Compact<u16>, Compact<u16>)>,   // Vec of (uid, bond)
        pruning_score: Compact<u16>,
    }

    #[pymethods]
    impl NeuronInfo {
        #[pyo3(name = "decode")]
        #[staticmethod]
        fn py_decode(encoded: &[u8]) -> Self {
            NeuronInfo::decode(&mut &encoded[..]).expect("Failed to decode NeuronInfo")
        }

        #[pyo3(name = "decode_vec")]
        #[staticmethod]
        fn py_decode_vec(encoded: &[u8]) -> Vec<NeuronInfo> {
            Vec::<NeuronInfo>::decode(&mut &encoded[..]).expect("Failed to decode Vec<NeuronInfo>")
        }
    }

    #[pyclass(name = "NeuronInfoLite", get_all)]
    #[derive(Clone, Encode, Decode)]
    struct NeuronInfoLite {
        hotkey: AccountId,
        coldkey: AccountId,
        uid: Compact<u16>,
        netuid: Compact<u16>,
        active: bool,
        axon_info: AxonInfo,
        prometheus_info: PrometheusInfo,
        stake: Vec<(AccountId, Compact<u64>)>, // map of coldkey to stake on this neuron/hotkey (includes delegations)
        rank: Compact<u16>,
        emission: Compact<u64>,
        incentive: Compact<u16>,
        consensus: Compact<u16>,
        trust: Compact<u16>,
        validator_trust: Compact<u16>,
        dividends: Compact<u16>,
        last_update: Compact<u64>,
        validator_permit: bool,
        // has no weights or bonds
        pruning_score: Compact<u16>,
    }

    #[pymethods]
    impl NeuronInfoLite {
        #[pyo3(name = "decode")]
        #[staticmethod]
        fn py_decode(encoded: &[u8]) -> Self {
            NeuronInfoLite::decode(&mut &encoded[..]).expect("Failed to decode NeuronInfoLite")
        }

        #[pyo3(name = "decode_vec")]
        #[staticmethod]
        fn py_decode_vec(encoded: &[u8]) -> Vec<NeuronInfoLite> {
            Vec::<NeuronInfoLite>::decode(&mut &encoded[..])
                .expect("Failed to decode Vec<NeuronInfoLite>")
        }
    }

    #[pyclass(name = "SubnetIdentity", get_all)]
    #[derive(Clone, Encode, Decode)]
    struct SubnetIdentity {
        subnet_name: Vec<u8>,
        /// The github repository associated with the chain identity
        github_repo: Vec<u8>,
        /// The subnet's contact
        subnet_contact: Vec<u8>,
    }

    #[pymethods]
    impl SubnetIdentity {
        #[pyo3(name = "decode")]
        #[staticmethod]
        fn py_decode(encoded: &[u8]) -> Self {
            SubnetIdentity::decode(&mut &encoded[..]).expect("Failed to decode SubnetIdentity")
        }

        #[pyo3(name = "decode_vec")]
        #[staticmethod]
        fn py_decode_vec(encoded: &[u8]) -> Vec<SubnetIdentity> {
            Vec::<SubnetIdentity>::decode(&mut &encoded[..])
                .expect("Failed to decode Vec<SubnetIdentity>")
        }
    }

    #[pyclass(name = "SubnetInfo", get_all)]
    #[derive(Clone, Encode, Decode)]
    struct SubnetInfo {
        netuid: Compact<u16>,
        rho: Compact<u16>,
        kappa: Compact<u16>,
        difficulty: Compact<u64>,
        immunity_period: Compact<u16>,
        max_allowed_validators: Compact<u16>,
        min_allowed_weights: Compact<u16>,
        max_weights_limit: Compact<u16>,
        scaling_law_power: Compact<u16>,
        subnetwork_n: Compact<u16>,
        max_allowed_uids: Compact<u16>,
        blocks_since_last_step: Compact<u64>,
        tempo: Compact<u16>,
        network_modality: Compact<u16>,
        network_connect: Vec<[u16; 2]>,
        emission_values: Compact<u64>,
        burn: Compact<u64>,
        owner: AccountId,
    }

    #[pymethods]
    impl SubnetInfo {
        #[pyo3(name = "decode")]
        #[staticmethod]
        fn py_decode(encoded: &[u8]) -> Self {
            SubnetInfo::decode(&mut &encoded[..]).expect("Failed to decode SubnetInfo")
        }

        #[pyo3(name = "decode_vec")]
        #[staticmethod]
        fn py_decode_vec(encoded: &[u8]) -> Vec<SubnetInfo> {
            Vec::<SubnetInfo>::decode(&mut &encoded[..]).expect("Failed to decode Vec<SubnetInfo>")
        }
    }

    #[pyclass(name = "SubnetInfoV2", get_all)]
    #[derive(Clone, Encode, Decode)]
    struct SubnetInfoV2 {
        netuid: Compact<u16>,
        rho: Compact<u16>,
        kappa: Compact<u16>,
        difficulty: Compact<u64>,
        immunity_period: Compact<u16>,
        max_allowed_validators: Compact<u16>,
        min_allowed_weights: Compact<u16>,
        max_weights_limit: Compact<u16>,
        scaling_law_power: Compact<u16>,
        subnetwork_n: Compact<u16>,
        max_allowed_uids: Compact<u16>,
        blocks_since_last_step: Compact<u64>,
        tempo: Compact<u16>,
        network_modality: Compact<u16>,
        network_connect: Vec<[u16; 2]>,
        emission_values: Compact<u64>,
        burn: Compact<u64>,
        owner: AccountId,
        identity: Option<SubnetIdentity>,
    }

    #[pymethods]
    impl SubnetInfoV2 {
        #[pyo3(name = "decode")]
        #[staticmethod]
        fn py_decode(encoded: &[u8]) -> Self {
            SubnetInfoV2::decode(&mut &encoded[..]).expect("Failed to decode SubnetInfoV2")
        }

        #[pyo3(name = "decode_vec")]
        #[staticmethod]
        fn py_decode_vec(encoded: &[u8]) -> Vec<SubnetInfoV2> {
            Vec::<SubnetInfoV2>::decode(&mut &encoded[..])
                .expect("Failed to decode Vec<SubnetInfoV2>")
        }
    }

    #[pyclass(name = "SubnetHyperparameters", get_all)]
    #[derive(Decode, Encode, Clone, Debug)]
    pub struct SubnetHyperparams {
        rho: Compact<u16>,
        kappa: Compact<u16>,
        immunity_period: Compact<u16>,
        min_allowed_weights: Compact<u16>,
        max_weights_limit: Compact<u16>,
        tempo: Compact<u16>,
        min_difficulty: Compact<u64>,
        max_difficulty: Compact<u64>,
        weights_version: Compact<u64>,
        weights_rate_limit: Compact<u64>,
        adjustment_interval: Compact<u16>,
        activity_cutoff: Compact<u16>,
        registration_allowed: bool,
        target_regs_per_interval: Compact<u16>,
        min_burn: Compact<u64>,
        max_burn: Compact<u64>,
        bonds_moving_avg: Compact<u64>,
        max_regs_per_block: Compact<u16>,
        serving_rate_limit: Compact<u64>,
        max_validators: Compact<u16>,
        adjustment_alpha: Compact<u64>,
        difficulty: Compact<u64>,
        commit_reveal_weights_interval: Compact<u64>,
        commit_reveal_weights_enabled: bool,
        alpha_high: Compact<u16>,
        alpha_low: Compact<u16>,
        liquid_alpha_enabled: bool,
    }

    #[pymethods]
    impl SubnetHyperparams {
        #[pyo3(name = "decode")]
        #[staticmethod]
        fn py_decode(encoded: &[u8]) -> Self {
            SubnetHyperparams::decode(&mut &encoded[..])
                .expect("Failed to decode SubnetHyperparams")
        }

        #[pyo3(name = "decode_vec")]
        #[staticmethod]
        fn py_decode_vec(encoded: &[u8]) -> Vec<SubnetHyperparams> {
            Vec::<SubnetHyperparams>::decode(&mut &encoded[..])
                .expect("Failed to decode Vec<SubnetHyperparams>")
        }
    }

    #[pyclass]
    #[derive(Decode, Encode, Clone, Debug)]
    struct StakeInfo {
        hotkey: AccountId,
        coldkey: AccountId,
        stake: Compact<u64>,
    }

    #[pymethods]
    impl StakeInfo {
        #[pyo3(name = "decode")]
        #[staticmethod]
        fn py_decode(encoded: &[u8]) -> Self {
            StakeInfo::decode(&mut &encoded[..]).expect("Failed to decode StakeInfo")
        }

        #[pyo3(name = "decode_vec")]
        #[staticmethod]
        fn py_decode_vec(encoded: &[u8]) -> Vec<StakeInfo> {
            Vec::<StakeInfo>::decode(&mut &encoded[..]).expect("Failed to decode Vec<StakeInfo>")
        }

        #[pyo3(name = "decode_vec_tuple_vec")]
        #[staticmethod]
        fn py_decode_vec_tuple_vec(encoded: &[u8]) -> Vec<(AccountId, Vec<StakeInfo>)> {
            Vec::<(AccountId, Vec<StakeInfo>)>::decode(&mut &encoded[..])
                .expect("Failed to decode Vec<(AccountId, Vec<StakeInfo>)>")
        }
    }

    #[pyclass]
    #[derive(Decode, Encode, Clone, Debug)]
    struct DelegateInfo {
        delegate_ss58: AccountId,
        take: Compact<u16>,
        nominators: Vec<(AccountId, Compact<u64>)>, // map of nominator_ss58 to stake amount
        owner_ss58: AccountId,
        registrations: Vec<Compact<u16>>, // Vec of netuid this delegate is registered on
        validator_permits: Vec<Compact<u16>>, // Vec of netuid this delegate has validator permit on
        return_per_1000: Compact<u64>, // Delegators current daily return per 1000 TAO staked minus take fee
        total_daily_return: Compact<u64>, // Delegators current daily return
    }

    #[pymethods]
    impl DelegateInfo {
        #[pyo3(name = "decode")]
        #[staticmethod]
        fn py_decode(encoded: &[u8]) -> Self {
            DelegateInfo::decode(&mut &encoded[..]).expect("Failed to decode DelegateInfo")
        }

        #[pyo3(name = "decode_vec")]
        #[staticmethod]
        fn py_decode_vec(encoded: &[u8]) -> Vec<DelegateInfo> {
            Vec::<DelegateInfo>::decode(&mut &encoded[..])
                .expect("Failed to decode Vec<DelegateInfo>")
        }

        #[pyo3(name = "decode_delegated")]
        #[staticmethod]
        fn py_decode_delegated(encoded: &[u8]) -> Vec<(DelegateInfo, Compact<u64>)> {
            Vec::<(DelegateInfo, Compact<u64>)>::decode(&mut &encoded[..])
                .expect("Failed to decode Vec<(DelegateInfo, Compact<u64>)>")
        }
    }

    // #[pyfunction(name = "decode")]
    // fn py_decode(
    //     type_string: &str,
    //     encoded_metadata_v15: &[u8],
    //     encoded: &[u8],
    // ) -> PyResult<MetadataProof> {
    //     let metadata_v15 = RuntimeMetadataPrefixed::decode(&mut &encoded_metadata_v15[..])
    //         .expect("Failed to decode metadataV15")
    //         .1;

    //     Ok(metadata_proof)
    // }
}
