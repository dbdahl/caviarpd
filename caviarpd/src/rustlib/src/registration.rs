// Generated by cargo: do not edit by hand

// If usage of .Call() functions in the package's R code changes, update this
// file by rerunning "cargo::register_calls(DIR)", where DIR is the root
// directory of this package.

/*
// Below is commented-out skeleton code that you can copy to your
// 'src/rustlib/src/lib.rs' file. You can change the body and arguments
// names of the functions, but changing the function name necessitates
// a corresponding change in the R code.

mod registration;

// Help: https://docs.rs/libR-sys, https://github.com/hadley/r-internals
use dahl_roxido::*;

#[no_mangle]
extern "C" fn sample_epa(nSamples: SEXP, similarity: SEXP, mass: SEXP, discount: SEXP, nCores: SEXP) -> SEXP {
    libR_sys::R_NilValue
}

#[no_mangle]
extern "C" fn caviarpd_n_clusters(nSamplesSearch: SEXP, similarity: SEXP, mass: SEXP, discount: SEXP, unnamed1: SEXP, unnamed2: SEXP, maxNClusters: SEXP, nCores: SEXP) -> SEXP {
    libR_sys::R_NilValue
}
*/

use dahl_roxido::libR_sys;

#[no_mangle]
extern "C" fn R_init_caviarpd_librust(info: *mut libR_sys::DllInfo) {
    let mut call_routines = Vec::new();
    let mut names = Vec::new();
    names.push(std::ffi::CString::new(".sample_epa").unwrap());
    call_routines.push(libR_sys::R_CallMethodDef {
        name: names.last().unwrap().as_ptr(),
        fun: unsafe { std::mem::transmute(crate::sample_epa as *const u8) },
        numArgs: 5,
    });
    names.push(std::ffi::CString::new(".caviarpd_n_clusters").unwrap());
    call_routines.push(libR_sys::R_CallMethodDef {
        name: names.last().unwrap().as_ptr(),
        fun: unsafe { std::mem::transmute(crate::caviarpd_n_clusters as *const u8) },
        numArgs: 8,
    });
    call_routines.push(libR_sys::R_CallMethodDef {
        name: std::ptr::null(),
        fun: None,
        numArgs: 0,
    });
    unsafe {
        libR_sys::R_registerRoutines(
            info,
            std::ptr::null(),
            call_routines.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
        );
        libR_sys::R_useDynamicSymbols(info, 1);
        libR_sys::R_forceSymbols(info, 1);
    }
}

