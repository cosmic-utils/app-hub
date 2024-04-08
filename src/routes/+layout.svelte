<script lang="ts">
    import "../app.css";
    import Navbar from "$lib/components/Navbar.svelte";
    import {settingsState} from "../stores/settingsState.js";
    import {invoke} from "@tauri-apps/api/core";
    import type {AppSettings} from "$lib/models/Settings";
    import {onMount} from "svelte";
    import {set_theme} from "$lib/helpers/themeController";
    import Modal from "$lib/components/Modal.svelte";
    import {t} from "$lib/i18n/i18n";
    import {getSettings} from "$lib/helpers/tauriCommands/appSettingsCommands";

    let errorModalOpen = false;

    let readSettings = async () => {
        try {
            const settings: AppSettings = await getSettings();
            console.log('Settings read', settings);
            settingsState.update(() => settings);

            set_theme(settings.theme);
        }
        catch (e) {
            console.error('Error reading settings', e);
            errorModalOpen = true;
        }
    }

    onMount(() => {
        readSettings();
    });

</script>

<Navbar>
    <slot/>
</Navbar>

<Modal modalOpen={errorModalOpen} closeCallback={()=>{errorModalOpen = false}}>
    <div class="flex flex-col">
        <p class="font-bold text-2xl">{$t("settings.read_settings_error_title")}</p>
        <p>{$t("settings.read_settings_error_des")}</p>
    </div>
</Modal>
