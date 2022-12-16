import React, { useState, useEffect } from 'react'
import { connect, useDispatch } from 'react-redux'
import { action_group } from '../store/action_group.js'

const Associates = ({
    auth,
    lang,
    langs,
    group,
}) => {

    const dispatch = useDispatch()
    const [schema, set_schema] = useState({ user: {}, label: {}, associate: {}, item: {}})


    const array_form = [{ name: 'id', fieldKey: 'id', rules_req: true, rules_message: 'Missing base id', placeholder: 'Base ID' },
    { name: '_keyword', fieldKey: '_keyword', rules_req: true, rules_message: 'Missing Keyword', placeholder: 'Keyword' },
    { name: 'vendor', fieldKey: 'vendor', rules_req: true, rules_message: 'Missing Vendor', placeholder: 'Vendor' },]
    return (
        <>
           
        </>
    )
}


export default connect(props=>{
    return {
        auth: props.auth,
    	lang: props.lang,
        langs: props.langs,
        group: props.group,
    }
})(Associates)