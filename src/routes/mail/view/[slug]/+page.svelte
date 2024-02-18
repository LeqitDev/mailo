<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import type { PageData } from "./$types";
    
    export let data: PageData;

    let email: Data.Email;
    
    invoke('get_email', { id: parseInt(data.slug) }).then((result) => {
        email = result as Data.Email;
    }).catch((error) => {
        console.log('error', error);
    });
</script>

{#if email}
    <div class="h-full flex flex-col">
        <div class="w-full border-b flex justify-between items-center px-2 py-1 gap-5 h-12">
            <h2 class="text-lg font-semibold capitalize">{email.subject}</h2>
            <div class="flex gap-2 items-center">
                <p>{email.date}</p>
                <p>{email.sender}</p>
            </div>
        </div>
        <div class="flex-1 overflow-y-auto">
            <div class="p-2" style="white-space: pre-wrap;">
                {@html email.body}
            </div>
        </div>
    </div>
{/if}