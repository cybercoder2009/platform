import React from 'react'
import { connect, useDispatch } from 'react-redux'
import { Select, Input, Button } from 'antd'
import { GlobalOutlined } from '@ant-design/icons';

import { action_lang } from '../store/action_lang.js'
const langs = require('../langs.json')

const Lang = ({
    lang,
}) => {
    const dispatch = useDispatch()

    return (
        <Input.Group compact>
            <Button disabled icon={<GlobalOutlined />} />
            <Select defaultValue={lang} onChange={e=>dispatch(action_lang(e))}>
                <Select.Option value="en">{langs['lang'].en}</Select.Option>
                <Select.Option value="fr">{langs['lang'].fr}</Select.Option>
            </Select>
        </Input.Group>
    )
}

export default connect(props=>{
	return {
    	lang: props.lang,
    }
})(Lang)