const TEN = 10, FIVE = 5, FOUR = 4, TWO = 2, ONE = 1, ZERO = 0
const RED = '#FF0000', WHITE = '#FFFFFF', BLACK = '#000000'
const TEXT0 = 'text'
const FONT_FAMILY = 'Roboto'
const FONT_FAMILIES = [FONT_FAMILY, 'Mali']
const FONT_STYLES = ['normal', 'italic']
const FONT_SIZE = 18

/*
 * formats: https://github.com/lindell/JsBarcode/blob/master/src/barcodes/index.js#L10
 */
const BARCODES = {
	UPC: {sample: '123456789104'},
	EAN13: {sample: '0123456789012'},
}

const TEMPLATE = {
	id: '',
	keyword: '',
	width: 0,
	height: 0,
	thumbnail: '',
	elements: []
}

const IMAGES = ["image/png", "image/jpeg"]
const MAX_IMAGE_KB = 2410

const TRANSFORMER_PROPS = {
	resizeEnabled: false,
	borderDash: [2],
	anchorSize: 4,
	rotateAnchorOffset: 10,
}

const RECT = {
	kind: 'rect', index: ZERO,
    x: FIVE, y: FIVE, rotation: ZERO,
    width: 80,
    height: 40,
    stroke: BLACK,
    strokeWidth: ONE,
    fill: RED,
}

const CIRCLE = {
	kind: 'circle', index: ZERO,
	x: FIVE, y: FIVE, rotation: ZERO,
	radius: TEN,
	stroke: BLACK,
	strokeWidth: ONE,
	fill: RED, 
}

const CIRCLES = { 
	kind: 'circles', index: ZERO,
	value: FOUR, max: FIVE, key: '',
	x: FIVE, y: FIVE, ro20tation: ZERO,
	margin: TWO, 
	radius: TEN,
	stroke: BLACK,
	strokeWidth: ONE,
	fill: RED,
	fill_light: WHITE,
}

const STAR = {
	kind: 'star', index: ZERO,
	x: FIVE, y: FIVE, rotation: ZERO,
	numPoints: FIVE,
	innerRadius: TEN / 2,
	outerRadius: TEN,
	stroke: BLACK,
	strokeWidth: ONE,
	fill: RED,
}

const STARS = { 
	kind: 'stars', index: ZERO,
	value: FOUR, max: FIVE, key: '',
	margin: TWO,
	x: FIVE, y: FIVE, rotation: ZERO,
	numPoints: FIVE, 
	innerRadius: TEN / 2,
	outerRadius: TEN,
	stroke: BLACK,
	strokeWidth: ONE,
	fill: RED,
	fill_light: WHITE,
}

const TEXT = { 
	kind: 'text', index: ZERO,
	text: TEXT0, key: '',
    x: FIVE, y: FIVE, rotation: ZERO,
    width: 100,
    height: FONT_SIZE,
    fontSize: FONT_SIZE,
    fontStyle: FONT_STYLES[ZERO],
    fontFamily: FONT_FAMILY,
    fill: BLACK,
}

const QRCODE = {
	kind: 'qrcode', index: ZERO,
	text: 'https://reducing.ca', key: '',
    x: FIVE, y: FIVE, rotation: ZERO,
	width: 95,
	height: 95, 
	image_type: 'image/png',
	quality: 0.3,
	margin: 0,
	error_correction: "High",
	fill: BLACK,
	fill_light: WHITE,
}

const BARCODE = {
	kind: 'barcode', index: ZERO,
	text: BARCODES['UPC']['sample'], key: '',
	x: FIVE, y: FIVE, rotation: ZERO,
	format: 'UPC',
	width_bar: TWO,
	width: 120,
	height: 20,
	fill: BLACK,
	fill_light: WHITE,
}

const IMAGE = {
	kind: 'image', index: ZERO,
	x: FIVE, y: FIVE, rotation: ZERO,
	width: ZERO,
	height: ZERO,
	base64: '',
}

export {

	TRANSFORMER_PROPS,

	TEN, FIVE, TWO, ONE, ZERO,

	BLACK, WHITE, RED,

	TEMPLATE,

	IMAGES, QRCODE, MAX_IMAGE_KB,

	RECT, CIRCLE, CIRCLES, STAR, STARS, TEXT, IMAGE, BARCODE, // LINE,

	FONT_FAMILIES, FONT_STYLES,

	BARCODES,
}