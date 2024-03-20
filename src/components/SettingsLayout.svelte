<script lang="ts">
	import { Button } from "@/components/ui/button";
    import CustomLayout from "§/components/RootLayout.svelte";
	import SideNavButton from "./SideNavButton.svelte";
	import type { ButtonTypes } from "@/utils";
	import { afterUpdate } from "svelte";

    export let subsite: string;

    let active_variant: ButtonTypes = "link";
    let default_variant: ButtonTypes = "link";
    let active_classes = "underline";
</script>

<CustomLayout site="settings">
    <div class="flex gap-4 w-full">
        <div class="flex flex-col border-r max-w-48 w-full my-4">
                <SideNavButton bind:site={subsite} {active_variant} {default_variant} {active_classes} name="General" site_name="general" href="/settings#general" />
                <SideNavButton bind:site={subsite} {active_variant} {default_variant} {active_classes} name="Accounts" site_name="accounts" href="/settings#accounts" />
                <SideNavButton bind:site={subsite} {active_variant} {default_variant} {active_classes} name="Appearance" site_name="appearance" href="/settings#appearance" />
                <SideNavButton bind:site={subsite} {active_variant} {default_variant} {active_classes} name="Security" site_name="security" href="/settings#security" />
        </div>
        <div class="flex-1">
            <slot />
        </div>
    </div>
</CustomLayout>