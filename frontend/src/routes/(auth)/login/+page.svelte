<script lang="ts">
	import { goto } from '$app/navigation';

	import { authClient } from '$lib/auth-client';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	type Mode = 'signin' | 'signup';

	let mode = $state<Mode>('signin');
	let name = $state('');
	let email = $state('');
	let password = $state('');
	let isSubmitting = $state(false);
	let isSocialLoading = $state(false);
	let formError = $state('');

	function normalizeError(error: unknown): string {
		if (error instanceof Error) return error.message;
		if (typeof error === 'string') return error;
		return 'Đã có lỗi xảy ra. Vui lòng thử lại.';
	}

	async function handleSubmit(event: SubmitEvent): Promise<void> {
		event.preventDefault();
		formError = '';

		if (!email.trim() || !password.trim()) {
			formError = 'Vui lòng nhập đầy đủ email và mật khẩu.';
			return;
		}

		if (mode === 'signup' && !name.trim()) {
			formError = 'Vui lòng nhập tên hiển thị.';
			return;
		}

		isSubmitting = true;
		try {
			const response =
				mode === 'signin'
					? await authClient.signIn.email({
							email: email.trim(),
							password,
							callbackURL: data.redirectTo
						})
					: await authClient.signUp.email({
							name: name.trim(),
							email: email.trim(),
							password,
							callbackURL: data.redirectTo
						});

			if (response?.error) {
				formError = response.error.message ?? 'Đăng nhập thất bại.';
				return;
			}

			await goto(data.redirectTo, { invalidateAll: true });
		} catch (error) {
			formError = normalizeError(error);
		} finally {
			isSubmitting = false;
		}
	}

	async function handleGoogleSignIn(): Promise<void> {
		formError = '';
		isSocialLoading = true;
		try {
			await authClient.signIn.social({
				provider: 'google',
				callbackURL: data.redirectTo
			});
		} catch (error) {
			formError = normalizeError(error);
			isSocialLoading = false;
		}
	}
</script>

<svelte:head>
	<title>Đăng nhập - FetchTube</title>
</svelte:head>

<section class="mx-auto max-w-lg px-4 py-10">
	<div class="rounded-3xl border border-pink-200 bg-white p-6 shadow-card">
		<h1 class="text-3xl font-bold text-plum">Chào mừng quay lại</h1>
		<p class="mt-2 text-sm text-plum/70">
			Dùng tài khoản để đồng bộ subscription và mở khóa trải nghiệm Premium.
		</p>

		<button
			type="button"
			class="mt-6 flex w-full items-center justify-center rounded-full border border-plum/15 bg-white px-5 py-3 text-sm font-bold text-plum transition hover:border-primary/40 hover:bg-pink-50 disabled:cursor-not-allowed disabled:opacity-60"
			onclick={handleGoogleSignIn}
			disabled={isSocialLoading || isSubmitting}
		>
			{isSocialLoading ? 'Đang chuyển hướng Google...' : 'Tiếp tục với Google'}
		</button>

		<div class="my-5 flex items-center gap-3 text-xs text-plum/45">
			<div class="h-px flex-1 bg-plum/15"></div>
			<span>hoặc</span>
			<div class="h-px flex-1 bg-plum/15"></div>
		</div>

		<form class="space-y-4" onsubmit={handleSubmit}>
			{#if mode === 'signup'}
				<div class="space-y-1.5">
					<label for="name" class="text-xs font-bold uppercase tracking-wide text-plum/70">Tên hiển thị</label>
					<input
						id="name"
						type="text"
						autocomplete="name"
						bind:value={name}
						class="w-full rounded-2xl border border-pink-200 bg-pink-50 px-4 py-3 text-sm text-plum placeholder:text-plum/40 focus:border-primary focus:outline-none focus:ring-2 focus:ring-primary/20"
						placeholder="Khoa Bui"
						disabled={isSubmitting}
					/>
				</div>
			{/if}

			<div class="space-y-1.5">
				<label for="email" class="text-xs font-bold uppercase tracking-wide text-plum/70">Email</label>
				<input
					id="email"
					type="email"
					autocomplete="email"
					bind:value={email}
					class="w-full rounded-2xl border border-pink-200 bg-pink-50 px-4 py-3 text-sm text-plum placeholder:text-plum/40 focus:border-primary focus:outline-none focus:ring-2 focus:ring-primary/20"
					placeholder="you@example.com"
					disabled={isSubmitting}
				/>
			</div>

			<div class="space-y-1.5">
				<label for="password" class="text-xs font-bold uppercase tracking-wide text-plum/70">Mật khẩu</label>
				<input
					id="password"
					type="password"
					autocomplete={mode === 'signin' ? 'current-password' : 'new-password'}
					bind:value={password}
					class="w-full rounded-2xl border border-pink-200 bg-pink-50 px-4 py-3 text-sm text-plum placeholder:text-plum/40 focus:border-primary focus:outline-none focus:ring-2 focus:ring-primary/20"
					placeholder="••••••••"
					disabled={isSubmitting}
				/>
			</div>

			{#if formError}
				<p class="rounded-2xl border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700">{formError}</p>
			{/if}

			<button
				type="submit"
				class="w-full rounded-full bg-plum px-5 py-3 text-sm font-bold uppercase tracking-wide text-white transition hover:bg-plum/90 disabled:cursor-not-allowed disabled:opacity-60"
				disabled={isSubmitting || isSocialLoading}
			>
				{#if isSubmitting}
					{mode === 'signin' ? 'Đang đăng nhập...' : 'Đang tạo tài khoản...'}
				{:else}
					{mode === 'signin' ? 'Đăng nhập' : 'Tạo tài khoản'}
				{/if}
			</button>
		</form>

		<div class="mt-5 text-center text-sm text-plum/75">
			{#if mode === 'signin'}
				Chưa có tài khoản?
				<button
					type="button"
					class="font-bold text-primary hover:underline"
					onclick={() => {
						mode = 'signup';
						formError = '';
					}}
				>
					Tạo ngay
				</button>
			{:else}
				Đã có tài khoản?
				<button
					type="button"
					class="font-bold text-primary hover:underline"
					onclick={() => {
						mode = 'signin';
						formError = '';
					}}
				>
					Đăng nhập
				</button>
			{/if}
		</div>
	</div>
</section>
