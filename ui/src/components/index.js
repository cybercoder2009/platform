import React from 'react'
import {connect} from 'react-redux'
import {HashRouter, Routes, Route} from 'react-router-dom'

import Auth from './r_auth.js'
import LayoutAuthoried from './l_authoried.js'
import Users from './r_users.js'
import Home from './r_home.js'
import Groups from './r_groups.js'
import Labels from './r_labels.js'
import Items from './r_items.js'
import Templates from './r_templates.js'
import Associates from './r_associates.js'
import Logs from './r_logs.js'
import Bases from './r_bases.js'
import Settings from './r_settings.js'

const Index = (props) => {

    return ( 
        <HashRouter>
            <Routes>
                <Route path="/auth" exact element={<Auth />} />
                <Route path="/" exact element={<LayoutAuthoried />}>
                    <Route path="users" exact component={<Users />} />
                    <Route path="" exact element={<Home />} />
                    <Route path="groups" exact element={<Groups />} />
                    <Route path="templates" exact element={<Templates />} />
                    <Route path="labels" exact element={<Labels />} />
                    <Route path="items" exact element={<Items />} />
                    {/* <Route path="bases" exact element={<Bases />} /> */}
                    <Route path="associates" exact element={<Associates />} />
                    {/* <Route path="logs" exact element={<Logs />} /> */}
                    <Route path="settings" exact element={<Settings />} />      
                </Route>
            </Routes>
        </HashRouter>
    )   
}

export default connect(props=>{
    return props
})(Index)