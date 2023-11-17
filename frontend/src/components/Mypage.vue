<script setup lang="ts">
    import { useStore } from 'vuex'
    const store = useStore()
    const user_email = store.state.user;
    const response = await fetch('/api/getBalance', {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            email: user_email
        }),
    });
    const response_json = await response.json();
    const balance = response_json.balance;

</script>
<template>
<body>
    <div id="balance">
        <h1>Balance</h1>
        <h2>{{ balance }}</h2>
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