import React, {useState} from 'react'
import {connect} from 'react-redux'
import {Input, Button, Table, Popconfirm, Divider, message} from 'antd'

import Query from './w_query.js'
import Group from './w_group.js'
import {service_items} from '../services.js'
const langs = require('../langs.json')

// TODO: editable is not working on table with dynamic columns
const Items = ({
    auth,
    group,
    lang,
}) => {

    const [total, set_total] = useState(0)
    const [records, set_records] = useState([])
    const columns_header = [
        {title: langs['id'][lang], dataIndex: 'id', key: 'id', fixed: 'left'},
        {title: langs['keyword'][lang], dataIndex: 'keyword', key: 'keyword', fixed: 'left'},
        {title: langs['template'][lang], dataIndex: 'template', key: 'template', fixed: 'left'},
    ]
    const columns_tail = [
        {title: langs['actions'][lang], key: 'actions', fixed: 'right', width: 150,
            render: r => 
            <Input.Group compact>
                {/*<Popconfirm title="Save?" onConfirm={_=>service_items.post(auth.token, r.id_group, [r])
                    .then(res=>{
                        if(res && res.code === 'Success') message.success('Item Saved')
                    }, err=>console.error(err))
                }>
                    <Button type="primary" size="small">{langs['save'][lang]}</Button>
                </Popconfirm>*/}
                <Popconfirm title="Delete?" onConfirm={_=>service_items.del(auth.token, group.id, r.id)
                    .then(res=>{
                        if(res && res.code === 'Success') message.success('Item Deleted')
                    }, err=>console.error(err))
                }>
                    <Button type="primary" danger size="small">{langs['delete'][lang]}</Button>
                </Popconfirm>
            </Input.Group>,
        }
    ]
    const [columns, set_columns] = useState([...columns_header, ...columns_tail])

    return (
        <>
            <Input.Group>
                <Group />
            </Input.Group>
            <Divider />
            <Query limits={[20, 40]} total={total}
                langs={langs} lang={lang} placeholder_search="Item ID, keyword" searchable={group.id !== ''}
                on_query={q=>service_items.get(auth.token, group.id, q.keyword, q.skip, q.limit)
                    .then(res=>{
                        if(res && res.code.hasOwnProperty('Success')) {
                            const columns_middle = []
                            if(res.data && res.data.length > 0)
                                Object.keys(res.data[0]).map(k=>{
                                    if(['id', 'keyword', 'template'].indexOf(k) < 0)
                                        columns_middle.push({title: k, dataIndex: k, key: k})
                                })
                            set_total(res.total)
                            set_columns([...columns_header, ...columns_middle, ...columns_tail])
                            set_records(res.data)
                            message.success(`Items: ${res.total}`)
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
})(Items)