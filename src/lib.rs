#![allow(static_mut_refs)]

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
use android::{game_impl, gui_impl, fridgerator_impl, interceptor_impl, log_impl, symbols_impl};

/** Windows **/
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use windows::{game_impl, gui_impl, fridgerator_impl, interceptor_impl, log_impl, symbols_impl};

/** Stubs (Cross-platform cargo check) **/
#[cfg(not(any(target_os = "android", target_os = "windows")))]
mod stubs;

#[cfg(not(any(target_os = "android", target_os = "windows")))]
use stubs::{game_impl, gui_impl, fridgerator_impl, interceptor_impl, log_impl, symbols_impl};
