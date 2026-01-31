#![allow(static_mut_refs, non_snake_case, non_camel_case_types)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate cstr;

rust_i18n::i18n!("assets/locales", fallback = "en");

#[macro_use]
pub mod core;
pub mod il2cpp;

/** Android **/
#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
pub(crate) use android::{
    fridgerator_impl, game_impl, gui_impl, interceptor_impl, log_impl, symbols_impl,
};

/** Windows **/
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub(crate) use windows::{
    fridgerator_impl, game_impl, gui_impl, interceptor_impl, log_impl, symbols_impl,
};
