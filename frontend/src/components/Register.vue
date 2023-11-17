<script lang="ts">
  export default {
    name: 'Inicio',
    data() {
      return {
        name: '',
        email: '',
        password: '',
      };
    },
    methods: {
        async handleRegister() {
            this.$router.push('/');
            const response = await this.checkEmailExists();
            console.log(response);
            if (response.emailExists) {
                alert('Email already exists');
            } else {
                this.registerUser();
            }
        },

        async checkEmailExists() {
            const response = await fetch('/api/checkEmailExists', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Content-Length':  '',
                },
                body: JSON.stringify({
                    email: this.email
                }),
            });
            return response.json();
        },

        async registerUser() {
            const response = await fetch('/api/createNewUser', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Content-Length':  '',
                },
                body: JSON.stringify({
                    name: this.name,
                    email: this.email,
                    password: this.password
                }),
            });
            return response.json();
        }
    },
    mounted() {
    console.log('Component mounted!')
    },
  };
  
</script>
<template>
  <form class="form" @submit.prevent="handleRegister">
    <div class="form-group">
      <label for="name" class="form-label">Nombre</label>
      <input type="text" id="name" class="form-input" v-model="name"/>
    </div>
    <div class="form-group">
      <label for="email" class="form-label">Email</label>
      <input type="email" id="email" class="form-input" v-model="email"/>
    </div>
    <div class="form-group">
      <label for="password" class="form-label">Password</label>
      <input type="password" id="password" class="form-input" v-model="password"/>
    </div>
    <div class="form-group">
      <label for="password-repeat" class="form-label">Repeat Password</label>
      <input type="password-repeat" id="password" class="form-input" />
    </div>
    <button type="submit" class="btn btn-primary">Register</button>
  </form>
</template>

<style scoped>
@import "../assets/base.css";

.form {
  max-width: 400px;
  margin: 0 auto;
}

.form-group {
  margin-bottom: 1rem;
}

.form-label {
  display: block;
  margin-bottom: 0.5rem;
  color: var(--color-text);
}

.form-input {
  display: block;
  width: 100%;
  padding: 0.5rem;
  font-size: 1rem;
  line-height: 1.5;
  color: var(--color-text);
  background-color: var(--color-background);
  border: 1px solid var(--color-border);
  border-radius: 0.25rem;
  transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
}

.form-input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 0.2rem rgba(38, 143, 255, 0.25);
}

.btn {
  display: inline-block;
  font-weight: 400;
  color: var(--color-button-text);
  text-align: center;
  vertical-align: middle;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
  background-color: var(--color-button-background);
  border: 1px solid var(--color-button-border);
  padding: 0.375rem 0.75rem;
  font-size: 1rem;
  line-height: 1.5;
  border-radius: 0.25rem;
  transition: color 0.15s ease-in-out, background-color 0.15s ease-in-out,
    border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
}

.btn:hover {
  color: var(--color-button-text-hover);
  background-color: var(--color-button-background-hover);
  border-color: var(--color-button-border-hover);
}

.btn:focus {
  outline: none;
  box-shadow: 0 0 0 0.2rem rgba(38, 143, 255, 0.25);
}

.btn-primary {
  color: var(--color-primary-text);
  background-color: var(--color-primary);
  border-color: var(--color-primary);
}

.btn-primary:hover {
  color: var(--color-primary-text-hover);
  background-color: var(--color-primary-hover);
  border-color: var(--color-primary-hover);
}

.btn-primary:focus {
  outline: none;
  box-shadow: 0 0 0 0.2rem rgba(38, 143, 255, 0.25);
}
</style>
