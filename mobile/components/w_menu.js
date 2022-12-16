import React from 'react'
import {Text} from 'react-native'
import ModalDropdown from 'react-native-modal-dropdown'

import {r_home} from '../app.json'
import styles from '../styles.js'
import { kv_set } from '../utilities'

const Menu = ({
    options,
    navigation,
}) => {

    return <ModalDropdown
        style={styles.menu.style}
        dropdownStyle={styles.menu.dropdownStyle}
        dropdownTextStyle={styles.menu.dropdownTextStyle}
        animated={false}
        options={options}
        onSelect={(_, value) => {
            if (value === r_home) {
                kv_get('auth').then(v=>{
                    v.token = ''
                    kv_set('auth', v).then(_=>navigation.navigate(r_home))
                }, err=>console.log(err))
            }
            else navigation.navigate(value)
        }}>
            <Text style={styles.menu.titleStyle}>...</Text>        
    </ModalDropdown>
}

export default Menu