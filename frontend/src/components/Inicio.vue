<script lang="ts">
  export default {
    name: 'Inicio',
    data() {
      return {
        host: 'localhost',
        name: '',
        email: ''
      };
    },
    methods: {
        logName() {
            console.log('Name is: ' + this.name);
        },
        async postData() {
            const response = await fetch('http://'+this.host+':8080/prueba/', {
                mode: 'cors',
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    name: this.name,
                    email: this.email
                }),
            });
            const data = await response.json();
            console.log(data);
        },
    },
    mounted() {
    console.log('Component mounted!')
    },
  };
  
</script>
<template>
<body>
    <div class="texto">
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Beatae quia consequuntur corrupti. Alias illo recusandae earum quibusdam tempora autem, perspiciatis optio libero, ducimus iusto, cum error facere? Eveniet, rem iste. Lorem ipsum dolor sit amet consectetur adipisicing elit. Recusandae ratione odit facilis quis, eaque blanditiis tempore temporibus est a nihil voluptatibus maiores corrupti quam, ipsum aliquid illo, at magni? Earum. Lorem ipsum dolor sit amet consectetur adipisicing elit. Nisi, laborum excepturi. Dolorem ex consequatur, veniam molestias at cum. Tenetur aspernatur architecto, suscipit praesentium laborum illo rem nostrum molestias et laudantium!
        Lorem, ipsum dolor sit amet consectetur adipisicing elit. Enim rerum illum culpa nulla perspiciatis beatae commodi! Ratione optio officiis tenetur. Eligendi officia at sequi numquam nesciunt similique illo expedita eveniet!
        <br><br>Peliculeo intenso
    </div>
    <div>
        <form @submit.prevent="postData">
        <label for="name">Name:</label>
        <input type="text" id="name" v-model="name">
        <label for="email">Email:</label>
        <input type="text" id="email" v-model="email">
        <button type="submit">Log Name</button>
        </form>
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
</style>