// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
//! Session management functions

use crate::context::Pkcs11;
use crate::error::{Result, Rv};
use crate::session::{Session, SessionFlags};
use crate::slot::Slot;
use std::convert::TryInto;

// See public docs on stub in parent mod.rs
#[inline(always)]
pub(super) fn open_session_no_callback(
    ctx: &Pkcs11,
    slot_id: Slot,
    flags: SessionFlags,
) -> Result<Session> {
    let mut session_handle = 0;

    unsafe {
        Rv::from(get_pkcs11!(ctx, C_OpenSession)(
            slot_id.try_into()?,
            flags.into(),
            // TODO: abstract those types or create new functions for callbacks
            std::ptr::null_mut(),
            None,
            &mut session_handle,
        ))
        .into_result()?;
    }

    Ok(Session::new(session_handle, ctx.clone()))
}
