<script lang="ts">
	import AppIcon from '$components/AppIcon.svelte';
	import { authClient } from '$lib/auth-client';
	import * as m from '$lib/paraglide/messages';

	type Mode = 'signin' | 'signup';

	type AuthModalProps = {
		open?: boolean;
		redirectTo?: string;
		onClose?: () => void;
		onSuccess?: (redirectTo: string) => void;
	};

	let {
		open = false,
		redirectTo = '/',
		onClose = () => {},
		onSuccess = () => {}
	}: AuthModalProps = $props();

	let mode = $state<Mode>('signin');
	let name = $state('');
	let email = $state('');
	let password = $state('');
	let isSubmitting = $state(false);
	let isSocialLoading = $state(false);
	let formError = $state('');

	function normalizeRedirectTarget(target?: string): string {
		if (!target || !target.startsWith('/') || target.startsWith('//')) return '/';
		return target;
	}

	const safeRedirectTo = $derived(normalizeRedirectTarget(redirectTo));

	function normalizeError(error: unknown): string {
		if (error instanceof Error) return error.message;
		if (typeof error === 'string') return error;
		return m.auth_modal_error_generic();
	}

	function resetFormState(): void {
		formError = '';
		isSubmitting = false;
		isSocialLoading = false;
	}

	function handleClose(): void {
		resetFormState();
		onClose();
	}

	async function handleSubmit(event: SubmitEvent): Promise<void> {
		event.preventDefault();
		formError = '';

		if (!email.trim() || !password.trim()) {
			formError = m.auth_modal_error_missing_email_password();
			return;
		}

		if (mode === 'signup' && !name.trim()) {
			formError = m.auth_modal_error_missing_name();
			return;
		}

		isSubmitting = true;
		try {
			const response =
				mode === 'signin'
					? await authClient.signIn.email({
							email: email.trim(),
							password,
							callbackURL: safeRedirectTo
						})
					: await authClient.signUp.email({
							name: name.trim(),
							email: email.trim(),
							password,
							callbackURL: safeRedirectTo
						});

			if (response?.error) {
				formError = response.error.message ?? m.auth_modal_error_sign_in_failed();
				return;
			}

			onSuccess(safeRedirectTo);
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
				callbackURL: safeRedirectTo
			});
		} catch (error) {
			formError = normalizeError(error);
			isSocialLoading = false;
		}
	}
</script>

{#if open}
	<div class="fixed inset-0 z-[120] flex items-center justify-center px-4">
		<button
			type="button"
			class="absolute inset-0 bg-black/45 backdrop-blur-[1px]"
			aria-label={m.auth_modal_close_login_aria()}
			onclick={handleClose}
		></button>

		<div class="relative w-full max-w-lg rounded-3xl border border-pink-200 bg-white p-6 shadow-2xl">
			<button
				type="button"
				class="absolute right-4 top-4 rounded-full p-1 text-plum/60 transition hover:bg-pink-50 hover:text-plum"
				aria-label={m.auth_modal_close_aria()}
				onclick={handleClose}
			>
				<AppIcon name="close" />
			</button>

			<h2 class="text-3xl font-bold text-plum">
				{mode === 'signin' ? m.auth_modal_title_signin() : m.auth_modal_title_signup()}
			</h2>

			<button
				type="button"
				class="mt-4 flex w-full items-center justify-center rounded-full border border-plum/15 bg-white px-5 py-3 text-sm font-bold text-plum transition hover:border-primary/40 hover:bg-pink-50 disabled:cursor-not-allowed disabled:opacity-60"
				onclick={handleGoogleSignIn}
				disabled={isSocialLoading || isSubmitting}
			>
				{isSocialLoading ? m.auth_modal_google_redirecting() : m.auth_modal_continue_google()}
			</button>

			<div class="my-5 flex items-center gap-3 text-xs text-plum/45">
				<div class="h-px flex-1 bg-plum/15"></div>
				<span>{m.auth_modal_or()}</span>
				<div class="h-px flex-1 bg-plum/15"></div>
			</div>

			<form class="space-y-4" onsubmit={handleSubmit}>
				{#if mode === 'signup'}
					<div class="space-y-1.5">
						<label for="auth-modal-name" class="text-xs font-bold uppercase tracking-wide text-plum/70">{m.auth_modal_label_display_name()}</label>
						<input
							id="auth-modal-name"
							type="text"
							autocomplete="name"
							bind:value={name}
							class="w-full rounded-2xl border border-pink-200 bg-pink-50 px-4 py-3 text-sm text-plum placeholder:text-plum/40 focus:border-primary focus:outline-none focus:ring-2 focus:ring-primary/20"
							placeholder={m.auth_modal_placeholder_display_name()}
							disabled={isSubmitting}
						/>
					</div>
				{/if}

				<div class="space-y-1.5">
					<label for="auth-modal-email" class="text-xs font-bold uppercase tracking-wide text-plum/70">{m.auth_modal_label_email()}</label>
					<input
						id="auth-modal-email"
						type="email"
						autocomplete="email"
						bind:value={email}
						class="w-full rounded-2xl border border-pink-200 bg-pink-50 px-4 py-3 text-sm text-plum placeholder:text-plum/40 focus:border-primary focus:outline-none focus:ring-2 focus:ring-primary/20"
						placeholder={m.auth_modal_placeholder_email()}
						disabled={isSubmitting}
					/>
				</div>

				<div class="space-y-1.5">
					<label for="auth-modal-password" class="text-xs font-bold uppercase tracking-wide text-plum/70">{m.auth_modal_label_password()}</label>
					<input
						id="auth-modal-password"
						type="password"
						autocomplete={mode === 'signin' ? 'current-password' : 'new-password'}
						bind:value={password}
						class="w-full rounded-2xl border border-pink-200 bg-pink-50 px-4 py-3 text-sm text-plum placeholder:text-plum/40 focus:border-primary focus:outline-none focus:ring-2 focus:ring-primary/20"
						placeholder={m.auth_modal_placeholder_password()}
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
						{mode === 'signin'
							? m.auth_modal_submit_signin_loading()
							: m.auth_modal_submit_signup_loading()}
					{:else}
						{mode === 'signin' ? m.auth_modal_submit_signin() : m.auth_modal_submit_signup()}
					{/if}
				</button>
			</form>

			<div class="mt-5 text-center text-sm text-plum/75">
				{#if mode === 'signin'}
					{m.auth_modal_switch_signup_prompt()}
					<button
						type="button"
						class="font-bold text-primary hover:underline"
						onclick={() => {
							mode = 'signup';
							formError = '';
						}}
					>
						{m.auth_modal_switch_signup_cta()}
					</button>
				{:else}
					{m.auth_modal_switch_signin_prompt()}
					<button
						type="button"
						class="font-bold text-primary hover:underline"
						onclick={() => {
							mode = 'signin';
							formError = '';
						}}
					>
						{m.auth_modal_switch_signin_cta()}
					</button>
				{/if}
			</div>
		</div>
	</div>
{/if}
