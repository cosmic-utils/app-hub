<script lang="ts">
    import type {App, AppList} from "$lib/models/Applist";
    import {onMount} from "svelte";
    import {t} from "$lib/i18n/i18n";
    import {installedAppsList, uninstallApp} from "$lib/helpers/tauriCommands/appImageCommands";
    import {error, info} from "@tauri-apps/plugin-log";
    import Modal from "$lib/components/Modal.svelte";
    import {cloneDeep} from "lodash";
    import TrashIcon from "$lib/icons/TrashIcon.svelte";
    import { fade } from "svelte/transition";

    let appList: AppList = {
        apps: []
    };
    let filteredAppList: AppList = {
        apps: []
    };
    let modalOpen: boolean = false;
    let modalTitle: string;
    let modalMessage: string;
    let detailsModalOpen: boolean = false;
    let appDetails: App;

    const readApplist = async () => {
        try {
            appList = await installedAppsList();
            filteredAppList = cloneDeep(appList);
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
                modalOpen = true;
                modalTitle = $t("applist.success_modal_title");
                modalMessage = $t("applist.uninstallation_successful");
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

    const filterApps = (search: string) => {
        console.log("Filtering apps with search: " + search.length);
        if (search.length > 0){
            filteredAppList.apps = appList.apps.filter(app => app.name.toLowerCase().includes(search.toLowerCase()));
        }
        else {
            filteredAppList = cloneDeep(appList);
        }
    }

    const showDetails = (app: App) => {
        appDetails = app;
        console.log("Showing details for app: " + appDetails);
        detailsModalOpen = true;
    }

    onMount(() => {
        readApplist();
    });

</script>

<div class="flex flex-col bg-base-200 rounded-box mx-10 mt-10 p-5" in:fade={{duration: 500}}>
    <div class="mb-5 flex flex-row justify-between items-center">
        <p class="font-bold text-3xl">{$t("applist.title")}</p>
        <input on:input={(e)=>{ filterApps(e.target.value) }}
               type="text"
               class="input input-bordered w-1/3"
               placeholder={$t("applist.search")}>
    </div>

    {#each filteredAppList.apps as app}
        <div class="flex flex-row justify-between items-center mb-4">
            <div class="flex flex-row items-center">
                <img on:click={()=>{showDetails(app)}} height="40px" width="50px" alt="" src="{app.iconBase64}" class="rounded-lg mr-3 cursor-pointer">
                <p class="font-bold text-lg">{app.name}</p>
            </div>
            <button on:click={()=>{uninstall(app)}} class="btn btn-error">
                <TrashIcon width="30px" height="30px"/>
            </button>
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

<Modal modalOpen={detailsModalOpen} closeCallback={()=>{detailsModalOpen = false}}>
    {#if (appDetails)}
        <div class="flex flex-col">
            <p class="text-2xl font-bold">{$t("applist.app_details")}</p>
            <p class="mt-3">{$t("applist.app_name")}: {appDetails.name}</p>
            <p>{$t("applist.app_version")}: {appDetails.version}</p>
            <p>{$t("applist.app_category")}: {appDetails.category ? appDetails.category : ""}</p>
        </div>
    {/if}
</Modal>
