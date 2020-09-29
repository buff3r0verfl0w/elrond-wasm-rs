#![feature(prelude_import)]
#![no_std]
#![allow(unused_imports)] // for now
#![allow(unused_mut)]
#[prelude_import]
use core::prelude::v1::*;
#[macro_use]
extern crate core;
use elrond_wasm::{Box, Vec, String, Queue, VarArgs, BorrowedMutStorage};
use elrond_wasm::{SCError, SCResult, SCResult::Ok, SCResult::Err};
use elrond_wasm::{H256, Address, ErrorMessage};
use elrond_wasm::{
    ContractHookApi, ContractIOApi, BigIntApi, BigUintApi, OtherContractHandle, AsyncCallResult,
    AsyncCallError, ContractFactory, ContractFactoryImpl
};
use elrond_wasm::elrond_codec::{Encode, Decode, DecodeError};
use elrond_wasm::io::*;
use elrond_wasm::non_zero_util::*;
use elrond_wasm::err_msg;
use core::ops::{Add, Sub, Mul, Div, Rem};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use core::ops::{BitAnd, BitOr, BitXor, Shr, Shl};
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShrAssign, ShlAssign};
#[macro_use]
extern crate elrond_wasm;
pub trait Adder<'t, T, BigInt, BigUint>: ContractHookApi<BigInt, BigUint> + Sized
where
    BigUint: BigUintApi + 'static,
    for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: AddAssign<&'b BigUint>,
    for<'b> BigUint: SubAssign<&'b BigUint>,
    for<'b> BigUint: MulAssign<&'b BigUint>,
    for<'b> BigUint: DivAssign<&'b BigUint>,
    for<'b> BigUint: RemAssign<&'b BigUint>,
    for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: BitAndAssign<&'b BigUint>,
    for<'b> BigUint: BitOrAssign<&'b BigUint>,
    for<'b> BigUint: BitXorAssign<&'b BigUint>,
    for<'a> &'a BigUint: Shr<usize, Output = BigUint>,
    for<'a> &'a BigUint: Shl<usize, Output = BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output = BigInt>,
    for<'b> BigInt: AddAssign<&'b BigInt>,
    for<'b> BigInt: SubAssign<&'b BigInt>,
    for<'b> BigInt: MulAssign<&'b BigInt>,
    for<'b> BigInt: DivAssign<&'b BigInt>,
    for<'b> BigInt: RemAssign<&'b BigInt>,
    T: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint>,
{
    fn init(&mut self, initial_value: &BigInt) {
        self.set_sum(initial_value);
    }
    fn add(&mut self, value: &BigInt) -> SCResult<()> {
        let mut sum = self.get_sum();
        sum += value;
        self.set_sum(&sum);
        Ok(())
    }
    fn get_sum(&self) -> BigInt;
    fn set_sum(&mut self, sum: &BigInt);
    fn contract_proxy(&self, address: &Address) -> Box<OtherContractHandle<T, BigInt, BigUint>>;
    fn callback(&self);
}
pub struct AdderImpl<'t, T, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: AddAssign<&'b BigUint>,
    for<'b> BigUint: SubAssign<&'b BigUint>,
    for<'b> BigUint: MulAssign<&'b BigUint>,
    for<'b> BigUint: DivAssign<&'b BigUint>,
    for<'b> BigUint: RemAssign<&'b BigUint>,
    for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: BitAndAssign<&'b BigUint>,
    for<'b> BigUint: BitOrAssign<&'b BigUint>,
    for<'b> BigUint: BitXorAssign<&'b BigUint>,
    for<'a> &'a BigUint: Shr<usize, Output = BigUint>,
    for<'a> &'a BigUint: Shl<usize, Output = BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output = BigInt>,
    for<'b> BigInt: AddAssign<&'b BigInt>,
    for<'b> BigInt: SubAssign<&'b BigInt>,
    for<'b> BigInt: MulAssign<&'b BigInt>,
    for<'b> BigInt: DivAssign<&'b BigInt>,
    for<'b> BigInt: RemAssign<&'b BigInt>,
    T: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint>,
{
    api: &'t mut T,
    _phantom1: core::marker::PhantomData<BigInt>,
    _phantom2: core::marker::PhantomData<BigUint>,
}



