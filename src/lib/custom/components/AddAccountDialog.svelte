<script lang="ts">
    import * as Dialog from '$lib/components/ui/dialog';
	import { Button, buttonVariants } from '@/components/ui/button';
	import MailAccountForm from './MailAccountForm.svelte';

    let open: boolean;
    export let data: any;
</script>

<Dialog.Root bind:open={open}>
    <Dialog.Trigger
        ><slot /></Dialog.Trigger
    >
    <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
            <Dialog.Title>Add Account</Dialog.Title>
            <Dialog.Description>
                Add a new account with imap to your mail client.
            </Dialog.Description>
        </Dialog.Header>
        <MailAccountForm form_data={data} onSuccessfulSubmit={() => {open = false}} />
        <Dialog.Footer>
            <Button type="submit" form="mail-account-form">Save account</Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>