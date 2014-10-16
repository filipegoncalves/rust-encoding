// AUTOGENERATED FROM index-iso-8859-10.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-iso-8859-10.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 260, 274, 290, 298, 296, 310, 167, 315, 272, 352, 358, 381,
    173, 362, 330, 176, 261, 275, 291, 299, 297, 311, 183, 316, 273, 353, 359,
    382, 8213, 363, 331, 256, 193, 194, 195, 196, 197, 198, 302, 268, 201, 280,
    203, 278, 205, 206, 207, 208, 325, 332, 211, 212, 213, 214, 360, 216, 370,
    218, 219, 220, 221, 222, 223, 257, 225, 226, 227, 228, 229, 230, 303, 269,
    233, 281, 235, 279, 237, 238, 239, 240, 326, 333, 243, 244, 245, 246, 361,
    248, 371, 250, 251, 252, 253, 254, 312,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

static BACKWARD_TABLE_LOWER: &'static [u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 129, 130, 131, 132, 133,
    134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148,
    149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 0, 0, 0, 0, 0,
    0, 167, 0, 0, 0, 0, 0, 173, 0, 0, 176, 0, 0, 0, 0, 0, 0, 183, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 193, 194, 195, 196, 197, 198, 0, 0, 201, 0, 203, 0, 205,
    206, 207, 208, 0, 0, 211, 212, 213, 214, 0, 216, 0, 218, 219, 220, 221,
    222, 223, 0, 225, 226, 227, 228, 229, 230, 0, 0, 233, 0, 235, 0, 237, 238,
    239, 240, 0, 0, 243, 244, 245, 246, 0, 248, 0, 250, 251, 252, 253, 254, 0,
    192, 224, 0, 0, 161, 177, 0, 0, 0, 0, 0, 0, 200, 232, 0, 0, 169, 185, 162,
    178, 0, 0, 204, 236, 202, 234, 0, 0, 0, 0, 0, 0, 0, 0, 163, 179, 0, 0, 0,
    0, 165, 181, 164, 180, 0, 0, 199, 231, 0, 0, 0, 0, 0, 0, 166, 182, 255, 0,
    0, 168, 184, 0, 0, 0, 0, 0, 0, 0, 0, 209, 241, 0, 0, 0, 175, 191, 210, 242,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 186, 0, 0, 0, 0,
    171, 187, 215, 247, 174, 190, 0, 0, 0, 0, 0, 0, 217, 249, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 172, 188, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 189, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

static BACKWARD_TABLE_UPPER: &'static [u16] = &[
    0, 0, 64, 128, 192, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 320,
];

#[inline]
pub fn backward(code: u32) -> u8 {
    let offset = (code >> 6) as uint;
    let offset = if offset < 129 {BACKWARD_TABLE_UPPER[offset] as uint} else {0};
    BACKWARD_TABLE_LOWER[offset + ((code & 63) as uint)]
}

#[cfg(test)]
single_byte_tests!()