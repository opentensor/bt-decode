use codec::{Decode, Encode};
use custom_derive::pydecode;
use frame_metadata::{RuntimeMetadata, RuntimeMetadataPrefixed};

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

mod dyndecoder;

#[pymodule(name = "bt_decode")]
mod bt_decode {
    use std::{collections::HashMap, u128};

    use dyndecoder::{fill_memo_using_well_known_types, get_type_id_from_type_string};
    use frame_metadata::v15::RuntimeMetadataV15;
    use pyo3::types::{PyDict, PyInt, PyList, PyTuple};
    use scale_info::{form::PortableForm, TypeDefComposite};
    use scale_value::{
        self,
        scale::{decode_as_type, encode_as_type},
        Composite, Primitive, Value, ValueDef, Variant,
    };

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

    #[pydecode]
    #[pymethods]
    impl AxonInfo {}

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

    #[pydecode]
    #[pymethods]
    impl PrometheusInfo {}

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

    #[pydecode]
    #[pymethods]
    impl NeuronInfo {}

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

    #[pydecode]
    #[pymethods]
    impl NeuronInfoLite {}

    #[pyclass(name = "SubnetIdentity", get_all)]
    #[derive(Clone, Encode, Decode)]
    struct SubnetIdentity {
        subnet_name: Vec<u8>,
        /// The github repository associated with the chain identity
        github_repo: Vec<u8>,
        /// The subnet's contact
        subnet_contact: Vec<u8>,
    }

    #[pydecode]
    #[pymethods]
    impl SubnetIdentity {}

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

