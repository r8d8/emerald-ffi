#ifndef emerald_header
#define emerald_header


#ifdef __cplusplus
extern "C" {
#endif

fn createKeyfile(
    passphrase: *const c_char,
    sec_level: KdfDepthLevel,
    name: *const c_char,
    description: *const c_char,
) -> *mut KeyFile;


#ifdef __cplusplus
}
#endif

#endif