export const AUTH = 'AUTH'

export function action_auth(content){
    return {
        type: AUTH,
        content
    }
}