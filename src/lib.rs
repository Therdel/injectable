use std::os::raw::{c_ulong, c_void, c_int};
use windows::{
    Win32::Foundation::HINSTANCE,
    Win32::System::LibraryLoader::DisableThreadLibraryCalls,
    Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}
};

type HMODULE = HINSTANCE;
type DWORD = c_ulong;
type LPVOID = *mut c_void;
type BOOL = c_int;

#[no_mangle]
extern "stdcall" fn DllMain(h_module: HMODULE,
                            ul_reason_for_call: DWORD,
                            lp_reserved: LPVOID) -> BOOL {
    match ul_reason_for_call {
        DLL_PROCESS_ATTACH => {
            unsafe { DisableThreadLibraryCalls(h_module); }
        },
        DLL_PROCESS_DETACH => {
            if lp_reserved.is_null() {
                // reason: FreeLibrary was called
            } else {
                // reason: Process is terminating
            }
        }
        _ => {}
    }

    true as BOOL
}
