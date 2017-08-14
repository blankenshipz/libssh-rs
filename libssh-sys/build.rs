extern crate pkg_config;
extern crate cmake;

#[cfg(target_env = "msvc")]
extern crate vcpkg;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf, Path};
use std::process::Command;

fn main() {
    register_dep("OPENSSL");

    if let Ok(lib) = pkg_config::find_library("libssh") {
        for path in &lib.include_paths {
            println!("cargo:include={}", path.display());
        }
        return
    }
}

fn register_dep(dep: &str) {
    if let Some(s) = env::var_os(&format!("DEP_{}_ROOT", dep)) {
        prepend("PKG_CONFIG_PATH", Path::new(&s).join("lib/pkgconfig"));
        return
    }
    if let Some(s) = env::var_os(&format!("DEP_{}_INCLUDE", dep)) {
        let root = Path::new(&s).parent().unwrap();
        env::set_var(&format!("DEP_{}_ROOT", dep), root);
        let path = root.join("lib/pkgconfig");
        if path.exists() {
            prepend("PKG_CONFIG_PATH", path);
            return
        }
    }
}

fn prepend(var: &str, val: PathBuf) {
    let prefix = env::var(var).unwrap_or(String::new());
    let mut v = vec![val];
    v.extend(env::split_paths(&prefix));
    env::set_var(var, &env::join_paths(v).unwrap());
}
