use alloc::string::String;
use elrond_codec::Vec;

use crate::{
    abi::{TypeAbi, TypeDescriptionContainer},
    api::{EndpointFinishApi, ManagedTypeApi},
    ArgId, ContractCallArg, DynArg, DynArgInput, DynArgOutput, EndpointResult,
};

use super::{ManagedFrom, ManagedInto, ManagedVec, ManagedVecItem, ManagedVecIterator};

pub struct ManagedMultiResultVecEager<M: ManagedTypeApi, T: ManagedVecItem<M>>(ManagedVec<M, T>);

pub type ManagedVarArgsEager<M, T> = ManagedMultiResultVecEager<M, T>;

impl<M, T> From<ManagedVec<M, T>> for ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[inline]
    fn from(managed_vec: ManagedVec<M, T>) -> Self {
        ManagedMultiResultVecEager(managed_vec)
    }
}

impl<M, T> ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    #[inline]
    pub fn new(api: M) -> Self {
        ManagedMultiResultVecEager(ManagedVec::new(api))
    }

    #[inline]
    pub fn byte_len(&self) -> usize {
        self.0.byte_len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<T> {
        self.0.get(index)
    }

    pub fn slice(&self, start_index: usize, end_index: usize) -> Option<Self> {
        self.0
            .slice(start_index, end_index)
            .map(|value| Self(value))
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item)
    }

    pub fn from_single_item(api: M, item: T) -> Self {
        let mut result = ManagedMultiResultVecEager::new(api);
        result.push(item);
        result
    }

    pub fn overwrite_with_single_item(&mut self, item: T) {
        self.0.overwrite_with_single_item(item)
    }

    pub fn append_vec(&mut self, item: ManagedMultiResultVecEager<M, T>) {
        self.0.append_vec(item.0)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn into_vec(self) -> ManagedVec<M, T> {
        self.0
    }

    pub fn with_self_as_vec<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Vec<T>),
    {
        self.0.with_self_as_vec(f)
    }

    pub fn iter(&self) -> ManagedVecIterator<M, T> {
        ManagedVecIterator::new(&self.0)
    }
}

impl<M, T, I> ManagedFrom<M, Vec<I>> for ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
    I: ManagedInto<M, T>,
{
    fn managed_from(api: M, v: Vec<I>) -> Self {
        let mut result = Self::new(api.clone());
        for item in v.into_iter() {
            result.push(item.managed_into(api.clone()));
        }
        result
    }
}
impl<M, T> DynArg for ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M> + DynArg,
{
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        let mut result_vec: ManagedVec<M, T> = ManagedVec::new(loader.vm_api_cast());
        while loader.has_next() {
            result_vec.push(T::dyn_load(loader, arg_id));
        }
        ManagedMultiResultVecEager(result_vec)
    }
}

impl<M, T> EndpointResult for ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M> + EndpointResult,
{
    type DecodeAs = ManagedMultiResultVecEager<M, T>;

    #[inline]
    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        for elem in self.0.iter() {
            elem.finish(api.clone());
        }
    }
}
impl<M, T> ContractCallArg for &ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M> + ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        for elem in self.0.iter() {
            elem.push_dyn_arg(output);
        }
    }
}

impl<M, T> ContractCallArg for ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M> + ContractCallArg,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        (&self).push_dyn_arg(output)
    }
}

impl<M, T: TypeAbi> TypeAbi for ManagedMultiResultVecEager<M, T>
where
    M: ManagedTypeApi,
    T: ManagedVecItem<M>,
{
    fn type_name() -> String {
        let mut repr = String::from("variadic<");
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_multi_arg_or_result() -> bool {
        true
    }
}