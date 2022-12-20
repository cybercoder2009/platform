import React from 'react'
import {connect, useDispatch} from 'react-redux'
import {View, Button, TextInput, ToastAndroid} from 'react-native'

import Menu from './w_menu.js'
import {kv_set} from '../utilities.js'
import {action_config} from '../stores.js'
import {displayName, r_login, r_home} from '../app.json'

const Config = ({
    config,
    navigation
}) => {

    const dispatch = useDispatch()

    React.useLayoutEffect(() => {
        navigation.setOptions({
            title: displayName,
            headerLeft: _ => null,
            headerRight: _ => <Menu options={[r_home, r_login]} navigation={navigation} />
        });
    }, [navigation])

    return (
        <View style={{padding: 10}}>
            <TextInput value={config.endpoint} placeholder="Endpoint" autoCorrect={false}  autoCapitalize="none"
                onChangeText={e=>dispatch(action_config({...config, ...{endpoint: e.trim().toLowerCase()}}))} />
            <TextInput value={config.group} placeholder="Group" autoCorrect={false}  autoCapitalize="none"
                onChangeText={e=>dispatch(action_config({...config, ...{group: e.trim().toLowerCase()}}))} />    
            <Button title="Save" onPress={_=>{
                kv_set('config', config)
                ToastAndroid.show("Save Config", 5000)
            }} />
        </View>
    )
}

export default connect((props)=>{
    return {
        config: props.config
    }
})(Config)