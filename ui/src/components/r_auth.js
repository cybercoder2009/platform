import React, {useState} from 'react'
import {connect, useDispatch} from 'react-redux'
import {Card, Input, Space, message} from 'antd'
import {MailOutlined, LockOutlined, LoginOutlined, AndroidOutlined} from '@ant-design/icons'

import Lang from './w_lang.js'
import package_info from '../../package.json'
import {service_auth} from '../services.js'
import {action_auth} from '../store/action_auth.js'
import './r_auth.less'
const langs = require('../langs.json')

const Auth = ({
    lang,
    auth,
}) => {

    const dispatch = useDispatch()
    const [id, set_id] = useState('')
    const [password, set_password] = useState('')

    return (
        <div id="auth">
            <Card
                title={<span>{package_info.brand} <small>{package_info.version}</small></span>}
                extra={<Lang />}
                bordered={false}
                actions={[
                    <LoginOutlined onClick={_ => service_auth.post(id, password)
                        .then(json => {
                            if(json && json.code && json.data && json.code.hasOwnProperty('Success')) {
                                let _auth = {...auth}
                                _auth.id = id
                                _auth.token = json.data[0].token
                                _auth.role = json.data[0].role
                                window.localStorage.setItem('auth', JSON.stringify(_auth))
                                dispatch(action_auth(_auth))
                                window.location.href = '#/'    
                            } else message.warn(json.code['Error'])
                        }, err => message.error(err))} />,
                    <a href="app-release.apk" download><AndroidOutlined /></a>
                ]}
            >
                <Space direction="vertical">
                    <Input size="large"
                        placeholder={langs['email'][lang]}
                        prefix={<MailOutlined />}
                        value={id}
                        onChange={e => set_id(e.target.value)} />
                    <Input.Password size="large"
                        placeholder={langs['password'][lang]}
                        prefix={<LockOutlined />}
                        value={password}
                        onChange={e => set_password(e.target.value)} />
                </Space>
            </Card>
        </div>
    )
}

export default connect(props => {
    return {
        lang: props.lang,
        auth: props.auth,
    }
})(Auth)