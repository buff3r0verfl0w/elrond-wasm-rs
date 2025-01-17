use super::{ErrorApiImpl, Handle, ManagedTypeApiImpl};
use crate::types::EsdtTokenType;

pub trait CallValueApi {
    type CallValueApiImpl: CallValueApiImpl;

    fn call_value_api_impl() -> Self::CallValueApiImpl;
}

pub trait CallValueApiImpl: ErrorApiImpl + ManagedTypeApiImpl + Sized {
    fn check_not_payable(&self);

    /// Retrieves the EGLD call value from the VM.
    /// Will return 0 in case of an ESDT transfer (cannot have both EGLD and ESDT transfer simultaneously).
    fn load_egld_value(&self, dest_handle: Handle);

    /// Loads all ESDT call values into a managed vec. Overwrites destination.
    fn load_all_esdt_transfers(&self, dest_handle: Handle) {
        load_all_esdt_transfers_from_unmanaged(self, dest_handle);
    }

    fn esdt_num_transfers(&self) -> usize;

    /// Retrieves the ESDT call value from the VM.
    /// Will return 0 in case of an EGLD transfer (cannot have both EGLD and ESDT transfer simultaneously).
    fn load_single_esdt_value(&self, dest_handle: Handle);

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for EGLD,
    /// but the EGLD TokenIdentifier is serialized as `EGLD`.
    fn token(&self) -> Handle;

    /// Returns the nonce of the received ESDT token.
    /// Will return 0 in case of EGLD or fungible ESDT transfer.
    fn esdt_token_nonce(&self) -> u64;

    /// Returns the ESDT token type.
    /// Will return "Fungible" for EGLD.
    fn esdt_token_type(&self) -> EsdtTokenType;

    fn esdt_value_by_index(&self, index: usize) -> Handle;

    fn token_by_index(&self, index: usize) -> Handle;

    fn esdt_token_nonce_by_index(&self, index: usize) -> u64;

    fn esdt_token_type_by_index(&self, index: usize) -> EsdtTokenType;
}

pub fn load_all_esdt_transfers_from_unmanaged<A>(api: &A, dest_handle: Handle)
where
    A: CallValueApiImpl,
{
    let num_transfers = api.esdt_num_transfers();
    api.mb_overwrite(dest_handle, &[]);

    for i in 0..num_transfers {
        let token_identifier_handle = api.token_by_index(i);
        let token_nonce = api.esdt_token_nonce_by_index(i);
        let amount_handle = api.esdt_value_by_index(i);

        api.mb_append_bytes(dest_handle, &token_identifier_handle.to_be_bytes()[..]);
        api.mb_append_bytes(dest_handle, &token_nonce.to_be_bytes()[..]);
        api.mb_append_bytes(dest_handle, &amount_handle.to_be_bytes()[..]);
    }
}
