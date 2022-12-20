import React, {useState} from 'react'
import {connect} from 'react-redux'
import {Space, Input, Button, Table, Popconfirm, Divider, message} from 'antd'

import Query from './w_query.js'
import Group from './w_group.js'
import {service_associates} from '../services.js'
const langs = require('../langs.json')

const Associates = ({
    auth,
    group,
    lang,
}) => {

    const [total, set_total] = useState(0)
    const [record, set_record] = useState('')
    const [records, set_records] = useState([])
    const columns = [
        {title: langs['associate'][lang], dataIndex: 'id', key: 'id', fixed: 'left'},
        {title: langs['actions'][lang], key: 'actions', fixed: 'right', width: 150,
            render: r => 
            <Input.Group compact>
                <Popconfirm title="Delete?" onConfirm={_=>service_associates.del(auth.token, group.id, r.id)
                    .then(res=>{
                        if(res && res.code === 'Success') message.success(langs['associate_deleted'][lang])
                    }, err=>console.error(err))
                }>
                    <Button type="primary" danger size="small">{langs['delete'][lang]}</Button>
                </Popconfirm>
            </Input.Group>,
        }
    ]
 
    return (
        <>
            <Space>
                <Group />
                <Input.Group>
                    <Input addonBefore={langs['associate'][lang]} value={record.id} style={{width:212}} onChange={e=>set_record(e.target.value)} />
                    {
                        group.id !== ""
                        ? <Button type="primary" onClick={_=>service_associates.post(auth.token, group.id, [record])
                            .then(res=>{
                                if(res && res.code === 'Success')
                                    message.success('Associate Added')
                            }, err=>console.error(err))
                        }>+</Button>
                        : <Button disabled>+</Button>
                    }
                </Input.Group>
            </Space>
            <Divider />
            <Query limits={[20, 40]} total={total}
                langs={langs} lang={lang} placeholder_search={langs['id'][lang]} searchable={group.id !== ''}
                on_query={q=>service_associates.get(auth.token, group.id, q.keyword, q.skip, q.limit)
                    .then(res=>{
                        if(res && res.code.hasOwnProperty('Success')) {
                            set_total(res.total)
                            let _data = []
                            res.data.forEach(d=>_data.push({id: d}))
                            set_records(_data)
                            message.success(`Associates: ${res.total}`)
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
        group: props.group,
    }
})(Associates)