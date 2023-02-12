use serde_json::Value;
use serde_json::map::Map;
use lib_canvas::template::Template;
use lib_canvas::render_2_image;
use lib_utilities::random::u16;
use lib_vendors::yala::{Message, Content};

use crate::struct_label::Label;

fn kinds()      -> Vec<String> { vec!["text".to_string(), "barcode".to_string(), "qrcode".to_string(), "stars".to_string(), "circles".to_string()] }
fn kinds_text() -> Vec<String> { vec!["text".to_string(), "barcode".to_string(), "qrcode".to_string()] }

pub fn populate(
    item: &Map<String, Value>,
    template: &mut Template
) {

    for e in template.elements.iter_mut() {

        if !kinds().contains(&e.kind) { continue; }
        if e.key.is_none() { continue; }
        let key: &String = e.key.as_ref().unwrap();
        let opt_v: Option<&Value> = item.get(key);
        if opt_v.is_none() { continue }
        let v: &Value = opt_v.unwrap();

        if e.key.is_some() && item.contains_key(key) {
            if kinds_text().contains(&e.kind) {
                if v.is_string() { 
                    e.text = Some(v.as_str().unwrap().to_string()); 
                }
                else if v.is_number() {
                    e.text = Some(format!("{}", v.as_f64().unwrap()));
                }
            } else {
                if v.is_number() {
                    e.value = Some(v.as_u64().unwrap() as u32);
                }
            }
        }
    }
}

pub fn render_label_image (
    label: &Label,
    template: &Template,
    images: &str,
) -> Message {

    let image_name: String = format!("{}.jpg", &label.id);
    let image_path: String = format!("./public/images/{}", &image_name);
    let image_url: String = format!("{}/{}", images, &image_name);
    info!("item={} label={} image_path={} image_url={}",
        label.id_item, label.id, &image_path, &image_url);
    render_2_image(&template, &image_path);

    // vendor & model specific
    // TODO: wrap it to lib_vendors
    let img: image::DynamicImage = image::io::Reader::open(&image_path).unwrap().decode().unwrap();
    match (label.width, label.height) {
        (800, 480) | (400, 300) => {},
        _ => img.rotate270().save(&image_path).unwrap()
    }
    
    let mut content: Vec<Content> = Vec::new();
    content.push(Content {
        dataRef: String::from(&image_url),
        layerEnd: true
    });
    Message::new(
        u16(),
        &label.id,
        &label.mac,
        &label.version,
        3,
        1,
        content,
    )
}