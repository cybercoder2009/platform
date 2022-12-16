import React, {useRef, useEffect} from 'react'
import {Group, Circle, Transformer} from 'react-konva'

import {TRANSFORMER_PROPS} from './constants.js'

export const ElementCircles = ({props, isSelected, onSelect, onChange}) => {

    const ref_el = useRef()
    const ref_tf = useRef()

    const items = _ => {
        const arr = []
        for(let i = 0; i < props.max; i++)
            arr.push(i < props.value ? true : false)
        return arr
    }

    useEffect(_ => {
        if (isSelected) {
            ref_tf.current.nodes([ref_el.current])
            ref_tf.current.getLayer().batchDraw()
        }
    }, [isSelected])

    return (
        <>
            <Group {...props} draggable
                onClick={onSelect}
                onTap={onSelect}
                ref={ref_el}
                onDragEnd={e=>onChange({
                    ...props,
                    ...{
                        x: Math.round(e.target.x()),
                        y: Math.round(e.target.y()),
                    }
                })}
                onTransformEnd={e=>{
                    // const scaleX = e.target.scaleX()
                    e.target.scaleX(1)
                    e.target.scaleY(1)
                    onChange({
                        ...props,
                        ...{
                            x: Math.round(e.target.x()),
                            y: Math.round(e.target.y()),
                            rotation: Math.round(e.target.rotation()),
                            // radius: Math.round(props.radius * scaleX),
                        }
                    })
                }}
            >
            {
                items().map((v, k)=>{
                    return (<Circle key={k}
                        x={props.radius * 2 * k + props.margin * k} y={0} 
                        radius={props.radius} fill={v ? props.fill : '#ffffff'} stroke={props.stroke} strokeWidth={props.strokeWidth}
                    />)
                })
            }
            </Group>
            {isSelected && <Transformer {...TRANSFORMER_PROPS} ref={ref_tf} />}
        </>
    )
}