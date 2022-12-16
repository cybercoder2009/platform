use png::{BitDepth, ColorType, Encoder, Writer};

/*
 * raw: [ r,  g,  b,  a, ...]
 *      [u8, u8, u8, u8, ...]
 */
pub fn encode_base64(width: u32, height: u32, raw: &Vec<u8>) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![];
    {
        let mut encoder: Encoder<&mut Vec<u8>> = Encoder::new(&mut buffer, width, height);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        let mut writer: Writer<&mut Vec<u8>> = encoder.write_header().unwrap();
        /* TODO: error check */
        writer.write_image_data(raw).unwrap();
    }
    buffer
}
