<script lang="ts">
	import { events } from "@/stores/settings";

    function getTimeString(time: string | undefined) {
        if (!time) return '';
        return new Date(parseInt(time)).toLocaleString();
    }

    function getLogPayload(event_payload: any) {
        return event_payload as Data.LogPayload;
    }

    console.log('events', $events);
</script>

<div class="w-full h-full overflow-auto p-2">
    {#if $events}
        {#each $events as event}
            <div class="flex gap-3">
                <p class="min-w-max">{getTimeString(event.time)}</p>
                {#if event.event === "log"}
                    <p>{getLogPayload(event.payload).log_type}</p>
                    <p>{getLogPayload(event.payload).message}</p>
                {/if}
            </div>
        {/each}
    {/if}
</div>