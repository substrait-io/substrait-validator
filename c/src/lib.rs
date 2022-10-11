// SPDX-License-Identifier: Apache-2.0

// Functions dereferencing raw pointers are kind of par for the course in a C
// interface, and if we have to mark effectively all functions unsafe here, we
// can no longer selectively place unsafe {} blocks (there is no way to mark a
// function as unsafe to use without implicitly allowing unsafe code to be used
// in its implementation).
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::cell::RefCell;

thread_local! {
    /// Most recent error message, stored in thread-local storage for
    /// thread-safety.
    pub static LAST_ERROR: RefCell<std::ffi::CString> = RefCell::new(std::ffi::CString::default());
}

/// Pushes an error message.
fn set_last_error<S: AsRef<str>>(s: S) {
    LAST_ERROR.with(|f| {
        *f.borrow_mut() = std::ffi::CString::new(s.as_ref()).unwrap_or_default();
    });
}

/// Returns the most recent error message. Note that the returned pointer is
/// only valid until the next call that the current thread makes to this
/// library.
#[no_mangle]
pub extern "C" fn substrait_validator_get_last_error() -> *const libc::c_char {
    LAST_ERROR.with(|f| {
        let reference = f.borrow();
        reference.as_bytes_with_nul().as_ptr() as *const libc::c_char
    })
}

/// Parser/validator configuration handle.
pub struct ConfigHandle {
    pub config: substrait_validator::Config,
}

/// Creates a parser/validator configuration structure.
#[no_mangle]
pub extern "C" fn substrait_validator_config_new() -> *mut ConfigHandle {
    // Create a box to store the return value handle on the stack.
    let handle = Box::new(ConfigHandle {
        config: substrait_validator::Config::new(),
    });

    // Convert the box to its raw pointer and relinquish ownership.
    Box::into_raw(handle)
}

/// Frees memory associated with a configuration handle. No-op if given a
/// nullptr.
#[no_mangle]
pub extern "C" fn substrait_validator_config_free(handle: *mut ConfigHandle) {
    // Ignore null pointers.
    if handle.is_null() {
        return;
    }

    // UNSAFE: recover the box that we created the handle with and drop it.
    // Assumes that the pointer was created by substrait_validator_config_new().
    let config = unsafe { Box::from_raw(handle) };
    drop(config);
}

/// Queries which diagnostic codes are defined. If buf is non-null and size is
/// nonzero, up to size entries in buf are filled with valid diagnostic codes.
/// Regardless of how many entries were populated, the number of defined
/// diagnostic codes is returned.
pub extern "C" fn substrait_validator_diag_codes(buf: *mut u32, size: usize) -> usize {
    if !buf.is_null() && size > 0 {
        // UNSAFE: assumes that buf is properly aligned, that there is
        // read/write access to a region of size u32s from buf onwards, and
        // that nothing else is mutating the buffer.
        let slice = unsafe { std::slice::from_raw_parts_mut(buf, size) };

        for (code, class) in slice
            .iter_mut()
            .zip(substrait_validator::iter_diagnostics())
        {
            *code = class.code();
        }
    }

    substrait_validator::iter_diagnostics().count()
}

/// For the given diagnostic code, returns the code for the group it belongs
/// to. Configuring a level override for the parent of a group of diagnostic
/// codes will set the default override for all diagnostics contained within
/// that group.
pub extern "C" fn substrait_validator_diag_parent(code: u32) -> u32 {
    substrait_validator::Classification::parent(code)
}

