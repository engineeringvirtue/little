//we makin a frahemwurk boyos
//https://stackoverflow.com/questions/40332112/how-to-declare-typed-bitflags-in-rust
//copypasta :D

#[macro_export]
macro_rules! struct_bitflag_impl {
    ($p:ident) => {
        // Possible additions:
        // * left/right shift.
        // * Deref to forward methods to the underlying type.

        impl ::std::ops::BitAnd for $p {
            type Output = $p;
            fn bitand(self, _rhs: $p) -> $p { $p(self.0 & _rhs.0) }
        }
        impl ::std::ops::BitOr for $p {
            type Output = $p;
            fn bitor(self, _rhs: $p) -> $p { $p(self.0 | _rhs.0) }
        }
        impl ::std::ops::BitXor for $p {
            type Output = $p;
            fn bitxor(self, _rhs: $p) -> $p { $p(self.0 ^ _rhs.0) }
        }

        impl ::std::ops::Not for $p {
            type Output = $p;
            fn not(self) -> $p { $p(!self.0) }
        }

        impl ::std::ops::BitAndAssign for $p {
            fn bitand_assign(&mut self, _rhs: $p) { self.0 &= _rhs.0; }
        }
        impl ::std::ops::BitOrAssign for $p {
            fn bitor_assign(&mut self, _rhs: $p) { self.0 |= _rhs.0; }
        }
        impl ::std::ops::BitXorAssign for $p {
            fn bitxor_assign(&mut self, _rhs: $p) { self.0 ^= _rhs.0; }
        }

        // Other operations needed to be generally usable.
        impl PartialEq for $p {
            fn eq(&self, other: &$p) -> bool { self.0 == other.0 }
        }

        impl Copy for $p { }
        impl Clone for $p {
            fn clone(&self) -> $p { $p(self.0) }
        }
    }
}