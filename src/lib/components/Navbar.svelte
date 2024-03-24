<script lang="ts">
    import {t, locales, locale} from "$lib/i18n/i18n";

</script>

<div class="drawer">
    <input id="my-drawer-3" type="checkbox" class="drawer-toggle" />
    <div class="drawer-content flex flex-col">
        <!-- Navbar -->
        <div class="w-full navbar bg-base-300">
            <div class="flex-none">
                <label for="my-drawer-3" aria-label="open sidebar" class="btn btn-square btn-ghost">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                </label>
            </div>
            <div class="flex-1 px-2 mx-2">AppHub</div>
        </div>
        <!-- Page content here -->
        <slot/>
    </div>
    <div class="drawer-side">
        <label for="my-drawer-3" aria-label="close sidebar" class="drawer-overlay"></label>
        <ul class="menu p-4 w-80 min-h-full bg-base-200">
            <!-- Sidebar content here -->
            <li><a href="/">{$t("header.install_app")}</a></li>
            <li><a href="/settings">{$t("header.settings")}</a></li>
            <li><a href="/installedApps">{$t("header.installed")}</a></li>
            <li><a>{$t("header.app_info")}</a></li>
        </ul>
    </div>
</div>