import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals }) => {
	return {
		authUser: locals.user
			? {
					name: locals.user.name ?? null,
					email: locals.user.email,
					image: locals.user.image ?? null
				}
			: null
	};
};
