<script lang="ts">
    import {t} from "$lib/i18n/i18n";
    import BoxIcon from "$lib/icons/BoxIcon.svelte";
    import SettingsIcon from "$lib/icons/SettingsIcon.svelte";
    import InfoIcon from "$lib/icons/InfoIcon.svelte";
    import {settingsState} from "../../stores/settingsState";
    import {onMount} from "svelte";
    import InstallIcon from "$lib/icons/InstallIcon.svelte";
    import AppStoreIcon from "$lib/icons/AppStoreIcon.svelte";

    let theme: string = "light";
    let isDrawerOpen: boolean = false;

    onMount(() => {
        settingsState.subscribe(value => {
            console.log("settings changed from navbar", value);
            theme = value.theme;
        });
    });

    let closeDrawer = () => {
        isDrawerOpen = false;
    }

</script>

<div class="drawer">
    <input id="my-drawer-3" type="checkbox" bind:checked={isDrawerOpen} class="drawer-toggle" />
    <div class="drawer-content flex flex-col">
        <!-- Navbar -->
        <div class="w-full navbar bg-base-300">
            <div class="flex-none">
                <label for="my-drawer-3" aria-label="open sidebar" class="btn btn-square btn-ghost">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                </label>
            </div>
        </div>
        <!-- Page content -->
        <slot/>
    </div>
    <div class="drawer-side">
        <label for="my-drawer-3" aria-label="close sidebar" class="drawer-overlay"></label>
        <ul class="menu p-4 w-80 min-h-full bg-base-200">
            <!-- Sidebar content here -->
            <li>
                <a href="/" class="py-2 px-4 hover:bg-green-400 transition duration-300 ease-in-out flex flex-row" on:click={closeDrawer}>
                    <BoxIcon height="20px" width="20px" isWhite={theme === "dark"}/>
                    {$t("header.install_app")}
                </a>
            </li>
            <li>
                <a href="/settings" class="flex flex-row py-2 px-4 hover:bg-green-400 transition duration-300 ease-in-out" on:click={closeDrawer}>
                    <SettingsIcon height="20px" width="20px" isWhite={theme === "dark"}/>
                    {$t("header.settings")}
                </a>
            </li>
            <li>
                <a href="/installedApps" class="flex flex-row py-2 px-4 hover:bg-green-400 transition duration-300 ease-in-out" on:click={closeDrawer}>
                    <InstallIcon height="20px" width="20px" isWhite={theme === "dark"}/>
                    {$t("header.installed")}
                </a>
            </li>
            <li>
                <a href="/marketplace" class="flex flex-row py-2 px-4 hover:bg-green-400 transition duration-300 ease-in-out" on:click={closeDrawer}>
                    <AppStoreIcon height="20px" width="20px" isWhite={theme === "dark"}/>
                    {$t("header.app_store")}
                </a>
            </li>
            <li>
                <a href="/appInfo" class="flex flex-row py-2 px-4 hover:bg-green-400 transition duration-300 ease-in-out" on:click={closeDrawer}>
                    <InfoIcon height="20px" width="20px" isWhite={theme === "dark"}/>
                    {$t("header.app_info")}
                </a>
            </li>
        </ul>
    </div>

</div>