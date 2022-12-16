import React, {useRef, useEffect} from 'react'
import {Rect, Transformer} from 'react-konva'

import {TRANSFORMER_PROPS} from './constants.js'

export const ElementRect = ({props, isSelected, onSelect, onChange}) => {

    const ref_el = useRef()
    const ref_tf = useRef()

    useEffect(_=>{
        if (isSelected) {
            ref_tf.current.nodes([ref_el.current])
            ref_tf.current.getLayer().batchDraw()
        }
    }, [isSelected])

    return (
        <>
            <Rect {...props} draggable
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
                    // const scaleY = e.target.scaleY()
                    e.target.scaleX(1)
                    e.target.scaleY(1)
                    onChange({
                        ...props,
                        ...{
                            x: Math.round(e.target.x()),
                            y: Math.round(e.target.y()),
                            rotation: Math.round(e.target.rotation()),
                            // width: Math.round(e.target.width() * scaleX),
                            // height: Math.round(e.target.height() * scaleY),
                        }
                    })
                }}
            />
            {isSelected && <Transformer {...TRANSFORMER_PROPS} ref={ref_tf} />}
        </>
    )
}