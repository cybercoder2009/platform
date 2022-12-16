#![allow(non_snake_case)]

use std::fs;
use std::io::Write;
use log::{info, warn};
use core::f32::consts::PI;
use barcoders::generators::image::{Color as BarcodeColor, Image as BarcodeImage, Rotation};
use barcoders::sym::ean13::{EAN13, UPCA}; // UPC-A are EAN-13 that start with a 0.
use qrcodegen::{QrCode, QrCodeEcc};
use skia_safe::paint::Style;
use skia_safe::{
    AlphaType, Surface, Canvas, PathBuilder, Color, ColorType, Data,
    EncodedImageFormat, Paint, Path,
    Rect, Image, ImageInfo, 
    Font, FontStyle, Typeface,
    // TextBlob,
};

use constants::{BARCODE_UPC, BARCODE_EAN13, F360, F180, F90, F0};
use template::Template;

pub mod constants;
pub mod template;

fn draw_barcode(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    rotation: f32,
    width: f32,
    height: f32,
    text: &str,
    format: &str,
    fill: &str,
    fill_light: &str,
) {
    info!("draw_barcode x={} y={} width={} height={} text={} format={} rotation={}",
        x, y, width, height, text, format, rotation);

    let (rd, gd, bd, ad): (u8, u8, u8, u8) = parse_color(fill);
    let (rl, gl, bl, al): (u8, u8, u8, u8) = parse_color(fill_light);
    let cd: BarcodeColor = BarcodeColor::new([rd, gd, bd, ad]);
    let cl: BarcodeColor = BarcodeColor::new([rl, gl, bl, al]);
    let img: BarcodeImage = BarcodeImage::ImageBuffer {
        height: height as u32,
        xdim: width as u32,
        rotation: Rotation::Zero,
        foreground: cd,
        background: cl,
    };

    match format {
        BARCODE_UPC => match UPCA::new(text) {
            Ok(barcode) => match img.generate_buffer(&barcode.encode()) {
                Ok(generated) => draw_image(
                    canvas,
                    x,
                    y,
                    rotation,
                    generated.width() as f32,
                    generated.height() as f32,
                    generated.width() as f32,
                    height,
                    generated.as_ref(),
                    false,
                ),
                Err(err) => warn!("upca img-buffer err={:?}", err),
            },
            Err(err) => warn!("upca text={} err={:?}", text, err),
        },
        BARCODE_EAN13 => match EAN13::new(text) {
            Ok(barcode) => match img.generate_buffer(&barcode.encode()) {
                Ok(generated) => draw_image(
                    canvas,
                    x,
                    y,
                    rotation,
                    generated.width() as f32,
                    generated.height() as f32,
                    generated.width() as f32,
                    height,
                    generated.as_ref(),
                    false,
                ),
                Err(err) => warn!("ean13 img-buffer err={:?}", err),
            },
            Err(err) => warn!("ean13 text={} err={:?}", text, err),
        },
        _ => warn!("unknown barcode={}", format),
    };
}

fn draw_text(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    rotation: f32,
    fontSize: f32,
    fontStyle: &str,
    fontFamily: &str,
    text: &str,
    fill: &str,
) {
    let mut _fontStyle: FontStyle = FontStyle::normal(); if fontStyle == "italic" {_fontStyle = FontStyle::italic();}
    info!("draw_text x={} y={} fontSize={} fontFamily={} fontStyle={} text={} rotation={} fill={}",
        x, y, fontSize, fontFamily, fontStyle, text, rotation, fill);
    
    let mut paint: Paint = Paint::default();
    paint.set_anti_alias(true);
    canvas.save();
    canvas.translate((x, y + fontSize)).rotate(rotation, None);
    let (fr, fg, fb, fa): (u8, u8, u8, u8) = parse_color(fill);
    paint.set_argb(fa, fr, fg, fb);
    let tf = Typeface::from_name(fontFamily, _fontStyle).unwrap_or_default();
    let f = Font::from_typeface(tf, fontSize);
    
    canvas.draw_str(text, (F0, F0), &f, &paint);
    // let tb: TextBlob = TextBlob::from_str(text, &f).unwrap();
    // canvas.draw_text_blob(tb, (F0, F0), &paint);
    
    canvas.restore();
}

