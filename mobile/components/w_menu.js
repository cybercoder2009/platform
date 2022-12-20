import React from 'react'
import {Text} from 'react-native'
import ModalDropdown from 'react-native-modal-dropdown'

import {r_login} from '../app.json'
import styles from '../styles.js'
import {kv_set, kv_get} from '../utilities'

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
            console.log(`w_menu.value=${value}`)
            if (value === r_login) {
                kv_set('auth', {
                    id: '',
                    token: ''
                }).then(_=>navigation.navigate(r_login))
            }
            else navigation.navigate(value)
        }}>
            <Text style={styles.menu.titleStyle}>...</Text>        
    </ModalDropdown>
}

export default Menu