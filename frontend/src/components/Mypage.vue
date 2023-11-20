<script lang="ts">
export default {
  data () {
    return {
      balance: '0.0',
      address: 'nada'
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
    console.log('Mypage was created!!!!');
    await this.showBalance();
    console.log('Mypage finished creating! la pelicula es considerable')
  }
}
</script>
<template>
<body>
    <div id="title">
      <h1>Portfolio</h1>  
    </div><br>
    <div id="portfolio">
      <h2>{{ balance }} Ethereum on {{ address }}</h2>
      <h3>Grafico de eth</h3>
    </div>
</body>
</template>

<style scoped>
    body {
        color: rgb(200, 192, 147);
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