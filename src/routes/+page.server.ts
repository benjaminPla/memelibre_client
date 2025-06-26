import { env } from '$env/dynamic/public';
import { error } from '@sveltejs/kit';
import type { MemeWithUsernameAndCommentsCount } from '$lib/types';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const apiUrl = env.PUBLIC_API_URL;

	try {
		const res = await fetch(`${apiUrl}/meme/get`);

		if (!res.ok) {
			const errorMsg = await res.text();
			error(res.status, errorMsg);
		}

		const memes: MemeWithUsernameAndCommentsCount[] = await res.json();

		return { memes };
	} catch (e: any) {
		if (e?.status) throw e;

		console.error(`src/routes/+page.server.ts - Error 500 - ${e}`);
		error(500);
	}
};
