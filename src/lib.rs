#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::approx_constant)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const AV_CH_FRONT_LEFT: libc::c_ulonglong = 1;
pub const AV_CH_FRONT_RIGHT: libc::c_ulonglong = 2;
pub const AV_CH_FRONT_CENTER: libc::c_ulonglong = 4;
pub const AV_CH_LOW_FREQUENCY: libc::c_ulonglong = 8;
pub const AV_CH_BACK_LEFT: libc::c_ulonglong = 16;
pub const AV_CH_BACK_RIGHT: libc::c_ulonglong = 32;
pub const AV_CH_FRONT_LEFT_OF_CENTER: libc::c_ulonglong = 64;
pub const AV_CH_FRONT_RIGHT_OF_CENTER: libc::c_ulonglong = 128;
pub const AV_CH_BACK_CENTER: libc::c_ulonglong = 256;
pub const AV_CH_SIDE_LEFT: libc::c_ulonglong = 512;
pub const AV_CH_SIDE_RIGHT: libc::c_ulonglong = 1024;
pub const AV_CH_TOP_CENTER: libc::c_ulonglong = 2048;
pub const AV_CH_TOP_FRONT_LEFT: libc::c_ulonglong = 4096;
pub const AV_CH_TOP_FRONT_CENTER: libc::c_ulonglong = 8192;
pub const AV_CH_TOP_FRONT_RIGHT: libc::c_ulonglong = 16384;
pub const AV_CH_TOP_BACK_LEFT: libc::c_ulonglong = 32768;
pub const AV_CH_TOP_BACK_CENTER: libc::c_ulonglong = 65536;
pub const AV_CH_TOP_BACK_RIGHT: libc::c_ulonglong = 131072;
pub const AV_CH_STEREO_LEFT: libc::c_ulonglong = 536870912;
pub const AV_CH_STEREO_RIGHT: libc::c_ulonglong = 1073741824;
pub const AV_CH_WIDE_LEFT: libc::c_ulonglong = 2147483648;
pub const AV_CH_WIDE_RIGHT: libc::c_ulonglong = 4294967296;
pub const AV_CH_SURROUND_DIRECT_LEFT: libc::c_ulonglong = 8589934592;
pub const AV_CH_SURROUND_DIRECT_RIGHT: libc::c_ulonglong = 17179869184;
pub const AV_CH_LOW_FREQUENCY_2: libc::c_ulonglong = 34359738368;
pub const AV_CH_TOP_SIDE_LEFT: libc::c_ulonglong = 68719476736;
pub const AV_CH_TOP_SIDE_RIGHT: libc::c_ulonglong = 137438953472;
pub const AV_CH_BOTTOM_FRONT_CENTER: libc::c_ulonglong = 274877906944;
pub const AV_CH_BOTTOM_FRONT_LEFT: libc::c_ulonglong = 549755813888;
pub const AV_CH_BOTTOM_FRONT_RIGHT: libc::c_ulonglong = 1099511627776;
pub const AV_CH_LAYOUT_MONO: libc::c_ulonglong = 4;
pub const AV_CH_LAYOUT_STEREO: libc::c_ulonglong = 3;
pub const AV_CH_LAYOUT_2POINT1: libc::c_ulonglong = 11;
pub const AV_CH_LAYOUT_2_1: libc::c_ulonglong = 259;
pub const AV_CH_LAYOUT_SURROUND: libc::c_ulonglong = 7;
pub const AV_CH_LAYOUT_3POINT1: libc::c_ulonglong = 15;
pub const AV_CH_LAYOUT_4POINT0: libc::c_ulonglong = 263;
pub const AV_CH_LAYOUT_4POINT1: libc::c_ulonglong = 271;
pub const AV_CH_LAYOUT_2_2: libc::c_ulonglong = 1539;
pub const AV_CH_LAYOUT_QUAD: libc::c_ulonglong = 51;
pub const AV_CH_LAYOUT_5POINT0: libc::c_ulonglong = 1543;
pub const AV_CH_LAYOUT_5POINT1: libc::c_ulonglong = 1551;
pub const AV_CH_LAYOUT_5POINT0_BACK: libc::c_ulonglong = 55;
pub const AV_CH_LAYOUT_5POINT1_BACK: libc::c_ulonglong = 63;
pub const AV_CH_LAYOUT_6POINT0: libc::c_ulonglong = 1799;
pub const AV_CH_LAYOUT_6POINT0_FRONT: libc::c_ulonglong = 1731;
pub const AV_CH_LAYOUT_HEXAGONAL: libc::c_ulonglong = 311;
pub const AV_CH_LAYOUT_6POINT1: libc::c_ulonglong = 1807;
pub const AV_CH_LAYOUT_6POINT1_BACK: libc::c_ulonglong = 319;
pub const AV_CH_LAYOUT_6POINT1_FRONT: libc::c_ulonglong = 1739;
pub const AV_CH_LAYOUT_7POINT0: libc::c_ulonglong = 1591;
pub const AV_CH_LAYOUT_7POINT0_FRONT: libc::c_ulonglong = 1735;
pub const AV_CH_LAYOUT_7POINT1: libc::c_ulonglong = 1599;
pub const AV_CH_LAYOUT_7POINT1_WIDE: libc::c_ulonglong = 1743;
pub const AV_CH_LAYOUT_7POINT1_WIDE_BACK: libc::c_ulonglong = 255;
pub const AV_CH_LAYOUT_OCTAGONAL: libc::c_ulonglong = 1847;
pub const AV_CH_LAYOUT_HEXADECAGONAL: libc::c_ulonglong = 6442710839;
pub const AV_CH_LAYOUT_STEREO_DOWNMIX: libc::c_ulonglong = 1610612736;
pub const AV_CH_LAYOUT_22POINT2: libc::c_ulonglong = 2164663779327;

#[macro_use]
mod avutil;
pub use avutil::*;
