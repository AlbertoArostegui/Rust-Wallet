<script lang="ts">
export default {
  data () {
    return {
      balance: ''
    }
  }, 
  methods: {
    async showBalance() {
      console.log('showBalance() triggered in Mypage.vue');
      const balance = await this.getBalance();
      this.balance = balance;
    },
    async getBalance() {
      console.log('getBalance() triggered in Mypage.vue');
      const response = await fetch('/api/getBalance', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Content-Length':  '',
        },
        body: JSON.stringify({
          email: this.$store.state.user,
        }),
      });
      const response_json = await response.json();
      return response_json.balance;
    },
  },
  async created() {
    console.log('Mypage was created!');
    await this.showBalance();
    console.log('Mypage finished creating!')
  }
}
</script>
<template>
<body>
    <div id="balance">
        <h1>Balance</h1>
        <button @click="showBalance"></button>
        <h2>balance {{ balance }}</h2>
        <h2 v-if="balance">v-if balance</h2>
    </div>
</body>
</template>

<style scoped>
    body {
        color: rgb(200, 192, 147);
        display: flex;
        align-self: center;
    }

    .texto {
        padding: 5rem;
    }

    .imagen {
        padding: 5rem;
    }

    img {
        width: 50vw;
    }

    #balance {
        padding-top: 3rem;
        padding-left: 3rem;
    }
</style>