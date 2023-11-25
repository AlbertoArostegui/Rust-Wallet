<script lang="ts">

export default {
  components: {
    IconUpdate: () => import('./icons/IconUpdate.vue'),
  },
  data () {
    return {
      balance: '0.0',
      address: 'loading...',
      hash_error: '',
      address_to: '',
      amount: ''
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
    async sendTransaction() {
      console.log('sendTransaction() triggered in Mypage.vue');
      if (this.address_to.length < 42) {
        this.hash_error = 'Value must be a valid ETH address';
        return false;
      }
      let parsed_amout = parseFloat(this.amount);
      const response = await fetch('/api/sendTransaction', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Content-Length':  '',
        },
        body: JSON.stringify({
          email: this.$store.state.user,
          to: this.address_to,
          amount: parsed_amout
        }),
      });
      const response_json = await response.json();
      return response_json;
    }
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
      <form class="form" @submit.prevent="sendTransaction">
        <div class="form-group">
          <label for="address_to" class="form-label">Address to send it to</label>
          <input type="text" id="address_to" class="form-input" v-model="address_to"/>
          <p v-if="hash_error" class="error-text">{{ hash_error }}</p>
        </div>
        <div class="form-group">
          <label for="amount" class="form-label">Amount in ETH to send (Fees not included)</label>
          <input type="text" id="amount" class="form-input" v-model="amount"/>
        </div>
        <div id="submit-button">
      <button type="submit" class="btn btn-primary">Send</button>
    </div>
      </form>
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

    .form {
        padding-top: 3rem;
        max-width: 400px;
        margin: 0 auto;
        color: rgb(200, 192, 147);
    }

    .form-group {
        margin-bottom: 1rem;
    }

    .form-label {
      display: block;
      margin-bottom: 0.5rem;
      text-align: left;
      color: var(--color-text);
    }

    .form-input {
      display: block;
      width: 100%;
      text-align: left;
      padding: 0.5rem;
      font-size: 1rem;
      line-height: 1.5;
      color: var(--color-text);
      background-color: var(--color-background);
      border: 1px solid var(--color-border);
      border-radius: 0.25rem;
      transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
    }

    .error-text {
      color: red;
    }

    .form-input:focus {
      outline: none;
      border-color: var(--color-primary);
      box-shadow: 0 0 0 0.2rem rgba(38, 143, 255, 0.25);
    }
    img {
        width: 50vw;
    }

    #title {
        padding-top: 3rem;
        padding-left: 3rem;
    }

    #portfolio {
        padding-left: 3rem;
        border-radius: 12px;
    }

    #transaction-zone {
      padding-top: 3rem;
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
      box-shadow: 0 0 0 0.2rem rgba(200, 192, 147, 100);
    }

    #submit-button {
      text-align: center;
    }
</style>