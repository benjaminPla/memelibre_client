import { env } from '$env/dynamic/public';
import { error } from '@sveltejs/kit';
import type { Meme } from '$lib/types/index';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const apiUrl: string = env.PUBLIC_API_URL;

	try {
		const res = await fetch(`${apiUrl}/meme/get/${params.id}`);

		if (!res.ok) {
			const errorMsg = await res.text();
			error(res.status, errorMsg);
		}

		const meme: Meme = await res.json();

		return { meme };
	} catch (e: any) {
		if (e?.status) throw e;

		console.error(`src/routes/meme/[id]/+page.server.ts - Error/Params - ${e}/${params}`);
		error(500);
	}
};
