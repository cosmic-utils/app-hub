<script lang="ts">
    export let width: string = "198px";
    export let height: string = "198px";
    export let isWhite: boolean = false;
</script>

{#if (isWhite)}
    <svg width={width} height={height} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="#000000"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path fill="none" stroke="#ffffff" stroke-width="2" d="M19,13.5 L19,17.5 L12,22 L5,17.5 L5,13.5 M12,22 L12,13.5 M18.5,8.5 L12,4.5 L15.5,2 L22,6 L18.5,8.5 L18.5,8.5 L18.5,8.5 Z M5.5,8.5 L12,4.5 L8.5,2 L2,6 L5.5,8.5 L5.5,8.5 L5.5,8.5 Z M18.5,9 L12,13 L15.5,15.5 L22,11.5 L18.5,9 L18.5,9 L18.5,9 Z M5.5,9 L12,13 L8.5,15.5 L2,11.5 L5.5,9 L5.5,9 Z"></path> </g></svg>
{:else}
    <svg width={width} height={height} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="#000000"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path fill="none" stroke="#000000" stroke-width="2" d="M19,13.5 L19,17.5 L12,22 L5,17.5 L5,13.5 M12,22 L12,13.5 M18.5,8.5 L12,4.5 L15.5,2 L22,6 L18.5,8.5 L18.5,8.5 L18.5,8.5 Z M5.5,8.5 L12,4.5 L8.5,2 L2,6 L5.5,8.5 L5.5,8.5 L5.5,8.5 Z M18.5,9 L12,13 L15.5,15.5 L22,11.5 L18.5,9 L18.5,9 L18.5,9 Z M5.5,9 L12,13 L8.5,15.5 L2,11.5 L5.5,9 L5.5,9 Z"></path> </g></svg>
{/if}
