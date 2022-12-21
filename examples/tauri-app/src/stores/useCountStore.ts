import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event"
import { defineStore } from "pinia";
import { ref } from "vue";
import { Count } from "../../src-tauri/bindings/Count"

export const useCountStore = defineStore('count', () => {
    const count = ref<Count | null>(null);

    const init = async () => {
        count.value = await invoke<Count>('get_count')
        listen<Count>('synced-state://count-update', (event) => count.value = event.payload )
    }

    return { count, init }
})