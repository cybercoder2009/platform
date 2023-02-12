import React, {useState} from 'react'
import {connect} from 'react-redux'
import {Input, Button, Table, Popconfirm, Divider, message} from 'antd'

import Query from './w_query.js'
import Group from './w_group.js'
import {service_labels} from '../services.js'
const langs = require('../langs.json')

const Labels = ({
    auth,
    group,
    lang,
}) => {

    const [total, set_total] = useState(0)
    const [records, set_records] = useState([])
    const columns = [
        {title: langs['id'][lang], dataIndex: 'id', key: 'id', fixed: 'left'},
        {title: langs['mac'][lang], dataIndex: 'mac', key: 'mac', fixed: 'left'},
        {title: langs['version'][lang], dataIndex: 'version', key: 'version'},
        {title: langs['width'][lang], dataIndex: 'width', key: 'width'},
        {title: langs['height'][lang], dataIndex: 'height', key: 'height'},
        {title: langs['item'][lang], dataIndex: 'id_item', key: 'id_item',
            render: (value, record) => <Input allowClear value={value} onChange={e=>{
                const _records = [...records]
                _records[records.indexOf(record)].id_item = e.target.value
                set_records(_records)
            }} />
        },
        {title: langs['actions'][lang], key: 'actions', fixed: 'right', width: 150,
            render: r => 
            <Input.Group compact>
                <Popconfirm title="Save?" onConfirm={_=>service_labels.post(auth.token, group.id, [r])
                    .then(res=>{
                        if(res && res.code === 'Success') message.success('Label Saved')
                    }, err=>console.error(err))
                }>
                    <Button type="primary" size="small">{langs['save'][lang]}</Button>
                </Popconfirm>
                <Popconfirm title="Delete?" onConfirm={_=>service_labels.del(auth.token, group.id, r.id)
                    .then(res=>{
                        if(res && res.code === 'Success') message.success('Label Deleted')
                    }, err=>console.error(err))
                }>
                    <Button type="primary" danger size="small">{langs['delete'][lang]}</Button>
                </Popconfirm>
            </Input.Group>,
        }
    ]
    
    return (
        <>
            <Input.Group>
                <Group />
            </Input.Group>
            <Divider />
            <Query limits={[20, 40]} total={total} searchable={group.id !== ''}
                langs={langs} lang={lang} placeholder_search="Label ID"
                on_query={q=>service_labels.get(auth.token, group.id, q.keyword, q.skip, q.limit)
                    .then(res=>{
                        if(res && res.code.hasOwnProperty('Success')) {
                            set_total(res.total)
                            set_records(res.data)
                            message.success(`Labels: ${res.total}`)
                        }
                    }, err=>message.error(err))
                }
            />
            <Table columns={columns} dataSource={records} pagination={false} rowKey="id" size="small" />
        </>
    )
}

export default connect(props => {
    return {
        auth: props.auth,
        lang: props.lang,
        group: props.group,
    }
})(Labels)