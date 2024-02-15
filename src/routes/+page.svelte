<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import {
        AlignJustifyIcon,
        SearchIcon,
        InboxIcon,
        Trash2Icon,
        SendIcon,
        FileIcon,
        MailIcon,
        CalendarIcon,
    } from "lucide-svelte";
    import { Input } from "$lib/components/ui/input";
    import EmailPreview from "../components/EmailPreview.svelte";
    import * as Select from "$lib/components/ui/select";
</script>

<main class="w-full h-full flex">
    <div class="h-full border-r p-1 pt-14 flex flex-col gap-2">
        <Button variant="default" size="icon" href="/"><MailIcon /></Button>
        <Button variant="outline" size="icon"><CalendarIcon /></Button>
    </div>
    <div class="w-full h-full flex flex-col">
        <div class="w-full flex justify-between pt-2 px-2 border-b pb-1">
            <Select.Root>
                <Select.Trigger class="max-w-56">
                    <Select.Value placeholder="Mail-Account" />
                </Select.Trigger>
                <Select.Content>
                    <Select.Item value="all">All</Select.Item>
                </Select.Content>
            </Select.Root>
            <form class="flex w-full max-w-md items-center space-x-2">
                <Input />
                <Button variant="ghost" size="icon"
                    ><SearchIcon class="h-4 w-4" /></Button
                >
            </form>
            <div>
                <Button variant="outline" size="sm">Settings</Button>
                <Button variant="outline" size="sm">Accounts</Button>
            </div>
        </div>

        <div class="h-full w-full grow-1 flex">
            <div class="h-full w-full max-w-48 lg:max-w-sm border-r p-4">
                <h2 class="text-xl font-semibold pb-4">Mail</h2>
                <div class="grid gap-1">
                    <Button
                        variant="ghost"
                        size="sm"
                        class="justify-start gap-2"
                        ><InboxIcon class="w-4 h-4" />Inbox</Button
                    >
                    <Button
                        variant="ghost"
                        size="sm"
                        class="justify-start gap-2"
                        ><SendIcon class="w-4 h-4" />Sent</Button
                    >
                    <Button
                        variant="ghost"
                        size="sm"
                        class="justify-start gap-2"
                        ><FileIcon class="w-4 h-4" />Drafts</Button
                    >
                    <Button
                        variant="ghost"
                        size="sm"
                        class="justify-start gap-2"
                        ><Trash2Icon class="w-4 h-4" />Trash</Button
                    >
                </div>
            </div>
            <div class="h-full w-full overflow-y-auto">
                <EmailPreview time="2 years ago" />
                <EmailPreview />
                <EmailPreview />
            </div>
        </div>
    </div>
</main>
