import React from 'react'
import { HexColorPicker } from "react-colorful"

import { FONT_FAMILIES, FONT_STYLES } from './constants.js'

export const EditText = (props) => {

    const onChange = (key, value) => {
        const _shape = {...props.shape}
        _shape[key] = value
        if (['text', 'fontSize', 'fontStyle', 'fontFamily'].indexOf(key) >= 0) {
            // console.log(`key=${key} fontSize=${_shape.fontSize} fontStyle=${_shape.fontStyle} fontFamily=${_shape.fontFamily}`)
            let ctx = document.createElement("canvas").getContext("2d")
            ctx.font = `${_shape.fontSize}pt ${_shape.fontFamily} ${_shape.fontStyle}`
            _shape.width = ctx.measureText(_shape.text).width
            _shape.height = _shape.fontSize
        }
        props.onChange(_shape)
    }

    return (
        <>
            <button className="ant-btn ant-btn-primary ant-btn-dangerous" onClick={props.onDelete}>X</button>
            <div>
                <span>index</span>
                <input type="number" step="1" value={props.shape.index} onChange={e=>onChange('index', parseInt(e.target.value))} />
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
                <span>width</span>
                <input type="number" value={props.shape.width} disabled />
            </div>
            <div>
                <span>height</span>
                <input type="number" value={props.shape.height} disabled />
            </div>
            <div>
                <span>fill</span>
                <HexColorPicker color={props.shape.fill} onChange={e=>onChange('fill', e)} />
            </div>
            <div>
                <span>text</span>
                <input type="text" value={props.shape.text} onChange={e=>onChange('text', e.target.value)} />
            </div>
            <div>
                <span>font size</span>
                <input type="number" value={props.shape.fontSize} onChange={e=>onChange('fontSize', parseInt(e.target.value))} />
            </div>
            <div>
                <span>font style</span>
                <select value={props.shape.fontStyle} onChange={e=>onChange('fontStyle', e.target.value)}>
                {
                    FONT_STYLES.map((v, k)=>{
                        return <option key={k} value={v}>{v}</option>
                    })
                }
                </select>
            </div>
            <div>
                <span>font family</span>
                <select value={props.shape.fontFamily} onChange={e=>onChange('fontFamily', e.target.value)}>
                {
                    FONT_FAMILIES.map((ff, k)=>{
                        return (
                            <option key={k} value={ff}>{ff}</option>
                        )
                    })
                }
                </select>
            </div>
            <div>
                <span>rotation</span>
                <input type="number" value={props.shape.rotation} onChange={e=>onChange('rotation', parseInt(e.target.value))} />
            </div>
        </>
    )
}