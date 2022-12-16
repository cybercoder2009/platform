import JsBarcode from 'jsbarcode'
import {IMAGES, MAX_IMAGE_KB} from './constants.js'

/*
 * formats: https://github.com/lindell/JsBarcode/blob/master/src/barcodes/index.js#L10
 */
const create_barcode = (text, width_bar, height, fill, fill_light, format) => new Promise((res, rej)=>{
	try {
		const canvas = document.createElement("canvas")
		// console.log(`create_barcode format=${format} text=${text}`)
		JsBarcode(canvas, text, {
			displayValue: false,
			format,
			lineColor: fill,
			background: fill_light,
			width: width_bar,
			height,
			margin: 0,
			flat: true,
		})	
		const image = document.createElement("IMG")
		image.onload = _ => res(image)
		image.src = canvas.toDataURL()
	} catch (err) {
		rej(err) 
	}
})

const image_data = image => {
	const canvas = document.createElement('canvas')
	canvas.width = image.naturalWidth
	canvas.height = image.naturalHeight
	const ctx = canvas.getContext('2d')
	ctx.drawImage(
		image,
		0,
		0,
		image.naturalWidth,
		image.naturalHeight,
		0,
		0,
		image.naturalWidth,
		image.naturalHeight
	)
	/* Uint8ClampedArray */
	const raw = Array.from(ctx.getImageData(0, 0, canvas.width, canvas.height).data)
	// return Buffer.from(String.fromCharCode.apply(null, raw), 'base64')
	return btoa(String.fromCharCode.apply(null, raw))
}

const load_image = _ => new Promise((res, rej) => {
	const input = document.createElement('input')
	input.type = 'file'
	input.onchange = e => {
		if(e.target.files && e.target.files[0]) {
			if (IMAGES.indexOf(e.target.files[0].type) >= 0) {
				if (e.target.files[0].size <= 1024 * MAX_IMAGE_KB) {
					let file = e.target.files[0]
					let reader = new FileReader()
					reader.onload = e => {
						const base64 = e.target.result
						const image = document.createElement("IMG")
						image.onload = _ => {
							const base64_raw = image_data(image)
							res({image, base64, base64_raw})
						}
						image.src = base64
					}
					reader.readAsDataURL(file)
				} else rej(`Maximum file size ${MAX_IMAGE_KB}KB exceeded`)
			} else rej('Invalid image type')
		} else rej('No image selected')
	}
	input.click()
})

const base64_to_image = str_base64 => new Promise((res, rej) => {
	const image = document.createElement("IMG")
	image.onload = _ => res(image)
	image.onerror = e => rej(e)
	image.src = str_base64
})

export {
	create_barcode,
	load_image,
	base64_to_image,
}