/// Returns the name of the given diagnostic code. If buf is non-null and size
/// is nonzero, up to size-1 characters in buf are filled with this name,
/// followed by a null termination character. The null termination character is
/// considered to be part of size. If buf is non-null, size is nonzero, and
/// code is valid, it is always written, even if this means that the name is
/// cut short. Bytes in buf beyond the resulting string length but within the
/// size limit may be clobbered.
///
/// If code is valid, the function returns the minimum buffer size needed to
/// contain the complete name (being its string length + 1), regardless of the
/// supplied buffer. If code is invalid, 0 is returned, and an error message
/// can be retrieved with substrait_validator_get_last_error().
pub extern "C" fn substrait_validator_diag_name(
    code: u32,
    buf: *mut libc::c_char,
    size: usize,
) -> usize {
    if let Some(class) = substrait_validator::Classification::from_code(code) {
        let name = class.name();
        let name_bytes = name.as_bytes();

        if !buf.is_null() && size > 0 {
            // UNSAFE: assumes that buf is properly aligned, that there is
            // read/write access to a region of size bytes from buf onwards,
            // and that nothing else is mutating the buffer.
            let slice = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, size) };

            // Try to write name followed by a 0 to the first size-1 bytes
            // of the buffer.
            for (buf_byte, name_byte) in slice[..size - 1]
                .iter_mut()
                .zip(name_bytes.iter().cloned().chain(std::iter::once(0)))
            {
                *buf_byte = name_byte;
            }

            // Pessimistically always write a 0 to the last byte of the buffer,
            // even though we may already have written an early termination
            // character.
            slice[size - 1] = 0;
        }

        // Return the minimum buffer size.
        name_bytes.len() + 1
    } else {
        set_last_error(format!("{code} is not a valid diagnostic code"));
        0
    }
}

/// Returns the description of the given diagnostic code. If buf is non-null
/// and size is nonzero, up to size-1 characters in buf are filled with this
/// description, followed by a null termination character. The null
/// termination character is considered to be part of size. If buf is
/// non-null, size is nonzero, and code is valid, it is always written, even
/// if this means that the name is cut short. Bytes in buf beyond the
/// resulting string length but within the size limit may be clobbered.
///
/// If code is valid, the function returns the minimum buffer size needed to
/// contain the complete description (being its string length + 1), regardless
/// of the supplied buffer. If code is invalid, 0 is returned, and an error
/// message can be retrieved with substrait_validator_get_last_error().
pub extern "C" fn substrait_validator_diag_desc(
    code: u32,
    buf: *mut libc::c_char,
    size: usize,
) -> usize {
    if let Some(class) = substrait_validator::Classification::from_code(code) {
        let description = class.description();
        let description_bytes = description.as_bytes();

        if !buf.is_null() && size > 0 {
            // UNSAFE: assumes that buf is properly aligned, that there is
            // read/write access to a region of size bytes from buf onwards,
            // and that nothing else is mutating the buffer.
            let slice = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, size) };

            // Try to write name followed by a 0 to the first size-1 bytes
            // of the buffer.
            for (buf_byte, name_byte) in slice[..size - 1]
                .iter_mut()
                .zip(description_bytes.iter().cloned().chain(std::iter::once(0)))
            {
                *buf_byte = name_byte;
            }

            // Pessimistically always write a 0 to the last byte of the buffer,
            // even though we may already have written an early termination
            // character.
            slice[size - 1] = 0;
        }

        // Return the minimum buffer size.
        description_bytes.len() + 1
    } else {
        set_last_error(format!("{code} is not a valid diagnostic code"));
        0
    }
}

/// Instructs the validator to ignore protobuf fields that it doesn't know
/// about yet (i.e., that have been added to the Substrait protobuf
/// descriptions, but haven't yet been implemented in the validator) if the
/// fields are set to their default value. If this option isn't set, or if an
/// unknown field is not set to its default value, a warning is emitted.
///
/// Returns whether the function was successful. If false is returned, retrieve
/// the error message with substrait_validator_get_last_error().
#[no_mangle]
pub extern "C" fn substrait_validator_config_ignore_unknown_fields(
    config: *mut ConfigHandle,
) -> bool {
    // Check for null.
    if config.is_null() {
        set_last_error("received null configuration handle");
        return false;
    }

    // UNSAFE: unpack configuration handle. Assumes that the pointer was
    // created by substrait_validator_config_new(), or behavior is undefined.
    let config = unsafe { &mut (*config).config };

    // Update configuration and return success.
    config.ignore_unknown_fields();
    true
}

