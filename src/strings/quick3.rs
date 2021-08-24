#![allow(clippy::many_single_char_names)]
use crate::sort;
use crate::strings::util;
use std::cmp::Ordering;
use std::marker::PhantomData;

const CUTOFF: usize = 15; // cutoff to insertion sort

/// The Quick3String provides static methods for sorting an
/// array of strings using 3-way radix quicksort.
pub struct Quick3String<T> {
    _marker: PhantomData<T>,
}

/// The Quick3Way provides static methods for sorting an
/// array using quicksort with 3-way partitioning.
pub struct Quick3Way<T> {
    _marker: PhantomData<T>,
}

impl<T> Quick3String<T>
where
    T: AsRef<str> + Copy,
{
    /// Rearranges the array of strings in ascending order.
    pub fn sort(a: &mut [T]) {
        let n = a.len();
        Self::do_sort(a, 0, n.saturating_sub(1), 0);
    }

    fn do_sort(a: &mut [T], lo: usize, hi: usize, d: usize) {
        if hi <= lo + CUTOFF {
            sort::insert::sort_dth(a, lo, hi, d);
            return;
        }

        let mut lt = lo;
        let mut gt = hi;
        let v = util::char_at(a[lo].as_ref(), d);
        let mut i = lo + 1;
        while i <= gt {
            let t = util::char_at(a[i].as_ref(), d);
            match t.cmp(&v) {
                Ordering::Less => {
                    a.swap(lt, i);
                    lt += 1;
                    i += 1;
                }
                Ordering::Equal => i += 1,
                Ordering::Greater => {
                    a.swap(i, gt);
                    gt -= 1;
                }
            }
        }

        // a[lo..lt-1] < v = a[lt..gt] < a[gt+1..hi]
        Self::do_sort(a, lo, lt.saturating_sub(1), d);
        if v >= 0 {
            Self::do_sort(a, lt, gt, d + 1);
        }
        Self::do_sort(a, gt + 1, hi, d);
    }
}

impl<T> Quick3Way<T>
where
    T: Ord + Clone,
{
    pub fn sort(a: &mut [T]) {
        let n = a.len();
        Self::do_sort(a, 0, n.saturating_sub(1));
    }

    fn do_sort(a: &mut [T], lo: usize, hi: usize) {
        if hi <= lo {
            return;
        }

        let mut lt = lo;
        let mut gt = hi;
        let v = a[lo].clone();
        let mut i = lo + 1;
        while i <= gt {
            match a[i].cmp(&v) {
                Ordering::Less => {
                    a.swap(lt, i);
                    lt += 1;
                    i += 1;
                }
                Ordering::Equal => i += 1,
                Ordering::Greater => {
                    a.swap(i, gt);
                    gt -= 1;
                }
            }
        }

        Self::do_sort(a, lo, lt.saturating_sub(1));
        Self::do_sort(a, gt + 1, hi);
    }
}
