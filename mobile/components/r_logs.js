import React, {useEffect, useState} from 'react'
import {connect} from 'react-redux'
import {Text, View} from 'react-native'
import ModalDropdown from 'react-native-modal-dropdown'

import Menu from './w_menu.js'
import {displayName} from '../app.json'
import styles from '../styles.js'

const Logs = ({
    logs,
    navigation,
}) => {

    const [ kv_loaded, set_kv_loaded ] = useState(false) 

    useEffect(()=>{
        if(!kv_loaded) {
            set_kv_loaded(true)
            
        }     
    })

    React.useLayoutEffect(() => {
        navigation.setOptions({
            title: displayName,
            headerLeft: _ => null,
            headerRight: _ => <Menu options={['home', 'config', 'logout']} navigation={navigation} />
        });
    }, [navigation])

    return (
        <View>
            <Text>Logs</Text>
        </View>
    )
}

export default connect((props)=>{
    return {
        logs: props.logs
    }
})(Logs)