    #[pydecode]
    #[pymethods]
    impl SubnetInfo {
        #[pyo3(name = "decode_vec_option")]
        #[staticmethod]
        fn py_decode_vec_option(encoded: &[u8]) -> Vec<Option<SubnetInfo>> {
            Vec::<Option<SubnetInfo>>::decode(&mut &encoded[..])
                .expect("Failed to decode Vec<Option<SubnetInfo>>")
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

    #[pydecode]
    #[pymethods]
    impl SubnetInfoV2 {
        #[pyo3(name = "decode_vec_option")]
        #[staticmethod]
        fn py_decode_vec_option(encoded: &[u8]) -> Vec<Option<SubnetInfoV2>> {
            Vec::<Option<SubnetInfoV2>>::decode(&mut &encoded[..])
                .expect("Failed to decode Vec<Option<SubnetInfoV2>>")
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

    #[pydecode]
    #[pymethods]
    impl SubnetHyperparams {}

    #[pyclass(get_all)]
    #[derive(Decode, Encode, Clone, Debug)]
    struct StakeInfo {
        hotkey: AccountId,
        coldkey: AccountId,
        stake: Compact<u64>,
    }

    #[pydecode]
    #[pymethods]
    impl StakeInfo {
        #[pyo3(name = "decode_vec_tuple_vec")]
        #[staticmethod]
        fn py_decode_vec_tuple_vec(encoded: &[u8]) -> Vec<(AccountId, Vec<StakeInfo>)> {
            Vec::<(AccountId, Vec<StakeInfo>)>::decode(&mut &encoded[..])
                .expect("Failed to decode Vec<(AccountId, Vec<StakeInfo>)>")
        }
    }

    #[pyclass(get_all)]
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

    #[pydecode]
    #[pymethods]
    impl DelegateInfo {
        #[pyo3(name = "decode_delegated")]
        #[staticmethod]
        fn py_decode_delegated(encoded: &[u8]) -> Vec<(DelegateInfo, Compact<u64>)> {
            Vec::<(DelegateInfo, Compact<u64>)>::decode(&mut &encoded[..])
                .expect("Failed to decode Vec<(DelegateInfo, Compact<u64>)>")
        }
    }

    #[pyclass(name = "MetadataV15")]
    #[derive(Clone, Encode, Decode, Debug)]
    struct PyMetadataV15 {
        metadata: RuntimeMetadataV15,
    }

    #[pymethods]
    impl PyMetadataV15 {
        fn to_json(&self) -> String {
            serde_json::to_string(&self.metadata).unwrap()
        }

        #[staticmethod]
        fn decode_from_metadata_option(encoded_metadata_v15: &[u8]) -> Self {
            let option_vec = Option::<Vec<u8>>::decode(&mut &encoded_metadata_v15[..])
                .ok()
                .flatten()
                .expect("Failed to Option metadata");

            let metadata_v15 = RuntimeMetadataPrefixed::decode(&mut &option_vec[..])
                .expect("Failed to decode metadata")
                .1;

            match metadata_v15 {
                RuntimeMetadata::V15(metadata) => PyMetadataV15 { metadata },
                _ => panic!("Invalid metadata version"),
            }
        }

        #[pyo3(name = "value")]
        fn value(&self, py: Python) -> PyResult<Py<PyAny>> {
            let dict = pythonize::pythonize(py, &self.metadata)?;
            Ok(dict.into())
        }
    }

    #[pyclass(name = "PortableRegistry")]
    #[derive(Clone, Decode, Encode, Debug)]
    pub struct PyPortableRegistry {
        pub registry: scale_info::PortableRegistry,
    }

    #[pymethods]
    impl PyPortableRegistry {
        #[staticmethod]
        fn from_json(json: &str) -> Self {
            let registry: scale_info::PortableRegistry = serde_json::from_str(json).unwrap();
            PyPortableRegistry { registry }
        }

        #[getter]
        fn get_registry(&self) -> String {
            serde_json::to_string(&self.registry).unwrap()
        }

        #[staticmethod]
        fn from_metadata_v15(metadata: PyMetadataV15) -> Self {
            let registry = metadata.metadata.types;
            PyPortableRegistry { registry }
        }
    }

    fn composite_to_py_object(py: Python, value: Composite<u32>) -> PyResult<Py<PyAny>> {
        match value {
            Composite::Named(inner_) => {
                let dict = PyDict::new_bound(py);
                for (key, val) in inner_.iter() {
                    let val_py = value_to_pyobject(py, val.clone())?;
                    dict.set_item(key, val_py)?;
                }

                Ok(dict.to_object(py))
            }
            Composite::Unnamed(inner_) => {
                let tuple = PyTuple::new_bound(
                    py,
                    inner_
                        .iter()
                        .map(|val| value_to_pyobject(py, val.clone()))
                        .collect::<PyResult<Vec<Py<PyAny>>>>()?,
                );

                Ok(tuple.to_object(py))
            }
        }
    }

    fn value_to_pyobject(py: Python, value: Value<u32>) -> PyResult<Py<PyAny>> {
        match value.value {
            ValueDef::<u32>::Primitive(inner) => {
                let value = match inner {
                    Primitive::U128(value) => value.to_object(py),
                    Primitive::U256(value) => value.to_object(py),
                    Primitive::I128(value) => value.to_object(py),
                    Primitive::I256(value) => value.to_object(py),
                    Primitive::Bool(value) => value.to_object(py),
                    Primitive::Char(value) => value.to_object(py),
                    Primitive::String(value) => value.to_object(py),
                };

                Ok(value)
            }
            ValueDef::<u32>::BitSequence(inner) => {
                let value = inner.to_vec().to_object(py);

                Ok(value)
            }
            ValueDef::<u32>::Composite(inner) => {
                let value = composite_to_py_object(py, inner)?;

                Ok(value)
            }
            ValueDef::<u32>::Variant(inner) => {
                if inner.name == "None" || inner.name == "Some" {
                    match inner.name.as_str() {
                        "None" => Ok(py.None()),
                        "Some" => {
                            let some = composite_to_py_object(py, inner.values.clone())?;
                            if inner.values.len() == 1 {
                                let tuple = some
                                    .downcast_bound::<PyTuple>(py)
                                    .expect("Failed to downcast back to a tuple");
                                Ok(tuple
                                    .get_item(0)
                                    .expect("Failed to get item from tuple")
                                    .to_object(py))
                            } else {
                                Ok(some.to_object(py))
                            }
                        }
                        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                            "Invalid variant name: {} for Option",
                            inner.name
                        ))),
                    }
                } else {
                    let value = PyDict::new_bound(py);
                    value.set_item(
                        inner.name.clone(),
                        composite_to_py_object(py, inner.values)?,
                    )?;

                    Ok(value.to_object(py))
                }
            }
        }
    }

