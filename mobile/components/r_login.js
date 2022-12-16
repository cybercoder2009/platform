import React, {useState, useEffect} from 'react'
import {connect} from 'react-redux'
import {Text, View, TextInput, Button} from 'react-native'
import {version} from '../package.json'

import {r_config, r_home, color_primary} from '../app.json'
import {kv_set, kv_get, login_post} from '../utilities.js'

const Login = ({
    auth,
    config,
    navigation
}) => {

    const [loaded, set_loaded] = useState(false)
    const [_id, set_id] = useState('')
    const [password, set_password] = useState('')

    useEffect(_=>{
        if(!loaded) {
            set_loaded(true)
            kv_get('auth').then(v=>{
                if (v && v.token && v.token !== '')
                    navigation.navigate(r_home)
            }, err=>console.log(err))
        }
    })

    return (
        <View style={{padding: 10}}>
            <Text style={{
                textAlign: 'center',
                fontSize: 30,
                lineHeight: 30,
                marginTop: 40,
                color: color_primary,
            }}>Reducing</Text>
            <Text style={{
                textAlign: 'center',
                marginTop: 10,
                color: '#000',
            }}>{version}</Text>
            <TextInput value={_id} placeholder="Email" autoCorrect={false} autoCapitalize="none"
                style={{marginTop: 30}}
                onChangeText={e=>set_id(e.trim())} />
            <TextInput value={password} placeholder="Password" secureTextEntry={true} autoCorrect={false}  autoCapitalize="none"
                onChangeText={e=>set_password(e.trim())} />
            <Button title="Login" type="warning" style={{}}
                onPress={_=>login_post(config.endpoint, _id, password).then(json=>{
                        if(json && json.code === 'Success')
                            kv_set('auth', {...auth, ...{_id, token: json.data[0].token}}).then(_=>navigation.navigate(r_home))
                        else console.error(json)
                    }, err=>console.error(err))
                }
            />
            <Text style={{
                textAlign: 'center',
                marginTop: 30,
            }} onPress={_=>navigation.navigate(r_config)}>Config</Text>
        </View>
    )
}

export default connect((props)=>{
    return {
        auth: props.auth,
        config: props.config,
    }
})(Login)