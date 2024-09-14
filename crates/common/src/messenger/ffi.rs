// MIT License
//
// Copyright (c) 2024 ZARK-WAF
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors: I. Zeqiri, E. Gjergji

use std::ffi::{CStr, CString};
use std::ptr;
use crate::utils::uid::Uid;
use crate::messenger::{Topic, SubscriberId, Message, Callback};

// define a type alias for a raw pointer to a c_void
type RawPointer = *mut std::ffi::c_void;

// define a type alias for a callback function pointer

// ffi module: handles low-level FFI interactions with the messenger library

use libloading::{Library, Symbol};
use once_cell::sync::OnceCell;
use std::ffi::c_void;
use std::os::raw::c_char;
use thiserror::Error;

static LIBRARY: OnceCell<Library> = OnceCell::new();

// define FFI function signatures
type ZarkMessengerCreate = unsafe extern "C" fn() -> *mut c_void;
type ZarkMessengerDestroy = unsafe extern "C" fn(*mut c_void);
type ZarkMessengerSend = unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char, usize) -> bool;
type ZarkMessengerSubscribe = unsafe extern "C" fn(*mut c_void, *const c_char, extern "C" fn(*const c_char, usize, *mut c_void), *mut c_void) -> *const c_char;
type ZarkMessengerUnsubscribe = unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> bool;

#[derive(Error, Debug)]
pub enum FFIError {
    #[error("Library not loaded")]
    LibraryNotLoaded,
    #[error("Failed to create CString: {0}")]
    CStringError(#[from] std::ffi::NulError),
    #[error("Symbol not found: {0}")]
    SymbolNotFound(String),
    #[error("FFI operation failed: {0}")]
    OperationFailed(String),
}

pub struct FFIMessenger {
    inner: *mut c_void,
}

impl FFIMessenger {
    pub fn new<P: AsRef<std::ffi::OsStr>>(library_path: P) -> Result<Self, FFIError> {
        let library = LIBRARY.get_or_try_init(|| unsafe { Library::new(library_path) })
            .map_err(|e| FFIError::OperationFailed(e.to_string()))?;
        
        let create: Symbol<ZarkMessengerCreate> = unsafe { 
            library.get(b"zark_messenger_create")
                .map_err(|e| FFIError::SymbolNotFound(e.to_string()))?
        };
        
        let inner = unsafe { create() };
        Ok(Self { inner })
    }

    pub fn send(&self, topic: &str, message: &[u8]) -> Result<bool, FFIError> {
        let topic = CString::new(topic)?;
        let library = LIBRARY.get().ok_or(FFIError::LibraryNotLoaded)?;
        let send: Symbol<ZarkMessengerSend> = unsafe {
            library.get(b"zark_messenger_send")
                .map_err(|e| FFIError::SymbolNotFound(e.to_string()))?
        };
        Ok(unsafe { send(self.inner, topic.as_ptr(), message.as_ptr() as *const c_char, message.len()) })
    }

    pub fn subscribe(&self, topic: &str, callback: Callback) -> Result<SubscriberId, FFIError> {
        let topic = CString::new(topic)?;
        let library = LIBRARY.get().ok_or(FFIError::LibraryNotLoaded)?;
        let subscribe: Symbol<ZarkMessengerSubscribe> = unsafe {
            library.get(b"zark_messenger_subscribe")
                .map_err(|e| FFIError::SymbolNotFound(e.to_string()))?
        };

        extern "C" fn callback_trampoline(data: *const c_char, size: usize, user_data: *mut c_void) {
            let callback = unsafe { &*(user_data as *const Callback) };
            let message = unsafe { std::slice::from_raw_parts(data as *const u8, size) }.to_vec();
            callback(message);
        }

        let callback_box = Box::new(callback);
        let callback_ptr = Box::into_raw(callback_box) as *mut c_void;

        let subscriber_id_ptr = unsafe { 
            subscribe(self.inner, topic.as_ptr(), callback_trampoline, callback_ptr)
        };

        let subscriber_id_str = unsafe { CStr::from_ptr(subscriber_id_ptr) }.to_str()
            .map_err(|e| FFIError::OperationFailed(e.to_string()))?;
        
        SubscriberId::from_string(subscriber_id_str)
            .ok_or_else(|| FFIError::OperationFailed("Invalid subscriber ID returned".to_string()))
    }

    pub fn unsubscribe(&self, topic: &str, subscriber_id: &SubscriberId) -> Result<bool, FFIError> {
        let topic = CString::new(topic)?;
        let subscriber_id_str = CString::new(subscriber_id.to_string())?;
        let library = LIBRARY.get().ok_or(FFIError::LibraryNotLoaded)?;
        let unsubscribe: Symbol<ZarkMessengerUnsubscribe> = unsafe {
            library.get(b"zark_messenger_unsubscribe")
                .map_err(|e| FFIError::SymbolNotFound(e.to_string()))?
        };
        
        Ok(unsafe { unsubscribe(self.inner, topic.as_ptr(), subscriber_id_str.as_ptr()) })
    }
}

impl Drop for FFIMessenger {
    fn drop(&mut self) {
        if let Some(library) = LIBRARY.get() {
            if let Ok(destroy) = unsafe { library.get::<ZarkMessengerDestroy>(b"zark_messenger_destroy") } {
                unsafe { destroy(self.inner) };
            }
        }
    }
}

unsafe impl Send for FFIMessenger {}
unsafe impl Sync for FFIMessenger {}
