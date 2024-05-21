<script lang="ts">
    import {t} from "$lib/i18n/i18n";
    import { fade } from "svelte/transition";
</script>

<div class="flex flex-col bg-gradient-to-br from-blue-500 to-purple-500 rounded-lg shadow-lg mx-10 mt-10 p-8 justify-center items-center text-white" in:fade={{duration: 500}}>
    <img src="appHubLogo.png" width="200px" class="rounded-full border-4 border-white">
    <p class="font-bold text-4xl mt-4">AppHub</p>
    <p class="mt-2">{$t("appInfo.website")}: <a class="link" href="https://github.com/francesco-gaglione/AppHub">GitHub repo</a></p>
    <p>{$t("appInfo.open_issue")}: <a class="link" href="https://github.com/francesco-gaglione/AppHub/issues">GitHub issues</a></p>
    <p>{$t("appInfo.acknowledgment")}: AppHub team</p>
    <p>{$t("appInfo.legal_notes")}: <a class="link" href="https://github.com/francesco-gaglione/AppHub?tab=MIT-1-ov-file">License</a></p>
    <p>{$t("appInfo.author")}: <a class="link" href="https://github.com/francesco-gaglione">francesco-gaglione</a></p>

</div>
