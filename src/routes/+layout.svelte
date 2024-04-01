<script lang="ts">
    import "../app.css";
    import Navbar from "$lib/components/Navbar.svelte";
    import {settingsState} from "../stores/settingsState.js";
    import {invoke} from "@tauri-apps/api/core";
    import type {AppSettings} from "$lib/models/Settings";
    import {onMount} from "svelte";
    import {set_theme} from "$lib/helpers/themeController";


    let readSettings = async () => {
        try {
            const settings: AppSettings = await invoke<AppSettings>("read_settings");
            console.log('Settings read', settings);
            settingsState.update(() => settings);

            set_theme(settings.theme);

        } catch (error) {
            console.error('Error reading settings', error);
            //TODO: show error message
        }
    }

    onMount(() => {
        readSettings();
    });

</script>

<Navbar>
    <slot/>
</Navbar>
