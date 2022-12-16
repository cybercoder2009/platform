import React, {useState} from 'react'
import {connect} from 'react-redux'
import {Input, Button, message} from 'antd'
import {UserOutlined} from '@ant-design/icons'

import { service_users } from '../services.js'
const langs = require('../langs.json')

const Settings = ({
    auth,
    lang,
}) => {

    const [password, set_password] = useState('')

    return (
        <Input.Group compact>
            <Input.Password style={{width:200}} value={password} placeholder={langs['new_password'][lang]} allowClear prefix={<UserOutlined />} 
                onChange={e=>set_password(e.target.value)} />
            <Button type="primary" onClick={_=>service_users.patch(auth.token, auth.email, password)
                .then(res => {
                    if(res && res.code === 'Success'){
                        set_password('')
                        message.success(langs['password_updated'][lang])
                    }
                }, err=>console.error(err))}>{langs['update'][lang]}</Button>
        </Input.Group>
    )
}

export default connect(props => {
    return {
        auth: props.auth,
        lang: props.lang,
    }
})(Settings)