use crate::EmEnv;

pub fn confstr(_ctx: &mut EmEnv, _name: i32, _buf_pointer: i32, _len: i32) -> i32 {
    debug!("unistd::confstr({}, {}, {})", _name, _buf_pointer, _len);
    0
}
