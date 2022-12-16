import React from 'react'
import { HexColorPicker } from "react-colorful"

export const EditCircle = (props) => {

    const onChange = (key, value) => {
        const _shape = {...props.shape}
        _shape[key] = value
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
                <span>radius</span>
                <input type="number" value={props.shape.radius} onChange={e=>onChange('radius', parseInt(e.target.value))} />
            </div>
            <div>
                <span>fill</span>
                <HexColorPicker color={props.shape.fill} onChange={e=>onChange('fill', e)} />
            </div>
            <div>
                <span>stroke</span>
                <HexColorPicker color={props.shape.stroke} onChange={e=>onChange('stroke', e)} />
            </div>
            <div>
                <span>strokeWidth</span>
                <input type="number" step="1" value={props.shape.strokeWidth} onChange={e=>onChange('strokeWidth', parseInt(e.target.value))} />
            </div>
        </>
    )
}