fn draw_rect(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    rotation: f32,
    width: f32,
    height: f32,
    fill: &str,
    stroke: &str,
    strokeWidth: f32,
) {
    // info!("draw_rect x={} y={} rotation={} width={} height={} strokeWidth={}", x, y, rotation, width, height, strokeWidth);

    let path: Path = Path::rect(Rect::new(F0, F0, width, height), None);
    let mut paint = Paint::default();
    // paint.set_anti_alias(true);
    canvas.save();
    canvas.translate((x, y)).rotate(rotation, None);
    let (fr, fg, fb, fa): (u8, u8, u8, u8) = parse_color(fill);
    paint.set_argb(fa, fr, fg, fb);
    canvas.draw_path(&path, &paint);
    let (sr, sg, sb, sa): (u8, u8, u8, u8) = parse_color(stroke);
    paint
        .set_style(Style::Stroke)
        .set_argb(sa, sr, sg, sb)
        .set_stroke_width(strokeWidth);
    canvas.draw_path(&path, &paint);
    canvas.restore();
}

fn draw_circle(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    radius: f32,
    fill: &str,
    stroke: &str,
    strokeWidth: f32,
) {
    // info!("draw_circle x={} y={} rotation radius={} strokeWidth={}", x, y, _rotation, radius, strokeWidth);

    let path: Path = Path::circle((x, y), radius, None);
    let mut paint = Paint::default();
    // paint.set_anti_alias(true);

    let (fr, fg, fb, fa): (u8, u8, u8, u8) = parse_color(fill);
    paint.set_argb(fa, fr, fg, fb);
    canvas.draw_path(&path, &paint);
    let (sr, sg, sb, sa): (u8, u8, u8, u8) = parse_color(stroke);
    paint
        .set_style(Style::Stroke)
        .set_argb(sa, sr, sg, sb)
        .set_stroke_width(strokeWidth);
    canvas.draw_path(&path, &paint);
}

fn draw_circles(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    rotation: f32,
    margin: f32,
    radius: f32,
    value: u32,
    max: u32,
    fill: &str,
    fill_light: &str,
    stroke: &str,
    strokeWidth: f32,
) {
    let mut values: Vec<bool> = vec![];
    for i in 0..max {
        if i < value {
            values.push(true);
        } else {
            values.push(false);
        }
    }

    info!("draw_circles x={} y={} radius={} strokeWidth={} rotation={} margin={} value={} max={} values={:?}", x, y, radius, strokeWidth, rotation, margin, value, max, values);

    canvas.save();
    canvas.translate((x, y)).rotate(rotation, None);
    let mut i: f32 = 0.0;
    for v in values {
        let mut _fill: &str = fill_light;
        if v {
            _fill = fill;
        }
        draw_circle(
            canvas,
            i * (radius * 2.0 + margin),
            F0,
            radius,
            _fill,
            stroke,
            strokeWidth,
        );
        i = i + 1.0;
    }
    canvas.restore();
}

