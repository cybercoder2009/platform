import React, {useState} from 'react'
import {connect, useDispatch} from 'react-redux'
import {Outlet, Link, useLocation} from 'react-router-dom'

import package_info from '../../package.json'
import Lang from './w_lang.js'
import {action_auth} from '../store/action_auth.js'
import {Layout, Button, Menu, Space} from 'antd'
import {
    ShopOutlined,
    UsergroupAddOutlined,
    BarChartOutlined,
    // ApiOutlined,
    ShoppingCartOutlined,
    TeamOutlined,
    // SnippetsOutlined,
    LogoutOutlined,
    TagOutlined,
    FileImageOutlined,
    ToolOutlined,
} from '@ant-design/icons'
import 'antd/dist/antd.min.css'
import './l_authoried.less'
const {Header, Sider, Content} = Layout
const langs = require('../langs.json')

const LayoutAuthoried = (props) => {

    const [collapsed, set_collapsed] = useState(true)
    const dispatch = useDispatch()
    const location = useLocation()

    const items = [
        {key: '/', label: <Link to="/">{langs['home'][props.lang]}</Link>, icon: <BarChartOutlined />},
        {key: '/groups', label: <Link to="/groups">{langs['groups'][props.lang]}</Link>, icon: <ShopOutlined />},
        {key: '/templates', label: <Link to="/templates">{langs['templates'][props.lang]}</Link>, icon: <FileImageOutlined />},
        {key: '/labels', label: <Link to="/labels">{langs['labels'][props.lang]}</Link>, icon: <TagOutlined />},
        {key: '/items', label: <Link to="/items">{langs['items'][props.lang]}</Link>, icon: <ShoppingCartOutlined />},
        // {key: '/bases', label: <Link to="/bases">{langs['bases'][props.lang]}</Link>, icon: <ApiOutlined />},
        {key: '/associates', label: <Link to="/associates">{langs['associates'][props.lang]}</Link>, icon: <TeamOutlined />},
        // {key: '/logs', label: <Link to="/logs">{langs['logs'][props.lang]}</Link>, icon: <SnippetsOutlined />},
        {key: '/settings', label: <Link to="/settings">{langs['settings'][props.lang]}</Link>, icon: <ToolOutlined />},
    ]

    if(props.auth.role === 0)
        items.unshift({key: '0', label: <Link to="/users">{langs['users'][props.lang]}</Link>, icon: <UsergroupAddOutlined />})

    return ( 
        <Layout style={{minHeight: '100vh'}}>
            <Sider theme="light" 
                collapsible 
                collapsed={collapsed}
                collapsedWidth={40}
                onCollapse={_=>set_collapsed(!collapsed)}>
                <div id="brand">{collapsed ? package_info.brand_short : package_info.brand}</div>
                <Menu defaultSelectedKeys={['/']} selectedKeys={location.pathname} mode="inline" items={items}></Menu>
            </Sider>
            <Layout>
                <Header>
                    <Space><span>v {package_info.version}</span></Space>
                    <Space>
                        <Lang />
                        <Button type="primary" danger
                            style={{width: 'auto', padding: '0 10px'}}
                            onClick={_=>{
                                let _auth = Object.assign({}, props.auth)
                                _auth.email = ''
                                _auth.password = ''
                                _auth.token = ''
                                dispatch(action_auth(_auth))
                                window.localStorage.removeItem('auth')
                                window.location.href = '#/auth';
                            }}
                        >{props.auth.email} <LogoutOutlined /></Button>
                    </Space> 
                </Header>
                <Content style={{padding: 10, minWidth: '50vw'}}>
                    <Outlet />
                </Content>
            </Layout>
        </Layout>
    )
}

export default connect(props=>{
    return props
})(LayoutAuthoried)