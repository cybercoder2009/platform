import React, {useState} from 'react'
import {connect} from 'react-redux'
import {Space, Drawer, Input, Button, Table, Popconfirm, Divider, message} from 'antd'

import {Designer, TEMPLATE} from 'react-designer'
import Query from './w_query.js'
import Group from './w_group.js'
import {service_templates} from '../services.js'
const langs = require('../langs.json')

const SIZES = {
    yl_154_200x200: {width: 200, height: 200},
    yl_213_250x122: {width: 250, height: 122},
    yl_266_296x152: {width: 296, height: 152},
    yl_290_296x128: {width: 296, height: 128},
    yl_420_400x300: {width: 400, height: 300},
    yl_750_640x384: {width: 640, height: 384},
}
const SIZE = 'yl_154_200x200'

const Tempaltes = ({
    auth,
    group,
    lang,
}) => {

    const [design, set_design] = useState(false)
    const [record, set_record] = useState({...TEMPLATE,...{id_group: group.id}})
    const [records, set_records] = useState([])
    const [total, set_total] = useState(0)
    const [size, set_size] = useState(SIZE)

    const columns = [
        {title: langs['id'][lang], dataIndex: 'key', key: "key", fixed: 'left'},
        {title: langs['width'][lang], dataIndex: 'width', key: "width"},
        {title: langs['height'][lang], dataIndex: 'height', key: "height"},
        {title: langs['thumbnail'][lang], key: "thumbnail", render: (_, record)=><img src={record.thumbnail} />},
        {title: langs['actions'][lang], key: 'action', fixed: 'right', width: 150,
            render: r => (
            <Input.Group compact>
                <Button type="primary" size="small" onClick={_=>{
                    set_record({...r, ...{id_group: group.id}})
                    Object.entries(SIZES).map((v, _)=>{
                        if(v[1].width === r.width && v[1].height === r.height){
                            set_size(v[0])
                            return
                        }
                    })
                    set_design(true)
                }}>{langs['edit'][lang]}</Button>
                <Popconfirm title="Delete?" onConfirm={_=>service_templates.del(auth.token, group.id, r.id)
                    .then(res=>{
                        if(res && res.code === 'Success') message.success('Template Deleted')
                    }, err=>console.error(err))
                }>
                    <Button type="primary" danger size="small">{langs['delete'][lang]}</Button>
                </Popconfirm>
            </Input.Group>
        )},
    ]

    return (
        <>
            <Drawer
                title={langs['designer'][lang]}
                height='100%'
                style={{position:'absolute'}}
                placement="top"
                closable={true}
                onClose={_=>{
                    set_record({...TEMPLATE,...{id_group: group.id}})
                    set_design(false)
                }}
                open={design}
                getContainer={false}>
                <Designer
                    template={record}
                    sizes={SIZES}
                    size={size}
                    on_change={t=>set_record(t)}
                    on_save={t=>{
                        service_templates.post(auth.token, t.id_group, [t])
                        .then(_=>{
                            set_record({...TEMPLATE,...{id_group: group.id}})
                            set_design(false)
                        },err=>console.error(err))
                    }}
                />
            </Drawer>
            <>
                <Space>
                    <Group />
                    {
                        group.id !== ""
                        ? <Button type="primary" onClick={_=>{
                            set_record({
                                ...TEMPLATE,
                                ...{id_group: group.id},
                                ...{width: SIZES[SIZE].width, height: SIZES[SIZE].height}
                            })
                            set_design(true)
                        }}>+</Button>
                        : <Button disabled>+</Button>
                    }
                </Space>
                <Divider />
                <Query limits={[20, 40]} total={total} searchable={group.id !== ''}
                    langs={langs} lang={lang} placeholder_search="Template Name"
                    on_query={q=>service_templates.get(auth.token, group.id, q.keyword, q.skip, q.limit)
                        .then(res=>{
                            if(res.code.hasOwnProperty('Success')) {
                                res.data.map(d=>{d.key = d.id})
                                set_total(res.total)
                                set_records(res.data)
                                message.success(`Templates: ${res.total}`)
                            } else message.error(res.code.Error)
                        }, err=>message.error(err))      
                    }
                />
                <Table columns={columns} dataSource={records} pagination={false} size="small" />
            </>
        </>
    )
}

export default connect(props=>{
    return {
        auth: props.auth,
        group: props.group,
    	lang: props.lang,
    }
})(Tempaltes)