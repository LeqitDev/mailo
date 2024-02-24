<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import type { PageData } from "./$types";
	import { Button } from "@/components/ui/button";
	import { ArrowLeftIcon } from "lucide-svelte";
	import { search_string } from "@/stores/settings";
    
    export let data: PageData;

    let email: Data.Email;
    let date: Date;
    let body: string;
    
    invoke('get_email', { id: parseInt(data.slug) }).then((result) => {
        email = result as Data.Email;
        const [datePart, timePart] = email.date.split(' ');
        const [year, month, day] = datePart.split('-').map(Number);
        const [hours, minutes, seconds] = timePart.split(':').map(Number);
        date = new Date(year, month - 1, day, hours, minutes, seconds);
        body = email.body;
        if ($search_string) {
            body = email.body.replace(new RegExp($search_string, 'gi'), (match) => {
                return `<mark>${match}</mark>`;
            });
        }
    }).catch((error) => {
        console.log('error', error);
    });

    search_string.subscribe((value) => {
        if (value && email) {
            body = email.body.replace(new RegExp(value, 'gi'), (match) => {
                return `<mark>${match}</mark>`;
            });
            console.log(email.body);
        } else if (email) {
            body = email.body;
        }
    });
</script>

{#if email}
    <div class="h-full flex flex-col">
        <div class="w-full border-b flex justify-between items-center px-2 py-1 gap-5 h-12">
            <div class="flex items-center gap-2">
                <Button href="/mail/inbox" variant="ghost" size="icon"><ArrowLeftIcon /></Button>
                <div>
                    <h2 class="text font-semibold capitalize">{email.subject}</h2>
                    <p class="text-xs">Von: {email.sender}</p>
                </div>
            </div>
            <div class="flex gap-2 items-center text-sm">
                <p>{date.toLocaleString()}</p>
            </div>
        </div>
        <div class="flex-1 overflow-y-auto">
            <div class="p-2">
                {@html body}
            </div>
        </div>
    </div>
{/if}