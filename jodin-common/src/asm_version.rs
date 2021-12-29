//! The jodin asm version string

/// The current version of the jodin asm
#[derive(Debug)]
pub struct Version;
const VERSION_STRING: &str = "1.0";

impl Version {
    /// Gets the jodin asm bytecode string
    pub const fn version_string(&self) -> &str {
        VERSION_STRING
    }

    /// Gets the 8-byte magic number for this version number
    pub fn to_magic_number(&self) -> u64 {
        let version_string_full = format!("jodin_asm_version_{}", VERSION_STRING);
        let mut sum = 0u64;
        for (index, byte) in version_string_full.bytes().enumerate() {
            let mult = index as u64 + 1;
            let pow = u32::wrapping_sub(31, index as u32);
            let add = u64::wrapping_mul((byte as u64).wrapping_pow(pow), mult);
            sum = u64::wrapping_add(sum, add);
        }
        sum
    }

    /// Check whether the given magic number if valid for this bytecode version
    pub fn verify_magic_number(&self, number: u64) -> bool {
        self.to_magic_number() == number
    }
}
