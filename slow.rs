#[derive(Copy, Clone)]
pub struct FieldElement(pub (crate) [u64; 5]);

impl Default for FieldElement {
    #[inline(always)]
    fn default() -> Self {
        FieldElement([ 1, 0, 0, 0, 0 ])
    }
}

macro_rules! impl_basepoint_table {
    (Name = $name:ident, LookupTable = $table:ident) => {
        #[derive(Clone)]
        pub struct $name(pub(crate) [$table<FieldElement>; 32]);

        impl $name {
            #[inline(never)]
            pub fn create() -> $name {
                $name([$table::default(); 32])
            }
        }
    }
}

impl_basepoint_table! {Name = EdwardsBasepointTableRadix16, LookupTable = LookupTableRadix16}
impl_basepoint_table! {Name = EdwardsBasepointTableRadix32, LookupTable = LookupTableRadix32}
impl_basepoint_table! {Name = EdwardsBasepointTableRadix64, LookupTable = LookupTableRadix64}
impl_basepoint_table! {Name = EdwardsBasepointTableRadix128, LookupTable = LookupTableRadix128}
impl_basepoint_table! {Name = EdwardsBasepointTableRadix256, LookupTable = LookupTableRadix256}

macro_rules! impl_lookup_table {
    (Name = $name:ident, Size = $size:expr) => {

        #[derive(Copy, Clone)]
        pub struct $name<T>(pub(crate) [T; $size]);

        impl<T: Copy + Default> Default for $name<T> {
            #[inline(always)]
            fn default() -> $name<T> {
                $name([T::default(); $size])
            }
        }

    }
}

impl_lookup_table! {Name = LookupTableRadix16,  Size =   8}
impl_lookup_table! {Name = LookupTableRadix32,  Size =  16}
impl_lookup_table! {Name = LookupTableRadix64,  Size =  32}
impl_lookup_table! {Name = LookupTableRadix128, Size =  64}
impl_lookup_table! {Name = LookupTableRadix256, Size = 128}
