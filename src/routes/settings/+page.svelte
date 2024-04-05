<script lang="ts">

    import {t, locales, locale} from "$lib/i18n/i18n";
    import {themes} from "$lib/themes";
    import {onMount} from "svelte";
    import {settingsState} from "../../stores/settingsState.js";
    import type {AppSettings} from "$lib/models/Settings";
    import {set_theme} from "$lib/helpers/themeController";
    import {saveSettings} from "$lib/helpers/tauriCommands/appSettingsCommands";
    import {pickDirectory} from "$lib/helpers/tauriCommands/dialogCommands";
    import Modal from "$lib/components/Modal.svelte";
    import LoadingOverlay from "$lib/components/LoadingOverlay.svelte";

    let activeMenuIndex = 0;
    let settings: AppSettings;
    let modalOpen: boolean = false;
    let modalTitle: string;
    let modalMessage: string;

    const save = async () => {
        try {
            await saveSettings(
                settings.theme,
                $locale,
                settings.installPath,
            );
            console.log(settings);
            modalOpen = true;
            modalTitle = $t("settings.success_modal_title");
            modalMessage = $t("settings.success_modal_save");
        }
        catch (e) {
            console.error(e);
            modalOpen = true;
            modalTitle = $t("settings.error_modal_title");
            modalMessage = $t("settings.save_error");
        }
    }

    const selectInstalaltionDir = async () => {
        try {
            const dir: string = await pickDirectory();
            console.log("picked dir: ", dir);
            settings.installPath = dir;
        }
        catch (e) {
            console.error(e);
            modalOpen = true;
            modalTitle = $t("settings.error_modal_title");
            modalMessage = $t("settings.error_modal_choose_dir");
        }
    }

    onMount(() => {
        settingsState.subscribe((value) => {
            console.log(value);
            settings = value;
            console.log("Settings after subscribe: ", settings);
        });
    });

    const changeTheme = (event: Event) => {
        const target = event.target as HTMLSelectElement;
        const theme = target.value;
        set_theme(theme);
        settings.theme = theme;
        settingsState.update((s) => {
            if (!!theme){
                s = {...s, theme: theme};
            }
            return s;
        });
    }


</script>

{#if !!settings}
    <div class="flex flex-row bg-base-200 rounded-box mx-10 mt-10 p-5">
        <div class={`flex flex-col w-[20%] rounded-lg shadow-lg p-3 my-3 ${settings.theme !== 'dark' ? 'bg-white' : ''}`}>
            <button class={"btn " + (activeMenuIndex === 0 ? "font-bold text-xl" : " ")} on:click={()=>{activeMenuIndex = 0}}>
                {$t("settings.menu.general_label")}
            </button>
            <button class={"mt-2 btn " + (activeMenuIndex === 1 ? "font-bold text-xl" : " ")} on:click={()=>{activeMenuIndex = 1}}>
                {$t("settings.menu.advanced")}
            </button>
        </div>
        <div class="flex flex-col grow w-[80%] p-3">

            {#if activeMenuIndex === 0}
                <div class={`rounded-lg shadow-lg p-6 ${settings.theme !== 'dark' ? 'bg-white' : ''}`}>
                    <p class="font-bold text-xl mb-3">{$t("settings.theme.theme_label")}</p>
                    <div class="my-3">
                        <select data-choose-theme class="select select-bordered max-w-3xl text-xl capitalize" bind:value={settings.theme} on:change={changeTheme}>
                            <option disabled selected>{$t("settings.theme.choose_theme")}</option>
                            {#each themes as theme}
                                <option value={theme} class="capitalize">{theme}</option>
                            {/each}
                        </select>
                    </div>
                    <div class="mt-6">
                        <p class="font-bold text-xl mb-3">{$t("settings.language.language_label")}</p>
                        <div class="flex flex-col">
                            <p class="mt-3">{$t("settings.language.app_lang")}:</p>
                            <select class="select select-bordered w-full max-w-xs mt-1" bind:value={$locale}>
                                {#each locales as l}
                                    <option value={l}>{l}</option>
                                {/each}
                            </select>
                            <p class="mt-3">{$t("settings.language.workspace_default_lang")}</p>
                        </div>
                    </div>
                </div>
            {/if}

            {#if activeMenuIndex === 1}
                <div class={`rounded-lg shadow-lg p-6 ${settings.theme !== 'dark' ? 'bg-white' : ''}`}>
                    <p class="font-bold text-xl mb-4">{$t("settings.advanced.installation_dir")}</p>
                    <p>{$t("settings.installation_path")}: {settings.installPath}</p>
                    <button on:click={selectInstalaltionDir} class="btn btn-success mt-3">{$t("settings.change_dir")}</button>
                </div>
            {/if}

        </div>
    </div>
    <div class="flex justify-end mx-10 mt-3">
        <button on:click={save} class="btn btn-success">{$t("settings.save_button")}</button>
    </div>

{:else}
    <LoadingOverlay loading={!settings}/>
{/if}

<Modal modalOpen={modalOpen} closeCallback={()=>{modalOpen = false}}>
    <div class="flex flex-col">
        <p class="text-2xl font-bold">{modalTitle}</p>
        <p class="mt-3">{modalMessage}</p>
    </div>
</Modal>

