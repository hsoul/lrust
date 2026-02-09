//! Ltask integration for lrust.
//! Calls the C API from ltask_ext.c: send_integer_message, send_message (no pool; C uses global).

use std::os::raw::c_char;

// C API (ltask_ext.h). Use "C-unwind" to match lib.rs declarations and avoid clashing.
unsafe extern "C-unwind" {
    fn send_integer_message(type_: u8, receiver: u32, session: i64, val: isize);
    fn send_message(type_: u8, receiver: u32, session: i64, data: *const c_char, len: usize);
    #[link_name = "ltask_push_log"]
    fn c_ltask_push_log(sender_id: u32, data: *const c_char, len: usize);
}

/// Seri format constants (must match lua-seri.c).
const TYPE_SHORT_STRING: u8 = 3;
const TYPE_LONG_STRING: u8 = 4;
const MAX_COOKIE: usize = 32;

/// Encode one string in Lua seri format (short or long string). Appends to `out`.
fn seri_encode_string(out: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    let len = bytes.len();
    if len < MAX_COOKIE {
        out.push(TYPE_SHORT_STRING | (len as u8) << 3);
        out.extend_from_slice(bytes);
    } else if len < 0x10000 {
        out.push(TYPE_LONG_STRING | 2 << 3);
        out.extend_from_slice(&(len as u16).to_le_bytes());
        out.extend_from_slice(bytes);
    } else {
        out.push(TYPE_LONG_STRING | 4 << 3);
        out.extend_from_slice(&(len as u32).to_le_bytes());
        out.extend_from_slice(bytes);
    }
}

/// Pack (level, message) as Lua seri so that ltask.unpack_remove(msg, sz) returns level, message.
fn seri_pack_log(level: &str, message: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(level.len() + message.len() + 8);
    seri_encode_string(&mut out, level);
    seri_encode_string(&mut out, message);
    out
}

/// Send a heap-allocated value to Lua (same semantics as moon_send).
/// Lua side decodes the pointer via the registered protocol decoder (e.g. c.decode).
pub fn ltask_send<T>(protocol_type: u8, owner: u32, session: i64, res: T) {
    if session == 0 {
        return;
    }
    let ptr = Box::into_raw(Box::new(res));
    unsafe {
        send_integer_message(protocol_type, owner, session, ptr as isize);
    }
}

/// Send raw bytes to an ltask service (C send_message).
pub fn ltask_send_bytes(protocol_type: u8, owner: u32, session: i64, data: &[u8]) {
    if session == 0 {
        return;
    }
    unsafe {
        send_message(
            protocol_type,
            owner,
            session,
            data.as_ptr() as *const c_char,
            data.len(),
        );
    }
}

/// Push async log from extension into ltask log queue (same path as ltask.pushlog).
/// `sender_id`: service id to attribute the log to (e.g. current service that triggered the call).
/// `data`: log payload (e.g. level byte + message string, same format as Lua pushlog).
pub fn ltask_push_log(sender_id: u32, data: &[u8]) {
    if data.is_empty() {
        return;
    }
    unsafe {
        c_ltask_push_log(sender_id, data.as_ptr() as *const c_char, data.len());
    }
}

/// Push log from extension: encodes (level, message) in Lua seri format and sends via ltask_push_log.
/// Root/logger will unpack with ltask.unpack_remove and get level + message.
/// Buffer format must match lua-seri: [4-byte length (LE)][seri data]; C seri_unpack reads length first.
pub fn ltask_log(sender_id: u32, level: &str, message: &str) {
    let payload = seri_pack_log(level, message);
    if payload.is_empty() {
        return;
    }
    let len = payload.len() as u32;
    let mut buf = Vec::with_capacity(4 + payload.len());
    buf.extend_from_slice(&len.to_le_bytes());
    buf.extend_from_slice(&payload);
    ltask_push_log(sender_id, &buf);
}
