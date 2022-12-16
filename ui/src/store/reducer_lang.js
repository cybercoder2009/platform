import { LANG } from './action_lang.js'

export const lang = (state = 'en', action) => {
    switch(action.type){
        case LANG:
            return action.content
        default: 
            return state
    }
}