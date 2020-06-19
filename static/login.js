Vue.component('login', {
	props:{},
	computed: {
		auth_state: function(){
			return this.$store.state.auth !== null;
		}
	},
	data: function() {
		return {
			username: null,
			password: null,
		};
	},
	methods: {
		login: function(){
			this.$store.dispatch("login", {username: this.username, password: this.password});
		},
		register: function(){
			this.$store.dispatch("register", {username: this.username, password: this.password});
		}
	},
	watch: {
		auth_state: function(new_state, old_state){
			if (old_state == false && new_state == true){
				this.$router.replace("/");
			}
		}
	},
	template: `
		<div>
			<h3>
				Login/Register
			</h3>
			<form @submit.prevent="login" style="display:inline-block">
				<div>Username <input v-model="username" @keydown.enter.prevent="login"></input></div>
				<div>Password <input type=password v-model="password" @keydown.enter.prevent="login"></input></div>
				<button @click.prevent="register">Register</button>
				<button type="submit" style="float:right">Login</button>
			</form>
		</div>
	`
})
