import { AUTH } from './action_auth.js'

const init = {
	email: '',
    token: '',
    role: -1,
}

export const auth = (state = init, action) => {
    switch(action.type){
        case AUTH:
            return action.content
        default: 
            return state
    }
}