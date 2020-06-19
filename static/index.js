function getCookie(name) {
	var regexp = new RegExp("(?:^" + name + "|;\s*"+ name + ")=(.*?)(?:;|$)", "g");
	var result = regexp.exec(document.cookie);
	return (result === null) ? null : result[1];
}
function deleteCookie(name) {   
    document.cookie = name+'=; Max-Age=-99999999;';  
}

function main(){
	Vue.use(Vuex)
	Vue.use(VueRouter)
	const st = new Vuex.Store({
		state: {
			errors: [],
			auth: null,
			
		},
		getters:{
			check_flag: (state) => (flag_name) => {
				if (state.auth == null) {
					return false;
				}
				if (flag_name === null){
					return true;
				}
				if (state.auth.auth_flags.includes(flag_name)){
					return true;
				}
				return false;
			}
		},
		mutations:{
			error: function(state, payload){
				var error = {msg:payload.msg};
				if (payload.timeout != null) {
					error.timer = setTimeout(function(){
						error.timer = null;
						var index = state.errors.indexOf(error);
						if (error == -1){
							return;
						}
						state.errors.splice(index, 1);
					}, payload.timeout*1000);
				}
				state.errors.push(error);
				
			},
			clear_error: function(state, payload){
				var index = payload.index;
				var error = state.errors[index];
				if (error.timer != null){
					clearTimeout(error.timer);
				}
				state.errors.splice(index, 1);
			},
			loadAuthData: function(state, payload){
				var payload = getCookie("auth_token");
				if (payload == null) {
					state.auth = null;
					return;
				}
				var parts = payload.split(":");
				if (parts.length != 2) {
					return false;
				}
				var data = JSON.parse(atob(parts[0]));
				state.auth = data;
			},
			expireAuthData: function(state){
				state.auth = null;
				deleteCookie("auth_token")
			}
		},
		actions:{
			login: function({commit, state}, params){
				fetch("/api/login?username="+params.username+"&password="+params.password)
				.then((response)=>{
					return response.json()
				})
				.then((data) => {
					if (data.error == null){
						commit("loadAuthData")
					}else{
						commit('error', {msg:data.error, timeout: 10});
					}
				})
				.catch((error)=>{
					commit('error', {msg:error, timeout: 10});
				});
			},
			register: function({commit, state}, params){
				fetch("/api/register?username="+params.username+"&password="+params.password)
				.then((response)=>{
					return response.json()
				})
				.then((data) => {
					if (data.error == null){
						commit("loadAuthData")
					}else{
						commit('error', {msg:data.error, timeout: 10});
					}
				})
				.catch((error)=>{
					commit('error', {msg:error, timeout: 10});
				});
			},
		}
	});

	const router = new VueRouter({
		mode: "history",
		routes: [
			{
				path: "/login/",
				component: Vue.component("login"),
				props:true,
			},
		]
	});

	v = new Vue({
		el: "#root",
		store: st,
		router: router,
	});

	v.$store.commit("loadAuthData");
}
