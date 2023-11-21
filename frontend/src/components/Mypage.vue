<script lang="ts">

export default {
  components: {
    IconUpdate: () => import('./icons/IconUpdate.vue'),
  },
  data () {
    return {
      balance: '0.0',
      address: 'loading...'
    }
  }, 
  methods: {
    async showBalance() {
      console.log('showBalance() triggered in Mypage.vue');
      const response_json = await this.getBalance();
      this.balance = response_json.balance;
      this.address = response_json.address;
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
      return response_json;
    },
  },
  async created() {
    
    await this.showBalance();
    
  }
}
</script>
<template>
<body>
    <div id="title">
      <h1>Portfolio</h1>
      <h2><a @click="showBalance">Actualizar</a></h2>
    </div><br>
    <div id="portfolio">
      <h2>{{ balance }} Ethereum on {{ address }}</h2>
    </div>
    <div id="transaction-zone">
      <input type="text">
    </div>
</body>
</template>

<style scoped>
    body {
        color: rgb(200, 192, 147);
        align-self: center;
    }

    a {
        text-decoration: none;
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

    #title {
        padding-top: 3rem;
        padding-left: 3rem;
    }

    #portfolio {
        background-color: darkslateblue;
        padding-left: 3rem;
        border-radius: 12px;
    }
</style>