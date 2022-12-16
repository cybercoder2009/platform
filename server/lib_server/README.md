### Standard
- id - random::string(16)
- token - random::string(10)

### Template
```
ELEMENT_RECT => elements.push(doc! {
    "_type": el._type,

    "x": el.x, "y": el.y, "rotation": el.rotation,
    "width": el.width,
    "height": el.height,
    "stroke": el.stroke,
    "strokeWidth": el.strokeWidth,
    "fill": el.fill,
}),
ELEMENT_CIRCLE => elements.push(doc! {
    "_type": el._type,

    "x": el.x, "y": el.y, "rotation": el.rotation,
    "radius": el.radius,
    "stroke": el.stroke,
    "strokeWidth": el.strokeWidth,
    "fill": el.fill,
}),
ELEMENT_CIRCLES => elements.push(doc! {
    "_type": el._type,
    "key": el.key, "value": min(el.value, 10), "max": min(el.max, 10),

    "x": el.x, "y": el.y, "rotation": el.rotation,
    "margin": el.margin,
    "radius": el.radius,
    "stroke": el.stroke,
    "strokeWidth": el.strokeWidth,
    "fill": el.fill,
    "fill_light": el.fill_light,
}),
ELEMENT_STAR => elements.push(doc! {
    "_type": el._type,

    "x": el.x, "y": el.y, "rotation": el.rotation,
    "numPoints": el.numPoints,
    "innerRadius": el.innerRadius,
    "outerRadius": el.outerRadius,
    "stroke": el.stroke,
    "strokeWidth": el.strokeWidth,
    "fill": el.fill,
}),
ELEMENT_STARS => elements.push(doc! {
    "_type": el._type,
    "key": el.key, "value": min(el.value, 10), "max": min(el.max, 10),

    "x": el.x, "y": el.y, "rotation": el.rotation,
    "margin": el.margin,
    "numPoints": el.numPoints,
    "innerRadius": el.innerRadius,
    "outerRadius": el.outerRadius,
    "stroke": el.stroke,
    "strokeWidth": el.strokeWidth,
    "fill": el.fill,
    "fill_light": el.fill_light,
}),
ELEMENT_TEXT => elements.push(doc! {
    "_type": el._type,
    "key": el.key, "text": el.text,

    "x": el.x, "y": el.y, "rotation": el.rotation,
    "width": el.width,
    "height": el.height,
    "fontSize": el.fontSize,
    "fontStyle": el.fontStyle,
    "fontFamily": el.fontFamily,
    "fill": el.fill,
}),
ELEMENT_IMAGE => elements.push(doc! {
    "_type": el._type,

    "x": el.x, "y": el.y, "rotation": el.rotation,
    "width": el.width,
    "height": el.height,
    "naturalWidth": el.naturalWidth,
    "naturalHeight": el.naturalHeight,
    "base64": el.base64,
    "base64_raw": el.base64_raw,
}),
ELEMENT_BARCODE => elements.push(doc! {
    "_type": el._type,
    "text": el.text, "key": el.key,

    "x": el.x, "y": el.y, "rotation": el.rotation,
    "format": el.format,
    "width_bar": el.width_bar,
    "width": el.width,
    "height": el.height,
    "fill": el.fill,
    "fill_light": el.fill_light,
}),
ELEMENT_QRCODE => elements.push(doc! {
    "_type": el._type,
    "text": el.text, "key": el.key,

    "x": el.x, "y": el.y, "rotation": el.rotation,
    "width": el.width,
    "height": el.height,
    "fill": el.fill,
    "fill_light": el.fill_light,
}),
```