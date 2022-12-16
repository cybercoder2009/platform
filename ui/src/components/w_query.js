import React, {useState} from 'react'
import {Select, Input, Button, Pagination} from 'antd'
const {Option} = Select

import './w_query.less'

const Query = ({
    lang,
    langs,
    on_query,
    limits,
    total,
    placeholder_search,
    searchable,
}) => {

    const [limit, set_limit] = useState(limits[0])
    const [keyword, set_keyword] = useState('')

    return (
        <Input.Group compact>
            <Button disabled>{placeholder_search}</Button>
            <Input allowClear style={{width:200}} onChange={e=>set_keyword(e.target.value)} />
            {
                searchable
                ? <Button type="primary" onClick={_=>on_query({keyword, skip: 0, limit})}>{langs['search'][lang]}</Button>
                : <Button type="primary" disabled>{langs['search'][lang]}</Button> 
            }
            
            <Button disabled>{langs['limit'][lang]}</Button>
            <Select style={{width: 60}} defaultValue={limit} onChange={e=>set_limit(e)}>
                {limits.map((v, k) => <Option key={k} value={v}>{v}</Option>)}
            </Select>
            <Pagination simple defaultCurrent={1} pageSize={limit} total={total}    
                onChange={(page, pageSize) =>on_query({keyword, skip: (page - 1) * pageSize, limit})}
            />
        </Input.Group>
    )
}

export default Query