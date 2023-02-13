import React, { useState, useRef, useEffect } from 'react'
import {Stage, Layer, Transformer} from 'react-konva'
const QRCode = require('qrcode')

import {ElementRect } from './src/element_rect.js'
import {ElementCircle } from './src/element_circle.js'
import {ElementStar } from './src/element_star.js'
import {ElementCircles } from './src/element_circles.js'
import {ElementStars } from './src/element_stars.js'
import {ElementText } from './src/element_text.js'
import {ElementImage } from './src/element_image.js'

import {EditRect} from './src/edit_rect.js'
import {EditCircle} from './src/edit_circle.js'
import {EditCircles} from './src/edit_circles.js'
import {EditStar} from './src/edit_star.js'
import {EditStars} from './src/edit_stars.js'
import {EditText} from './src/edit_text.js'
import {EditImage} from './src/edit_image.js'
import {EditBarcode} from './src/edit_barcode.js'
import {EditQRcode} from './src/edit_qrcode.js'

import {TEMPLATE, RECT, CIRCLE, CIRCLES, STAR, STARS, TEXT, BARCODE, IMAGE, QRCODE} from './src/constants.js'
import {load_image, base64_to_image, create_barcode} from './src/utilities.js'

import './index.less'

const Designer = (props) => {
	
	const ref_stage = useRef(null)
	const ref_tf = useRef(null)
	const [select, set_select] = useState(null)

	const on_click = e => {
		if (e.target === ref_stage.current) {
			ref_tf.current.nodes([])
			set_select(null)
       		return
		}
		// else if (e.target.constructor.name === 'Text'){
		// 	console.log(`e.target.getTextWidth()=${e.target.getTextWidth()}`)
		// 	console.log(`e.target.getTextHeight()=${e.target.getTextHeight()}`)
		// 	console.log(`e.target.getWidth()=${e.target.getWidth()}`)
		// 	console.log(`e.target.getHeight()=${e.target.getHeight()}`)
		// 	console.log(`e.target.width()=${e.target.width()}`)
		// 	console.log(`e.target.height()=${e.target.height()}`)
		// 	console.log(`e.target.getClientRect()=${JSON.stringify(e.target.getClientRect())}`)
		// }
	}

	const on_template_keyword_change = e => {
		const _template = {...props.template}
		_template.keyword = e.target.value
		_template.id = e.target.value.trim().split(' ').join('_').toLowerCase() + '-' + _template.width + 'x' + _template.height
		props.on_change(_template)
	}

	const on_template_size_change = e => {
		const _template = { ...props.template }
		_template.model = e.target.value
		_template.width = props.sizes[e.target.value].width
		_template.height = props.sizes[e.target.value].height
		_template.id = _template.keyword.trim().split(' ').join('_').toLowerCase() + '-' + _template.width + 'x' + _template.height
		props.on_change(_template)
	}

	const on_template_save = _ => {
		// ref_tf.current.nodes([])
		const img = document.createElement('img')
		img.onload = _ => {
			const canvas = document.createElement('canvas')
			const ctx = canvas.getContext('2d')
			const _template = {...props.template}
			const _width = 40 * _template.width / _template.height
			const _height = 40
			canvas.width = _width
			canvas.height = _height
			ctx.drawImage(img, 0, 0, _width, _height)
			_template["thumbnail"] = canvas.toDataURL('image/png')
			props.on_save(_template)
		}
		img.src = ref_stage.current.toDataURL()
	}

	const on_element_change = (e, k) => {
		const _template = {...props.template}
		if(e.kind === 'barcode') {
			create_barcode(e.text, e.width_bar, e.height, e.fill, e.fill_light, e.format)
			.then(image=>{
				e.image = image
				e.width = image.width
				e.height = image.height
				_template.elements[k] = e
				const k_before = _template.elements[k + 1]
				const k_after = _template.elements[k - 1]
				props.on_change(_template)
				if ((k_before != undefined && e.index > k_before.index) || (k_after != undefined && e.index < k_after.index)) {
					_template.elements.sort((a, b) => { return a.index > b.index ? 1 : -1 })
					set_select(_template.elements.indexOf(e))
				}
			}, err=>console.log(err))
		} else {
			_template.elements[k] = e
			const k_before = _template.elements[k + 1]
			const k_after = _template.elements[k - 1]
			props.on_change(_template)
			if ((k_before != undefined && e.index > k_before.index) || (k_after != undefined && e.index < k_after.index)) {
				_template.elements.sort((a, b) => { return a.index > b.index ? 1 : -1 })
				set_select(_template.elements.indexOf(e))
			}
		}
	}

	const on_element_delete = k => {
		const _template = {...props.template}
		_template.elements.splice(k, 1)
		props.on_change(_template)
		set_select(null)
	}

	useEffect(_ => {
		props.template.elements.forEach((v, k) => {
			if (
				(v.kind === 'image' || v.kind === 'barcode' || v.kind === 'qrcode')
				&& (!v.image || v.image.currentSrc === undefined)
			) {
				switch (v.kind) {
					case 'image':
						v.image = new Image()
						v.image.onload = _ => on_element_change(v, k)
						v.image.src = v.base64
						break
					case 'barcode':
						create_barcode(v.text, v.width_bar, v.height, v.fill, v.fill_light, v.format)
						.then(image=>{
							v.image = image
							v.width = image.width
							v.height = image.height
							on_element_change(v, k)
						}, err=>console.log(err))
						break
					case 'qrcode':
						QRCode.toDataURL(v.text, {type: 'image/png', quality: 0.3, margin: 0, width: 95, color: { dark: v.fill, light: v.fill_light }, errorCorrectionLevel: 'High'}).then(res => {
							base64_to_image(res)
								.then(image => {
									const _template = { ...props.template }
									_template.elements[k].index = k
									_template.elements[k].image = image
									props.on_change(_template)
								}, err => console.log(err))
						})
						break
				}
			}
		})
	})

	return (
		<div id="designer">
			<Stage ref={ref_stage} 
				width={props.template.width}
				height={props.template.height}
				onClick={on_click}>
				<Layer>
					{
						props.template.elements.map((e, k) => {
							if (!e.index) e.index = k
							switch (e.kind) {
								case 'rect': return <ElementRect key={k} isSelected={k===select} props={{...e}}
									onSelect={_ => set_select(k)} onChange={e => on_element_change(e, k)}
								/>
								case 'circle': return <ElementCircle key={k} isSelected={k===select} props={{...e}}
									onSelect={_ => set_select(k)} onChange={e => on_element_change(e, k)}
								/>
								case 'circles': return <ElementCircles key={k} isSelected={k===select} props={{...e}}
									onSelect={_ => set_select(k)} onChange={e => on_element_change(e, k)}
								/>
								case 'star': return <ElementStar key={k} isSelected={k == select} props={{...e}}
									onSelect={_ => set_select(k)} onChange={e => on_element_change(e, k)}
								/>
								case 'stars': return <ElementStars key={k} isSelected={k===select} props={{...e}}
									onSelect={_ => set_select(k)} onChange={e => on_element_change(e, k)}
								/>
								case 'text': return <ElementText key={k} isSelected={k == select} props={{...e}}
									onSelect={_ => set_select(k)} onChange={e => on_element_change(e, k)}
								/>
								case 'image': return e.image
									? <ElementImage key={k} isSelected={k == select} props={{...e}}
										onSelect={_ => set_select(k)} onChange={e => on_element_change(e, k)}
									/> : null
								case 'qrcode': return e.image
									? <ElementImage key={k} isSelected={k == select} props={{...e}}
										onSelect={_ => set_select(k)} onChange={e => on_element_change(e, k)}
									/> : null
								case 'barcode': return e.image
									? <ElementImage key={k} isSelected={k == select} props={{...e}}
										onSelect={_ => set_select(k)} onChange={e => on_element_change(e, k)}
									/> : null
								default:
									return null
							}
						})
					}
					<Transformer ref={ref_tf} resizeEnabled={false} flipEnabled={false} rotateEnabled={false} />
				</Layer>
			</Stage>
			<div id="designer-select">
				{
					props.template.elements.map((e, k) => <span key={k} className="ant-tag ant-tag-blue" onClick={_=>{
						set_select(k)
					}}>{k + 1}. {e.kind}</span>)
				}
			</div>
			<div id="designer-toolbar">
				{
					props.template.id === ''
						? <button className="ant-btn-primary" disabled>Save</button>
						: <button className="ant-btn-primary" onClick={_ => on_template_save()}>Save</button>
				}
				<div>
					<span>ID</span>
					<input type="text" value={props.template.id} disabled />
				</div>
				<div>
					<span>Keyword</span>
					<input type="text" value={props.template.keyword} onChange={e => on_template_keyword_change(e)} />
				</div>
				<div>
					<span>Size</span>
					<select value={props.size} onChange={e => on_template_size_change(e)}>
						{
							Object.entries(props.sizes).map((v, k) => (
								<option key={k} value={v[0]}>{v[0]}-{v[1].width}x{v[1].height}</option>
							))
						}
					</select>
				</div>
				<div>
					<span>Width</span>
					<input type="number" value={props.template.width} disabled />
				</div>
				<div>
					<span>Height</span>
					<input type="number" value={props.template.height} disabled />
				</div>
				<div className="buttons">
					<button className="ant-btn" onClick={_ => {const _template = { ...props.template }; _template.elements.push({ ...RECT }); props.on_change(_template);}}>Rect</button>
					<button className="ant-btn" onClick={_ => {const _template = { ...props.template }; _template.elements.push({ ...TEXT }); props.on_change(_template);}}>Text</button>
					<button className="ant-btn" onClick={_ => {const _template = { ...props.template }; _template.elements.push({ ...CIRCLE }); props.on_change(_template);}}>Circle</button>
					<button className="ant-btn" onClick={_ => {const _template = { ...props.template }; _template.elements.push({ ...CIRCLES }); props.on_change(_template);}}>Circles</button>
					<button className="ant-btn" onClick={_ => {const _template = { ...props.template }; _template.elements.push({ ...STAR }); props.on_change(_template);}}>Star</button>
					<button className="ant-btn" onClick={_ => {const _template = { ...props.template }; _template.elements.push({ ...STARS }); props.on_change(_template);}}>Stars</button>
					<button className="ant-btn" onClick={_ => load_image().then(res => {
						const _image = {...IMAGE}
						_image.width = res.image.width
						_image.height = res.image.height
						_image.naturalWidth = res.image.naturalWidth
						_image.naturalHeight = res.image.naturalHeight
						_image.image = res.image
						_image.base64 = res.base64
						_image.base64_raw = res.base64_raw
						const _template = {...props.template}
							_template.elements.push(_image)
							props.on_change(_template)
					}, err => window.confirm(err))}>Image</button>
					<button className="ant-btn" onClick={_ => {
						create_barcode(BARCODE.text, BARCODE.width_bar, BARCODE.height, BARCODE.fill, BARCODE.fill_light, BARCODE.format)
						.then(image=>{
							const _barcode = {...BARCODE}
							_barcode.width = image.width
							_barcode.height = image.height
							_barcode.image = image
							const _template = {...props.template}
							_template.elements.push(_barcode)
							props.on_change(_template)
						}, err=>console.log(err))
					}}>Barcode</button>
					<button className="ant-btn" onClick={_ => {
						QRCode.toDataURL(QRCODE.text, { type: 'image/png', quality: 0.3, margin: 0, width: QRCODE.width, color: { dark: QRCODE.fill_dark, light: QRCODE.fill_light }, errorCorrectionLevel: "High" }).then(res => {
							const _qrcode = {...QRCODE}
							_qrcode.base64 = res
							base64_to_image(_qrcode.base64)
								.then(image => {
									_qrcode.image = image
									_qrcode.naturalWidth = image.naturalWidth
									_qrcode.naturalHeight = image.naturalHeight
									const _template = {...props.template}
									_template.elements.push(_qrcode)
									props.on_change(_template)
								}, err => console.log(err))
						})
					}}>QR</button>
				</div>
			</div>
			<div id="designer-properties">
			{
				!props.template.elements[select] 
				? null
				: {
                	'rect': <EditRect shape={props.template.elements[select]} onChange={e => on_element_change(e, select)} onDelete={_ => on_element_delete(select)} />,
                	'circle': <EditCircle shape={props.template.elements[select]} onChange={e => on_element_change(e, select)} onDelete={_ => on_element_delete(select)} />,
                	'circles': <EditCircles shape={props.template.elements[select]} onChange={e => on_element_change(e, select)} onDelete={_ => on_element_delete(select)} />,
                	'star': <EditStar shape={props.template.elements[select]} onChange={e => on_element_change(e, select)} onDelete={_ => on_element_delete(select)} />,
                	'stars': <EditStars shape={props.template.elements[select]} onChange={e => on_element_change(e, select)} onDelete={_ => on_element_delete(select)} />,
                	'text': <EditText shape={props.template.elements[select]} onChange={e => on_element_change(e, select)} onDelete={_ => on_element_delete(select)} />,
                	'image': <EditImage shape={props.template.elements[select]} onChange={e => on_element_change(e, select)} onDelete={_ => on_element_delete(select)} />,
                	'barcode': <EditBarcode shape={props.template.elements[select]} onChange={e => on_element_change(e, select)} onDelete={_ => on_element_delete(select)} />,
                	'qrcode': <EditQRcode shape={props.template.elements[select]} onChange={e => on_element_change(e, select)} onDelete={_ => on_element_delete(select)} />,
				}[props.template.elements[select].kind]
			}
			</div>
		</div>
	)
}

export {
	Designer,
	TEMPLATE,
}