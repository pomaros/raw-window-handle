// SPDX-License-Identifier: MIT
// Copyright (c) 2026 John Knipper

use core::{ffi::c_void, ptr::NonNull};

/// The native AROS Intuition display, selected by EGL_DEFAULT_DISPLAY.
///
/// No display resource is borrowed. Windows retain their own Intuition screen
/// relationship; this handle does not transfer ownership of it.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArosDisplayHandle {}

impl ArosDisplayHandle {
    /// Create a handle for the native Intuition display.
    pub fn new() -> Self {
        Self {}
    }
}

/// A native AROS Intuition window.
///
/// An implementer of HasWindowHandle must keep the window alive for the
/// borrowed handle's lifetime. GPU surfaces must be destroyed before CloseWindow.
/// This is the actual struct Window pointer, never a toolkit wrapper pointer.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArosWindowHandle {
    /// Pointer to the live Intuition struct Window.
    pub window: NonNull<c_void>,
}

impl ArosWindowHandle {
    /// Describe an Intuition window without taking ownership.
    pub fn new(window: NonNull<c_void>) -> Self {
        Self { window }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RawDisplayHandle, RawWindowHandle};

    #[test]
    fn retains_native_identity() {
        let mut window_storage = 0u8;
        let pointer = NonNull::from(&mut window_storage).cast();
        let window: RawWindowHandle = ArosWindowHandle::new(pointer).into();
        assert!(matches!(window, RawWindowHandle::Aros(h) if h.window == pointer));
        let display: RawDisplayHandle = ArosDisplayHandle::new().into();
        assert!(matches!(display, RawDisplayHandle::Aros(_)));
    }
}
