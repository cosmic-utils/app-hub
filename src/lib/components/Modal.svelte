<script lang="ts">
    import {afterUpdate, onMount} from "svelte";

    export let closeCallback: (modalState: boolean) => void;

    let modal: HTMLDialogElement;

    const openModal = () => {
        if (modal) {
            modal.classList.add('modal-open');
        }
    }

    const closeModal = () => {
        if (modal) {
            modal.classList.remove('modal-open');
            closeCallback(false);
        }
    }

    onMount(() => {
        checkModalState();
    });

    afterUpdate(() => {
        checkModalState();
    });

    function checkModalState() {
        if (modal) {
            openModal();
        } else {
            closeModal();
        }
    }

</script>

<dialog bind:this={modal} class="modal">
    <div class="modal-box">
        <form method="dialog">
            <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" on:click={closeModal}>✕</button>
        </form>
        <slot/>
    </div>
</dialog>