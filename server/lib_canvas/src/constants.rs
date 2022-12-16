pub const REGEX_EAN13: &str = r"^\d{12,13}$";
pub const REGEX_CODE39: &str = r"^[0-9A-Z\-.$/\+%\*\s]*$";
// pub const REGEX_MSI: &str = r"^[0-9]{12,13}";

pub const ELEMENT_RECT: &str = "rect";
pub const ELEMENT_BARCODE: &str = "barcode";
pub const ELEMENT_QRCODE: &str = "qrcode";
pub const ELEMENT_IMAGE: &str = "image";
pub const ELEMENT_CIRCLE: &str = "circle";
pub const ELEMENT_CIRCLES: &str = "circles";
pub const ELEMENT_STAR: &str = "star";
pub const ELEMENT_STARS: &str = "stars";
pub const ELEMENT_TEXT: &str = "text";

pub const BARCODE_UPC: &str = "UPC";
pub const BARCODE_EAN13: &str = "EAN13";
// pub const BARCODE_CODE39: &str = "CODE39";
// pub const BARCODE_MSI: &str = "MSI";

pub const F360: f32 = 360.0;
pub const F180: f32 = 180.0;
pub const F90: f32 = 90.0;
pub const F0: f32 = 0.0;