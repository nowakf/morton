use std::ops::Range;

pub fn range_points(p1: (u16, u16), p2: (u16, u16)) -> Vec<(u16, u16)> {
    let range = range(p1, p2);
    let mut points = Vec::new();
    for &(start, end) in &range {
        for i in start..end {
            points.push(decode_2d(i));
        }
    }
    points
}
pub fn range_iter<'a>(p1: (u16, u16), p2: (u16, u16)) -> impl Iterator<Item=Range<u32>> + 'a {
    use std::cmp;
    let (p1, p2) = ((cmp::min(p1.0, p2.0), cmp::min(p1.1, p2.1)),
                    (cmp::max(p1.0, p2.0), cmp::max(p1.1, p2.1)));
    let (min, max) = (encode_2d(p1.0, p1.1), encode_2d(p2.0, p2.1));

    let mut frontier = vec![(min, max)];

    std::iter::from_fn(move || {
        while let Some((min, max)) = frontier.pop() {
            if min == max || is_contiguous(min, max) {
                return Some(Range{start: min, end: max})
            } else {
                let (mid_high, mid_low) = split(min, max);
                frontier.push((min, mid_low));
                frontier.push((mid_high, max));
            }
        }
        None
    })
}

pub fn range(p1: (u16, u16), p2: (u16, u16)) -> Vec<(u32, u32)> {
    use std::cmp;
    let (p1, p2) = ((cmp::min(p1.0, p2.0), cmp::min(p1.1, p2.1)),
                    (cmp::max(p1.0, p2.0), cmp::max(p1.1, p2.1)));
    let (min, max) = (encode_2d(p1.0, p1.1), encode_2d(p2.0, p2.1));

    let mut frontier = vec![(min, max)];
    let mut out = Vec::new();

    while let Some((min, max)) = frontier.pop() {
        if min == max {
            out.push((min, max));
            continue
        } if is_contiguous(min, max) {
            out.push((min, max));
        } else {
            let (mid_high, mid_low) = split(min, max);
            frontier.push((min, mid_low));
            frontier.push((mid_high, max));
        }
    }
    out
}

#[inline] 
fn is_contiguous(min: u32, max: u32) -> bool {
        let common_prefix_len = (min ^ max).leading_zeros();
           (min & (u32::MAX >> common_prefix_len)) == 0 
        && (max & (u32::MAX >> common_prefix_len)) == u32::MAX >> common_prefix_len
}
#[inline] 
fn split(min: u32, max: u32) -> (u32, u32) {
    let prefix_len = (min ^ max).leading_zeros();
    let mask = 0xAAAAAAAA >> prefix_len;
    let high = min ^ (min & mask);
    let low = max | mask;
    let low = low ^ (01 << (31 - prefix_len));
    let high = high | (01 << (31 - prefix_len));
    (high, low)
}




const MASK_2D_U32: [u32; 6] = [0xFFFFFFFF, 0x0000FFFF,
                               0x00FF00FF, 0x0F0F0F0F,
                               0x33333333, 0x55555555];

#[inline]
pub fn encode_2d(x: u16, y: u16) -> u32 { 
    split_by_second_bits(x as u32) |
    (split_by_second_bits(y as u32) << 01) 
}
#[inline]
pub fn decode_2d(i: u32) -> (u16, u16) {
    (get_second_bits(i) as u16, get_second_bits(i >> 01) as u16)
}
#[inline]
fn split_by_second_bits(x: u32) -> u32 {
        let mut x = x;
        x = (x | x << 16) & MASK_2D_U32[1];
        x = (x | x << 8)  & MASK_2D_U32[2];
        x = (x | x << 4)  & MASK_2D_U32[3];
        x = (x | x << 2)  & MASK_2D_U32[4];
        x = (x | x << 1)  & MASK_2D_U32[5];
        x
}
#[inline]
fn get_second_bits(x: u32) -> u32 {
        let mut x = x;
        x &= MASK_2D_U32[5];
        x = (x ^ (x >> 1))  & MASK_2D_U32[4];
        x = (x ^ (x >> 2))  & MASK_2D_U32[3];
        x = (x ^ (x >> 4))  & MASK_2D_U32[2];
        x = (x ^ (x >> 8))  & MASK_2D_U32[1];
        x = (x ^ (x >> 16)) & MASK_2D_U32[0];
        x
}

//not very good tests
#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::{is_contiguous, decode_2d, split};
    #[test]
    fn test_is_contiguous() {
        let testvals = [
            ((0b000000, 0b001111), true),
            ((0b001000, 0b100111), false),
            ((0b011000, 0b011111), true),
            ((0b100000, 0b100111), true),
            ((0b010110, 0b011101), false),
        ];
        for &((min, max), contiguous) in &testvals {
            assert_eq!(is_contiguous(min, max), contiguous);
        }

    }
    #[test]
    fn test_split_msb() {
        let testvals = [
            ((0b000000, 0b000111), (0b000011, 0b000100)),
            ((0b011000, 0b011111), (0b011011, 0b011100)), 
            ((0b100101, 0b111010), (0b101111, 0b110000)), 
            ((0b100000, 0b111111), (0b101111, 0b110000)),
            ((0b001010, 0b110101), (0b011111, 0b100000)),
            //
            ((0b100000, 0b101011), (0b100011, 0b101000)),
            ((0b110100, 0b111111), (0b110111, 0b111100)),
            ((0b001100, 0b101111), (0b001111, 0b100100)), 
            ((0b100100, 0b110111), (0b100111, 0b110000)), 
        ];
        let mut all_passed = true;
        for &((min, max), (emin, emax)) in &testvals {
            assert!(min <= max && emin <= emax);
            let (rmax, rmin) = split(min, max);
            if  emin != rmin || emax != rmax {
                all_passed = false;
            }
        }
        assert!(all_passed);

    }
}
