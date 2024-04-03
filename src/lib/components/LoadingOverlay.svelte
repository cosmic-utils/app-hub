<script lang="ts">
    export let loading = false;
</script>

{#if loading}
    <div class="fixed inset-0 flex justify-center items-center bg-black bg-opacity-70">
        <div class="animate-spin rounded-full h-32 w-32 border-t-4 border-b-4 border-purple-500"></div>
    </div>
{/if}