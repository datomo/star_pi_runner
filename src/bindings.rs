/* automatically generated by rust-bindgen 0.56.0 */

extern "C" {
    pub fn tofGetModel(
        model: *mut ::std::os::raw::c_int,
        revision: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tofReadDistance() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tofInit(
        iChan: ::std::os::raw::c_int,
        iAddr: ::std::os::raw::c_int,
        bLongRange: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}