#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

use core::cmp::Ordering;

/// Key to strings comparison by natural (human-readable) sort.
///
/// This supports `no-std`.
#[derive(Debug)]
pub struct NaturalSortKey<'a> {
    inner: &'a str,
}

impl<'a> NaturalSortKey<'a> {
    pub fn from_str(input: &'a str) -> Self {
        NaturalSortKey { inner: input }
    }
}

impl<'a> PartialEq for NaturalSortKey<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<'a> Eq for NaturalSortKey<'a> {}

impl<'a> PartialOrd for NaturalSortKey<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for NaturalSortKey<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_iter = NaturalSortKeyIter::new(self.inner);
        let other_iter = NaturalSortKeyIter::new(other.inner);

        self_iter.cmp(other_iter)
    }
}

#[derive(Debug)]
struct NaturalSortKeyIter<'a> {
    inner: &'a str,

    next_cursor: usize,
}

impl<'a> NaturalSortKeyIter<'a> {
    fn new(input: &'a str) -> Self {
        NaturalSortKeyIter {
            inner: input,
            next_cursor: 0,
        }
    }
}

impl<'a> Iterator for NaturalSortKeyIter<'a> {
    type Item = NaturalSortPart<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner;
        let inner_size = self.inner.len();
        let range_start = self.next_cursor;

        if range_start >= inner_size {
            return None;
        }

        let mut part_kind: Option<NaturalSortPartKind> = None;

        let inner_bytes = inner.as_bytes();
        let mut cursor = range_start;
        while cursor < inner_size {
            let value = inner_bytes[cursor];

            let part_kind_at_cursor = NaturalSortPartKind::from_ascii(value);

            if let Some(part_kind) = &part_kind {
                if part_kind != &part_kind_at_cursor {
                    break;
                }
            } else {
                part_kind = Some(part_kind_at_cursor)
            }

            cursor += 1;
        }

        self.next_cursor = cursor;

        let part = &inner[range_start..cursor];
        let kind = part_kind?;
        let result = NaturalSortPart { part, kind };

        Some(result)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum NaturalSortPartKind {
    Number,
    String,
}

impl NaturalSortPartKind {
    fn from_ascii(chr: u8) -> Self {
        if chr.is_ascii_digit() {
            Self::Number
        } else {
            Self::String
        }
    }
}

#[derive(Debug)]
struct NaturalSortPart<'a> {
    part: &'a str,
    kind: NaturalSortPartKind,
}

impl<'a> PartialEq for NaturalSortPart<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a> Eq for NaturalSortPart<'a> {}

impl<'a> PartialOrd for NaturalSortPart<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for NaturalSortPart<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.kind == NaturalSortPartKind::Number) && (other.kind == NaturalSortPartKind::Number)
        {
            return Self::cmp_str_as_number(self.part, other.part);
        }

        Self::cmp_str_as_natural_str(self.part, other.part)
    }
}

impl<'a> NaturalSortPart<'a> {
    fn cmp_str_as_natural_str(a: &str, b: &str) -> Ordering {
        let a_natural_key = a.as_bytes().iter().map(Self::char_small_large_order_key);
        let b_natural_key = b.as_bytes().iter().map(Self::char_small_large_order_key);

        a_natural_key.cmp(b_natural_key)
    }

    fn char_small_large_order_key(c: &u8) -> u8 {
        if c.is_ascii_lowercase() {
            return c.to_ascii_uppercase();
        }

        if c.is_ascii_uppercase() {
            return c.to_ascii_lowercase();
        }

        *c
    }

    fn cmp_str_as_number(a: &str, b: &str) -> Ordering {
        let a_trimmed = Self::trim_head_zero(a).as_bytes();
        let b_trimmed = Self::trim_head_zero(b).as_bytes();

        if a_trimmed.len() == b_trimmed.len() {
            let length = a_trimmed.len();

            for i in 0..length {
                let p = a_trimmed[i].cmp(&b_trimmed[i]);

                if p != Ordering::Equal {
                    return p;
                }
            }

            let order_when_both_number_is_same = a.len().cmp(&b.len());
            return order_when_both_number_is_same;
        }

        a_trimmed.len().cmp(&b_trimmed.len())
    }

