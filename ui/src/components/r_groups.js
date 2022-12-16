import React, {useState} from 'react'
import {connect} from 'react-redux'
import {Divider, Input, Button, Table, Popconfirm, message} from  'antd'

import Query from './w_query.js'
import {service_groups} from '../services.js'
import {format} from '../utilities.js'
const langs = require('../langs.json')

const Groups = ({
    auth,
    lang,
}) => {

    const [group_name, set_group_name] = useState('')
    const [total, set_total] = useState(0)
    const [records, set_records] = useState([])
    const columns = [
        {title: langs['id'][lang], dataIndex: 'id', key: 'id', fixed: 'left', width: 150, ellipsis: true},
        {title: langs['name'][lang], dataIndex: 'name', key: 'name', fixed: 'left',
            render: (value, record) => <Input value={value} onChange={e=>{
                const _records = [...records]
                _records[records.indexOf(record)].name = e.target.value
                set_records(_records)
            }} />
        },
        {title: langs['items'][lang], dataIndex: 'items', key: 'items'},
        {title: langs['labels'][lang], dataIndex: 'labels', key: 'labels'},
        {title: langs['templates'][lang], dataIndex: 'templates', key: 'templates'},
        {title: langs['associates'][lang], dataIndex: 'associates', key: 'associates'},
        {title: langs['actions'][lang], key: 'actions', fixed: 'right', width: 150,
            render: r => 
            <Input.Group compact>
                <Popconfirm title="Save?" onConfirm={_=>service_groups.patch(auth.token, r.id, r.name)
                    .then(res=>{
                        if(res.code.hasOwnProperty('Success')) message.success('Group Saved')
                        else message.error(err) 
                    }, err=>message.error(err))
                }>
                    <Button type="primary" size="small">{langs['save'][lang]}</Button>
                </Popconfirm>
                <Popconfirm title="Delete?" onConfirm={_=>service_groups.del(auth.token, r.id)
                    .then(res=>{
                        if(res.code.hasOwnProperty('Success')) message.success('Group Deleted')
                        else message.error(err) 
                    }, err=>message.error(err))
                }>
                    <Button type="primary" danger size="small">{langs['delete'][lang]}</Button>
                </Popconfirm>
            </Input.Group>,
        }
    ]

    return (
        <>
            <Input.Group compact>
                <Button disabled>{langs['name'][lang]}</Button>
                <Input value={group_name} style={{width:200}} onChange={e=>set_group_name(e.target.value)} />
                <Button type="primary" onClick={_=>service_groups.post(auth.token, group_name)
                    .then(res=>{
                        if(res && res.code === 'Success')
                            message.success('Group Added')
                    }, err=>console.error(err))
                }>{langs['create'][lang]}</Button>
            </Input.Group>
            <Divider />
            <Query limits={[20]} total={total} searchable={true}
                langs={langs} lang={lang} placeholder_search="Group Name"
                on_query={q=>service_groups.get(auth.token, q.keyword, q.skip, q.limit)
                    .then(res=>{
                        if(res && res.code.hasOwnProperty('Success')) {
                            set_total(res.total)
                            set_records(res.data)
                            message.success(`Groups: ${res.total}`)
                        }
                    }, err=>console.error(err))      
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
    }
})(Groups)