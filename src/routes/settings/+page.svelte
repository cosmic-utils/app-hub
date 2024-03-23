<script lang="ts">

    import {t, locales, locale} from "$lib/i18n/i18n";


    let activeMenuIndex = 0;

    let createDesktopEntry = true;


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
                <div class="dropdown mb-72">
                    <div tabindex="0" role="button" class="btn m-1">
                        {$t("settings.theme.change_theme")}
                        <svg width="12px" height="12px" class="h-2 w-2 fill-current opacity-60 inline-block"
                             xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2048 2048">
                            <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
                        </svg>
                    </div>
                    <ul class="dropdown-content z-[1] p-2 shadow-2xl bg-base-300 rounded-box w-52">
                        <li>
                            <input type="radio"
                                   name="theme-dropdown"
                                   class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                                   aria-label="Light"
                                   value="light"/>
                        </li>
                        <li>
                            <input type="radio"
                                   name="theme-dropdown"
                                   class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                                   aria-label="Dark"
                                   value="dark"/>
                        </li>
                        <li>
                            <input type="radio"
                                   name="theme-dropdown"
                                   class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                                   aria-label="Cupcake"
                                   value="cupcake"/>
                        </li>
                    </ul>
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

