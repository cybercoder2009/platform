import React, {useState, useEffect} from 'react'
import {connect} from 'react-redux'
import {Row, Col, Card, List, message} from 'antd'
import {Bar} from '@ant-design/charts'

import {service_groups, service_logs, service_tasks} from '../services.js'
const langs = require('../langs.json')
const STATUS = [401, 403]

const Home = ({
    auth,
    lang,
}) => {
  
    const [load, set_load] = useState(false)
    const [summary, set_summary] = useState({
        groups: 0,
        // bases: 0,
        associates: 0,
        items: 0,
        labels: 0,
        templates: 0,
    })
    // const [events, set_events] = useState([])
    // const [logs, set_logs] = useState([])
    
    useEffect(_=>{
        if(!load && auth && auth.token && auth.token !== ''){
            set_load(true)

            service_groups.get_summary(auth.token)
            .then(json=>{
                set_summary(json.data[0])
            }, xhr=>{if(STATUS.indexOf(xhr.status) >= 0) window.location.href = '#/auth'; else message.warn(xhr.responseText)})

            // let to = parseInt(new Date().getTime() / 1000)
            // service_logs.get(auth.token, '', '', to - 86400, to, 'timestamp', 1, 0, 20)
            // .then(json=>set_logs(json.data), xhr=>{if(STATUS.indexOf(xhr.status) >= 0) window.location.href = '#/auth'; else message.warn(xhr.responseText)})

            // service_tasks.get_summary(auth.token)
            // .then(json=>{
            //     if(json.data && json.data.length > 0) {
            //         let _events = []
            //         for (let i = 0, m = json.data.length; i < m; i++){
            //             _events.push({date: json.data[i].date, type: 'tasks-send', value: json.data[i].tasks})
            //             _events.push({date: json.data[i].date, type: 'items-import', value: json.data[i].items})
            //             _events.push({date: json.data[i].date, type: 'labels-import', value: json.data[i].labels})
            //         }
            //         set_events(_events)
            //     }
            // }, xhr=>{if(STATUS.indexOf(xhr.status) >= 0) window.location.href = '#/auth'; else message.warn(xhr.responseText)})
        }
    })

    return (
        <>
            <Row gutter={8}>
                <Col xs={24} sm={8} md={8} lg={8} xl={8}><Card title={langs['groups'][lang]} size="small" style={{textAlign: 'center'}}>{summary.groups}</Card></Col>
                {/* <Col xs={12} sm={8} md={4} lg={4} xl={4}><Card title={langs['bases'][lang]} size="small" style={{textAlign: 'center'}}>{summary.sum_bases}</Card></Col> */}
                <Col xs={24} sm={8} md={8} lg={8} xl={8}><Card title={langs['templates'][lang]} size="small" style={{textAlign: 'center'}}>{summary.templates}</Card></Col>
                <Col xs={24} sm={8} md={8} lg={8} xl={8}><Card title={langs['associates'][lang]} size="small" style={{textAlign: 'center'}}>{summary.associates}</Card></Col>
            </Row>
            <Row gutter={8}>
                <Col xs={24} sm={12} md={12} lg={12} xl={12}><Card title={langs['items'][lang]} size="small" style={{textAlign: 'center'}}>{summary.items}</Card></Col>
                <Col xs={24} sm={12} md={12} lg={12} xl={12}><Card title={langs['labels'][lang]} size="small" style={{textAlign: 'center'}}>{summary.labels}</Card></Col>
            </Row>
            {/*<Row gutter={8}>
                <Col xs={24} sm={24} md={24} lg={16} xl={16}>
                    <Card title="Events(last 5 days)" size="small">
                        <Bar data={events}
                            isGroup={true}
                            xField='value'
                            yField='date'
                            seriesField='type'
                            marginRatio={0}
                            label={{
                                position: 'middle',
                                layout: [{type: 'interval-adjust-position' }, {type: 'interval-hide-overlap'}, {type: 'adjust-color'}]
                            }}
                        />
                    </Card>
                </Col>
                <Col xs={24} sm={24} md={24} lg={8} xl={8}>
                    <Card title="Logs" size="small" style={{minHeight: "100%"}} bodyStyle={{maxHeight: 420, overflow: "auto"}}>
                        <List
                            size="small"
                            bordered
                            dataSource={logs}
                            renderItem={log => <List.Item>{new Date(log.timestamp * 1000).toLocaleString()} {log.level} {log.keyword}</List.Item>}
                        />
                    </Card>
                </Col>
            </Row>*/}
        </>
    )
}

export default connect(props=>{
    return {
        auth: props.auth,
    	lang: props.lang,
    }
})(Home)