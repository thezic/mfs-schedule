import type { PageLoad } from './$types';
import { browser } from '$app/environment';

export const load: PageLoad = async ({ params }) => {
	if (!browser) return;
	const { invoke } = await import('@tauri-apps/api');

	const data = await invoke('get_person_by_id', { id: parseInt(params.id) });

	return data;
};
