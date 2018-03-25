pub mod benching;
pub mod init;
pub mod slotmap;
pub mod consume_spsc;
pub mod replace_spsc;

pub fn new_boxed_slice<T>(len: usize) -> Box<[T]> {
    let mut items: Vec<T> = Vec::<T>::with_capacity(len);
    unsafe {
        items.set_len(len);
    }
    items.into_boxed_slice()
}
