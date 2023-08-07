import type { PageLoad } from './$types';
import { browser } from '$app/environment';
import { getPersonById } from 'bindings';

export const load: PageLoad = async ({ params }) => {
	if (!browser) return;
	// const { invoke } = await import('@tauri-apps/api');

	const data = await getPersonById(parseInt(params.id));

	return data;
};
