// libs
import React from 'react'
import {createRoot} from 'react-dom/client'
import {Provider} from 'react-redux'
import {configureStore} from '@reduxjs/toolkit'

// components
import Index from './components/index.js'
import './index.less'

// store
import { lang } from './store/reducer_lang.js'
import { auth } from './store/reducer_auth.js'
import { group } from './store/reducer_group.js'
const store = configureStore({
	reducer: {
		lang,
		auth,
		group,
	}
})
window.getState = store.getState
 
// actions
import { action_auth } from './store/action_auth.js'

// local storage
let local_auth = window.localStorage.getItem('auth')
if(local_auth) store.dispatch(action_auth(JSON.parse(local_auth)))

// render
const root = createRoot(document.getElementById('app'))
root.render(<Provider store={store}><Index /></Provider>)