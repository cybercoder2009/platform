import {StyleSheet} from 'react-native'

import {color_grey} from './app.json'

const styles = StyleSheet.create({
    menu: {
        style: {
            marginRight: 10,
            marginTop: -18,
            borderWidth: 0,
            width: 100,
        },
        titleStyle: {
            fontWeight: 'bold',
            fontSize: 30,
            textAlign: 'right',
            color: '#000',
        },
        dropdownStyle: {
            borderWidth: 1,
            borderColor: color_grey,
            borderRadius: 2
        },
        dropdownTextStyle: {
            fontSize: 20,
            color: '#000',
            borderWidth: 0,
        }
    }
})

export default styles