fn draw_star(
    single: bool,
    canvas: &mut Canvas,
    mut x: f32,
    mut y: f32,
    rotation: f32,
    innerRadius: f32,
    outerRadius: f32,
    numPoints: u32,
    fill: &str,
    stroke: &str,
    strokeWidth: f32,
) {
    // info!("draw_star x={} y={} rotation={} innerRadius={} outerRadius={} numPoints={} strokeWidth={}", x, y, rotation, innerRadius, outerRadius, numPoints, strokeWidth);

    if single && rotation != F0 {
        canvas.save();
        canvas.translate((x, y)).rotate(rotation, None);
        x = F0;
        y = F0;
    }

    let angle_0: f32 = F360 / (numPoints as f32);
    let angle_1: f32 = F90 - angle_0;
    let angle_2: f32 = F90 - angle_0 / 2.0;
    let mut pb: PathBuilder = PathBuilder::new();

    pb.move_to((
        x + f32::cos(angle_1 / F180 * PI) * outerRadius,
        y - f32::sin(angle_1 / F180 * PI) * outerRadius,
    ));

    for u in 0..numPoints {
        pb.line_to((
            x + f32::cos((angle_1 + angle_0 * u as f32) / F180 * PI) * outerRadius,
            y - f32::sin((angle_1 + angle_0 * u as f32) / F180 * PI) * outerRadius,
        ));
        pb.line_to((
            x + f32::cos((angle_2 + angle_0 * u as f32) / F180 * PI) * innerRadius,
            y - f32::sin((angle_2 + angle_0 * u as f32) / F180 * PI) * innerRadius,
        ));
    }
    pb.close();
    let path: Path = pb.snapshot();
    let mut paint = Paint::default();
    // paint.set_anti_alias(true);
    let (fr, fg, fb, fa): (u8, u8, u8, u8) = parse_color(fill);
    paint.set_argb(fa, fr, fg, fb);
    canvas.draw_path(&path, &paint);
    let (sr, sg, sb, sa): (u8, u8, u8, u8) = parse_color(stroke);
    paint
        .set_style(Style::Stroke)
        .set_argb(sa, sr, sg, sb)
        .set_stroke_width(strokeWidth);
    canvas.draw_path(&path, &paint);

    if single && rotation != F0 {
        canvas.restore();
    }
}

fn draw_stars(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    rotation: f32,
    margin: f32,
    innerRadius: f32,
    outerRadius: f32,
    numPoints: u32,
    value: u32,
    max: u32,
    fill: &str,
    fill_light: &str,
    stroke: &str,
    strokeWidth: f32,
) {
    let mut values: Vec<bool> = vec![];
    for i in 0..max {
        if i < value {
            values.push(true);
        } else {
            values.push(false);
        }
    }

    info!("draw_stars x={} y={} innerRadius={} outerRadius={} strokeWidth={} rotation={} margin={} value={} max={} values={:?}", x, y, innerRadius, outerRadius, strokeWidth, rotation, margin, value, max, values);

    canvas.save();
    canvas.translate((x, y)).rotate(rotation, None);
    let mut i: f32 = 0.0;
    for v in values {
        let mut _fill: &str = fill_light;
        if v {
            _fill = fill;
        }
        draw_star(
            false,
            canvas,
            i * (outerRadius * 2.0 + margin),
            F0,
            F0,
            innerRadius,
            outerRadius,
            numPoints,
            _fill,
            stroke,
            strokeWidth,
        );
        i = i + 1.0;
    }
    canvas.restore();
}

fn draw_image(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    rotation: f32,
    natural_width: f32,
    natural_height: f32,
    width: f32,
    height: f32,
    rgba: &[u8],
    dither: bool,
) {
    info!("draw_image x={} y={} natural_width={} natural_height={} width={} height={} rgba.len()={} rotation={}", x, y, natural_width, natural_height, width, height, rgba.len(), rotation);

    let info: ImageInfo = ImageInfo::new(
        (natural_width as i32, natural_height as i32),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );
    match Image::from_raster_data(&info, Data::new_copy(rgba), info.min_row_bytes()) {
        Some(image) => {
            let mut paint = Paint::default();
            // paint.set_anti_alias(true);
            if dither {
                paint.set_dither(true);
            }
            canvas.save();
            canvas.translate((x, y)).rotate(rotation, None);
            canvas.draw_image_rect(image, None, Rect::new(F0, F0, width, height), &paint);
            canvas.restore();
        }
        None => warn!("draw_image invalid image data"),
    }
}

