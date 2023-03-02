use encoding::all::GB18030;
use encoding::{DecoderTrap, Encoding};
use std::borrow::Cow;

pub fn set_cstr_from_str_truncate_i8(buffer: &mut [i8], text: &str) {
    let v = unsafe { std::slice::from_raw_parts_mut(buffer.as_ptr() as *mut u8, buffer.len()) };
    set_cstr_from_str_truncate(v, text)
}

pub fn set_cstr_from_str_truncate(buffer: &mut [u8], text: &str) {
    for (place, data) in buffer
        .split_last_mut()
        .expect("buffer len 0 in set_cstr_from_str_truncate")
        .1
        .iter_mut()
        .zip(text.as_bytes().iter())
    {
        *place = *data;
    }
    unsafe {
        *buffer.get_unchecked_mut(text.len()) = 0u8;
    }
}

/// 创建目录
pub fn check_make_dir(dir: &str) {
    match std::fs::create_dir_all(dir) {
        Ok(_) => (),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
            } else {
                panic!("创建dir={}目录失败 {}", dir, e);
            }
        }
    }
}

#[macro_export]
macro_rules! print_rsp_info {
    ($p:expr) => {
        if let Some(p) = $p {
            info!(
                "ErrorID={} Msg={}",
                p.ErrorID,
                gb18030_cstr_to_str_i8(&p.ErrorMsg).to_string()
            );
        }
    };
}

pub fn gb18030_cstr_to_str_i8(v: &[i8]) -> Cow<str> {
    let v = unsafe { std::slice::from_raw_parts(v.as_ptr() as *mut u8, v.len()) };
    gb18030_cstr_to_str(v)
}

pub fn gb18030_cstr_to_str(v: &[u8]) -> Cow<str> {
    let slice = v.split(|&c| c == 0u8).next().unwrap();
    if slice.is_ascii() {
        unsafe {
            return Cow::Borrowed::<str>(std::str::from_utf8_unchecked(slice));
        }
    }
    match GB18030.decode(slice, DecoderTrap::Replace) {
        Ok(s) => Cow::Owned(s),
        Err(e) => e,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
