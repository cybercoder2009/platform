import React, {useState} from 'react'
import {connect} from 'react-redux'
import {Input, Button, Table, Popconfirm, Divider, message} from 'antd'

import Query from './w_query.js'
import Group from './w_group.js'
import {service_bases} from '../services.js'
const langs = require('../langs.json')

const Bases = ({
    auth,
    group,
    lang,
}) => {

    const [total, set_total] = useState(0)
    const [record, set_record] = useState({id: ''})
    const [records, set_records] = useState([])
    const columns = [
        {title: langs['id'][lang], dataIndex: 'id', key: 'id', fixed: 'left'},
        {title: langs['group'][lang], dataIndex: 'name_group', key: 'name_group'},
        {title: langs['actions'][lang], key: 'actions', fixed: 'right', width: 150,
            render: r => 
            <Input.Group compact>
                {/*<Popconfirm title="Save?" onConfirm={_=>service_bases.post(auth.token, r.id_group, [r])
                    .then(res=>{
                        if(res && res.code === 'Success') message.success(langs['base_updated'][lang])
                    }, err=>console.error(err))
                }>
                    <Button type="primary" size="small">{langs['save'][lang]}</Button>
                </Popconfirm>*/}
                <Popconfirm title="Delete?" onConfirm={_=>service_bases.del(auth.token, r.id_group, r.id)
                    .then(res=>{
                        if(res && res.code === 'Success') message.success(langs['base_deleted'][lang])
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
                <Input addonBefore={langs['id'][lang]} value={record.id} style={{width:212}} onChange={e=>set_record({...record, ...{id: e.target.value}})} />
                {
                        group.id !== ""
                        ? <Button type="primary" onClick={_=>service_bases.post(auth.token, group.id, [record])
                            .then(res=>{
                                if(res && res.code === 'Success')
                                    message.success('Base Added')
                            }, err=>console.error(err))
                        }>{langs['create'][lang]}</Button>
                        : <Button disabled>{langs['create'][lang]}</Button>
                }
            </Input.Group>
            <Divider />
            <Query limits={[20, 40]} total={total}
                langs={langs} lang={lang} placeholder_search={langs['id'][lang]}
                on_query={q=>service_bases.get(auth.token, group.id, q.keyword, q.skip, q.limit)
                    .then(res=>{
                        if(res && res.code === 'Success') {
                            set_total(res.total)
                            set_records(res.data)
                            message.success(`Bases: ${res.total}`)
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
})(Bases)