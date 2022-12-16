import { GROUP } from './action_group.js'

export const group = (state = {id: '', name: ''}, action) => {
    switch(action.type){
        case GROUP:
            return action.content
        default: 
            return state
    }
}