<script setup lang="ts">
import { useStore } from 'vuex'
import { useRouter } from 'vue-router'
import { computed } from 'vue';
const store = useStore()
const router = useRouter()
store.dispatch('logout')
</script>
