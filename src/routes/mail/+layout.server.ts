import { mailAccountSchema } from "./schema";
import { superValidate } from "sveltekit-superforms/server";

export const load = async () => {
    const form = await superValidate(mailAccountSchema);
    return {
        form
    }
}