import React, {useEffect, useState} from 'react'
import {connect} from 'react-redux'
import {ScrollView, View, TouchableOpacity, TextInput, Text, Button, ToastAndroid} from 'react-native'
import {Picker} from '@react-native-picker/picker'

import Menu from './w_menu.js'
import {kv_get, groups_get, items_get, labels_get, labels_post} from '../utilities.js'
import {color_grey, displayName, r_login} from '../app.json'

const Home = ({
    config,
    navigation,
}) => {

    const [auth, set_auth] = useState({})
    const [groups, set_groups] = useState([])
    const [group_id, set_group_id] = useState('')
    const [keyword_item, set_keyword_item] = useState('')
    const [keyword_label, set_keyword_label] = useState('')
    const [items, set_items] = useState([])
    const [labels, set_labels] = useState([])

    React.useLayoutEffect(() => {
        navigation.setOptions({
            title: displayName,
            headerLeft: _ => null,
            headerRight: _ => <Menu options={[r_login]} navigation={navigation} />
        });
    }, [navigation])

    useEffect(_=>{
        if (!auth._id) {
            kv_get("auth").then(v=>{
                if(v && v._id && v.token && v._id !== '' && v.token !== ''){
                    set_auth(v)
                    groups_get(config.endpoint, v.token).then(json=>{
                        if(json && json.code === 'Success'){
                            set_groups(json.data)
                            if(json.data.length > 0) set_group_id(json.data[0]._id)
                        }
                    }, err=>{
                        if (err && err.error && err.error.code === 403) navigation.navigate(r_login)
                        else console.error(err)
                    })
                } else navigation.navigate(r_login)
            }, _=>navigation.navigate(r_login))
        }
    })

    return (
        <ScrollView style={{
            paddingLeft: 10,
            paddingRight: 10,
            paddingBottom: 10,
        }}>
            <Picker selectedValue={group_id} onValueChange={(v, _)=>set_group_id(v)}>
                {groups.map((v, k)=><Picker.Item key={k} label={v.name} value={v._id} />)}
            </Picker>
            <View style={{flexDirection: 'row'}}>
                <View style={{flex: 1, paddingLeft: 10}}>
                    <TextInput value={keyword_item} placeholder="ITEM" autoCorrect={false} autoCapitalize="none"
                        onChangeText={e=>{
                            set_keyword_item(e)
                            items_get(config.endpoint, auth.token, group_id, e).then(json=>{
                                if(json && json.code === 'Success'){
                                    set_labels([])
                                    set_items(json.data)
                                }
                            }, err=>console.error(err))
                        }}
                    />
                </View>
                <View style={{flex: 1, paddingLeft: 10}}>
                    <TextInput value={keyword_label} placeholder="LABEL" autoCorrect={false} autoCapitalize="none"
                        onChangeText={e=>{
                            set_keyword_label(e)
                            labels_get(config.endpoint, auth.token, group_id, e).then(json=>{
                                if(json && json.code === 'Success'){
                                    set_items([])
                                    set_labels(json.data)
                                }
                            }, err=>console.error(err))  
                        }} />
                </View>
            </View>
            {
                items.map((v, k)=>
                    <TouchableOpacity key={k} onPress={_=>set_keyword_item(v._id)}>
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
                                    <Text style={{lineHeight: 30, paddingLeft: 10}}>{v._id}</Text>
                                </View>
                                <View style={{flex: 1}}>
                                    <Text style={{lineHeight: 30, paddingLeft: 10}}>{v._template}</Text>
                                </View>
                            </View>
                            <View><Text style={{lineHeight: 20, paddingLeft: 10}}>{v._keyword}</Text></View>
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
                                <Text style={{lineHeight: 30, paddingLeft: 10}}>{v._id}({v.width}x{v.height})</Text>
                            </View>
                            <View style={{flex: 1}}>
                                <Text style={{lineHeight: 30, paddingLeft: 10}}>{v._id_item}</Text>
                            </View>
                        </View>
                        <View style={{flexDirection: 'row'}}>
                            <View style={{flex: 1}}>
                            {
                                keyword_item !== ''
                                ? <Button title="bind" color="#6ba547"
                                    onPress={_=>labels_post(config.endpoint, auth.token, group_id, [{...v, ...{_id_item: keyword_item}}])
                                    .then(
                                        _=>ToastAndroid.show(`Bind ${v._id} + ${keyword_item}`, 5000),
                                        err=>console.error(err)    
                                    )}
                                />
                                : <Button title="bind" color="#6ba547" disabled />
                            }
                            </View>
                            <View style={{flex: 1}}>
                                <Button title="unbind" color="#e64345"
                                    onPress={_=>labels_post(config.endpoint, auth.token, group_id, [{...v, ...{_id_item: ''}}])
                                    .then(
                                        _=>ToastAndroid.show(`Unbind ${v._id}`, 5000),
                                        err=>console.error(err)    
                                    )}
                                />
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