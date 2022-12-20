<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { onMounted } from 'vue';
import { useCountStore } from './stores/useCountStore';
import { useCountTomlStore } from './stores/useCountTomlStore';

import { invoke } from "@tauri-apps/api";

let countStore = useCountStore();
let countTomlStore = useCountTomlStore();

const { count } = storeToRefs(countStore)
const { countToml } = storeToRefs(countTomlStore)

onMounted(async () => {
  await countStore.init()
  await countTomlStore.init()

})

const plusCount = async () => {
  await invoke('plus_count')
}

const resetCount = async () => {
  await invoke('reset_count')
}

const plusCountToml = async () => {
  await invoke('plus_count_toml')
}

const resetCountToml = async () => {
  await invoke('reset_count_toml')
}

</script>

<template>
  <div>
    <!-- <p>Pepega</p> -->
    <p>Count: {{ count?.count }}</p>
    <p>This count is preserved when you reload the page</p>
    <button @click="plusCount">+1</button>
    <button @click="resetCount">Reset</button>
    <hr/>
    <p>Count: {{ countToml?.count }}</p>
    <p>This count is preserved when you close and launch the app again</p>
    <button @click="plusCountToml">+1</button>
    <button @click="resetCountToml">Reset</button>
  </div>
</template>
