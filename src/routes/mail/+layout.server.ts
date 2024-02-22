import { mailAccountSchema } from "$lib/custom/components/schema";
import { superValidate } from "sveltekit-superforms/server";

export const load = async () => {
    const form = await superValidate(mailAccountSchema);
    return {
        form
    }
}