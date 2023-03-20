<script lang="ts">
    import {open, save} from "@tauri-apps/api/dialog"
    import {homeDir, sep} from "@tauri-apps/api/path";
    import {
        isPermissionGranted,
        requestPermission,
        sendNotification
    } from '@tauri-apps/api/notification';
    import invariant from "tiny-invariant";

    import { onMount } from "svelte";

    import FilePicker from "./FilePicker.svelte";
    import Select from "./Select.svelte";
    import {invoke} from "@tauri-apps/api";

    let inputPath: string | null = null;
    let outputPath: string | null = null;
    let noDataValue: number | null = null;
    let compression: string = "DEFLATE";
    const compressionOptions = [
        // TODO: Support all options
        // ["JPEG", "JPEG"],
        // ["WEBP", "WEBP"],
        ["ZSTD", "ZSTD"],
        ["LZW", "LZW"],
        ["Deflate", "DEFLATE"],
        ["Packbits", "PACKBITS"],
        ["LZMA", "LZMA"],
        ["LERC", "LERC"],
        ["LERC Deflate", "LERC_DEFLATE"],
        ["LERC ZSTD", "LERC_ZSTD"],
        ["None", "NONE"]
    ];
    let resampling: string = "CUBIC";
    const resamplingOptions = [
        ["Nearest", "NEAREST"],
        ["Average", "AVERAGE"],
        ["Bilinear", "BILINEAR"],
        ["Cubic", "CUBIC"],
        ["Cubic Spline", "CUBICSPLINE"],
        ["Lanczos", "LANCZOS"],
        ["Mode", "MODE"],
        ["RMS", "RMS"],
    ];
    let overviews: string = "IGNORE_EXISTING";
    const overviewsOptions = [
        ["Auto", "AUTO"],
        ["Ignore Existing", "IGNORE_EXISTING"],
        ["Force Use Existing", "FORCE_USE_EXISTING"],
        ["None", "NONE"],
    ];
    let bigTiff: string = "IfSafer";
    const bigTiffOptions = [
        ["Yes", "YES"],
        ["No", "NO"],
        ["If Safer", "IfSafer"],
        ["If Needed", "IfNeeded"]
    ];

    $: {
        if (outputPath === null && inputPath) {
            // If inputPath is defined, and outputPath is not, then append "_cog.tif" to the input path.
            const inputDir = inputPath.split(sep).slice(0, -1).join(sep);
            const fname = inputPath.split(sep).pop().split(".").slice(0, -1);
            outputPath = `${inputDir}${sep}${fname}_cog.tif`
        }
    }

    let convertInProgress = false;
    let disableConvert: boolean;
    $: disableConvert = convertInProgress || !inputPath;

    async function selectInputHandler() {
        const result = await open({
            title: "Select input file",
            filters: [{name: "TIFF", extensions: ["tif"]}],
            defaultPath: await homeDir(),
            multiple: false,
        });
        invariant(!Array.isArray(result), "Multi-select not currently supported.");
        inputPath = result;
    }

    async function selectOutputHandler() {
        let defaultPath = await homeDir();
        if (outputPath) {
            defaultPath = outputPath.split("/").slice(0, -1).join("/");
        } else if (inputPath) {
            defaultPath = inputPath.split("/").slice(0, -1).join("/");
        }

        outputPath = await save({
            title: "Choose save location",
            filters: [{name: "TIFF", extensions: ["tif"]}],
            defaultPath,
        });
    }


    let permissionGranted: boolean = false;
    async function getPermission() {
        permissionGranted = await isPermissionGranted();
        if (!permissionGranted) {
            const permission = await requestPermission();
            permissionGranted = permission === 'granted';
        }
    }
    onMount(getPermission);

    async function convert() {
        convertInProgress = true;
        await invoke("convert", {
            inputPath,
            outputPath,
            noDataValue,
            compression,
            bigTiff,
        });
        if (permissionGranted) {
          sendNotification({ title: 'COGGERATOR', body: `Created COG at: ${outputPath}` });
        }
        convertInProgress = false;
    }
</script>

<form on:submit|preventDefault={convert} id="form-fields" class="h-full p-3 text-sans">
    <div class="h-full flex flex-col ">
        <div class="grow">
            <div class="mb-4">
                <FilePicker id="input-path" label="Input path" bind:value={inputPath}
                            on:click={selectInputHandler}/>
            </div>

            <div class="mb-4">
                <FilePicker id="output-path" label="Output path" bind:value={outputPath}
                            on:click={selectOutputHandler}/>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2"
                           for="background-color">
                        NoData Value
                    </label>
                    <input class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                           id="background-color" type="number" bind:value={noDataValue} min="0"/>
                </div>

                <div class="mb-4">
                    <Select label="Compression" id="compression" bind:value={compression}
                            options={compressionOptions}/>
                </div>

                <div class="mb-4">
                    <Select label="Resampling" id="resampling" bind:value={resampling}
                            options={resamplingOptions}/>
                </div>

                <div class="mb-4">
                    <Select label="Overviews" id="overviews" bind:value={overviews}
                            options={overviewsOptions}/>
                </div>

                <div class="mb-6">
                    <Select label="Big Tiff" id="big-tiff" bind:value={bigTiff}
                            options={bigTiffOptions}/>
                </div>
            </div>
        </div>

        <div id="form-buttons" class="flex justify-center">
            <button type="submit" disabled={disableConvert}
                    class="disabled:cursor-not-allowed bg-pink-500 hover:bg-pink-400 text-white font-bold py-3 px-8 border-b-4 border-pink-700 hover:border-pink-500 rounded-lg disabled:border-pink-700 disabled:bg-pink-500 disabled:opacity-50">
                {#if convertInProgress}
                    <div class="flex">
                        <svg class="animate-spin -ml-1 mr-3 h-5 w-5"
                             xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor"
                                    stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor"
                                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        <span>Converting...</span>
                    </div>

                {:else}
                    Convert
                {/if}
            </button>
        </div>
    </div>
</form>
