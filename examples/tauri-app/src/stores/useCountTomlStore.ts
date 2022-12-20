import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event"
import { defineStore } from "pinia";
import { ref } from "vue";
import { Count } from "../../src-tauri/bindings/Count"

export const useCountTomlStore = defineStore('count_toml', () => {
    const countToml = ref<Count | null>(null);

    const init = async () => {
        countToml.value = await invoke<Count>('get_count_toml')
        listen<Count>('synced-state://count_toml-update', (event: any) => countToml.value = event.payload )
    }

    return { countToml, init }
})