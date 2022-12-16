
import 'react-native-gesture-handler'
import React, {useEffect, useState} from 'react'
import {Provider} from 'react-redux'
import {configureStore} from '@reduxjs/toolkit'
import {NavigationContainer} from '@react-navigation/native'
import {createStackNavigator} from '@react-navigation/stack'
const Stack = createStackNavigator()

import {r_home, r_login, r_config, r_logs} from './app.json'
import {kv_get} from './utilities.js'
import {auth, config, groups, logs, action_config} from './stores.js'
import Home from './components/r_home.js'
import Login from './components/r_login.js'
import Config from './components/r_config.js'
import Logs from './components/r_logs.js'
const store = configureStore({
    reducer: {auth, config, groups, logs}
})

const App = _ => {

    const [loaded, set_loaded] = useState(false)
    
    useEffect(_ =>{
        if(!loaded) {
            set_loaded(true)
            kv_get("config").then(
                config=>store.dispatch(action_config(config)),
                _ => {}
            )
        }     
    })

    return (
        <Provider store={store}>
            <NavigationContainer>
                <Stack.Navigator screenOptions={{animationEnabled: false}}>
                    <Stack.Screen name={r_login} component={Login} options={{headerShown: false}} />
                    <Stack.Screen name={r_home} component={Home} />
                    <Stack.Screen name={r_config} component={Config} />
                    <Stack.Screen name={r_logs} component={Logs} />
                </Stack.Navigator>
            </NavigationContainer>
        </Provider>
    )
}

export default App