    fn py_isinstance(py: Python, value: &Py<PyAny>, type_name: &str) -> PyResult<bool> {
        let locals = PyDict::new_bound(py);
        locals.set_item("value", value)?;

        py.run_bound(
            &*format!("ret = isinstance(value, {})", type_name),
            None,
            Some(&locals),
        )
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Error checking isinstance of: {}: {:?}",
                type_name, e
            ))
        })?;
        let ret = locals.get_item("ret").unwrap().unwrap();
        let result = ret.extract::<bool>()?;

        Ok(result)
    }

    fn py_is_positive(py: Python, value: &Py<PyInt>) -> PyResult<bool> {
        let locals = PyDict::new_bound(py);
        locals.set_item("value", value)?;

        py.run_bound("ret = value >= 0", None, Some(&locals))
            .unwrap();
        let ret = locals.get_item("ret").unwrap().unwrap();
        let result = ret.extract::<bool>()?;

        Ok(result)
    }

    fn py_has_dict_method(py: Python, value: &Py<PyAny>) -> PyResult<bool> {
        let locals = PyDict::new_bound(py);
        locals.set_item("value", value)?;

        py.run_bound("ret = hasattr(value, \'__dict__\')", None, Some(&locals))
            .unwrap();
        let ret = locals.get_item("ret").unwrap().unwrap();
        let result = ret.extract::<bool>()?;

        Ok(result)
    }

    fn py_to_dict<'py>(py: Python<'py>, value: &Py<PyAny>) -> PyResult<Bound<'py, PyDict>> {
        let ret = value.call_method0(py, "__dict__")?;

        let result = ret.downcast_bound::<PyDict>(py)?;

        Ok(result.clone())
    }

    fn int_type_def_to_value(
        py: Python,
        py_int: &Py<PyInt>,
        ty: &scale_info::Type<PortableForm>,
        type_id: u32,
    ) -> PyResult<Value<u32>> {
        if py_is_positive(py, py_int)? {
            let value = py_int.extract::<u128>(py)?;
            match &ty.type_def {
                scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::U128) => {
                    let value =
                        Value::with_context(ValueDef::Primitive(Primitive::U128(value)), type_id);
                    return Ok(value);
                }
                scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::U64) => {
                    let value =
                        Value::with_context(ValueDef::Primitive(Primitive::U128(value)), type_id);
                    return Ok(value);
                }
                scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::U32) => {
                    let value =
                        Value::with_context(ValueDef::Primitive(Primitive::U128(value)), type_id);
                    return Ok(value);
                }
                scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::U16) => {
                    let value =
                        Value::with_context(ValueDef::Primitive(Primitive::U128(value)), type_id);
                    return Ok(value);
                }
                scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::U8) => {
                    let value =
                        Value::with_context(ValueDef::Primitive(Primitive::U128(value)), type_id);
                    return Ok(value);
                }
                _ => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid type for u128 data: {}",
                        value
                    )));
                }
            }
        } else {
            let value = py_int.extract::<i128>(py)?;
            match ty.type_def {
                scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::I128) => {
                    let value =
                        Value::with_context(ValueDef::Primitive(Primitive::I128(value)), type_id);
                    return Ok(value);
                }
                scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::I64) => {
                    let value =
                        Value::with_context(ValueDef::Primitive(Primitive::I128(value)), type_id);
                    return Ok(value);
                }
                scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::I32) => {
                    let value =
                        Value::with_context(ValueDef::Primitive(Primitive::I128(value)), type_id);
                    return Ok(value);
                }
                scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::I16) => {
                    let value =
                        Value::with_context(ValueDef::Primitive(Primitive::I128(value)), type_id);
                    return Ok(value);
                }
                scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::I8) => {
                    let value =
                        Value::with_context(ValueDef::Primitive(Primitive::I128(value)), type_id);
                    return Ok(value);
                }
                _ => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid type for i128 data: {}",
                        value
                    )));
                }
            }
        }
    }

    fn pylist_to_value(
        py: Python,
        py_list: &Bound<'_, PyList>,
        ty: &scale_info::Type<PortableForm>,
        type_id: u32,
        portable_registry: &PyPortableRegistry,
    ) -> PyResult<Value<u32>> {
        match &ty.type_def {
            scale_info::TypeDef::Array(inner) => {
                let ty_param = inner.type_param;
                let ty_param_id: u32 = ty_param.id;
                let ty_ = portable_registry
                    .registry
                    .resolve(ty_param_id)
                    .expect(&format!("Failed to resolve type (1): {:?}", ty_param));

                let items = py_list
                    .iter()
                    .map(|item| {
                        pyobject_to_value(
                            py,
                            item.as_any().as_unbound(),
                            &ty_,
                            ty_param_id,
                            portable_registry,
                        )
                    })
                    .collect::<PyResult<Vec<Value<u32>>>>()?;

                let value =
                    Value::with_context(ValueDef::Composite(Composite::Unnamed(items)), type_id);
                return Ok(value);
            }
            scale_info::TypeDef::Tuple(_inner) => {
                dbg!(_inner, py_list);
                let items = py_list
                    .iter()
                    .zip(_inner.fields.clone())
                    .map(|(item, ty_)| {
                        let ty_id: u32 = ty_.id;
                        let ty_ = portable_registry
                            .registry
                            .resolve(ty_id)
                            .expect(&format!("Failed to resolve type (1): {:?}", ty_));
                        pyobject_to_value(
                            py,
                            item.as_any().as_unbound(),
                            &ty_,
                            ty_id,
                            portable_registry,
                        )
                    })
                    .collect::<PyResult<Vec<Value<u32>>>>()?;

                let value =
                    Value::with_context(ValueDef::Composite(Composite::Unnamed(items)), type_id);
                return Ok(value);
            }
            scale_info::TypeDef::Sequence(inner) => {
                let ty_param = inner.type_param;
                let ty_param_id: u32 = ty_param.id;
                let ty_ = portable_registry
                    .registry
                    .resolve(ty_param_id)
                    .expect(&format!("Failed to resolve type (1): {:?}", ty_param));

                let items = py_list
                    .iter()
                    .map(|item| {
                        pyobject_to_value(
                            py,
                            item.as_any().as_unbound(),
                            &ty_,
                            ty_param_id,
                            portable_registry,
                        )
                    })
                    .collect::<PyResult<Vec<Value<u32>>>>()?;

                let value =
                    Value::with_context(ValueDef::Composite(Composite::Unnamed(items)), type_id);
                return Ok(value);
            }
            scale_info::TypeDef::Composite(TypeDefComposite { fields }) => {
                if fields.len() == 0 {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Unexpected 0 fields for unnamed composite type: {:?}",
                        ty
                    )));
                }

                let vals = fields
                    .iter()
                    .zip(py_list)
                    .map(|(field, item)| {
                        let ty_ = portable_registry
                            .registry
                            .resolve(field.ty.id)
                            .expect(&format!("Failed to resolve type for field: {:?}", field));

                        pyobject_to_value(
                            py,
                            item.as_any().as_unbound(),
                            ty_,
                            field.ty.id,
                            portable_registry,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<Value<u32>>>();

                let value =
                    Value::with_context(ValueDef::Composite(Composite::Unnamed(vals)), type_id);
                return Ok(value);
            }
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid type for a list of data: {}",
                    py_list
                )));
            }
        }
    }

    fn pyobject_to_value_no_option_check(
        py: Python,
        to_encode: &Py<PyAny>,
        ty: &scale_info::Type<PortableForm>,
        type_id: u32,
        portable_registry: &PyPortableRegistry,
    ) -> PyResult<Value<u32>> {
        if to_encode.is_none(py) {
            // If none and NOT option,
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid type for None: {:?}",
                ty.type_def
            )));
        }

        if py_isinstance(py, to_encode, "bool")? {
            let value = to_encode.extract::<bool>(py)?;

            match ty.type_def {
                scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::Bool) => {
                    let value =
                        Value::with_context(ValueDef::Primitive(Primitive::Bool(value)), type_id);
                    return Ok(value);
                }
                _ => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid type for bool data: {}",
                        value
                    )));
                }
            }
        } else if py_isinstance(py, to_encode, "str")? {
            if to_encode.extract::<char>(py).is_ok()
                && matches!(
                    ty.type_def,
                    scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::Char)
                )
            {
                let char_value = to_encode.extract::<char>(py)?;
                let value =
                    Value::with_context(ValueDef::Primitive(Primitive::Char(char_value)), type_id);

                Ok(value)
            } else if let Ok(str_value) = to_encode.extract::<String>(py) {
                match ty.type_def {
                    scale_info::TypeDef::Primitive(scale_info::TypeDefPrimitive::Str) => {
                        let value = Value::with_context(
                            ValueDef::Primitive(Primitive::String(str_value)),
                            type_id,
                        );
                        return Ok(value);
                    }
                    _ => {
                        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                            "Invalid type for string data: {}",
                            str_value
                        )));
                    }
                }
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid type for string data: {}",
                    to_encode
                )));
            }
        } else if py_isinstance(py, to_encode, "int")?
            && matches!(&ty.type_def, scale_info::TypeDef::Primitive(_))
        {
            let as_py_int = to_encode.downcast_bound::<PyInt>(py)?.as_unbound();

            return int_type_def_to_value(py, as_py_int, ty, type_id);
        } else if py_isinstance(py, to_encode, "int")?
            && matches!(&ty.type_def, scale_info::TypeDef::Compact(_))
        {
            // Must be a Compact int
            let as_py_int = to_encode.downcast_bound::<PyInt>(py)?.as_unbound();

            match &ty.type_def {
                scale_info::TypeDef::Compact(inner) => {
                    let inner_type_id = inner.type_param.id;
                    let inner_type_ = portable_registry.registry.resolve(inner_type_id);
                    if let Some(inner_type) = inner_type_ {
                        let mut inner_value =
                            int_type_def_to_value(py, as_py_int, inner_type, inner_type_id)?;
                        inner_value.context = type_id;
                        return Ok(inner_value);
                    }
                }
                _ => {}
            }

            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid type for u128 data: {}",
                to_encode
            )));
        } else if py_isinstance(py, to_encode, "tuple")? {
            let tuple_value = to_encode.downcast_bound::<PyTuple>(py)?;
            let as_list = tuple_value.to_list();

            pylist_to_value(py, &as_list, ty, type_id, portable_registry).map_err(|_e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid type for tuple data: {}",
                    tuple_value
                ))
            })
        } else if py_isinstance(py, to_encode, "list")? {
            let as_list = to_encode.downcast_bound::<PyList>(py)?;

            pylist_to_value(py, &as_list, ty, type_id, portable_registry).map_err(|_e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid type for list data: {}",
                    as_list
                ))
            })
        } else if py_isinstance(py, to_encode, "dict")? {
            let py_dict = to_encode.downcast_bound::<PyDict>(py)?;

            match &ty.type_def {
                scale_info::TypeDef::Composite(inner) => {
                    let fields = &inner.fields;
                    let mut dict: Vec<(String, Value<u32>)> = vec![];

                    for field in fields.iter() {
                        let field_name =
                            field.name.clone().ok_or(PyErr::new::<
                                pyo3::exceptions::PyValueError,
                                _,
                            >(format!(
                                "Invalid type for dict, type: {:?}",
                                ty.type_def
                            )))?;

                        let value_from_dict =
                            py_dict.get_item(field_name.clone())?.ok_or(PyErr::new::<
                                pyo3::exceptions::PyValueError,
                                _,
                            >(
                                format!(
                                "Invalid type for dict; missing field {}, type: {:?}",
                                field_name.clone(),
                                ty.type_def
                            )
                            ))?;

                        let inner_type =
                            portable_registry
                                .registry
                                .resolve(field.ty.id)
                                .expect(&format!(
                                    "Inner type: {:?} was not in registry after being registered",
                                    field.ty
                                ));

                        let as_value = pyobject_to_value(
                            py,
                            value_from_dict.as_unbound(),
                            inner_type,
                            field.ty.id,
                            portable_registry,
                        )?;

                        dict.push((field_name, as_value));
                    }

                    let value =
                        Value::with_context(ValueDef::Composite(Composite::Named(dict)), type_id);
                    return Ok(value);
                }
                _ => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid type for dict data: {}",
                        py_dict
                    )));
                }
            }

        //} else if let Ok(value) = to_encode.downcast_bound::<PyBytes>(py) {
        } else if py_has_dict_method(py, to_encode)? {
            // Convert object to dict
            let py_dict = py_to_dict(py, to_encode)?;

            match &ty.type_def {
                scale_info::TypeDef::Composite(TypeDefComposite { fields }) => {
                    let mut dict: Vec<(String, Value<u32>)> = vec![];

                    for field in fields.iter() {
                        let field_name =
                            field.name.clone().ok_or(PyErr::new::<
                                pyo3::exceptions::PyValueError,
                                _,
                            >(format!(
                                "Invalid type for dict, type: {:?}",
                                ty.type_def
                            )))?;

                        let value_from_dict =
                            py_dict.get_item(field_name.clone())?.ok_or(PyErr::new::<
                                pyo3::exceptions::PyValueError,
                                _,
                            >(
                                format!(
                                "Invalid type for dict; missing field {}, type: {:?}",
                                field_name.clone(),
                                ty.type_def
                            )
                            ))?;

                        let inner_type =
                            portable_registry
                                .registry
                                .resolve(field.ty.id)
                                .expect(&format!(
                                    "Inner type: {:?} was not in registry after being registered",
                                    field.ty
                                ));

                        let as_value = pyobject_to_value(
                            py,
                            value_from_dict.as_unbound(),
                            inner_type,
                            field.ty.id,
                            portable_registry,
                        )?;

                        dict.push((field_name, as_value));
                    }

                    let value =
                        Value::with_context(ValueDef::Composite(Composite::Named(dict)), type_id);
                    return Ok(value);
                }
                _ => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid type for dict data: {}",
                        to_encode
                    )));
                }
            }
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid type for data: {} of type {}",
                to_encode,
                to_encode.getattr(py, "__class__").unwrap_or(py.None())
            )));
        }
    }

    fn pyobject_to_value(
        py: Python,
        to_encode: &Py<PyAny>,
        ty: &scale_info::Type<PortableForm>,
        type_id: u32,
        portable_registry: &PyPortableRegistry,
    ) -> PyResult<Value<u32>> {
        // Check if the expected type is an option
        match ty.type_def.clone() {
            scale_info::TypeDef::Variant(inner) => {
                let is_option: bool = if inner.variants.len() == 2 {
                    let variant_names = inner
                        .variants
                        .iter()
                        .map(|v| &*v.name)
                        .collect::<Vec<&str>>();
                    variant_names.contains(&"Some") && variant_names.contains(&"None")
                } else {
                    false
                };

                if is_option {
                    if to_encode.is_none(py) {
                        // None
                        let none_variant: scale_value::Variant<u32> =
                            Variant::unnamed_fields("None", vec![]); // No fields because it's None

                        return Ok(Value::with_context(
                            ValueDef::Variant(none_variant),
                            type_id,
                        ));
                    } else {
                        // Some
                        // Get inner type
                        let inner_type_id: u32 = inner.variants[1].fields[0].ty.id;
                        let inner_type: &scale_info::Type<PortableForm> = portable_registry
                            .registry
                            .resolve(inner_type_id)
                            .ok_or(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                                "Could not find inner_type: {:?} for Option: {:?}",
                                ty.type_def, inner_type_id
                            )))?;
                        let inner_value: Value<u32> = pyobject_to_value_no_option_check(
                            py,
                            to_encode,
                            inner_type,
                            inner_type_id,
                            portable_registry,
                        )?;
                        let some_variant: scale_value::Variant<u32> =
                            Variant::unnamed_fields("Some", vec![inner_value]); // No fields because it's None

                        return Ok(Value::with_context(
                            ValueDef::Variant(some_variant),
                            type_id,
                        ));
                    }
                } // else: Regular conversion
            }
            _ => {} // Regular conversion
        }

        return pyobject_to_value_no_option_check(py, to_encode, ty, type_id, portable_registry);
    }

    #[pyfunction(name = "decode")]
    fn py_decode(
        py: Python,
        type_string: &str,
        portable_registry: &PyPortableRegistry,
        encoded: &[u8],
    ) -> PyResult<Py<PyAny>> {
        // Create a memoization table for the type string to type id conversion
        let mut memo = HashMap::<String, u32>::new();

        let mut curr_registry = portable_registry.registry.clone();

        fill_memo_using_well_known_types(&mut memo, &curr_registry);

        let type_id: u32 = get_type_id_from_type_string(&mut memo, type_string, &mut curr_registry)
            .expect("Failed to get type id from type string");

        let decoded =
            decode_as_type(&mut &encoded[..], type_id, &curr_registry).expect("Failed to decode");

        value_to_pyobject(py, decoded)
    }

    #[pyfunction(name = "encode")]
    fn py_encode(
        py: Python,
        type_string: &str,
        portable_registry: &PyPortableRegistry,
        to_encode: Py<PyAny>,
    ) -> PyResult<Vec<u8>> {
        // Create a memoization table for the type string to type id conversion
        let mut memo = HashMap::<String, u32>::new();

        let mut curr_registry = portable_registry.registry.clone();

        fill_memo_using_well_known_types(&mut memo, &curr_registry);

        let type_id: u32 = get_type_id_from_type_string(&mut memo, type_string, &mut curr_registry)
            .expect("Failed to get type id from type string");

        let ty = curr_registry
            .resolve(type_id)
            .expect(&format!("Failed to resolve type (0): {:?}", type_string));

        let as_value: Value<u32> =
            pyobject_to_value(py, &to_encode, ty, type_id, portable_registry)?;

        let mut encoded: Vec<u8> = Vec::<u8>::new();
        encode_as_type(&as_value, type_id, &curr_registry, &mut encoded).expect("Failed to encode");

        Ok(encoded)
    }
}
