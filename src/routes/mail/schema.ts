import { z } from 'zod';

export const mailAccountSchema = z.object({
    email: z.string().email().min(1),
    username: z.string().min(1),
    password: z.string().min(1),
    imap_host: z.string().min(1),
    imap_port: z.string().min(1),
    });

export type MailAccount = typeof mailAccountSchema;