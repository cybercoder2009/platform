import React, {useRef, useEffect} from 'react'
import {Circle, Transformer} from 'react-konva'

import {TRANSFORMER_PROPS} from './constants.js'

export const ElementCircle = ({props, isSelected, onSelect, onChange}) => {

    const ref_el = useRef()
    const ref_tf = useRef()
    
    useEffect(_ => {
        if (isSelected) {
            ref_tf.current.nodes([ref_el.current])
            ref_tf.current.getLayer().batchDraw()
        }
    }, [isSelected])

    return (
        <>
            <Circle {...props} draggable
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
            />
            {isSelected && <Transformer {...TRANSFORMER_PROPS} ref={ref_tf} />}
        </>
    )
}