/// Explicitly allows a protobuf message type for use in advanced extensions,
/// despite the fact that the validator can't validate it. If an advanced
/// extension is encountered that isn't explicitly allowed, a warning is
/// emitted. The type URL pattern may include * and ? wildcards for glob-like
/// matching (see https://docs.rs/glob/latest/glob/struct.Pattern.html for the
/// complete syntax).
///
/// Returns whether the function was successful. If false is returned, retrieve
/// the error message with substrait_validator_get_last_error().
#[no_mangle]
pub extern "C" fn substrait_validator_config_allow_proto_any_url(
    config: *mut ConfigHandle,
    pattern: *const libc::c_char,
) -> bool {
    // Check for nulls.
    if config.is_null() {
        set_last_error("received null configuration handle");
        return false;
    }
    if pattern.is_null() {
        set_last_error("received null pattern");
        return false;
    }

    // UNSAFE: unpack configuration handle. Assumes that the pointer was
    // created by substrait_validator_config_new(), or behavior is undefined.
    let config = unsafe { &mut (*config).config };

    // UNSAFE: unpack pattern string. Assumes that the pointer points to a
    // null-terminated string.
    let pattern = unsafe { std::ffi::CStr::from_ptr(pattern) };

    // Parse the pattern.
    let pattern = match pattern.to_str() {
        Ok(u) => u,
        Err(e) => {
            set_last_error(format!("received invalid pattern: {e}"));
            return false;
        }
    };
    let pattern = match substrait_validator::Pattern::new(pattern) {
        Ok(p) => p,
        Err(e) => {
            set_last_error(format!("received invalid pattern: {e}"));
            return false;
        }
    };

    // Update configuration and return success.
    config.allow_proto_any_url(pattern);
    true
}

/// Converts a positive/zero/negative integer into Info/Warning/Error
/// respectively.
fn int_to_level(x: i32) -> substrait_validator::Level {
    match x {
        1..=i32::MAX => substrait_validator::Level::Info,
        0 => substrait_validator::Level::Warning,
        i32::MIN..=-1 => substrait_validator::Level::Error,
    }
}

/// Sets a minimum and/or maximum error level for the given class of diagnostic
/// messages. Any previous settings for this class are overridden. The levels
/// are encoded as integers, where any positive value means info, zero means
/// warning, and negative means error.
///
/// Returns whether the function was successful. If false is returned, retrieve
/// the error message with substrait_validator_get_last_error().
#[no_mangle]
pub extern "C" fn substrait_validator_config_override_diagnostic_level(
    config: *mut ConfigHandle,
    class: u32,
    minimum: i32,
    maximum: i32,
) -> bool {
    // Check for null.
    if config.is_null() {
        set_last_error("received null configuration handle");
        return false;
    }

    // UNSAFE: unpack configuration handle. Assumes that the pointer was
    // created by substrait_validator_config_new(), or behavior is undefined.
    let config = unsafe { &mut (*config).config };

    // Parse the diagnostic class/code.
    let class = match substrait_validator::Classification::from_code(class) {
        Some(c) => c,
        None => {
            set_last_error(format!("unknown diagnostic class {class}"));
            return false;
        }
    };

    // Parse the minimum and maximum levels.
    let minimum = int_to_level(minimum);
    let maximum = int_to_level(maximum);

    // Update configuration and return success.
    config.override_diagnostic_level(class, minimum, maximum);
    true
}

