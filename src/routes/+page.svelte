<script lang="ts">
    import {t} from "$lib/i18n/i18n";
    import {pickAppIcon, pickAppImage} from "$lib/helpers/tauriCommands/dialogCommands";
    import {installAppImage} from "$lib/helpers/tauriCommands/appImageCommands";

    let appPath: string;
    let iconPath: string;
    let enableAdvancedOptions: boolean = false;
    let appName: string;
    let appDescription: string;
    let appType: string = "Application"
    let appTerminal: boolean = false;
    let noSandbox: boolean = false;

    const chooseFile = async () => {
        try {
            appPath = await pickAppImage();
        } catch (e) {
            console.error(e);
            //TODO show error
        }
    }

    const chooseIcon = async () => {
        try {
            iconPath = await pickAppIcon();
        } catch (e) {
            console.error(e);
            //TODO show error
        }
    }

    const installApp = async () => {
        try {
            const res = installAppImage(
                appPath,
                iconPath,
                appName,
                appType,
                appTerminal,
                appDescription,
                noSandbox
            )
            console.log("Installation result", res);
        } catch (e) {
            console.error(e);
            //TODO show error
        }
    }

</script>


<div class="flex flex-col bg-base-200 rounded-box mx-10 mt-10 p-5">

    {#if !!appPath}
        <p class="font-bold text-2xl">{$t("install_file_selected_title")}</p>
        <div class="mt-3">
            <p>{$t("install_file_selected_des")} {appPath}</p>

            <p class="mt-3 font-bold text-2xl">Select icon</p>
            {#if !!iconPath}
                <p>{$t("install_icon_selected")} {iconPath}</p>
            {:else }
                <button on:click={chooseIcon} class="btn btn-neutral mt-5">{$t("install_choose_icon_button")}</button>
            {/if}

            <div class="flex flex-col justify-start items-start mt-4">
                <div class="tooltip tooltip-right"
                     data-tip={$t("install_file.advanced_options.app_name_des")}>
                    <p class="text-xl">{$t("install_file.advanced_options.app_name")}</p>
                </div>
                <input bind:value={appName} type="text" class="input input-bordered mt-2"
                       placeholder={$t("install_file.advanced_options.app_name")}/>
            </div>

            <div class="mt-4 form-control w-1/5">
                <label class="label cursor-pointer">
                    <span class="label-text">Advanced options</span>
                    <input bind:value={enableAdvancedOptions} type="checkbox" checked={enableAdvancedOptions}
                           class="checkbox"/>
                </label>
            </div>

            {#if enableAdvancedOptions}
                <div class="mt-5">
                    <div class="flex flex-col justify-start items-start mt-4">
                        <div class="tooltip tooltip-right"
                             data-tip={$t("install_file.advanced_options.app_des_des")}>
                            <p class="text-xl">{$t("install_file.advanced_options.app_des")}</p>
                        </div>
                        <textarea class="textarea textarea-bordered" bind:value={appDescription}
                                  placeholder={$t("install_file.advanced_options.app_des")}></textarea>
                    </div>
                    <div class="flex flex-col justify-start items-start mt-4">
                        <div class="tooltip tooltip-right"
                             data-tip={$t("install_file.advanced_options.app_type_des")}>
                            <p class="text-xl">{$t("install_file.advanced_options.app_type")}</p>
                        </div>
                        <input type="text" bind:value={appType} class="input input-bordered mt-2"
                               placeholder={$t("install_file.advanced_options.app_type")}/>
                    </div>
                    <div class="flex flex-col justify-start items-start mt-4">
                        <div class="tooltip tooltip-right"
                             data-tip={$t("install_file.advanced_options.app_terminal_des")}>
                            <p class="text-xl">{$t("install_file.advanced_options.app_terminal")}</p>
                        </div>
                        <input bind:checked={appTerminal} type="checkbox" class="checkbox"/>
                    </div>

                    <div class="flex flex-col justify-start items-start mt-4">
                        <div class="tooltip tooltip-right"
                             data-tip={$t("install_file.advanced_options.no_sandbox_des")}>
                            <p class="text-xl">{$t("install_file.advanced_options.no_sandbox")}</p>
                        </div>
                        <input bind:checked={noSandbox} type="checkbox" class="checkbox"/>
                    </div>
                </div>
            {/if}

        </div>
    {:else }
        <div>
            <p class="font-bold text-2xl">{$t("install_choose_file")}</p>
            <button on:click={chooseFile} class="btn btn-neutral mt-5">{$t("install_choose_button")}</button>
        </div>
    {/if}

</div>
<div class="my-5 mx-10">
    <button
            on:click={installApp}
            class="btn btn-success w-full"
            disabled="{!appPath || !iconPath || !appName}">
        {$t("install_file_install_button")}
    </button>
</div>
