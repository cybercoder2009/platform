import React, {useState} from 'react'
import {connect, useDispatch} from 'react-redux'
import {AutoComplete, Button, Input, message} from 'antd'

import {service_groups} from '../services.js'
import {action_group} from '../store/action_group.js'
const langs = require('../langs.json')

const Group = ({
    auth,
    group,
    lang,
}) => {

    const [value, set_value] = useState(group.name)
    const [options, set_options] = useState([])
    const dispatch = useDispatch()
    
    return (
        <>
            <Button disabled>{langs['group'][lang]}</Button>
            <AutoComplete allowClear
                options={options}
                style={{width:'100px'}}
                value={value}
                onSelect={(_, e)=>{
                    set_value(e.label)
                    dispatch(action_group({id: e.value, name: e.label}))
                }}
                onClear={_=>dispatch(action_group({id: '', name: ''}))}
                onSearch={v=>{
                    set_value(v)
                    service_groups.get(auth.token, v, 0, 20)
                    .then(res=>{
                        if(res.code.hasOwnProperty('Success')) {
                            const _options = []
                            res.data.map(e=>{_options.push({value: e.id, label: e.name})})
                            set_options(_options)
                        } else message.error(res.code.Error)
                    },err=>message.error(res.code.Error))
                }}
            ></AutoComplete>
            {
                group.id !== ''
                ? <Input value={group.id} style={{width:'208px'}} disabled />
                : null
            }
        </>
    )
}

export default connect(props=>{
	return {
    	auth: props.auth,
        group: props.group,
        lang: props.lang,
    }
})(Group)