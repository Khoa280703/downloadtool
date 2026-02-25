// Web Share Target API handler (GET method — no CSRF issues)
// Android "Share" from YouTube app → browser sends GET /share-target?url=<youtube-url>
// We redirect immediately to the main page with the url pre-filled.

import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = ({ url }) => {
  const videoUrl = url.searchParams.get('url') ?? '';

  if (videoUrl) {
    redirect(302, `/?url=${encodeURIComponent(videoUrl)}`);
  }

  redirect(302, '/');
};
