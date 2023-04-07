// GStreamer
//
// Copyright (C) 2015-2023 Sebastian Dröge <sebastian@centricular.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.
//
// SPDX-License-Identifier: MPL-2.0

use crate::error::Error;

pub fn set_priority() -> Result<(), Error> {
    #[cfg(windows)]
    {
        use std::io;

        use crate::{bail, ffi::windows::*};

        // SAFETY: Getting a handle to the current thread is safe at any time
        let thread = unsafe { GetCurrentThread() };

        // SAFETY: SetThreadPriority() requires a valid thread handle, which was given above,
        // and will return 0 on errors.
        unsafe {
            if SetThreadPriority(thread, THREAD_PRIORITY_TIME_CRITICAL) == 0 {
                bail!(
                    source: io::Error::last_os_error(),
                    "Failed to set thread priority"
                );
            }
        }
    }
    Ok(())
}
