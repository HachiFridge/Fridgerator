use crate::core::{
    game::{Game, Region},
    interceptor::HookHandle,
    Error, Fridgerator,
};
use serde::{Deserialize, Serialize};
use std::os::raw::c_void;
use std::path::PathBuf;

pub mod log_impl {
    pub fn init(_filter_level: log::LevelFilter, _file_logging: bool) {}
}

pub mod game_impl {
    use super::*;
    pub fn init() -> Game {
        Game { package_name: String::new(), region: Region::Japan, data_dir: PathBuf::new() }
    }
    pub fn get_package_name() -> String {
        String::new()
    }
    pub fn get_region(_package_name: &str) -> Region {
        Region::Japan
    }
    pub fn get_data_dir(_package_name: &str) -> PathBuf {
        PathBuf::new()
    }
}

pub mod fridgerator_impl {
    use super::*;
    pub fn is_il2cpp_lib(_name: &str) -> bool {
        false
    }
    pub fn is_criware_lib(_name: &str) -> bool {
        false
    }
    pub fn on_hooking_finished(_fridgerator: &Fridgerator) {}

    #[derive(Deserialize, Serialize, Clone, Default)]
    pub struct Config {}
}

pub mod gui_impl {
    pub fn init() {}
}

pub mod symbols_impl {
    use super::*;
    pub fn init() {}
    pub fn dlsym(_handle: *mut c_void, _symbol: &str) -> usize {
        0
    }
}

pub mod interceptor_impl {
    use super::*;
    pub fn init() {}
    pub unsafe fn unhook(_handle: &HookHandle) -> Result<(), Error> {
        Ok(())
    }
    pub unsafe fn unhook_vtable(_handle: &HookHandle) -> Result<(), Error> {
        Ok(())
    }
    pub unsafe fn hook(_target: usize, _detour: usize) -> Result<usize, Error> {
        Ok(0)
    }
    pub unsafe fn hook_vtable(
        _vtable: *mut usize,
        _index: usize,
        _detour: usize,
    ) -> Result<HookHandle, Error> {
        unimplemented!("Stub")
    }
    pub unsafe fn get_vtable_from_instance(_instance_addr: usize) -> *mut usize {
        std::ptr::null_mut()
    }
    pub unsafe fn find_symbol_by_name(_module: &str, _symbol: &str) -> Result<usize, Error> {
        Ok(0)
    }
}
