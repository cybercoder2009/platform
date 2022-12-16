import React, {useState} from 'react'
import {connect} from 'react-redux'
import {Table, Input, DatePicker, Divider, Button, Select, message} from 'antd'

import Query from './w_query.js'
import {service_logs} from '../services'
import {format} from '../utilities.js'
const langs = require('../langs.json')

const Logs = ({
    auth,
    lang,
}) => {

    const [from, set_from] = useState(0)
    const [to, set_to] = useState(0)
    const [level, set_level] = useState('')
    const [total, set_total] = useState(0)
    const [records, set_records] = useState([])
    const columns = [
        {title: langs['level'][lang], dataIndex: 'level', key: 'level'},
        {title: langs['time'][lang], dataIndex: 'timestamp', key: 'timestamp',
            render: r => <>{format(r)}</>
        },
        {title: langs['keyword'][lang], dataIndex: 'keyword', key: 'keyword'},
        {title: langs['extra'][lang], dataIndex: 'extra', key: 'extra',
            render: r => <>{JSON.stringify(r)}</>
        },
        // {title: langs['initiator'][lang], dataIndex: 'initiator', key: 'initiator'},
    ];
    
    return (
        <>
            <Input.Group>
                <DatePicker.RangePicker showTime onChange={(_, date_strings)=>{
                    set_from(new Date(date_strings[0]).getTime() / 1000)
                    set_to(new Date(date_strings[1]).getTime() / 1000)
                }} />
                <Button disabled>{langs['level'][lang]}</Button>
                <Select value={level} style={{width: 80}} onChange={v=>set_level(v)}>
                    <Select.Option value=''>All</Select.Option>
                    <Select.Option value='Info'>Info</Select.Option>
                    <Select.Option value='Warn'>Warn</Select.Option>
                    <Select.Option value='Error'>Error</Select.Option>
                </Select>
            </Input.Group>
            <Divider />
            <Query limits={[10, 20]} total={total} 
                langs={langs} lang={lang} placeholder_search={langs['keyword'][lang]}
                on_query={q=>service_logs.get(auth.token, q.keyword, level, from, to, q.skip, q.limit)
                    .then(res=>{
                        if(res && res.code === 'Success') {
                            set_total(res.total)
                            set_records(res.data)
                            message.success(`Logs: ${res.total}`)
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
})(Logs)