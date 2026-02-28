type SaveDirectoryHandle = {
	getFileHandle: (
		name: string,
		options?: { create?: boolean }
	) => Promise<{
		createWritable: () => Promise<FileWritableHandle>;
	}>;
};

type FileWritableHandle = WritableStream & {
	close?: () => Promise<void>;
	abort?: (reason?: unknown) => Promise<void>;
};

export interface SaveDownloadOptions {
	requireFsaa?: boolean;
	allowAnchorFallback?: boolean;
}

let saveDirectoryHandle: SaveDirectoryHandle | null = null;

export function hasSelectedSaveDirectory(): boolean {
	return saveDirectoryHandle !== null;
}

export async function pickSaveDirectory(): Promise<boolean> {
	if (typeof window === 'undefined') return false;

	const picker = (window as Window & { showDirectoryPicker?: () => Promise<SaveDirectoryHandle> })
		.showDirectoryPicker;
	if (!picker) return false;

	try {
		saveDirectoryHandle = await picker();
		return true;
	} catch {
		return false;
	}
}

export function isAbortError(error: unknown): boolean {
	return error instanceof DOMException && error.name === 'AbortError';
}

export async function saveDownload(
	url: string,
	filename: string,
	signal: AbortSignal,
	options: SaveDownloadOptions = {}
): Promise<void> {
	const requireFsaa = options.requireFsaa === true;
	const allowAnchorFallback = options.allowAnchorFallback !== false;

	if (saveDirectoryHandle) {
		try {
			await saveWithDirectory(url, filename, saveDirectoryHandle, signal);
			return;
		} catch (error) {
			if (isAbortError(error)) throw error;
			if (!allowAnchorFallback) throw error;
			downloadViaAnchor(url, filename);
			return;
		}
	}

	if (requireFsaa) {
		throw new Error('No save directory selected');
	}

	if (!allowAnchorFallback) {
		throw new Error('Anchor fallback is disabled');
	}

	downloadViaAnchor(url, filename);
}

async function saveWithDirectory(
	url: string,
	filename: string,
	dirHandle: SaveDirectoryHandle,
	signal: AbortSignal
): Promise<void> {
	const response = await fetch(url, { signal });
	if (!response.ok) throw new Error(`Download failed with status ${response.status}`);

	const fileHandle = await dirHandle.getFileHandle(filename, { create: true });
	const writable = await fileHandle.createWritable();

	if (!response.body) {
		if (typeof writable.close === 'function') {
			await writable.close();
		}
		throw new Error('Readable stream is not available');
	}

	try {
		await response.body.pipeTo(writable as WritableStream, { signal });
	} catch (error) {
		if (typeof writable.abort === 'function') {
			try {
				await writable.abort(error);
			} catch {
				// Ignore secondary abort errors.
			}
		}
		throw error;
	}
}

function downloadViaAnchor(url: string, filename: string): void {
	const anchor = document.createElement('a');
	anchor.href = url;
	anchor.download = filename;
	anchor.style.display = 'none';
	document.body.appendChild(anchor);
	anchor.click();
	document.body.removeChild(anchor);
}
