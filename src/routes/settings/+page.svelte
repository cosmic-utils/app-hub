<script lang="ts">

    import {t, locales, locale} from "$lib/i18n/i18n";
    import {themes} from "$lib/themes";


    let activeMenuIndex = 0;

    let createDesktopEntry = true;

    function set_theme(event: Event) {
        const select = event.target as HTMLSelectElement
        const theme = select.value
        if (themes.includes(theme)) {
            const one_year = 60 * 60 * 24 * 365
            window.localStorage.setItem('theme', theme)
            document.cookie = `theme=${theme}; max-age=${one_year}; path=/;`
            document.documentElement.setAttribute('data-theme', theme)
            current_theme = theme
        }
    }

</script>

<div class="flex flex-row bg-base-200 rounded-box mx-10 mt-10 p-5">
    <div class="flex flex-col w-[20%] p-3">
        <button class={"btn " + (activeMenuIndex === 0 ? "font-bold text-xl" : " ")}
                on:click={()=>{activeMenuIndex = 0}}>{$t("settings.menu.general_label")}</button>
        <button class={"btn " + (activeMenuIndex === 1 ? "font-bold text-xl" : " ")}
                on:click={()=>{activeMenuIndex = 1}}>{$t("settings.menu.advanced")}</button>
    </div>
    <div class="flex flex-col grow w-[80%] p-3">

        {#if activeMenuIndex === 0}
            <div>
                <p class="font-bold text-xl">{$t("settings.theme.theme_label")}</p>
                <div class="my-3">
                    <select
                            data-choose-theme
                            class="select select-bordered max-w-3xl text-xl capitalize"
                            on:change={set_theme}
                    >
                        <option disabled selected>{$t("settings.theme.choose_theme")}</option>
                        {#each themes as theme}
                            <option value={theme} class="capitalize">{theme}</option>
                        {/each}
                    </select>
                </div>

                <div class="mt-3">
                    <p class="font-bold text-xl">{$t("settings.language.language_label")}</p>
                    <p class="mt-3">{$t("settings.language.app_lang")}:</p>
                    <select class="select select-bordered w-full max-w-xs mt-1" bind:value={$locale}>
                        {#each locales as l}
                            <option value={l}>{l}</option>
                        {/each}
                    </select>
                    <p class="mt-3">{$t("settings.language.workspace_default_lang")}</p>
                </div>
            </div>
        {/if}

        {#if activeMenuIndex === 1}
            <div class="">
                <p class="font-bold text-2xl mb-4">{$t("settings.advanced.installation_dir")}</p>
                <input type="text" placeholder="some/path/here" class="input input-bordered w-full max-w-xs mt-3 mb-6"/>

                <p class="font-bold text-2xl mt-5 mb-4">{$t("settings.advanced.desktop_entry_title")}</p>
                <div class="grid grid-cols-2 gap-4">
                    <p class="mt-3">{$t("settings.advanced.desktop_entry_des")}</p>
                    <div class="flex items-center">
                        <input bind:value={createDesktopEntry} type="checkbox" name="radio-1" class="checkbox"/>
                    </div>
                </div>
            </div>
        {/if}


    </div>
</div>
<div class="flex flex-row-reverse mx-10 mt-3">
    <button class="btn btn-success">{$t("settings.save_button")}</button>
</div>

