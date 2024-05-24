<script lang="ts">
    import {onMount} from "svelte";
    import {installAppFromRemote, readAppsDatabase} from "$lib/helpers/tauriCommands/appDatabaseCommands";
    import {RemoteAppInfo} from "$lib/models/AppDatabase";
    import {t} from "$lib/i18n/i18n";
    import Modal from "$lib/components/Modal.svelte";
    import LoadingOverlay from "$lib/components/LoadingOverlay.svelte";

    let database: RemoteAppInfo[];
    let filteredDatabase: RemoteAppInfo[];

    let modalOpen = false;
    let modalTitle = "";
    let modalMessage = "";

    let isLoading = false;

    onMount(() => {
        console.log("Page mounted");
        readAppsDatabase().then((res) => {
            console.log("database ", database)
            database = res.appList;
            filteredDatabase = database;
        })
    });

    const filter = (event: Event) => {
        const target = event.target as HTMLInputElement;
        const valueToFilter = target.value?.toUpperCase() || '';

        filteredDatabase = database.filter((el) =>
            el.name.toUpperCase().includes(valueToFilter)
        );
    }

    const installApp = async (app: RemoteAppInfo) => {
        console.log("Installing app ", app);
        isLoading = true;
        try {
            let res = await installAppFromRemote(app.download_url, app.name);
            modalOpen = true;
            modalTitle = "Success";
            modalMessage = "App installed successfully";
        }
        catch (e) {
            modalOpen = true;
            modalTitle = "Error";
            modalMessage = "Error installing app";
        }
        finally {
            isLoading = false;
        }
    }

</script>

<div class="flex flex-row justify-between m-8">
    <p class="font-bold text-2xl">{$t("marketplace.title")}</p>
    <input type="text" placeholder={$t("marketplace.search")} on:change={filter}
           class="input input-bordered w-full max-w-xs"/>
</div>

{#if filteredDatabase && filteredDatabase && filteredDatabase.length > 0}
    <div class="flex flex-col m-5">
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
            {#each filteredDatabase as app}
                <div class="card w-full m-3 bg-base-100 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">{app.name}</h2>
                        <div class="card-actions justify-end">
                            <button class="btn btn-primary" on:click={()=>{installApp(app)}}>Install</button>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    </div>
{:else}
    <div class="card w-full m-3 bg-base-100 shadow-xl">
        <div class="card-body">
            <h2 class="card-title mx-auto">{$t("marketplace.not_found")}</h2>
        </div>
    </div>
{/if}

<Modal bind:modalOpen={modalOpen} closeCallback="{()=>modalOpen = false}">
    <div class="flex flex-col">
        <p class="font-bold text-2xl">{modalTitle}</p>
        <p>{modalMessage}</p>
    </div>
</Modal>

<LoadingOverlay bind:loading={isLoading}/>