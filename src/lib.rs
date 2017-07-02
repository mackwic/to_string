//!
//! # Example:
//!
//! ```rust
//! use to_string::*;
//! assert_eq!("0b1111000", 120.to_bin());
//! assert_eq!("0xcafe", 51966.to_hexa());
//! assert_eq!("0xdeadbef0", (-559038736).to_hexa());
//! assert_eq!("0o1747", 999.to_octal_string());
//! assert_eq!("4.21e1", (42.1).to_exp());
//! ```


use std::fmt::{Debug, Binary, LowerHex, Octal, LowerExp, Pointer};

pub trait BinaryDisplay {
    /// ```
    /// use to_string::BinaryDisplay;
    /// assert_eq!("0b0", 0.to_binary_string())
    /// ```
    fn to_binary_string(&self) -> String;
    /// ```
    /// use to_string::BinaryDisplay;
    /// assert_eq!("0b0", 0.to_bin())
    /// ```
    fn to_bin(&self) -> String { self.to_binary_string() }
}

impl<T : Binary> BinaryDisplay for T {
    fn to_binary_string(&self) -> String {
        format!("{:#b}", self)
    }
}

pub trait HexaDisplay {
    /// ```
    /// use to_string::HexaDisplay;
    /// assert_eq!("0x12", 0x12.to_hexa_string());
    /// assert_eq!("0xffffffee", (-0x12).to_hexa_string())
    /// ```
    fn to_hexa_string(&self) -> String;
    /// ```
    /// use to_string::HexaDisplay;
    /// assert_eq!("0x12", 0x12.to_hexa());
    /// assert_eq!("0xffffffee", (-0x12).to_hexa())
    /// ```
    fn to_hexa(&self) -> String { self.to_hexa_string() }
}

impl<T : LowerHex> HexaDisplay for T {
    fn to_hexa_string(&self) -> String {
        format!("{:#x}", self)
    }
}

pub trait OctalDisplay {
    /// ```
    /// use to_string::OctalDisplay;
    /// assert_eq!("0o12", 0o12.to_octal_string())
    /// ```
    fn to_octal_string(&self) -> String;
    /// ```
    /// use to_string::OctalDisplay;
    /// assert_eq!("0o12", 0o12.to_octal())
    /// ```
    fn to_octal(&self) -> String { self.to_octal_string() }

}

impl<T : Octal> OctalDisplay for T {
    fn to_octal_string(&self) -> String {
        format!("{:#o}", self)
    }
}

pub trait ExpDisplay {
    /// ```
    /// use to_string::ExpDisplay;
    /// assert_eq!("4.21e1", (42.1).to_exp_string())
    /// ```
    fn to_exp_string(&self) -> String;
    /// ```
    /// use to_string::ExpDisplay;
    /// assert_eq!("4.21e1", (42.1).to_exp())
    /// ```
    fn to_exp(&self) -> String { self.to_exp_string() }
}

impl<T : LowerExp> ExpDisplay for T {
    fn to_exp_string(&self) -> String {
        format!("{:#e}", self)
    }
}

pub trait PointerDisplay {
    /// ```
    /// use to_string::PointerDisplay;
    /// let val = 12;
    /// println!("{}", (&val).to_pointer()) // something like "0x00007fff5af30fa8"
    /// ```
    fn to_pointer_string(&self) -> String;
    /// ```
    /// use to_string::PointerDisplay;
    /// let val = 12;
    /// println!("{}", (&val).to_pointer()) // something like "0x00007fff5af30fa8"
    /// ```
    fn to_pointer(&self) -> String { self.to_pointer_string() }
}

impl<T : Pointer> PointerDisplay for T {
    fn to_pointer_string(&self) -> String {
        format!("{:#p}", self)
    }
}

pub trait DebugDisplay {
    /// ```
    /// use to_string::DebugDisplay;
    /// assert_eq!("12", 12.to_debug_string())
    /// ```
    fn to_debug_string(&self) -> String;
    /// ```
    /// use to_string::DebugDisplay;
    /// assert_eq!("12", 12.to_debug_string())
    /// ```
    fn to_debug(&self) -> String { self.to_debug_string() }
}

impl<T : Debug> DebugDisplay for T {
    fn to_debug_string(&self) -> String {
        format!("{:#?}", self)
    }
}

