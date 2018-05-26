#[inline]
#[cfg(not(all(target_os = "nacl", target_arch = "le32")))]
pub fn black_box<T>(dummy: T) -> T {
    unsafe { asm!("" : : "r"(&dummy)) }
    dummy
}
