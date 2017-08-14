extern crate libc;
extern crate libz_sys;
extern crate openssl_sys;

use libc::{c_int, size_t, c_void, c_char, c_long, c_uchar, c_uint, c_ulong};
use libc::ssize_t;

// It seems not to matter that ssh_session is really a struct
pub enum ssh_session {}
// It seems not to matter that ssh_channel is really a struct
pub enum ssh_channel {}

#[repr(C)]
#[allow(non_snake_case)]
pub enum ssh_options_e {
  SSH_OPTIONS_HOST,
  SSH_OPTIONS_PORT,
  SSH_OPTIONS_PORT_STR,
  SSH_OPTIONS_FD,
  SSH_OPTIONS_USER,
  SSH_OPTIONS_SSH_DIR,
  SSH_OPTIONS_IDENTITY,
  SSH_OPTIONS_ADD_IDENTITY,
  SSH_OPTIONS_KNOWNHOSTS,
  SSH_OPTIONS_TIMEOUT,
  SSH_OPTIONS_TIMEOUT_USEC,
  SSH_OPTIONS_SSH1,
  SSH_OPTIONS_SSH2,
  SSH_OPTIONS_LOG_VERBOSITY,
  SSH_OPTIONS_LOG_VERBOSITY_STR,
  SSH_OPTIONS_CIPHERS_C_S,
  SSH_OPTIONS_CIPHERS_S_C,
  SSH_OPTIONS_COMPRESSION_C_S,
  SSH_OPTIONS_COMPRESSION_S_C,
  SSH_OPTIONS_PROXYCOMMAND,
  SSH_OPTIONS_BINDADDR,
  SSH_OPTIONS_STRICTHOSTKEYCHECK,
  SSH_OPTIONS_COMPRESSION,
  SSH_OPTIONS_COMPRESSION_LEVEL,
  SSH_OPTIONS_KEY_EXCHANGE,
  SSH_OPTIONS_HOSTKEYS,
  SSH_OPTIONS_GSSAPI_SERVER_IDENTITY,
  SSH_OPTIONS_GSSAPI_CLIENT_IDENTITY,
  SSH_OPTIONS_GSSAPI_DELEGATE_CREDENTIALS,
  SSH_OPTIONS_HMAC_C_S,
  SSH_OPTIONS_HMAC_S_C
}

extern {
    pub fn ssh_new() -> ssh_session;
    pub fn ssh_free(session: *mut ssh_session) -> c_void;
    pub fn ssh_disconnect(session: *mut ssh_session) -> c_void;

    pub fn ssh_channel_new(session: *mut ssh_session) -> ssh_channel;
    pub fn ssh_channel_open_session(channel: *mut ssh_channel) -> c_int;
    pub fn ssh_channel_close(channel: *mut ssh_channel) -> c_int;
    pub fn ssh_channel_free(channel: *mut ssh_channel) -> c_void;

    pub fn ssh_channel_read(channel: *mut ssh_channel, dest: *mut c_void, count: c_uint, is_stderr: c_int);
    pub fn ssh_channel_request_exec(channel: *mut ssh_channel, cmd: *mut c_char) -> c_int;
    pub fn ssh_channel_send_eof(channel: *mut ssh_channel) -> c_int;

    pub fn ssh_finalize() -> c_int;
}

#[test]
fn base_test() {
    unsafe {
      let mut my_ssh_session = ssh_new();
      ssh_free(&mut my_ssh_session);
    }
}