fn draw_qrcode(
    canvas: &mut Canvas,
    x: f32,
    y: f32,
    rotation: f32,
    width: f32,
    height: f32,
    text: &str,
    fill: &str,
    fill_light: &str,
) {
    // info!("draw_qrcode x={} y={} width={} height={} text={} rotation={}", x, y, width, height, text, rotation);

    let (rd, gd, bd, ad) = parse_color(fill);
    let (rl, gl, bl, al) = parse_color(fill_light);
    let qr = QrCode::encode_text(text, QrCodeEcc::Low).unwrap();
    let mut rgba: Vec<u8> = vec![];
    let size: i32 = qr.size();
    for y in 0..size {
        for x in 0..size {
            match qr.get_module(x, y) {
                true => {
                    rgba.push(rd);
                    rgba.push(gd);
                    rgba.push(bd);
                    rgba.push(ad);
                }
                false => {
                    rgba.push(rl);
                    rgba.push(gl);
                    rgba.push(bl);
                    rgba.push(al);
                }
            }
        }
    }
    draw_image(
        canvas,
        x,
        y,
        rotation,
        size as f32,
        size as f32,
        width,
        height,
        &rgba,
        false,
    );
}

fn render_elements(canvas: &mut Canvas, template: &Template) {
    // for el in template.elements {
    template.elements.iter().for_each(|el| {
        match el.kind.as_ref() {
            "rect" => draw_rect(
                canvas,
                el.x as f32,
                el.y as f32,
                el.rotation as f32,
                el.width.unwrap() as f32,
                el.height.unwrap() as f32,
                el.fill.as_ref().unwrap(),
                el.stroke.as_ref().unwrap(),
                el.strokeWidth.unwrap() as f32,
            ),
            "circle" => draw_circle(
                canvas,
                el.x as f32,
                el.y as f32,
                el.radius.unwrap() as f32,
                el.fill.as_ref().unwrap(),
                el.stroke.as_ref().unwrap(),
                el.strokeWidth.unwrap() as f32,
            ),
            "circles" => draw_circles(
                canvas,
                el.x as f32,
                el.y as f32,
                el.rotation as f32,
                el.margin.unwrap() as f32,
                el.radius.unwrap() as f32,
                el.value.unwrap(),
                el.max.unwrap(),
                el.fill.as_ref().unwrap(),
                el.fill_light.as_ref().unwrap(),
                el.stroke.as_ref().unwrap(),
                el.strokeWidth.unwrap() as f32,
            ),
            "star" => draw_star(
                true,
                canvas,
                el.x as f32,
                el.y as f32,
                el.rotation as f32,
                el.innerRadius.unwrap() as f32,
                el.outerRadius.unwrap() as f32,
                el.numPoints.unwrap(),
                el.fill.as_ref().unwrap(),
                el.stroke.as_ref().unwrap(),
                el.strokeWidth.unwrap() as f32,
            ),
            "stars" => draw_stars(
                canvas,
                el.x as f32,
                el.y as f32,
                el.rotation as f32,
                el.margin.unwrap() as f32,
                el.innerRadius.unwrap() as f32,
                el.outerRadius.unwrap() as f32,
                el.numPoints.unwrap(),
                el.value.unwrap(),
                el.max.unwrap(),
                el.fill.as_ref().unwrap(),
                el.fill_light.as_ref().unwrap(),
                el.stroke.as_ref().unwrap(),
                el.strokeWidth.unwrap() as f32,
            ),
            "text" => draw_text(
                canvas,
                el.x as f32,
                el.y as f32,
                el.rotation as f32,
                el.fontSize.unwrap() as f32,
                el.fontStyle.as_ref().unwrap(),
                el.fontFamily.as_ref().unwrap(),
                el.text.as_ref().unwrap(),
                el.fill.as_ref().unwrap(),
            ),
            "image" => match base64::decode(el.base64_raw.as_ref().unwrap()) {
                Ok(raw) => draw_image(
                    canvas,
                    el.x as f32,
                    el.y as f32,
                    el.rotation as f32,
                    el.naturalWidth.unwrap() as f32,
                    el.naturalHeight.unwrap() as f32,
                    el.width.unwrap() as f32,
                    el.height.unwrap() as f32,
                    &raw,
                    true,
                ),
                Err(err) => warn!("image err={:?}", err),
            },
            "barcode" => draw_barcode(
                canvas,
                el.x as f32,
                el.y as f32,
                el.rotation as f32,
                el.width.unwrap() as f32,
                el.height.unwrap() as f32,
                el.text.as_ref().unwrap(),
                el.format.as_ref().unwrap(),
                el.fill.as_ref().unwrap(),
                el.fill_light.as_ref().unwrap(),
            ),
            "qrcode" => draw_qrcode(
                canvas,
                el.x as f32,
                el.y as f32,
                el.rotation as f32,
                el.width.unwrap() as f32,
                el.height.unwrap() as f32,
                el.text.as_ref().unwrap(),
                el.fill.as_ref().unwrap(),
                el.fill_light.as_ref().unwrap(),
            ),
            _ => {}
        }
    })
}