impl<'t, T, BigInt, BigUint> AdderImpl<'t, T, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: AddAssign<&'b BigUint>,
    for<'b> BigUint: SubAssign<&'b BigUint>,
    for<'b> BigUint: MulAssign<&'b BigUint>,
    for<'b> BigUint: DivAssign<&'b BigUint>,
    for<'b> BigUint: RemAssign<&'b BigUint>,
    for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: BitAndAssign<&'b BigUint>,
    for<'b> BigUint: BitOrAssign<&'b BigUint>,
    for<'b> BigUint: BitXorAssign<&'b BigUint>,
    for<'a> &'a BigUint: Shr<usize, Output = BigUint>,
    for<'a> &'a BigUint: Shl<usize, Output = BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output = BigInt>,
    for<'b> BigInt: AddAssign<&'b BigInt>,
    for<'b> BigInt: SubAssign<&'b BigInt>,
    for<'b> BigInt: MulAssign<&'b BigInt>,
    for<'b> BigInt: DivAssign<&'b BigInt>,
    for<'b> BigInt: RemAssign<&'b BigInt>,
    T: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + 'static,
{
    pub fn new(api: &'t mut T) -> Self {
        AdderImpl {
            api,
            _phantom1: core::marker::PhantomData,
            _phantom2: core::marker::PhantomData,
        }
    }
}

#[derive(Clone, Copy)]
pub struct AdderFactory<T, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: AddAssign<&'b BigUint>,
    for<'b> BigUint: SubAssign<&'b BigUint>,
    for<'b> BigUint: MulAssign<&'b BigUint>,
    for<'b> BigUint: DivAssign<&'b BigUint>,
    for<'b> BigUint: RemAssign<&'b BigUint>,
    for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: BitAndAssign<&'b BigUint>,
    for<'b> BigUint: BitOrAssign<&'b BigUint>,
    for<'b> BigUint: BitXorAssign<&'b BigUint>,
    for<'a> &'a BigUint: Shr<usize, Output = BigUint>,
    for<'a> &'a BigUint: Shl<usize, Output = BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output = BigInt>,
    for<'b> BigInt: AddAssign<&'b BigInt>,
    for<'b> BigInt: SubAssign<&'b BigInt>,
    for<'b> BigInt: MulAssign<&'b BigInt>,
    for<'b> BigInt: DivAssign<&'b BigInt>,
    for<'b> BigInt: RemAssign<&'b BigInt>,
    T: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint>,
{
    _phantom0: core::marker::PhantomData<T>,
    _phantom1: core::marker::PhantomData<BigInt>,
    _phantom2: core::marker::PhantomData<BigUint>,
}


