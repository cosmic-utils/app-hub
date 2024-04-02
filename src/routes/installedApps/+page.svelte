<script lang="ts">
    import type {App, AppList} from "$lib/models/Applist";
    import {onMount} from "svelte";
    import {t} from "$lib/i18n/i18n";
    import {installedAppsList, uninstallApp} from "$lib/helpers/tauriCommands/appImageCommands";
    import {error, info} from "@tauri-apps/plugin-log";

    let appList: AppList = {
        apps: []
    };

    const readApplist = async () => {
        try {
            appList = await installedAppsList();
            info("App list read successfully");
        }
        catch (e) {
            error(e + "");
        }
    }

    const uninstall = async (app: App) => {
        try {
            info("Uninstalling app: " + app);
            const uninstalled: boolean = await uninstallApp(app);
            if (uninstalled) {
                console.log("App uninstalled successfully");
                readApplist();
            }
            else {
                error("App could not be uninstalled");
            }
        }
        catch (e) {
            console.error(e);
        }
    }

    onMount(() => {
        readApplist();
    });

</script>

<div class="flex flex-col bg-base-200 rounded-box mx-10 mt-10 p-5">
    <div class="mb-5 flex flex-row justify-between">
        <p class="font-bold text-2xl">{$t("applist.title")}</p>
        <input type="text" class="input input-bordered" placeholder={$t("applist.search")}>
    </div>

    {#each appList.apps as app}
        <div class="flex flex-row justify-between">
            <div class="flex flex-row">
                <img height="40px" width="40px" alt="" src="{app.iconBase64}">
                <p class="font-bold ml-3">{app.name}</p>
            </div>
            <div>
                <button on:click={()=>{uninstall(app)}} class="btn btn-error">{$t("applist.uninstall")}</button>
            </div>
        </div>
        <div class="divider"></div>

    {/each}


</div>