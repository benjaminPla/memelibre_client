import { env } from '$env/dynamic/public';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies }) => {
	const sessionToken: string | undefined = cookies.get('session_token');

	if (!sessionToken) {
		const apiUrl = env.PUBLIC_API_URL;
		redirect(302, `${apiUrl}/auth/login`);
	}
};
