<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type {AppList} from "$lib/models/Applist";
    import {onMount} from "svelte";

    let appList: AppList = {
        apps: []
    };

    const readApplist = async () => {
        try {
            appList = await invoke("read_app_list");
            console.log(appList);
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
        <p class="font-bold text-2xl">App list</p>
        <input type="text" class="input input-bordered" placeholder="Search">
    </div>

    {#each appList.apps as app}
        <div class="flex flex-row justify-between">
            <div class="flex flex-row">
                <img height="40px" width="40px" alt="" src="{app.iconBase64}">
                <p class="font-bold ml-3">{app.name}</p>
            </div>
            <div>
                <button class="btn btn-error">Delete</button>
            </div>
        </div>
        <div class="divider"></div>

    {/each}


</div>