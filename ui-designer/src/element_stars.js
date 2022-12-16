import React, {useRef, useEffect} from 'react'
import {Group, Star, Transformer} from 'react-konva'

import {TRANSFORMER_PROPS} from './constants.js'

export const ElementStars = ({props, isSelected, onSelect, onChange}) => {

    const ref_el = useRef()
    const ref_tf = useRef()

    const items = _ => {
        const arr = []
        for(let i = 0; i < props.max; i++){
            arr.push(i < props.value ? true : false)
        }
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
                            // innerRadius: Math.round(props.innerRadius * scaleX),
                            // outerRadius: Math.round(props.outerRadius * scaleX),
                        }
                    })
                }}
            >
            {
                items().map((v, k)=>{
                    return (<Star key={k}
                        x={props.outerRadius * 2 * k + props.margin * k} y={0} 
                        fill={v ? props.fill : '#ffffff'} stroke={props.stroke} strokeWidth={props.strokeWidth} rotation={0}
                        innerRadius={props.innerRadius} outerRadius={props.outerRadius}
                        numPoints={props.numPoints}
                    />)
                })
            }
            </Group>
            {isSelected && <Transformer {...TRANSFORMER_PROPS} ref={ref_tf} />}
        </>
    )
}