import AsyncStorage from '@react-native-async-storage/async-storage'

export const kv_set = (k, v) => new Promise((res, _) => AsyncStorage.setItem(k, JSON.stringify(v)).then(_=>res()))

export const kv_get = k => new Promise((res, rej) => AsyncStorage.getItem(k).then(v=> v === null ? rej() : res(JSON.parse(v.toString()))))

const headers = {'Accept': 'application/json', 'Content-Type': 'application/json'}

export const login_post = (endpoint, id, password) => new Promise((res, rej) => {
	console.log(`utilities.login_post endpoint=${endpoint} id=${id} password=${password}`)
	fetch(`${endpoint}/auth`, {
		method: 'POST',
		headers,
		body: JSON.stringify({id, password})
	})
	.then(resp => resp.json())
	.then(json=>{
		console.log(`utilities.login_post json=${JSON.stringify(json)}`)
		if(json && json.code.hasOwnProperty('Success')) res(json)
		else rej(json)
	})
	.catch(err=>rej(err))
})

export const items_get = (endpoint, authorization, id_group, keyword) => new Promise((res, rej) => {
	console.log(`utilities.items_get endpoint=${endpoint} id_group=${id_group} keyword=${keyword}`)
	fetch(`${endpoint}/groups/${id_group}/items/q/${keyword.trim()===''?'%20':keyword}/s/0/l/20`, {
		method: 'GET',
		headers: {...headers, ...{'Authorization': authorization}},
	})
	.then(resp => resp.json())
	.then(json => {
		console.log(`utilities.items_get json=${JSON.stringify(json)}`)
		if(json && json.code.hasOwnProperty('Success')) res(json)
		else rej(json)
	})
	.catch(err=>rej(err))
})

export const labels_get = (endpoint, authorization, id_group, keyword) => new Promise((res, rej) => {
	console.log(`utilities.labels_get endpoint=${endpoint} id_group=${id_group} keyword=${keyword}`)
	fetch(`${endpoint}/groups/${id_group}/labels/q/${keyword.trim()===''?'%20':keyword}/s/0/l/20`, {
		method: 'GET',
		headers: {...headers, ...{'Authorization': authorization}},
	})
	.then(resp => resp.json())
	.then(json => {
		console.log(`utilities.labels_get json=${JSON.stringify(json)}`)
		if(json && json.code.hasOwnProperty('Success')) res(json)
		else rej(json)
	})
	.catch(err=>rej(err))
})

export const items_post = (endpoint, authorization, id_group, items) => new Promise((res, rej) => {
	console.log(`utilities.items_post endpoint=${endpoint} id_group=${id_group} items=${JSON.stringify(items)}`)
	fetch(`${endpoint}/groups/${id_group}/items`, {
		method: 'POST',
		headers: {...headers, ...{'Authorization': authorization}},
		body: JSON.stringify(items)
	})
	.then(resp => resp.json())
	.then(json => {
		console.log(`utilities.items_post json=${JSON.stringify(json)}`)
		if(json && json.code.hasOwnProperty('Success')) res(json)
		else rej(json)
	})
	.catch(err=>rej(err))
})

export const labels_post = (endpoint, authorization, id_group, labels) => new Promise((res, rej) => {
	console.log(`utilities.labels_post endpoint=${endpoint} id_group=${id_group} labels=${JSON.stringify(labels)}`)
	fetch(`${endpoint}/groups/${id_group}/labels`, {
		method: 'POST',
		headers: {...headers, ...{'Authorization': authorization}},
		body: JSON.stringify(labels)
	})
	.then(resp => resp.json())
	.then(json => {
		console.log(`utilities.labels_post json=${JSON.stringify(json)}`)
		if(json && json.code.hasOwnProperty('Success')) res(json)
		else rej(json)
	})
	.catch(err=>rej(err))
})