#![feature(generic_associated_types)]

use elrond_wasm::{
    api::ManagedTypeApi,
    derive::ManagedVecItem,
    elrond_codec,
    elrond_codec::elrond_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    types::{BigUint, EsdtTokenPayment, ManagedByteArray, ManagedType, TokenIdentifier},
};
use elrond_wasm_debug::DebugApi;

// to test, run the following command in elrond-wasm-debug folder:
// cargo expand --test derive_managed_vec_item_esdt_token_payment_test > expanded.rs

const ETH_ADDR_WIDTH: usize = 20;

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Clone, Debug,
)]
pub struct ManagedStructWithToken<M: ManagedTypeApi> {
    pub token: elrond_wasm::types::EsdtTokenPayment<M>,
    pub num: u32,
    pub eth_address_1: ManagedByteArray<M, ETH_ADDR_WIDTH>,
    pub eth_address_2: ManagedByteArray<M, 20>, // const generic also works
}

#[test]
fn struct_with_numbers_static() {
    assert_eq!(
        <ManagedStructWithToken<DebugApi> as elrond_wasm::types::ManagedVecItem>::PAYLOAD_SIZE,
        28
    );
    assert!(
        !<ManagedStructWithToken<DebugApi> as elrond_wasm::types::ManagedVecItem>::SKIPS_RESERIALIZATION
    );
}

#[test]
fn struct_to_bytes_writer() {
    let _ = DebugApi::dummy();
    let s = ManagedStructWithToken::<DebugApi> {
        token: EsdtTokenPayment {
            token_identifier: TokenIdentifier::from(&b"MYTOKEN-12345"[..]),
            token_nonce: 0u64,
            token_type: elrond_wasm::types::EsdtTokenType::Fungible,
            amount: BigUint::from(42u64),
        },
        num: 0x12345,
        eth_address_1: ManagedByteArray::new_from_bytes(&[1u8; 20]),
        eth_address_2: ManagedByteArray::new_from_bytes(&[2u8; 20]),
    };
    let mut arr: [u8; 28] = [0u8;
        <ManagedStructWithToken<DebugApi> as elrond_wasm::types::ManagedVecItem>::PAYLOAD_SIZE];

    let handle1 = s.token.token_identifier.get_raw_handle().to_be_bytes();
    let handle2 = s.token.amount.get_raw_handle().to_be_bytes();
    let handle3 = s.eth_address_1.get_raw_handle().to_be_bytes();
    let handle4 = s.eth_address_2.get_raw_handle().to_be_bytes();
    let expected = [
        0xff, 0xff, 0xff, handle1[3], 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xff, handle2[3], 0x00, 0x01, 0x23, 0x45, 0xff, 0xff, 0xff, handle3[3], 0xff, 0xff, 0xff,
        handle4[3],
    ];

    <ManagedStructWithToken<DebugApi> as elrond_wasm::types::ManagedVecItem>::to_byte_writer(
        &s,
        |bytes| {
            arr[0..<ManagedStructWithToken::<DebugApi> as elrond_wasm::types::ManagedVecItem>::PAYLOAD_SIZE].copy_from_slice(bytes);

            assert_eq!(arr, expected);
        },
    );
}

#[test]
fn struct_from_bytes_reader() {
    let _ = DebugApi::dummy();
    let s = ManagedStructWithToken::<DebugApi> {
        token: EsdtTokenPayment {
            token_identifier: TokenIdentifier::from(&b"MYTOKEN-12345"[..]),
            token_nonce: 0u64,
            token_type: elrond_wasm::types::EsdtTokenType::Fungible,
            amount: 42u64.into(),
        },
        num: 0x12345,
        eth_address_1: ManagedByteArray::new_from_bytes(&[1u8; 20]),
        eth_address_2: ManagedByteArray::new_from_bytes(&[2u8; 20]),
    };

    let handle1 = s.token.token_identifier.get_raw_handle().to_be_bytes();
    let handle2 = s.token.amount.get_raw_handle().to_be_bytes();
    let handle3 = s.eth_address_1.get_raw_handle().to_be_bytes();
    let handle4 = s.eth_address_2.get_raw_handle().to_be_bytes();
    let arr: [u8; 28] = [
        0xff, 0xff, 0xff, handle1[3], 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xff, handle2[3], 0x00, 0x01, 0x23, 0x45, 0xff, 0xff, 0xff, handle3[3], 0xff, 0xff, 0xff,
        handle4[3],
    ];

    let struct_from_bytes =
        <ManagedStructWithToken<DebugApi> as elrond_wasm::types::ManagedVecItem>::from_byte_reader(
            |bytes| {
                bytes.copy_from_slice(
                    &arr
                        [0
                            ..<ManagedStructWithToken::<DebugApi> as elrond_wasm::types::ManagedVecItem>::PAYLOAD_SIZE],
                );
            },
        );

    assert_eq!(s, struct_from_bytes);
}
