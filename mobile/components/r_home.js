import React, {useEffect, useState} from 'react'
import {connect} from 'react-redux'
import {ScrollView, View, TouchableOpacity, TextInput, Text, Button, ToastAndroid} from 'react-native'

import Menu from './w_menu.js'
import {kv_get, items_get, labels_get, labels_post} from '../utilities.js'
import {color_grey, r_login, r_config} from '../app.json'

const Home = ({
    config,
    navigation,
}) => {

    const [auth, set_auth] = useState({})
    const [keyword_item, set_keyword_item] = useState('')
    const [keyword_label, set_keyword_label] = useState('')
    const [items, set_items] = useState([])
    const [labels, set_labels] = useState([])

    React.useLayoutEffect(_ => {
        navigation.setOptions({
            title: `${config.group}`,
            headerLeft: _ => null,
            headerRight: _ => <Menu options={[r_login, r_config]} navigation={navigation} />
        });
    }, [navigation])

    useEffect(_=>{
        if (!auth.id || auth.id === '') {
            kv_get("auth").then(v=>{
                if(v && v.id && v.token && v.id !== '' && v.token !== '') set_auth(v)
                else navigation.navigate(r_login)
            }, _=>navigation.navigate(r_login))
        }
    })

    return (
        <ScrollView style={{
            paddingLeft: 10,
            paddingRight: 10,
            paddingBottom: 10,
        }}>
            <View style={{flexDirection: 'row'}}>
                <View style={{flex: 1, paddingLeft: 10}}>
                    <TextInput value={keyword_item} placeholder="ITEM" autoCorrect={false} autoCapitalize="none"
                        onChangeText={e=>{
                            if (e.length > keyword_item.length)
                                items_get(config.endpoint, auth.token, config.group, e).then(json=>{
                                    if(json && json.code.hasOwnProperty('Success')){
                                        set_labels([])
                                        set_items(json.data)
                                    }
                                }, err=>console.error(err))
                            set_keyword_item(e)
                        }}
                    />
                </View>
                <View style={{flex: 1, paddingLeft: 10}}>
                    <TextInput value={keyword_label} placeholder="LABEL" autoCorrect={false} autoCapitalize="none"
                        onChangeText={e=>{
                            if (e.length > keyword_label.length)
                                labels_get(config.endpoint, auth.token, config.group, e).then(json=>{
                                    if(json && json.code.hasOwnProperty('Success')){
                                        set_items([])
                                        set_labels(json.data)
                                    }
                                }, err=>console.error(err))
                            set_keyword_label(e)  
                        }} />
                </View>
            </View>
            {
                items.map((v, k)=>
                    <TouchableOpacity key={k} onPress={_=>{
                        set_keyword_item(v.id)
                        set_items([])
                    }}>
                        <View style={{
                            borderWidth: 1,
                            borderRadius: 3,
                            borderColor: color_grey,
                            paddingLeft: 10,
                            paddingRight: 10,
                            paddingBottom: 10,
                            marginTop: 10
                        }}>
                            <View style={{flexDirection: 'row'}}>
                                <View style={{flex: 1}}>
                                    <Text style={{lineHeight: 30, paddingLeft: 10}}>{v.id}</Text>
                                </View>
                                <View style={{flex: 1}}>
                                    <Text style={{lineHeight: 30, paddingLeft: 10}}>{v.template}</Text>
                                </View>
                            </View>
                            <View><Text style={{lineHeight: 20, paddingLeft: 10}}>{v.keyword}</Text></View>
                        </View>
                    </TouchableOpacity>
                )
            }
            {
                labels.map((v, k)=>
                    <View key={k} style={{
                        borderWidth: 1,
                        borderColor: color_grey,
                        borderRadius: 3,
                        paddingLeft: 10,
                        paddingRight: 10,
                        paddingBottom: 10,
                        marginTop: 10,
                    }}>
                        <View style={{flexDirection: 'row'}}>
                            <View style={{flex: 1}}>
                                <Text style={{lineHeight: 30, paddingLeft: 10}}>{v.id}({v.width}x{v.height})</Text>
                            </View>
                            <View style={{flex: 1}}>
                                <Text style={{lineHeight: 30, paddingLeft: 10}}>{v.id_item}</Text>
                            </View>
                        </View>
                        <View style={{flexDirection: 'row'}}>
                            <View style={{flex: 1}}>
                            {
                                keyword_item !== ''
                                ? <Button title="bind" color="#6ba547"
                                    onPress={_=>labels_post(config.endpoint, auth.token, config.group, [{...v, ...{id_item: keyword_item}}])
                                    .then(
                                        _ => {
                                            ToastAndroid.show(`Bind ${v.id} + ${keyword_item}`, 5000)
                                            set_labels([])
                                        }, err=>console.error(err)    
                                    )}
                                />
                                : <Button title="bind" color="#6ba547" disabled />
                            }
                            </View>
                            <View style={{flex: 1}}>
                                {
                                    v.id_item === '' ? <></>
                                    : <Button title="unbind" color="#e64345"
                                        onPress={_=>labels_post(config.endpoint, auth.token, config.group, [{...v, ...{id_item: ''}}])
                                        .then(
                                            _ => {
                                                ToastAndroid.show(`Unbind ${v.id}`, 5000)
                                                set_labels([])
                                            }, err=>console.error(err)    
                                        )}
                                />
                                }
                                
                            </View>
                        </View>
                    </View>
                )
            }
        </ScrollView>        
    )
}

export default connect((props)=>{
    return {
        config: props.config,
    }
})(Home)