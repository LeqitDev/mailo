// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	namespace Data {
		interface Account {
			id: string;
			username: string;
			email: string;
			password: string;
			imap_host: string;
			imap_port: number;
			display_name: string;
		}

		interface Email {
			id: string;
			email_id: string;
			account_id: string;
			subject: string;
			body: string;
			date: string;
			sender: string;
			flags: EmailFlags;
		}

		interface EmailFlags {
			seen: boolean;
			answered: boolean;
			flagged: boolean;
			deleted: boolean;
			draft: boolean;
			recent: boolean;
			may_create: boolean;
			custom: string[];
		}

		interface EventPayload {
			event: string;
			payload: LogPayload | ActionPayload;
		}

		interface LogPayload {
			message: string;
			log_type: string;
		}

		interface ActionPayload {
			action: string;
			payload: string;
		}
	}
}

export {};
