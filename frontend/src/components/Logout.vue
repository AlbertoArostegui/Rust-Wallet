<script lang="ts">
  export default {
    name: 'Inicio',
    data() {
      return {
      };
    },
    computed() {
        this.$store.dispatch('logout');
    },
    mounted() {
    console.log('Component mounted!')
    },
  };
  
</script>
<template>
</template>
