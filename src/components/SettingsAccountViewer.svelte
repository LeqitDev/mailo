<script lang="ts">
	import { Button } from "@/components/ui/button";
	import AddAccountDialog from "§/components/AddAccountDialog.svelte";
    import EditAccountDialog from "§/components/EditAccountDialog.svelte";
    import { PlusIcon, UserCogIcon } from "lucide-svelte";
    import * as Avatar from "@/components/ui/avatar";
    import { accounts } from "@/store";

    export let data: any;


    function getEmailInitials(email: string) {
		return email
			.split('@')[0]
			.split('.')
			.map((part) => part[0])
			.join('');
	}

    function hashStr(str: string) {
		var hash = 0,
			i,
			chr;
		if (str.length === 0) return hash;
		for (i = 0; i < str.length; i++) {
			chr = str.charCodeAt(i);
			hash = (hash << 5) - hash + chr;
			hash |= 0; // Convert to 32bit integer
		}
		return hash;
	}


</script>

<div>
    <AddAccountDialog data={data.add_account_form}>
        <Button variant="secondary"><PlusIcon class="mr-2" /> Add Account</Button>
    </AddAccountDialog>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2 mt-4">
        {#each $accounts as account}
            <div class="flex items-center gap-2">
                <Avatar.Root>
                    <Avatar.Image
                        src="https://api.dicebear.com/7.x/shapes/svg?seed={hashStr(account.email)}"
                        alt="Profile picture"
                    />
                    <Avatar.Fallback>{getEmailInitials(account.email)}</Avatar.Fallback>
                </Avatar.Root>
                <div class="w-full">
                    <div class="flex justify-between w-full">
                        <div>
                            <p class="font-semibold">{account.display_name}</p>
                            <p class="text-muted-foreground overflow-ellipsis">{account.email}</p>
                        </div>
                        <EditAccountDialog form_data={data.account_settings_form} {account} >
                            <Button variant="ghost" size="icon"><UserCogIcon /></Button>
                        </EditAccountDialog>
                    </div>
                </div>
            </div>
        {/each}
    </div>
</div>