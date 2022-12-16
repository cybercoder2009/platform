import React from 'react'
import { HexColorPicker } from "react-colorful"

import { create_barcode } from './utilities.js'
import { BARCODES } from './constants.js'

export const EditBarcode = (props) => {

    const onChange = (key, value) => {
        const _shape = {...props.shape}
        _shape[key] = value
        create_barcode(_shape.text, _shape.width_bar, _shape.height, _shape.fill, _shape.fill_light, _shape.format)
        .then(image=>{
            _shape.image = image
            props.onChange(_shape)
        }, err=>console.log(err))
    }

    return (
        <>
            <button className="ant-btn-primary ant-btn-dangerous" onClick={props.onDelete}>X</button>
            <div>
                <span>index</span>
                <input type="number" step="1" value={props.shape.index} onChange={e=>onChange('index', parseInt(e.target.value))} />
            </div>
            <div>
                <span>format</span>
                <select value={props.shape.format} onChange={e=>{
                    const _shape = {...props.shape}
                    _shape['format'] = e.target.value
                    _shape['text'] = BARCODES[e.target.value]['sample']
                    create_barcode(_shape.text, _shape.width_bar, _shape.height, _shape.fill, _shape.fill_light, _shape.format)
                    .then(image=>{
                        _shape.image = image
                        props.onChange(_shape)
                    }, err=>console.log(err))    
                }}>
                {Object.keys(BARCODES).map((v, k)=><option key={k} value={v}>{v}</option>)}
                </select>
            </div>
            <div>
                <span>text</span>
                <input type="text" value={props.shape.text} disabled />
            </div>
            <div>
                <span>key</span>
                <input type="text" value={props.shape.key} onChange={e=>onChange('key', e.target.value)} />
            </div>
            <div>
                <span>x</span>
                <input type="number" value={props.shape.x} onChange={e=>onChange('x', parseInt(e.target.value))} />
            </div>
            <div>
                <span>y</span>
                <input type="number" value={props.shape.y} onChange={e=>onChange('y', parseInt(e.target.value))} />
            </div>
            <div>
                <span>rotation</span>
                <input type="number" value={props.shape.rotation} onChange={e=>onChange('rotation', parseInt(e.target.value))} />
            </div>
            <div>
                <span>bar width</span>
                <input type="number" value={props.shape.width_bar} onChange={e=>onChange('width_bar', parseInt(e.target.value))} />
            </div>
            <div>
                <span>width</span>
                <input type="number" value={props.shape.width} disabled />
            </div>
            <div>
                <span>height</span>
                <input type="number" value={props.shape.height} onChange={e=>onChange('height', parseInt(e.target.value))} />
            </div>
            <div>
                <span>fill dark</span>
                <HexColorPicker color={props.shape.fill} onChange={e=>onChange('fill', e)} />
            </div>
            <div>
                <span>fill light</span>
                <HexColorPicker color={props.shape.fill_light} onChange={e=>onChange('fill_light', e)} />
            </div>
        </>
    )
}