/// Overrides the resolution behavior for (YAML) URIs matching the given
/// pattern. The pattern may include * and ? wildcards for glob-like matching
/// (see https://docs.rs/glob/latest/glob/struct.Pattern.html for the complete
/// syntax). If resolve_as is null, the URI will not be resolved; otherwise, it
/// will be resolved as if the URI in the plan had been that string.
///
/// Returns whether the function was successful. If false is returned, retrieve
/// the error message with substrait_validator_get_last_error().
#[no_mangle]
pub extern "C" fn substrait_validator_config_override_uri(
    config: *mut ConfigHandle,
    pattern: *const libc::c_char,
    resolve_as: *const libc::c_char,
) -> bool {
    // Check for nulls.
    if config.is_null() {
        set_last_error("received null configuration handle");
        return false;
    }
    if pattern.is_null() {
        set_last_error("received null pattern");
        return false;
    }

    // UNSAFE: unpack configuration handle. Assumes that the pointer was
    // created by substrait_validator_config_new(), or behavior is undefined.
    let config = unsafe { &mut (*config).config };

    // UNSAFE: unpack pattern string. Assumes that the pointer points to a
    // null-terminated string.
    let pattern = unsafe { std::ffi::CStr::from_ptr(pattern) };

    // Parse the pattern.
    let pattern = match pattern.to_str() {
        Ok(p) => p,
        Err(e) => {
            set_last_error(format!("received invalid pattern: {e}"));
            return false;
        }
    };
    let pattern = match substrait_validator::Pattern::new(pattern) {
        Ok(p) => p,
        Err(e) => {
            set_last_error(format!("received invalid pattern: {e}"));
            return false;
        }
    };

    // Unpack and parse resolve_as.
    let resolve_as = if resolve_as.is_null() {
        None
    } else {
        // UNSAFE: unpack resolve_as string. Assumes that the pointer points to
        // a null-terminated string.
        let resolve_as = unsafe { std::ffi::CStr::from_ptr(resolve_as) };

        Some(match resolve_as.to_str() {
            Ok(p) => p,
            Err(e) => {
                set_last_error(format!("received invalid replacement URI: {e}"));
                return false;
            }
        })
    };

    // Update configuration and return success.
    config.override_uri(pattern, resolve_as);
    true
}

/// Callback function for deleting a buffer allocated by the user application.
pub type Deleter =
    Option<unsafe extern "C" fn(user: *mut libc::c_void, buf: *const u8, size: usize)>;

/// (YAML) URI resolution callback function.
///
/// The first argument (uri) is set to a null-terminated UTF-8 string
/// representing the URI that is to be resolved. If resolution succeeds, the
/// function must return the binary result buffer via buf and size and return
/// true. If it fails, it should instead write a UTF-8 error message to this
/// buffer (but it may also set buf to nullptr or leave it unchanged) and
/// return false.
///
/// The buffer must remain valid only until the validator library returns
/// control to the application. Thus, the application may keep track of the
/// current buffer via thread-local storage or a global. It may also assign a
/// deleter function to the deleter parameter, which will be called by the
/// validator library when it is done with the buffer. deleter_user may be
/// used to pass additional contextual information to the deleter; it is not
/// used by the validator library for any purpose other than calling the
/// deleter function.
///
/// All output parameters will be set to zero by the validator library before
/// the callback is called.
pub type Resolver = Option<
    unsafe extern "C" fn(
        uri: *const libc::c_char,
        buf: *mut *const u8,
        size: *mut usize,
        deleter: *mut Deleter,
        deleter_user: *mut *mut libc::c_void,
    ) -> bool,
>;

/// Wraps a buffer returned by Resolver.
struct ApplicationBuffer {
    pub buf: *const u8,
    pub size: usize,
    pub deleter: Deleter,
    pub deleter_user: *mut libc::c_void,
}

impl Default for ApplicationBuffer {
    fn default() -> Self {
        Self {
            buf: std::ptr::null(),
            size: 0,
            deleter: None,
            deleter_user: std::ptr::null_mut(),
        }
    }
}

impl Drop for ApplicationBuffer {
    fn drop(&mut self) {
        if let Some(deleter) = self.deleter {
            // UNSAFE: assumes that the deleter function passed by the user is
            // valid.
            unsafe { deleter(self.deleter_user, self.buf, self.size) }
        }
    }
}

impl AsRef<[u8]> for ApplicationBuffer {
    fn as_ref(&self) -> &[u8] {
        // UNSAFE: assumes that the pointer to the buffer returned by the
        // application is non-null, that the pointed-to byte up to that byte
        // plus self.size bytes can be dereferenced.
        unsafe { std::slice::from_raw_parts(self.buf, self.size) }
    }
}

/// Rust representation of an error returned by the Resolver callback function.
#[derive(Debug, thiserror::Error)]
struct ApplicationError {
    msg: String,
}

impl ApplicationError {
    fn new<S: Into<String>>(msg: S) -> Self {
        ApplicationError { msg: msg.into() }
    }
}

