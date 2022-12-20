import package_info from '../package.json'

const send = (
	str_method,
	str_url,
	str_token,
	obj_json,
) => new Promise((res, rej)=>{
	console.debug(`services.send str_method=${str_method} str_url=${encodeURI(str_url)}`)
	let xhr = new XMLHttpRequest()
	xhr.onload = () => {
		if (xhr.status == 403 || xhr.status == 404) {
			window.location.href = '#/auth'
			rej(xhr.status)
		} else if (xhr.status >= 200 && xhr.status < 300) {
			try{
	        	res(JSON.parse(xhr.responseText))
        	} catch(e) {
        		rej(xhr)
        	}
		}
    	else rej(xhr)
	}
	xhr.open(str_method, str_url)
	xhr.setRequestHeader('Content-Type', 'application/json')
	xhr.setRequestHeader('Authorization', str_token)
	xhr.send(JSON.stringify(obj_json))
})

const parse = str => str.trim() === '' ? ' ' : str

export const service_auth = {
	post: (str_id, str_password) => send('POST', `${package_info.api}/auth`, '', {id: str_id, password: str_password})
	
}

export const service_logs = {
	get: (str_token, str_keyword, str_level, uint_from, uint_to, uint_skip, uint_limit) => send('GET', `${package_info.api}/logs/q/${parse(str_keyword)}/l/${parse(str_level)}/f/${uint_from}/t/${uint_to}/s/${uint_skip}/l/${uint_limit}`, str_token, {}),
}

export const service_tasks = {
	get: (str_token) => send('GET', `${package_info.api}/tasks`, str_token, {}),
	get_summary: (str_token) => send('GET', `${package_info.api}/tasks/summary`, str_token, {}),
}

export const service_groups = {
	post: (str_token, str_group_name) => send('POST', `${package_info.api}/groups`, str_token, {name: str_group_name}),
	patch: (str_token, str_id_group, str_group_name) => send('PATCH', `${package_info.api}/groups/${str_id_group}`, str_token, {name: str_group_name}),
	get: (str_token, str_keyword, uint_skip, uint_limit) => send('GET', `${package_info.api}/groups/q/${parse(str_keyword)}/s/${uint_skip}/l/${uint_limit}`, str_token, {}),
	get_summary: (str_token) => send('GET', `${package_info.api}/groups/summary`, str_token, {}),
	del: (str_token, str_id_group) => send('DELETE', `${package_info.api}/groups/${str_id_group}`, str_token, {}),
}

export const service_templates = {
	post: (str_token, str_id_group, obj_payload) => send('POST', `${package_info.api}/groups/${str_id_group}/templates`, str_token, obj_payload),
	get: (str_token, str_id_group, str_keyword, uint_skip, uint_limit) => send('GET', `${package_info.api}/groups/${parse(str_id_group)}/templates/q/${parse(str_keyword)}/s/${uint_skip}/l/${uint_limit}`, str_token, {}),
	del: (str_token, str_id_group, str_id_template) => send('DELETE', `${package_info.api}/groups/${str_id_group}/templates/${str_id_template}`, str_token, {}),
}

export const service_items = {
	post: (str_token, str_id_group, json_items) => send('POST', `${package_info.api}/groups/${str_id_group}/items`, str_token, json_items),
	get: (str_token, str_id_group, str_keyword, uint_skip, uint_limit) => send('GET', `${package_info.api}/groups/${parse(str_id_group)}/items/q/${parse(str_keyword)}/s/${uint_skip}/l/${uint_limit}`, str_token, {}),
	del: (str_token, str_id_group, str_id_item) => send('DELETE', `${package_info.api}/groups/${str_id_group}/items/${str_id_item}`, str_token, {}),
}

export const service_labels = {
	post: (str_token, str_id_group, json_labels) => send('POST', `${package_info.api}/groups/${str_id_group}/labels`, str_token, json_labels),
	get: (str_token, str_id_group, str_keyword, uint_skip, uint_limit) => send('GET', `${package_info.api}/groups/${parse(str_id_group)}/labels/q/${parse(str_keyword)}/s/${uint_skip}/l/${uint_limit}`, str_token, {}),
	get_summary: (str_token, str_id_group) => send('GET', `${package_info.api}/groups/${parse(str_id_group)}/labels/summary`, str_token, {}),
	del: (str_token, str_id_group, str_id_label) => send('DELETE', `${package_info.api}/groups/${str_id_group}/labels/${str_id_label}`, str_token, {}),
}

export const service_bases = {
	post: (str_token, str_id_group, json_bases) => send('POST', `${package_info.api}/groups/${str_id_group}/bases`, str_token, json_bases),
	get: (str_token, str_id_group, str_keyword, uint_skip, uint_limit) => send('GET', `${package_info.api}/groups/${parse(str_id_group)}/bases/q/${parse(str_keyword)}/s/${uint_skip}/l/${uint_limit}`, str_token, {}),
	get_summary: (str_token, str_id_group) => send('GET', `${package_info.api}/groups/${parse(str_id_group)}/bases/summary`, str_token, {}),
	del: (str_token, str_id_group, str_id_base) => send('DELETE', `${package_info.api}/groups/${str_id_group}/bases/${str_id_base}`, str_token, {}),
}

export const service_users = {
	post: (str_token, json_users) => send('POST', `${package_info.api}/users`, str_token, json_users),
	get: (str_token, str_keyword, uint_skip, uint_limit) => send('GET', `${package_info.api}/users/q/${parse(str_keyword)}/s/${uint_skip}/l/${uint_limit}`, str_token, {}),
	del: (str_token, str_id_user) => send('DELETE', `${package_info.api}/users/${str_id_user}`, str_token, {}),
	patch:  (str_token, str_id_user, str_user_password) => send('PATCH', `${package_info.api}/users/${parse(str_id_user)}/password`, str_token, { password: str_user_password}),
}

export const service_associates = {
	post: (str_token, str_id_group, json_users) => send('POST', `${package_info.api}/groups/${str_id_group}/associates`, str_token, json_users),
	get: (str_token, str_id_group, str_keyword, uint_skip, uint_limit) => send('GET', `${package_info.api}/groups/${parse(str_id_group)}/associates/q/${parse(str_keyword)}/s/${uint_skip}/l/${uint_limit}`, str_token, {}),
	del: (str_token, str_id_group, str_id_user) => send('DELETE', `${package_info.api}/groups/${str_id_group}/associates/${str_id_user}`, str_token, {}),
}

export const service_parse_csv = (file, delimiter = ',') => new Promise((res, rej)=>{
	let fr = new FileReader()
	fr.onload = function(){
		let str = fr.result
		let cols = str.slice(0, str.indexOf('\n')).split(delimiter)
		for(let i = 0, m = cols.length; i < m; i ++)
			cols[i] = cols[i].trim().toLowerCase().split(' ').join('_')
	    let array = str.slice(str.indexOf('\n') + 1).split('\n')
	    let rows = []
	    for(let i = 0, m = array.length; i < m; i ++)
	    	rows.push(array[i].split(delimiter))
	    res({cols, rows})
	}
	fr.readAsText(file)
})