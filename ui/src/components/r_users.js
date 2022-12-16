import React, { useEffect, useState } from 'react'
import { connect, useDispatch } from 'react-redux'

import { action_group } from '../store/action_group.js'
import { service_schemas } from '../services.js'
const langs = require('../langs.json')

const Users = ({
    auth,
    lang,
    group
}) => {

    const dispatch = useDispatch()
    
    return (
        <>
         
        </>
    )
}
export default connect(props => {
    return {
        auth: props.auth,
        lang: props.lang,
        group: props.group,
    }
})(Users)