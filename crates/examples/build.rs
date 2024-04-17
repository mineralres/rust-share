use std::env;

fn main() {
    if cfg!(windows) {
        let key = "PATH";
        match env::var(key) {
            Ok(val) => {
                let path = format!("{val};crates\\ctp_futures\\v_current");
                println!("cargo:rustc-env=PATH={path}");
            }
            Err(e) => println!("couldn't interpret {key}: {e}"),
        }
    } else {
        println!("cargo:rustc-env=LD_LIBRARY_PATH=crates/ctp_futures/v_current");
    }
}
