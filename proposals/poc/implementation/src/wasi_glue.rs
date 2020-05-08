use std::convert::TryInto;

use crate::types as guest_types;
use crate::WasiCryptoCtx;

impl crate::wasi_ephemeral_crypto_common::WasiEphemeralCryptoCommon for WasiCryptoCtx {
    // --- options

    fn options_open(
        &self,
        options_type: guest_types::OptionsType,
    ) -> Result<guest_types::Options, guest_types::CryptoErrno> {
        Ok(self.ctx.options_open(options_type.into())?.into())
    }

    fn options_close(
        &self,
        options_handle: guest_types::Options,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self.ctx.options_close(options_handle.into())?.into())
    }

    fn options_set(
        &self,
        options_handle: guest_types::Options,
        name_str: &wiggle::GuestPtr<'_, str>,
        value_ptr: &wiggle::GuestPtr<'_, u8>,
        value_len: guest_types::Size,
    ) -> Result<(), guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let name_str: &str = unsafe { &*name_str.as_raw(&mut guest_borrow)? };
        let value: &[u8] = unsafe {
            &*value_ptr
                .as_array(value_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .options_set(options_handle.into(), name_str, value)?
            .into())
    }

    fn options_set_guest_buffer(
        &self,
        options_handle: guest_types::Options,
        name_str: &wiggle::GuestPtr<'_, str>,
        buffer_ptr: &wiggle::GuestPtr<'_, u8>,
        buffer_len: guest_types::Size,
    ) -> Result<(), guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let name_str: &str = unsafe { &*name_str.as_raw(&mut guest_borrow)? };
        let buffer: &'static mut [u8] = unsafe {
            &mut *buffer_ptr
                .as_array(buffer_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .options_set_guest_buffer(options_handle.into(), name_str, buffer)?
            .into())
    }

    fn options_set_u64(
        &self,
        options_handle: guest_types::Options,
        name_str: &wiggle::GuestPtr<'_, str>,
        value: u64,
    ) -> Result<(), guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let name_str: &str = unsafe { &*name_str.as_raw(&mut guest_borrow)? };
        Ok(self
            .ctx
            .options_set_u64(options_handle.into(), name_str, value)?
            .into())
    }

    // --- array

    fn array_output_len(
        &self,
        array_output_handle: guest_types::ArrayOutput,
    ) -> Result<guest_types::Size, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .array_output_len(array_output_handle.into())?
            .try_into()?)
    }

    fn array_output_pull(
        &self,
        array_output_handle: guest_types::ArrayOutput,
        buf_ptr: &wiggle::GuestPtr<'_, u8>,
        buf_len: guest_types::Size,
    ) -> Result<guest_types::Size, guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let buf: &mut [u8] =
            unsafe { &mut *buf_ptr.as_array(buf_len as _).as_raw(&mut guest_borrow)? };
        Ok(self
            .ctx
            .array_output_pull(array_output_handle.into(), buf)?
            .try_into()?)
    }

    // --- key_manager

    fn key_manager_open(
        &self,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::KeyManager, guest_types::CryptoErrno> {
        let options_handle = match *options_handle {
            guest_types::OptOptions::Some(options_handle) => Some(options_handle),
            guest_types::OptOptions::None => None,
        };
        Ok(self
            .ctx
            .key_manager_open(options_handle.map(Into::into))?
            .into())
    }

    fn key_manager_close(
        &self,
        key_manager_handle: guest_types::KeyManager,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self.ctx.key_manager_close(key_manager_handle.into())?)
    }

    fn key_manager_invalidate(
        &self,
        key_manager_handle: guest_types::KeyManager,
        key_id_ptr: &wiggle::GuestPtr<'_, u8>,
        key_id_len: guest_types::Size,
        key_version: guest_types::Version,
    ) -> Result<(), guest_types::CryptoErrno> {
        let mut guest_borrow = wiggle::GuestBorrows::new();
        let key_id: &[u8] = unsafe {
            &*key_id_ptr
                .as_array(key_id_len as _)
                .as_raw(&mut guest_borrow)?
        };
        Ok(self
            .ctx
            .key_manager_invalidate(key_manager_handle.into(), key_id, key_version.into())?
            .into())
    }
}