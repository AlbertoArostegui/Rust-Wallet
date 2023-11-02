<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <body>
    <div>
      <header>
        <nav class="navbar">
          <div class="navbar-left">
            <RouterLink to="/">Home</RouterLink>
            <RouterLink to="/about">About</RouterLink>
            <RouterLink to="/forum">Forum</RouterLink>
          </div>
          <div class="navbar-right">
            <RouterLink to="/login">Login</RouterLink>
            <RouterLink to="/login">Register</RouterLink>
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
  border-left: 1px solid var(--color-border);
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
