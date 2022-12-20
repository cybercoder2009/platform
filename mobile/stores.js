export const AUTH = 'AUTH'
export const action_auth = (auth) => {return {type: AUTH, auth}}
export const auth = (state = {
    _id: '',
    token: '',
}, action) => {
    switch(action.type){
        case AUTH: return action.auth
        default: return state
    }
}

export const CONFIG = 'CONFIG'
export const action_config = (config) => {return {type: CONFIG, config}}
export const config = (state = {
    endpoint: 'https://na0.reducing.ca/api',
    group: 't6zNqWHbjg2C21uY',
}, action) => {
    switch(action.type){
        case CONFIG: return action.config
        default: return state
    }
}

export const GROUPS = 'GROUPS'
export const action_groups = (groups) => {return {type: GROUPS, groups}}
export const groups = (state = [], action) => {
    switch(action.type){
        case GROUPS: return action.groups
        default: return state
    }
}


export const LOGS = 'LOGS'
export const action_logs = (logs) => {return {type: LOGS, logs}}
export const logs = (state = [], action) => {
    switch(action.type){
        case LOGS: return action.logs
        default: return state
    }
}