import React from 'react'
import { base64_to_image } from './utilities.js'
import { HexColorPicker } from "react-colorful"

export const EditQRcode = (props) => {
    
    const onChange = (key, value) => {
        const _shape = { ...props.shape }
        _shape[key] = value
        const QRCode = require('qrcode')
        QRCode.toDataURL(_shape.text, { type: 'image/png', quality: 0.3, margin: 0, width: _shape.width, color: { dark: _shape.fill, light: _shape.fill_light }, errorCorrectionLevel: "High" }).then(res => {
            base64_to_image(res).then(({ image, data }) => {
                _shape.image = image
                _shape.data = data
                props.onChange(_shape)
            })
        })
    }

    return (
        <>
            <button className="ant-btn-primary ant-btn-dangerous" onClick={props.onDelete}>X</button>
            <div>
                <span>index</span>
                <input type="number" step="1" value={props.shape.index} onChange={e => onChange('index', parseInt(e.target.value))} />
            </div>
            <div>
                <span>text</span>
                <input type="text" value={props.shape.text} onChange={e => onChange('text', e.target.value)} />
            </div>
            <div>
                <span>key</span>
                <input type="text" value={props.shape.key} onChange={e => onChange('key', e.target.value)} />
            </div>
            <div>
                <span>x</span>
                <input type="number" value={props.shape.x} onChange={e => onChange('x', parseInt(e.target.value))} />
            </div>
            <div>
                <span>y</span>
                <input type="number" value={props.shape.y} onChange={e => onChange('y', parseInt(e.target.value))} />
            </div>
            <div>
                <span>rotation</span>
                <input type="number" value={props.shape.rotation} onChange={e => onChange('rotation', parseInt(e.target.value))} />
            </div>
            <div>
                <span>height</span>
                <input type="number" value={props.shape.height} onChange={e => { onChange('height', parseInt(e.target.value)) }} />
            </div>
            <div>
                <span>width</span>
                <input type="number" value={props.shape.width} onChange={e => { onChange('width', parseInt(e.target.value)) }} />
            </div>
            <div>
                <span>fill</span>
                <HexColorPicker color={props.shape.fill} onChange={e => onChange('fill', e)} />
            </div>
            <div>
                <span>fill light</span>
                <HexColorPicker color={props.shape.fill_light} onChange={e => onChange('fill_light', e)} />
            </div>
        </>
    )
}