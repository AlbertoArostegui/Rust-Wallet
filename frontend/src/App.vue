<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import { useStore } from 'vuex';
import { computed } from 'vue';
const store = useStore();
const is_logged_in = computed(() => store.state.is_logged_in);
console.log(is_logged_in);
</script>

<template>
  <body>
    <div>
      <header>
        <nav class="navbar">
          <div class="navbar-left">
            <div>
              <RouterLink to="/">Home</RouterLink>
              <RouterLink to="/about">About</RouterLink>
            </div>
            <div v-if="is_logged_in">
              <RouterLink to="/mypage">My Page</RouterLink>
            </div>
          </div>

          <div v-if="is_logged_in" class="navbar-right">
            <RouterLink to="/logout">Logout</RouterLink>
          </div>
          <div v-else class="navbar-right">
            <RouterLink to="/login">Login</RouterLink>
            <RouterLink to="/register">Register</RouterLink>
          </div>
          
        </nav>
      </header>
    </div>
    <div>
      <RouterView/>
    </div>
  </body>
  
</template>

<style scoped>

header {
  line-height: 3;
}

nav {
  background-color: rgb(40, 42, 54);
  border-radius: 10px;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  font-size: 18px;
  display: inline-block;
  padding: 0 1rem;
}

nav a:first-of-type {
  border: 0;
}

.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.navbar-left, .navbar-right {
  display: flex;
}

RouterLink {
  text-decoration: none;
  color: inherit;
}

@media (min-width: 1024px) {

  nav {
    text-align: left;
    font-size: 1rem;

    margin-top: 1rem;

  }
}
</style>