impl<A, BigInt, BigUint> AdderFactory<A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: AddAssign<&'b BigUint>,
    for<'b> BigUint: SubAssign<&'b BigUint>,
    for<'b> BigUint: MulAssign<&'b BigUint>,
    for<'b> BigUint: DivAssign<&'b BigUint>,
    for<'b> BigUint: RemAssign<&'b BigUint>,
    for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: BitAndAssign<&'b BigUint>,
    for<'b> BigUint: BitOrAssign<&'b BigUint>,
    for<'b> BigUint: BitXorAssign<&'b BigUint>,
    for<'a> &'a BigUint: Shr<usize, Output = BigUint>,
    for<'a> &'a BigUint: Shl<usize, Output = BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output = BigInt>,
    for<'b> BigInt: AddAssign<&'b BigInt>,
    for<'b> BigInt: SubAssign<&'b BigInt>,
    for<'b> BigInt: MulAssign<&'b BigInt>,
    for<'b> BigInt: DivAssign<&'b BigInt>,
    for<'b> BigInt: RemAssign<&'b BigInt>,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint>,
{
    pub fn new() -> Self {
        AdderFactory {
            _phantom0: core::marker::PhantomData,
            _phantom1: core::marker::PhantomData,
            _phantom2: core::marker::PhantomData,
        }
    }
}
impl<A, BigInt, BigUint> ContractFactory<A> for AdderFactory<A, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: AddAssign<&'b BigUint>,
    for<'b> BigUint: SubAssign<&'b BigUint>,
    for<'b> BigUint: MulAssign<&'b BigUint>,
    for<'b> BigUint: DivAssign<&'b BigUint>,
    for<'b> BigUint: RemAssign<&'b BigUint>,
    for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: BitAndAssign<&'b BigUint>,
    for<'b> BigUint: BitOrAssign<&'b BigUint>,
    for<'b> BigUint: BitXorAssign<&'b BigUint>,
    for<'a> &'a BigUint: Shr<usize, Output = BigUint>,
    for<'a> &'a BigUint: Shl<usize, Output = BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output = BigInt>,
    for<'b> BigInt: AddAssign<&'b BigInt>,
    for<'b> BigInt: SubAssign<&'b BigInt>,
    for<'b> BigInt: MulAssign<&'b BigInt>,
    for<'b> BigInt: DivAssign<&'b BigInt>,
    for<'b> BigInt: RemAssign<&'b BigInt>,
    A: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint>,
{
    fn new_contract<'t>(&self, state_ref: &'t mut A) -> Box<dyn CallableContract + 't> {
        Box::new(AdderImpl {
            api: state_ref,
            _phantom1: core::marker::PhantomData,
            _phantom2: core::marker::PhantomData,
        })
    }
}
impl<'t, T, BigInt, BigUint> ContractHookApi<BigInt, BigUint> for AdderImpl<'t, T, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: AddAssign<&'b BigUint>,
    for<'b> BigUint: SubAssign<&'b BigUint>,
    for<'b> BigUint: MulAssign<&'b BigUint>,
    for<'b> BigUint: DivAssign<&'b BigUint>,
    for<'b> BigUint: RemAssign<&'b BigUint>,
    for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: BitAndAssign<&'b BigUint>,
    for<'b> BigUint: BitOrAssign<&'b BigUint>,
    for<'b> BigUint: BitXorAssign<&'b BigUint>,
    for<'a> &'a BigUint: Shr<usize, Output = BigUint>,
    for<'a> &'a BigUint: Shl<usize, Output = BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output = BigInt>,
    for<'b> BigInt: AddAssign<&'b BigInt>,
    for<'b> BigInt: SubAssign<&'b BigInt>,
    for<'b> BigInt: MulAssign<&'b BigInt>,
    for<'b> BigInt: DivAssign<&'b BigInt>,
    for<'b> BigInt: RemAssign<&'b BigInt>,
    T: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint>,
{
    #[inline]
    fn get_sc_address(&self) -> Address {
        self.api.get_sc_address()
    }
    #[inline]
    fn get_owner_address(&self) -> Address {
        self.api.get_owner_address()
    }
    #[inline]
    fn get_caller(&self) -> Address {
        self.api.get_caller()
    }
    #[inline]
    fn get_balance(&self, address: &Address) -> BigUint {
        self.api.get_balance(address)
    }
    #[inline]
    fn storage_store(&mut self, key: &[u8], value: &[u8]) {
        self.api.storage_store(key, value);
    }
    #[inline]
    fn storage_load(&self, key: &[u8]) -> Vec<u8> {
        self.api.storage_load(key)
    }
    #[inline]
    fn storage_load_len(&self, key: &[u8]) -> usize {
        self.api.storage_load_len(key)
    }
    #[inline]
    fn storage_store_bytes32(&mut self, key: &[u8], value: &[u8; 32]) {
        self.api.storage_store_bytes32(key, value);
    }
    #[inline]
    fn storage_load_bytes32(&self, key: &[u8]) -> [u8; 32] {
        self.api.storage_load_bytes32(key)
    }
    #[inline]
    fn storage_store_big_uint(&mut self, key: &[u8], value: &BigUint) {
        self.api.storage_store_big_uint(key, value);
    }
    #[inline]
    fn storage_load_big_uint(&self, key: &[u8]) -> BigUint {
        self.api.storage_load_big_uint(key)
    }
    #[inline]
    fn storage_store_big_int(&mut self, key: &[u8], value: &BigInt) {
        self.api.storage_store_big_int(key, value);
    }
    #[inline]
    fn storage_load_big_int(&self, key: &[u8]) -> BigInt {
        self.api.storage_load_big_int(key)
    }
    #[inline]
    fn storage_store_i64(&mut self, key: &[u8], value: i64) {
        self.api.storage_store_i64(key, value);
    }
    #[inline]
    fn storage_load_i64(&self, key: &[u8]) -> Option<i64> {
        self.api.storage_load_i64(key)
    }
    #[inline]
    fn get_call_value_big_uint(&self) -> BigUint {
        self.api.get_call_value_big_uint()
    }
    #[inline]
    fn send_tx(&mut self, to: &Address, amount: &BigUint, message: &str) {
        self.api.send_tx(to, amount, message);
    }
    #[inline]
    fn async_call(&self, to: &Address, amount: &BigUint, data: &[u8]) {
        self.api.async_call(to, amount, data);
    }
    #[inline]
    fn get_tx_hash(&self) -> H256 {
        self.api.get_tx_hash()
    }
    #[inline]
    fn get_gas_left(&self) -> i64 {
        self.api.get_gas_left()
    }
    #[inline]
    fn get_block_timestamp(&self) -> u64 {
        self.api.get_block_timestamp()
    }
    #[inline]
    fn get_block_nonce(&self) -> u64 {
        self.api.get_block_nonce()
    }
    #[inline]
    fn get_block_round(&self) -> u64 {
        self.api.get_block_round()
    }
    #[inline]
    fn get_block_epoch(&self) -> u64 {
        self.api.get_block_epoch()
    }
    #[inline]
    fn sha256(&self, data: &[u8]) -> [u8; 32] {
        self.api.sha256(data)
    }
    #[inline]
    fn keccak256(&self, data: &[u8]) -> [u8; 32] {
        self.api.keccak256(data)
    }
}
impl<'t, T, BigInt, BigUint> Adder<'t, T, BigInt, BigUint> for AdderImpl<'t, T, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: AddAssign<&'b BigUint>,
    for<'b> BigUint: SubAssign<&'b BigUint>,
    for<'b> BigUint: MulAssign<&'b BigUint>,
    for<'b> BigUint: DivAssign<&'b BigUint>,
    for<'b> BigUint: RemAssign<&'b BigUint>,
    for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: BitAndAssign<&'b BigUint>,
    for<'b> BigUint: BitOrAssign<&'b BigUint>,
    for<'b> BigUint: BitXorAssign<&'b BigUint>,
    for<'a> &'a BigUint: Shr<usize, Output = BigUint>,
    for<'a> &'a BigUint: Shl<usize, Output = BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output = BigInt>,
    for<'b> BigInt: AddAssign<&'b BigInt>,
    for<'b> BigInt: SubAssign<&'b BigInt>,
    for<'b> BigInt: MulAssign<&'b BigInt>,
    for<'b> BigInt: DivAssign<&'b BigInt>,
    for<'b> BigInt: RemAssign<&'b BigInt>,
    T: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint>,
{
    fn get_sum(&self) -> BigInt {
        let key: &'static [u8] = &[115u8, 117u8, 109u8];
        elrond_wasm::storage_get(self.api, &key[..])
    }
    fn set_sum(&mut self, sum: &BigInt) {
        let key: &'static [u8] = &[115u8, 117u8, 109u8];
        elrond_wasm::storage_set(self.api, &key[..], &sum);
    }
    fn contract_proxy(&self, address: &Address) -> Box<OtherContractHandle<T, BigInt, BigUint>> {
        // let contract_proxy = OtherContractHandle::new(self.api.clone(), address);
        // Box::new(contract_proxy)
        panic!("contract_proxy")
    }
    fn callback(&self) {}
}
impl<'t, T, BigInt, BigUint> AdderImpl<'t, T, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: AddAssign<&'b BigUint>,
    for<'b> BigUint: SubAssign<&'b BigUint>,
    for<'b> BigUint: MulAssign<&'b BigUint>,
    for<'b> BigUint: DivAssign<&'b BigUint>,
    for<'b> BigUint: RemAssign<&'b BigUint>,
    for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: BitAndAssign<&'b BigUint>,
    for<'b> BigUint: BitOrAssign<&'b BigUint>,
    for<'b> BigUint: BitXorAssign<&'b BigUint>,
    for<'a> &'a BigUint: Shr<usize, Output = BigUint>,
    for<'a> &'a BigUint: Shl<usize, Output = BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output = BigInt>,
    for<'b> BigInt: AddAssign<&'b BigInt>,
    for<'b> BigInt: SubAssign<&'b BigInt>,
    for<'b> BigInt: MulAssign<&'b BigInt>,
    for<'b> BigInt: DivAssign<&'b BigInt>,
    for<'b> BigInt: RemAssign<&'b BigInt>,
    T: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint>,
{
    #[inline]
    fn call_get_sum(&mut self) {
        self.api.check_not_payable();
        if !self.api.check_num_arguments(0i32) {
            return;
        }
        let result = self.get_sum();
        EndpointResult::<T, BigInt, BigUint>::finish(&result, &mut self.api);
    }
    #[inline]
    fn call_init(&mut self) {
        self.api.check_not_payable();
        if !self.api.check_num_arguments(1i32) {
            return;
        }
        let initial_value = &elrond_wasm::load_single_arg::<T, BigInt, BigUint, BigInt>(
            &self.api,
            0i32,
            &[
                105u8, 110u8, 105u8, 116u8, 105u8, 97u8, 108u8, 95u8, 118u8, 97u8, 108u8, 117u8,
                101u8,
            ][..],
        );
        self.init(&initial_value);
    }
    #[inline]
    fn call_add(&mut self) {
        self.api.check_not_payable();
        if !self.api.check_num_arguments(1i32) {
            return;
        }
        let value = &elrond_wasm::load_single_arg::<T, BigInt, BigUint, BigInt>(
            &self.api,
            0i32,
            &[118u8, 97u8, 108u8, 117u8, 101u8][..],
        );
        let result = self.add(&value);
        EndpointResult::<T, BigInt, BigUint>::finish(&result, &mut self.api);
    }
}
#[allow(non_snake_case)]
pub mod endpoints {
    use super::*;
    use elrond_wasm_node::*;
    // fn new_arwen_instance() -> AdderImpl<'static, ArwenApiImpl, ArwenBigInt, ArwenBigUint> {
    //     let mut api = ArwenApiImpl {};
    //     AdderImpl::new(&mut api)
    // }
    #[no_mangle]
    pub fn getSum() {
        let mut api = ArwenApiImpl {};
        let mut inst = AdderImpl::new(&mut api);
        // let inst = new_arwen_instance();
        inst.call_get_sum();
    }
    #[no_mangle]
    pub fn init() {
        let mut api = ArwenApiImpl {};
        let mut inst = AdderImpl::new(&mut api);
        // let inst = new_arwen_instance();
        inst.call_init();
    }
    #[no_mangle]
    pub fn add() {
        let mut api = ArwenApiImpl {};
        let mut inst = AdderImpl::new(&mut api);
        // let inst = new_arwen_instance();
        inst.call_add();
    }
}
use elrond_wasm::CallableContract;
impl<'t, T, BigInt, BigUint> CallableContract for AdderImpl<'t, T, BigInt, BigUint>
where
    BigUint: BigUintApi + 'static,
    for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: AddAssign<&'b BigUint>,
    for<'b> BigUint: SubAssign<&'b BigUint>,
    for<'b> BigUint: MulAssign<&'b BigUint>,
    for<'b> BigUint: DivAssign<&'b BigUint>,
    for<'b> BigUint: RemAssign<&'b BigUint>,
    for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output = BigUint>,
    for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output = BigUint>,
    for<'b> BigUint: BitAndAssign<&'b BigUint>,
    for<'b> BigUint: BitOrAssign<&'b BigUint>,
    for<'b> BigUint: BitXorAssign<&'b BigUint>,
    for<'a> &'a BigUint: Shr<usize, Output = BigUint>,
    for<'a> &'a BigUint: Shl<usize, Output = BigUint>,
    BigInt: BigIntApi<BigUint> + 'static,
    for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output = BigInt>,
    for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output = BigInt>,
    for<'b> BigInt: AddAssign<&'b BigInt>,
    for<'b> BigInt: SubAssign<&'b BigInt>,
    for<'b> BigInt: MulAssign<&'b BigInt>,
    for<'b> BigInt: DivAssign<&'b BigInt>,
    for<'b> BigInt: RemAssign<&'b BigInt>,
    T: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint>,
{
    fn call(&mut self, fn_name: &[u8]) {
        match fn_name {
            [103u8, 101u8, 116u8, 83u8, 117u8, 109u8] => {
                self.call_get_sum();
            }
            [105u8, 110u8, 105u8, 116u8] => {
                self.call_init();
            }
            [97u8, 100u8, 100u8] => {
                self.call_add();
            }
            _other => panic!("No function with this name exists in contract."),
        }
    }
    fn clone_contract(&self) -> Box<dyn CallableContract> {
        // Box::new(AdderImpl::new(self.api.clone()))
        panic!("clone_contract")
    }
}
pub mod callback_endpoint {
    use super::*;
    use elrond_wasm_node::*;
    // fn new_arwen_instance() -> AdderImpl<'static, ArwenApiImpl, ArwenBigInt, ArwenBigUint> {
    //     let api = ArwenApiImpl {};
    //     AdderImpl::new(&mut api)
    // }
    #[no_mangle]
    #[allow(non_snake_case)]
    pub fn callBack() {
        let mut api = ArwenApiImpl {};
        let mut inst = AdderImpl::new(&mut api);
        // let inst = new_arwen_instance();
        inst.callback();
    }
}