    fn trim_head_zero(v: &str) -> &str {
        v.trim_start_matches('0')
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::{NaturalSortKey, NaturalSortKeyIter, NaturalSortPart, NaturalSortPartKind};

    impl<'a> NaturalSortPart<'a> {
        fn as_num(part: &'a str) -> Self {
            NaturalSortPart {
                part,
                kind: NaturalSortPartKind::Number,
            }
        }

        fn as_str(part: &'a str) -> Self {
            NaturalSortPart {
                part,
                kind: NaturalSortPartKind::String,
            }
        }
    }

    #[test]
    fn sort_key_ord() {
        test_a_greater_than_b(
            NaturalSortKey::from_str("abc00001"),
            NaturalSortKey::from_str("abc2"),
        );
        test_a_greater_than_b(
            NaturalSortKey::from_str("abc1"),
            NaturalSortKey::from_str("abc00002"),
        );

        test_a_greater_than_b(
            NaturalSortKey::from_str("abc1"),
            NaturalSortKey::from_str("abc00001"),
        );

        test_a_equal_b(
            NaturalSortKey::from_str("abc1"),
            NaturalSortKey::from_str("abc1"),
        );
    }

    #[test]
    fn sort_key_str_ord() {
        test_a_greater_than_b(NaturalSortKey::from_str("a"), NaturalSortKey::from_str("A"));

        test_a_equal_b(NaturalSortKey::from_str("a"), NaturalSortKey::from_str("a"));
        test_a_equal_b(NaturalSortKey::from_str("A"), NaturalSortKey::from_str("A"));
    }

    #[test]
    fn sort_key_num_ord() {
        test_a_greater_than_b(
            NaturalSortKey::from_str("12"),
            NaturalSortKey::from_str("123"),
        );
        test_a_greater_than_b(
            NaturalSortKey::from_str("122"),
            NaturalSortKey::from_str("123"),
        );

        test_a_greater_than_b(
            NaturalSortKey::from_str("123"),
            NaturalSortKey::from_str("0000123"),
        );

        test_a_equal_b(
            NaturalSortKey::from_str("123"),
            NaturalSortKey::from_str("123"),
        );
    }

    #[test]
    fn sort_key_empty() {
        test_a_equal_b(NaturalSortKey::from_str(""), NaturalSortKey::from_str(""));
    }

    #[test]
    fn sort_key_from_str_parse() {
        assert_eq!(
            NaturalSortKeyIter::new("").collect::<Vec<NaturalSortPart>>(),
            Vec::from([]),
        );

        assert_eq!(
            NaturalSortKeyIter::new("abc").collect::<Vec<NaturalSortPart>>(),
            Vec::from([NaturalSortPart::as_str("abc")]),
        );

        assert_eq!(
            NaturalSortKeyIter::new("123").collect::<Vec<NaturalSortPart>>(),
            Vec::from([NaturalSortPart::as_num("123")]),
        );

        assert_eq!(
            NaturalSortKeyIter::new("abc123").collect::<Vec<NaturalSortPart>>(),
            Vec::from([
                NaturalSortPart::as_str("abc"),
                NaturalSortPart::as_num("123"),
            ]),
        );

        assert_eq!(
            NaturalSortKeyIter::new("123abc").collect::<Vec<NaturalSortPart>>(),
            Vec::from([
                NaturalSortPart::as_num("123"),
                NaturalSortPart::as_str("abc"),
            ]),
        );

        assert_eq!(
            NaturalSortKeyIter::new("abc123abc").collect::<Vec<NaturalSortPart>>(),
            Vec::from([
                NaturalSortPart::as_str("abc"),
                NaturalSortPart::as_num("123"),
                NaturalSortPart::as_str("abc"),
            ]),
        );

        assert_eq!(
            NaturalSortKeyIter::new("123abc123").collect::<Vec<NaturalSortPart>>(),
            Vec::from([
                NaturalSortPart::as_num("123"),
                NaturalSortPart::as_str("abc"),
                NaturalSortPart::as_num("123"),
            ]),
        );
    }

    fn test_a_greater_than_b<T: Ord + Debug>(a_small: T, b_large: T) {
        assert_ne!(a_small, b_large);
        assert_eq!(a_small != b_large, true);

        assert_eq!(a_small < b_large, true);
        assert_eq!(b_large > a_small, true);

        assert_eq!(a_small <= b_large, true);
        assert_eq!(b_large >= a_small, true);
    }

    fn test_a_equal_b<T: Ord + Debug>(a: T, b: T) {
        assert_eq!(a, b);
        assert_eq!(a != b, false);

        assert_eq!(a < b, false);
        assert_eq!(b > a, false);

        assert_eq!(a <= b, true);
        assert_eq!(b >= a, true);
    }
}
