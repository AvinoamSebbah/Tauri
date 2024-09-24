use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::slice::from_raw_parts;
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::HLOCAL;
use windows::Win32::Globalization::lstrlenW;
use windows::Win32::System::Memory::LocalFree;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub sid: String,
    pub role: String,
    pub valid: bool,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub id: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
pub struct UserWaitingList {
    pub username: String,
    pub id: String,
}

pub struct LocalHeapString {
    inner: PWSTR,
}

impl LocalHeapString {
    pub fn as_mut_ptr(&mut self) -> &mut PWSTR {
        &mut self.inner
    }
}

impl Default for LocalHeapString {
    fn default() -> Self {
        Self {
            inner: PWSTR::null(),
        }
    }
}

impl Drop for LocalHeapString {
    fn drop(&mut self) {
        if self.inner != PWSTR::null() {
            let free_me: HLOCAL = HLOCAL(self.inner.0 as isize);
            self.inner = PWSTR::null();
            let _ = unsafe { LocalFree(free_me) };
        }
    }
}

impl From<LocalHeapString> for String {
    fn from(value: LocalHeapString) -> Self {
        let as_constant_wide_string: PCWSTR = PCWSTR(value.inner.0);
        let s = unsafe { lstrlenW(as_constant_wide_string) };
        let v = unsafe { from_raw_parts(as_constant_wide_string.0, s as usize) };
        let as_os_string = OsString::from_wide(v);
        let as_rust_string = as_os_string.to_string_lossy();
        as_rust_string.into_owned()
    }
}
