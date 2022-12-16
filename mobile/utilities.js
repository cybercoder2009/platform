import AsyncStorage from '@react-native-async-storage/async-storage'

export const kv_set = (k, v) => new Promise((res, _) => AsyncStorage.setItem(k, JSON.stringify(v)).then(_=>res()))

export const kv_get = k => new Promise((res, rej) => AsyncStorage.getItem(k).then(v=> v === null ? rej() : res(JSON.parse(v.toString()))))

const headers = {'Accept': 'application/json', 'Content-Type': 'application/json'}

export const login_post = (endpoint, _id, password) => new Promise((res, rej) => 
	fetch(`${endpoint}/auth`, {
		method: 'POST',
		headers,
		body: JSON.stringify({_id, password})
	})
	.then(resp => resp.json())
	.then(json=>{
		if(json && json.code === 'Success') res(json)
		else rej(json)
	})
	.catch(err=>rej(err))
)

export const groups_get = (endpoint, authorization) => new Promise((res, rej) => 
	fetch(`${endpoint}/groups/q/%20/s/name/so/1/s/0/l/20`, {
		method: 'GET',
		headers: {...headers, ...{'Authorization': authorization}},
	})
	.then(resp => resp.json())
	.then(json=>{
		if(json && json.code === 'Success') res(json)
		else rej(json)
	})
	.catch(err=>rej(err))
)

export const items_get = (endpoint, authorization, group_id, keyword) => new Promise((res, rej) =>
	fetch(`${endpoint}/groups/${group_id}/items/q/${keyword.trim()===''?'%20':keyword}/s/name/so/1/s/0/l/20`, {
		method: 'GET',
		headers: {...headers, ...{'Authorization': authorization}},
	})
	.then(resp => resp.json())
	.then(json => {
		if(json && json.code === 'Success') res(json)
		else rej(json)
	})
	.catch(err=>rej(err))
)

export const items_post = (endpoint, authorization, group_id, items) => new Promise((res, rej) =>
	fetch(`${endpoint}/groups/${group_id}/items`, {
		method: 'POST',
		headers: {...headers, ...{'Authorization': authorization}},
		body: JSON.stringify(items)
	})
	.then(resp => resp.json())
	.then(json => {
		if(json && json.code === 'Success') res(json)
		else rej(json)
	})
	.catch(err=>rej(err))
)

export const labels_get = (endpoint, authorization, group_id, keyword) => new Promise((res, rej) =>
	fetch(`${endpoint}/groups/${group_id}/labels/q/${keyword.trim()===''?'%20':keyword}/s/name/so/1/s/0/l/20`, {
		method: 'GET',
		headers: {...headers, ...{'Authorization': authorization}},
	})
	.then(resp => resp.json())
	.then(json => {
		if(json && json.code === 'Success') res(json)
		else rej(json)
	})
	.catch(err=>rej(err))
)

export const labels_post = (endpoint, authorization, group_id, labels) => new Promise((res, rej) =>
	fetch(`${endpoint}/groups/${group_id}/labels`, {
		method: 'POST',
		headers: {...headers, ...{'Authorization': authorization}},
		body: JSON.stringify(labels)
	})
	.then(resp => resp.json())
	.then(json => {
		if(json && json.code === 'Success') res(json)
		else rej(json)
	})
	.catch(err=>rej(err))
)