/*
 *
 * RGBA representation converter
 *
 *            Little-endian  Big-endian
 * RGBA8888   ABGR32         RGBA32
 * ARGB32     BGRA8888       ARGB8888
 * RGBA32     ABGR8888       RGBA8888
 */
// pub fn render_2_buffer(
//     template: &Template
// ) -> Vec<u8> {
//     // info!("template width={} height={}", template.width, template.height);

//     let info = ImageInfo::new(
//         (template.width as i32, template.height as i32),
//         ColorType::RGBA8888,
//         AlphaType::Unpremul,
//         None,
//     );
//     let mut bytes: Vec<u8> = vec![0u8; (template.height as usize) * info.min_row_bytes()];
//     let mut canvas: OwnedCanvas =
//         Canvas::from_raster_direct(&info, bytes.as_mut(), None, None).unwrap();
//     canvas.clear(Color::WHITE);
//     render_elements(canvas, template);
    
//     drop(canvas);
//     bytes
// }

/*
 *
 * rotate - vendor specific
 */
pub fn render_2_image(
    template: &Template,
    path: &str,
) {
    
    let mut surface = Surface::new_raster_n32_premul((template.width as i32, template.height as i32)).unwrap();
    let canvas: &mut Canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    render_elements(canvas, template);

    drop(canvas);
    let image = surface.image_snapshot();
    let data = image.encode_to_data(EncodedImageFormat::JPEG).unwrap();
    drop(image);
    drop(surface);

    let mut file = fs::File::create(path).expect("fail to create image");
    file.write_all(&data).expect("fail to write image");
    let _ = file.flush();
    drop(file);
}

/*
 * #FF0000 -> (255, 0, 0)
 */
fn parse_color(color: &str) -> (u8, u8, u8, u8) {
    let trimmed = color.trim_start_matches("#");
    if trimmed.len() != 6 {
        return (255, 255, 255, 255);
    }
    let r = u8::from_str_radix(&trimmed[0..2], 16);
    let g = u8::from_str_radix(&trimmed[2..4], 16);
    let b = u8::from_str_radix(&trimmed[4..6], 16);
    if r.is_ok() && g.is_ok() && b.is_ok() {
        return (r.unwrap(), g.unwrap(), b.unwrap(), 255);
    } else {
        return (255, 255, 255, 255);
    }
}

#[cfg(test)]
mod test {

    use skia_safe::FontMgr;

    #[test]
    fn test_typeface() {
        let fm = FontMgr::default();
        let count = fm.count_families();
        println!("familites = {}", count);
        for i in 0 .. count - 1 {
            println!("{}", fm.family_name(i));
        }
        
        // let tf = Typeface::default();
        // let family_names = tf.new_family_name_iterator();
        // drop(tf);

        // let mut any = false;
        // for name in family_names {
        //     println!("family: {}, language: {}", name.string, name.language);
        //     any = true
        // }
        // assert!(any);
    }
}
