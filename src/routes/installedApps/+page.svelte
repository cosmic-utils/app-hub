<script lang="ts">
    import type {App, AppList} from "$lib/models/Applist";
    import {onMount} from "svelte";
    import {t} from "$lib/i18n/i18n";
    import {installedAppsList, uninstallApp} from "$lib/helpers/tauriCommands/appImageCommands";
    import {error, info} from "@tauri-apps/plugin-log";
    import Modal from "$lib/components/Modal.svelte";

    let appList: AppList = {
        apps: []
    };
    let modalOpen: boolean = false;
    let modalTitle: string;
    let modalMessage: string;

    const readApplist = async () => {
        try {
            appList = await installedAppsList();
            info("App list read successfully");
        }
        catch (e) {
            error(e + "");
            modalOpen = true;
            modalTitle = $t("applist.error_modal_title");
            modalMessage = $t("applist.read_app_list_error");
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
                modalOpen = true;
                modalTitle = $t("applist.error_modal_title");
                modalMessage = $t("applist.error_modal_uninstall");
            }
        }
        catch (e) {
            console.error(e);
            modalOpen = true;
            modalTitle = $t("applist.error_modal_title");
            modalMessage = $t("applist.error_modal_uninstall");
        }
    }

    onMount(() => {
        readApplist();
    });

</script>

<div class="flex flex-col bg-base-200 rounded-box mx-10 mt-10 p-5">
    <div class="mb-5 flex flex-row justify-between items-center">
        <p class="font-bold text-3xl">{$t("applist.title")}</p>
        <input type="text" class="input input-bordered w-1/3" placeholder={$t("applist.search")}>
    </div>

    {#each appList.apps as app}
        <div class="flex flex-row justify-between items-center mb-4">
            <div class="flex flex-row items-center">
                <img height="40px" width="50px" alt="" src="{app.iconBase64}" class="rounded-lg mr-3">
                <p class="font-bold text-lg">{app.name}</p>
            </div>
            <button on:click={()=>{uninstall(app)}} class="btn btn-error">{$t("applist.uninstall")}</button>
        </div>
        <div class="divider"></div>
    {/each}
</div>


<Modal modalOpen={modalOpen} closeCallback={()=>{modalOpen = false}}>
    <div class="flex flex-col">
        <p class="text-2xl font-bold">{modalTitle}</p>
        <p class="mt-3">{modalMessage}</p>
    </div>
</Modal>
