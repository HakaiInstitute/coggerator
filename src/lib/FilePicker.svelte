<script lang="ts">
    export let value: string | undefined;
    export let id: string = "file-picker";
    export let label: string = "Select Files";
</script>

<label class="block text-gray-700 text-sm font-bold mb-2" for={id}>{label}</label>
<div class="flex">
    <input
            id={id}
            class="flex-auto grow shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            type="text"
            bind:value
            placeholder="No file selected.."
    />
    <button class="flex-none ml-2 bg-neutral-400 hover:bg-neutral-500 text-white font-bold px-3 rounded focus:outline-none focus:shadow-outline" on:click|preventDefault type="button">
        ...
    </button>
</div>