impl From<ApplicationBuffer> for ApplicationError {
    fn from(buf: ApplicationBuffer) -> Self {
        ApplicationError {
            msg: match std::str::from_utf8(buf.as_ref()) {
                Ok(e) => e.to_string(),
                Err(e) => format!("unknown error (failed to decode error message: {e})"),
            },
        }
    }
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

/// Registers a URI resolution function with this configuration. If the given
/// function fails, any previously registered function will be used as a
/// fallback.
///
/// See the documentation for the substrait_validator_resolver typedef for
/// more information about the semantics of the callback function.
///
/// Returns whether the function was successful. If false is returned, retrieve
/// the error message with substrait_validator_get_last_error().
#[no_mangle]
pub extern "C" fn substrait_validator_config_uri_resolver(
    config: *mut ConfigHandle,
    resolver: Resolver,
) -> bool {
    // Check for nulls.
    if config.is_null() {
        set_last_error("received null configuration handle");
        return false;
    }

    // UNSAFE: unpack configuration handle. Assumes that the pointer was
    // created by substrait_validator_config_new(), or behavior is undefined.
    let config = unsafe { &mut (*config).config };

    // Unpack resolution function.
    let resolver = match resolver {
        Some(r) => r,
        None => {
            set_last_error("received null resolution function pointer");
            return false;
        }
    };

    // Update configuration and return success.
    config.add_uri_resolver(move |uri| {
        let uri = match std::ffi::CString::new(uri) {
            Ok(u) => u,
            Err(_) => {
                return Err(ApplicationError::new(
                    "cannot resolve URI with embedded nul characters",
                ))
            }
        };
        let mut buffer = ApplicationBuffer::default();

        // UNSAFE: assumes that the resolver function passed by the user is
        // valid.
        let result = unsafe {
            resolver(
                uri.as_ptr(),
                &mut buffer.buf,
                &mut buffer.size,
                &mut buffer.deleter,
                &mut buffer.deleter_user,
            )
        };

        if result {
            if buffer.buf.is_null() {
                Err(ApplicationError::new(
                    "URI resolver callback returned success but also a null buffer",
                ))
            } else {
                Ok(buffer)
            }
        } else if buffer.buf.is_null() {
            Err(ApplicationError::new("URI resolver callback failed"))
        } else {
            Err(ApplicationError::from(buffer))
        }
    });
    true
}

/// Sets the maximum recursion depth for URI resolution, in the presence of
/// transitive dependencies. Setting this to a negative number disables the
/// limit, setting this to zero disables URI resolution entirely.
///
/// Returns whether the function was successful. If false is returned, retrieve
/// the error message with substrait_validator_get_last_error().
#[no_mangle]
pub extern "C" fn substrait_validator_config_max_uri_resolution_depth(
    config: *mut ConfigHandle,
    max_depth: i64,
) -> bool {
    // Check for nulls.
    if config.is_null() {
        set_last_error("received null configuration handle");
        return false;
    }

    // UNSAFE: unpack configuration handle. Assumes that the pointer was
    // created by substrait_validator_config_new(), or behavior is undefined.
    let config = unsafe { &mut (*config).config };

    // Update configuration and return success.
    if max_depth < 0 {
        config.set_max_uri_resolution_depth(None);
    } else if let Ok(max_depth) = usize::try_from(max_depth) {
        config.set_max_uri_resolution_depth(Some(max_depth));
    } else {
        set_last_error("specified depth is out of range");
        return false;
    }
    true
}

/// Parse/validation result handle.
pub struct ResultHandle {
    pub result: substrait_validator::ParseResult,
}

/// Parses the given byte buffer as a substrait.Plan message, using the given
/// configuration. If a null pointer is passed for the configuration, the
/// default configuration is used.
///
/// Returns a handle to the parse result. This handle must be freed using
/// substrait_validator_free() when it is no longer needed. Fails and returns
/// nullptr only if the incoming buffer is nullptr; any other failure to parse
/// or validate the buffer is embedded in the handle.
#[no_mangle]
pub extern "C" fn substrait_validator_parse(
    data: *const u8,
    size: u64,
    config: *const ConfigHandle,
) -> *mut ResultHandle {
    // Catch null pointers.
    if data.is_null() {
        set_last_error("received null input buffer");
        return std::ptr::null_mut();
    }

    // UNSAFE: convert the incoming buffer information into a slice.
    let data = unsafe { std::slice::from_raw_parts(data, size.try_into().unwrap()) };

    // Perform the actual parsing.
    let result = if config.is_null() {
        substrait_validator::parse(data, &substrait_validator::Config::default())
    } else {
        // UNSAFE: unpack configuration handle. Assumes that the pointer was
        // created by substrait_validator_config_new(), or behavior is undefined.
        substrait_validator::parse(data, unsafe { &(*config).config })
    };

    // Create a box to store the return value handle on the stack.
    let handle = Box::new(ResultHandle { result });

    // Convert the box to its raw pointer and relinquish ownership.
    Box::into_raw(handle)
}

/// Frees memory associated with a parse result handle. No-op if given a
/// nullptr.
#[no_mangle]
pub extern "C" fn substrait_validator_free(handle: *mut ResultHandle) {
    // Ignore null pointers.
    if handle.is_null() {
        return;
    }

    // UNSAFE: recover the box that we created the handle with and drop it.
    // Assumes that the pointer was created by substrait_validator_parse().
    let handle = unsafe { Box::from_raw(handle) };
    drop(handle);
}

/// Returns whether the given parse result handle refers to a valid (positive
/// return value), invalid (negative return value), or possibly valid plan
/// (0 return value).
#[no_mangle]
pub extern "C" fn substrait_validator_check(handle: *const ResultHandle) -> i32 {
    // UNSAFE: dereference the result handle. Assumes that the pointer was
    // created by substrait_validator_parse(), or that it is null (in which
    // case an exception is thrown safely).
    let handle = unsafe { handle.as_ref() };
    if handle.is_none() {
        return -1;
    }
    let result = &handle.as_ref().unwrap().result;

    // Perform the check.
    match result.check() {
        substrait_validator::Validity::Valid => 1,
        substrait_validator::Validity::MaybeValid => 0,
        substrait_validator::Validity::Invalid => -1,
    }
}

/// The guts for the export functions.
fn export(
    format: substrait_validator::export::Format,
    handle: *const ResultHandle,
    size: *mut u64,
) -> *mut u8 {
    // UNSAFE: dereference the result handle. Assumes that the pointer was
    // created by substrait_validator_parse(), or that it is null (in which
    // case an exception is thrown safely).
    let handle = unsafe { handle.as_ref() };
    if handle.is_none() {
        set_last_error("received null handle");
        return std::ptr::null_mut();
    }
    let result = &handle.as_ref().unwrap().result;

    // Create a byte vector as output. The first 16 bytes are reserved: we'll
    // store the length and capacity of the vector in there, and advance the
    // pointer beyond this length before passing the data to the user. This
    // allows us to fully recover the vector from just the returned pointer
    // later, which we need in order to drop it safely.
    let mut data: Vec<u8> = vec![0; 16];

    // Perform the actual export function.
    if let Err(e) = result.export(&mut data, format) {
        set_last_error(e.to_string());
        return std::ptr::null_mut();
    }

    // UNSAFE: pass the length to the user, if they wanted to know about it.
    // Assumes that the size pointer, if non-null, points to a writable and
    // appropriately aligned memory location.
    if let Some(size) = unsafe { size.as_mut() } {
        *size = (data.len() - 16).try_into().unwrap();
    }

    // Append a null character, to prevent pain and misery if the user treats
    // the buffer as a null-terminated string.
    data.push(0);

    // Save the length and capacity of the vector to the start of said
    // vector, so we can recover them later.
    let len: u64 = data.len().try_into().unwrap();
    data[..8].clone_from_slice(&len.to_ne_bytes());
    let capacity: u64 = data.capacity().try_into().unwrap();
    data[8..16].clone_from_slice(&capacity.to_ne_bytes());

    // Get the pointer to the vector, and relinquish ownership.
    let ptr = data.as_mut_ptr();
    std::mem::forget(data);

    // UNSAFE: advance the pointer beyond the bytes that we're using to store
    // the size of the vector. This assumes that advancing by 16 bytes doesn't
    // advance beyond the end of the buffer, which should not be possible, as
    // the buffer is at least 17 bytes long (8 bytes length, 8 bytes capacity,
    // and null termination byte).
    unsafe { ptr.add(16) }
}

/// Frees memory associated with an exported buffer. No-op if given a nullptr.
#[no_mangle]
pub extern "C" fn substrait_validator_free_exported(data: *mut u8) {
    // Don't do anything if the user passed nullptr.
    if data.is_null() {
        return;
    }

    // UNSAFE: recover the pointer to the vector data. Assumes that the pointer
    // was (ultimately) created using export(), in which case this just
    // reverses the pointer arithmetic done at the end of its body.
    let buffer_ptr = unsafe { data.sub(16) };

    // UNSAFE: recover the vector length from the first 8 bytes. Assumes that
    // these 8 bytes are readable.
    let length_ptr = buffer_ptr;
    let length = u64::from_ne_bytes(
        unsafe { std::slice::from_raw_parts(length_ptr, 8) }
            .try_into()
            .unwrap(),
    );
    let length = usize::try_from(length).unwrap();

    // UNSAFE: recover the vector capacity from the next 8 bytes. Assumes that
    // these 8 bytes are readable.
    let capacity_ptr = unsafe { buffer_ptr.add(8) };
    let capacity = u64::from_ne_bytes(
        unsafe { std::slice::from_raw_parts(capacity_ptr, 8) }
            .try_into()
            .unwrap(),
    );
    let capacity = usize::try_from(capacity).unwrap();

    // UNSAFE: recover the vector and drop it. Assumes that the recovered
    // pointer, length, and capacity do indeed form the raw parts of a valid
    // Vec.
    let vec = unsafe { Vec::from_raw_parts(buffer_ptr, length, capacity) };
    drop(vec);
}

/// Converts the given parse result to a multiline, null-terminated string,
/// where each line represents a diagnostic message. If size is non-null, the
/// length of the string (excluding null-termination byte) will be written to
/// it. The function will return nullptr upon failure, in which case
/// substrait_validator_get_last_error() can be used to retrieve an error
/// message. If the function succeeds, the returned pointer must eventually be
/// freed using substrait_validator_free_exported() in order to not leak
/// memory.
#[no_mangle]
pub extern "C" fn substrait_validator_export_diagnostics(
    handle: *const ResultHandle,
    size: *mut u64,
) -> *mut u8 {
    export(
        substrait_validator::export::Format::Diagnostics,
        handle,
        size,
    )
}

/// Same as substrait_validator_export_diagnostics(), but instead returns a
/// buffer filled with a HTML-based human-readable description of the parsed
/// plan.
#[no_mangle]
pub extern "C" fn substrait_validator_export_html(
    handle: *const ResultHandle,
    size: *mut u64,
) -> *mut u8 {
    export(substrait_validator::export::Format::Html, handle, size)
}

/// Same as substrait_validator_export_diagnostics(), but instead returns a
/// substrait.validator.Node message in its binary serialization format. The
/// buffer is null-terminated, but note that protobuf serialization is a binary
/// format, so you'll need to use the size argument to get an accurate size.
#[no_mangle]
pub extern "C" fn substrait_validator_export_proto(
    handle: *const ResultHandle,
    size: *mut u64,
) -> *mut u8 {
    export(substrait_validator::export::Format::Proto, handle, size)
}

/// Returns the version of the validator.
#[no_mangle]
pub extern "C" fn substrait_validator_version() -> *const libc::c_char {
    static VERSION: once_cell::sync::OnceCell<std::ffi::CString> = once_cell::sync::OnceCell::new();
    VERSION
        .get_or_init(|| std::ffi::CString::new(substrait_validator::version().to_string()).unwrap())
        .as_bytes_with_nul()
        .as_ptr() as *const libc::c_char
}

/// Returns the version of Substrait that the validator was built against.
#[no_mangle]
pub extern "C" fn substrait_validator_substrait_version() -> *const libc::c_char {
    static VERSION: once_cell::sync::OnceCell<std::ffi::CString> = once_cell::sync::OnceCell::new();
    VERSION
        .get_or_init(|| {
            std::ffi::CString::new(substrait_validator::substrait_version().to_string()).unwrap()
        })
        .as_bytes_with_nul()
        .as_ptr() as *const libc::c_char
}
