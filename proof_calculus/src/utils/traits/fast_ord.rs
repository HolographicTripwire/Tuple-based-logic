use std::cmp::Ordering;

pub trait FastOrd {
    fn fast_cmp(&self, other: &Self) -> Ordering;
}
impl FastOrd for u8 {    #[inline] fn fast_cmp(&self, other: &Self) -> Ordering { self.cmp(other) } }
impl FastOrd for u16 {   #[inline] fn fast_cmp(&self, other: &Self) -> Ordering { self.cmp(other) } }
impl FastOrd for u32 {   #[inline] fn fast_cmp(&self, other: &Self) -> Ordering { self.cmp(other) } }
impl FastOrd for u64 {   #[inline] fn fast_cmp(&self, other: &Self) -> Ordering { self.cmp(other) } }
impl FastOrd for u128 {  #[inline] fn fast_cmp(&self, other: &Self) -> Ordering { self.cmp(other) } }
impl FastOrd for usize { #[inline] fn fast_cmp(&self, other: &Self) -> Ordering { self.cmp(other) } }
impl FastOrd for i8 {    #[inline] fn fast_cmp(&self, other: &Self) -> Ordering { self.cmp(other) } }
impl FastOrd for i16 {   #[inline] fn fast_cmp(&self, other: &Self) -> Ordering { self.cmp(other) } }
impl FastOrd for i32 {   #[inline] fn fast_cmp(&self, other: &Self) -> Ordering { self.cmp(other) } }
impl FastOrd for i64 {   #[inline] fn fast_cmp(&self, other: &Self) -> Ordering { self.cmp(other) } }
impl FastOrd for i128 {  #[inline] fn fast_cmp(&self, other: &Self) -> Ordering { self.cmp(other) } }
impl FastOrd for isize { #[inline] fn fast_cmp(&self, other: &Self) -> Ordering { self.cmp(other) } }

pub fn fastcmp_for_sorted_slices<T: FastOrd>(s1: &[T],s2: &[T]) -> Ordering {
    match s1.len().partial_cmp(&s2.len()) {
        Some(Ordering::Equal) => {
            for i in 0..s1.len() {
                match &s1[i].fast_cmp(&s2[i]) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => {}
                }
            }
            Ordering::Equal
        },
        Some(nonequal_order) => { nonequal_order },
        None => { panic!{"Performed partial_cmp on two usize values and got None"} }
    }
}