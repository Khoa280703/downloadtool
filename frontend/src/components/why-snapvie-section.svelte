<script lang="ts">
	import AppIcon from '$components/AppIcon.svelte';
	import mikeAvatar from '$lib/assets/testimonials/mike.webp';
	import sarahAvatar from '$lib/assets/testimonials/sarah.webp';
	import user1Avatar from '$lib/assets/testimonials/user-1.webp';
	import user2Avatar from '$lib/assets/testimonials/user-2.webp';
	import user3Avatar from '$lib/assets/testimonials/user-3.webp';
	import * as m from '$lib/paraglide/messages';

	type WhySnapvieCard = {
		icon: string;
		kicker?: string;
		title: string;
		description: string;
		accentClass?: string;
	};

	const featureRotations = ['-5deg', '-2deg', '3deg', '5deg'] as const;
	let {
		title = m.home_why_title(),
		eyebrow = m.why_snapvie_eyebrow(),
		cards,
		ctaHref = '#home'
	}: {
		title?: string;
		eyebrow?: string;
		cards: WhySnapvieCard[];
		ctaHref?: string;
	} = $props();
</script>

<section class="why-snapvie-section">
	<div class="why-snapvie-shell">
		<div class="why-snapvie-stage">
			{#if eyebrow}
				<div class="why-snapvie-topline">
					<span class="why-snapvie-eyebrow">
						<AppIcon name="auto_awesome" class="text-[14px]" />
						{eyebrow}
					</span>
				</div>
			{/if}

			<h2 class="why-snapvie-title">{title}</h2>

			<div class="why-snapvie-features">
				{#each cards as card, index}
					<article
						class={`why-snapvie-feature ${card.accentClass ?? ''}`}
						style={`--feature-tilt:${featureRotations[index % featureRotations.length]}`}
					>
						<div class="why-snapvie-feature-head">
							<div class="why-snapvie-feature-icon">
								<AppIcon name={card.icon} />
							</div>
							{#if card.kicker}
								<span class="why-snapvie-feature-kicker">{card.kicker}</span>
							{/if}
						</div>
						<h3>{card.title}</h3>
						<p>{card.description}</p>
					</article>
				{/each}
			</div>

			<div class="why-snapvie-proof">
				<div class="why-snapvie-proof-intro">
					<span class="why-snapvie-trust-badge">{m.home_testimonials_badge()}</span>
					<h3>{m.home_testimonials_title()}</h3>
					<p>{m.home_testimonials_subtitle()}</p>

					<div class="why-snapvie-avatar-stack" aria-label={m.home_testimonials_badge()}>
						<div class="why-snapvie-avatar" title={m.home_testimonial_user_avatar({ index: '1' })}>
							<img alt={m.home_testimonial_user_avatar({ index: '1' })} src={user1Avatar} loading="lazy" decoding="async" width="40" height="40" />
						</div>
						<div class="why-snapvie-avatar" title={m.home_testimonial_user_avatar({ index: '2' })}>
							<img alt={m.home_testimonial_user_avatar({ index: '2' })} src={user2Avatar} loading="lazy" decoding="async" width="40" height="40" />
						</div>
						<div class="why-snapvie-avatar" title={m.home_testimonial_user_avatar({ index: '3' })}>
							<img alt={m.home_testimonial_user_avatar({ index: '3' })} src={user3Avatar} loading="lazy" decoding="async" width="40" height="40" />
						</div>
						<div class="why-snapvie-avatar why-snapvie-avatar-count">+9k</div>
					</div>
				</div>

				<article class="why-snapvie-quote-card quote-sarah">
					<div class="why-snapvie-quote-head">
						<div class="why-snapvie-quote-avatar">
							<img alt={m.home_testimonial_sarah_name()} src={sarahAvatar} loading="lazy" decoding="async" width="36" height="36" />
						</div>
						<div>
							<h4>{m.home_testimonial_sarah_name()}</h4>
							<div class="why-snapvie-quote-stars" aria-label="5 stars">★★★★★</div>
						</div>
					</div>
					<p class="why-snapvie-quote-copy">{m.home_testimonial_sarah_quote()}</p>
				</article>

				<article class="why-snapvie-quote-card quote-mike">
					<div class="why-snapvie-quote-head">
						<div class="why-snapvie-quote-avatar">
							<img alt={m.home_testimonial_mike_name()} src={mikeAvatar} loading="lazy" decoding="async" width="36" height="36" />
						</div>
						<div>
							<h4>{m.home_testimonial_mike_name()}</h4>
							<div class="why-snapvie-quote-stars" aria-label="5 stars">★★★★★</div>
						</div>
					</div>
					<p class="why-snapvie-quote-copy">{m.home_testimonial_mike_quote()}</p>
				</article>

				<div class="why-snapvie-cta-card">
					<div>
						<p class="why-snapvie-cta-title">{m.home_join_party_title()}</p>
						<p class="why-snapvie-cta-copy">{m.home_join_party_subtitle()}</p>
					</div>
					<a href={ctaHref} class="why-snapvie-cta-button">
						<span>{m.home_join_party_cta()}</span>
						<AppIcon name="arrow_forward" class="text-[16px]" />
					</a>
					<div class="why-snapvie-cta-badge" aria-hidden="true">🎉</div>
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.why-snapvie-section {
		position: relative;
		overflow: hidden;
		padding: 4rem 1.5rem 4.5rem;
	}

	.why-snapvie-section::before,
	.why-snapvie-section::after {
		content: '';
		position: absolute;
		border-radius: 999px;
		filter: blur(72px);
		pointer-events: none;
		opacity: 0.6;
	}

	.why-snapvie-section::before {
		top: 4rem;
		left: -5rem;
		width: 18rem;
		height: 18rem;
		background: rgba(255, 196, 216, 0.22);
	}

	.why-snapvie-section::after {
		right: -5rem;
		bottom: 3rem;
		width: 18rem;
		height: 18rem;
		background: rgba(255, 216, 168, 0.18);
	}

	.why-snapvie-shell {
		position: relative;
		z-index: 1;
		max-width: 80rem;
		margin: 0 auto;
	}

	.why-snapvie-stage {
		position: relative;
		overflow: hidden;
		padding: 1.35rem 1.45rem 1.5rem;
		border-radius: 0;
		border: none;
		background: transparent;
		box-shadow: none;
	}

	.why-snapvie-stage::before,
	.why-snapvie-stage::after {
		content: none;
	}

	.why-snapvie-topline {
		display: flex;
		justify-content: center;
		margin-bottom: 0.85rem;
	}

	.why-snapvie-eyebrow {
		display: inline-flex;
		align-items: center;
		gap: 0.45rem;
		padding: 0.28rem 0.65rem;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.72);
		color: #be185d;
		font-size: 0.56rem;
		font-weight: 800;
		letter-spacing: 0.18em;
		text-transform: uppercase;
		box-shadow: 0 14px 30px -24px rgba(45, 27, 54, 0.25);
	}

	.why-snapvie-title {
		font-family: 'Fredoka', sans-serif;
		font-size: clamp(1.85rem, 3.6vw, 2.4rem);
		line-height: 1;
		font-weight: 700;
		letter-spacing: -0.04em;
		text-align: center;
		color: #2d1b36;
		margin: 0 0 0.3rem;
	}

	.why-snapvie-features {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 0.9rem;
		margin-top: 1.15rem;
	}

	.why-snapvie-feature {
		position: relative;
		padding: 0.95rem 1rem 1rem;
		border-radius: 1.45rem;
		background: rgba(255, 255, 255, 0.86);
		border: 1px solid rgba(169, 146, 189, 0.2);
		box-shadow: 0 16px 34px -28px rgba(64, 31, 66, 0.24);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		transition:
			transform 220ms ease,
			box-shadow 220ms ease,
			border-color 220ms ease;
	}

	.why-snapvie-feature:hover {
		transform: translateY(-4px);
		box-shadow: 0 24px 40px -24px rgba(64, 31, 66, 0.28);
		border-color: rgba(255, 140, 185, 0.28);
	}

	.why-snapvie-feature-head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
		margin-bottom: 0.8rem;
	}

	.why-snapvie-feature-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 2.55rem;
		height: 2.55rem;
		border-radius: 0.95rem;
		font-size: 1.25rem;
		flex-shrink: 0;
	}

	.why-snapvie-feature-kicker {
		font-size: 0.68rem;
		font-weight: 800;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: rgba(45, 27, 54, 0.42);
	}

	.why-snapvie-feature h3 {
		font-family: 'Fredoka', sans-serif;
		font-size: 0.98rem;
		line-height: 1.18;
		font-weight: 700;
		color: #2d1b36;
		margin: 0 0 0.4rem;
	}

	.why-snapvie-feature p {
		margin: 0;
		font-size: 0.8rem;
		line-height: 1.48;
		font-weight: 700;
		color: rgba(45, 27, 54, 0.7);
	}

	.why-snapvie-proof {
		display: grid;
		gap: 1rem;
		margin-top: 3rem;
	}

	.why-snapvie-proof-intro,
	.why-snapvie-quote-card,
	.why-snapvie-cta-card {
		border-radius: 1.6rem;
		border: 1px solid rgba(255, 153, 194, 0.16);
		background: rgba(255, 255, 255, 0.82);
		box-shadow: 0 20px 40px -34px rgba(64, 31, 66, 0.24);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
	}

	.why-snapvie-proof-intro {
		padding: 1.15rem 1.2rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}

	.why-snapvie-trust-badge {
		display: inline-flex;
		align-items: center;
		align-self: flex-start;
		width: fit-content;
		max-width: 100%;
		padding: 0.35rem 0.9rem;
		border-radius: 999px;
		background: #dcfce7;
		color: #15803d;
		font-size: 0.74rem;
		font-weight: 800;
		letter-spacing: 0.14em;
		text-transform: uppercase;
	}

	.why-snapvie-proof-intro h3 {
		font-family: 'Nunito', sans-serif;
		font-size: clamp(1.9rem, 3.5vw, 2.35rem);
		line-height: 1.05;
		letter-spacing: -0.02em;
		color: #2d1b36;
		font-weight: 700;
		margin: 0.78rem 0 0.55rem;
	}

	.why-snapvie-proof-intro p {
		margin: 0;
		max-width: 24rem;
		font-size: 1rem;
		line-height: 1.55;
		font-weight: 700;
		color: rgba(45, 27, 54, 0.75);
	}

	.why-snapvie-avatar-stack {
		display: flex;
		align-items: center;
		margin-top: 1.1rem;
	}

	.why-snapvie-avatar {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 999px;
		overflow: hidden;
		border: 2px solid rgba(255, 255, 255, 0.95);
		background: #f5d3e1;
		box-shadow: 0 12px 24px -18px rgba(45, 27, 54, 0.45);
	}

	.why-snapvie-avatar + .why-snapvie-avatar {
		margin-left: -0.65rem;
	}

	.why-snapvie-avatar img {
		display: block;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.why-snapvie-avatar-count {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		background: #2d1b36;
		color: #ffffff;
		font-size: 0.72rem;
		font-weight: 800;
	}

	.why-snapvie-quote-card {
		padding: 0.92rem 0.98rem;
	}

	.why-snapvie-quote-head {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.7rem;
	}

	.why-snapvie-quote-avatar {
		width: 2.25rem;
		height: 2.25rem;
		border-radius: 999px;
		overflow: hidden;
		flex-shrink: 0;
		background: linear-gradient(135deg, rgba(255, 140, 185, 0.24), rgba(255, 219, 160, 0.24));
	}

	.why-snapvie-quote-avatar img {
		display: block;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.why-snapvie-quote-head h4 {
		margin: 0;
		font-family: 'Nunito', sans-serif;
		font-size: 0.9rem;
		font-weight: 700;
		color: #2d1b36;
	}

	.why-snapvie-quote-stars {
		display: flex;
		align-items: center;
		margin-top: 0.16rem;
		color: #facc15;
		font-size: 0.62rem;
		line-height: 1;
		letter-spacing: 0.02em;
	}

	.why-snapvie-quote-copy {
		margin: 0;
		font-size: 0.76rem;
		line-height: 1.6;
		font-weight: 500;
		color: rgba(45, 27, 54, 0.75);
	}

	.why-snapvie-cta-card {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
		align-items: center;
		padding: 0.88rem 1rem;
		background:
			linear-gradient(135deg, rgba(255, 77, 140, 0.12), rgba(255, 185, 56, 0.16)),
			linear-gradient(180deg, rgba(255, 250, 252, 0.8), rgba(255, 245, 248, 0.72));
		overflow: hidden;
	}

	.why-snapvie-cta-title {
		margin: 0 0 0.25rem;
		font-family: 'Fredoka', sans-serif;
		font-size: 0.96rem;
		color: #2d1b36;
	}

	.why-snapvie-cta-copy {
		margin: 0;
		font-size: 0.78rem;
		font-weight: 700;
		color: rgba(45, 27, 54, 0.72);
	}

	.why-snapvie-cta-button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.55rem;
		border: none;
		border-radius: 999px;
		background: #2d1b36;
		color: #ffffff;
		padding: 0.78rem 1.05rem;
		font-family: 'Fredoka', sans-serif;
		font-size: 0.82rem;
		font-weight: 700;
		box-shadow: 0 18px 30px -20px rgba(45, 27, 54, 0.6);
		cursor: pointer;
	}

	.why-snapvie-cta-badge {
		display: none;
		font-size: 2.1rem;
		line-height: 1;
	}

	.why-snapvie-feature.closing-feature-primary .why-snapvie-feature-icon {
		background: rgba(255, 77, 140, 0.14);
		color: #ff4d8c;
	}

	.why-snapvie-feature.closing-feature-secondary .why-snapvie-feature-icon {
		background: rgba(129, 140, 248, 0.16);
		color: #5b5cf0;
	}

	.why-snapvie-feature.closing-feature-accent .why-snapvie-feature-icon {
		background: rgba(255, 185, 56, 0.2);
		color: #dd8a00;
	}

	.why-snapvie-feature.closing-feature-neutral .why-snapvie-feature-icon {
		background: rgba(45, 27, 54, 0.08);
		color: #2d1b36;
	}

	:global(.page-root.theme-dark) .why-snapvie-stage {
		background: transparent;
		border: none;
		box-shadow: none;
	}

	:global(.page-root.theme-dark) .why-snapvie-eyebrow,
	:global(.page-root.theme-dark) .why-snapvie-proof-intro,
	:global(.page-root.theme-dark) .why-snapvie-quote-card {
		background: rgba(255, 255, 255, 0.06);
		border-color: rgba(255, 140, 185, 0.16);
	}

	:global(.page-root.theme-dark) .why-snapvie-feature {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 140, 185, 0.16);
		box-shadow: 0 22px 44px -32px rgba(0, 0, 0, 0.45);
	}

	:global(.page-root.theme-dark) .why-snapvie-title,
	:global(.page-root.theme-dark) .why-snapvie-feature h3,
	:global(.page-root.theme-dark) .why-snapvie-proof-intro h3,
	:global(.page-root.theme-dark) .why-snapvie-quote-head h4,
	:global(.page-root.theme-dark) .why-snapvie-cta-title {
		color: #ffffff;
	}

	:global(.page-root.theme-dark) .why-snapvie-feature p,
	:global(.page-root.theme-dark) .why-snapvie-proof-intro p,
	:global(.page-root.theme-dark) .why-snapvie-quote-copy,
	:global(.page-root.theme-dark) .why-snapvie-cta-copy {
		color: rgba(224, 208, 245, 0.72);
	}

	:global(.page-root.theme-dark) .why-snapvie-feature-kicker {
		color: rgba(224, 208, 245, 0.42);
	}

	:global(.page-root.theme-dark) .why-snapvie-trust-badge {
		background: rgba(34, 197, 94, 0.14);
		color: #9ae6b4;
	}

	:global(.page-root.theme-dark) .why-snapvie-cta-card {
		background:
			linear-gradient(135deg, rgba(255, 77, 140, 0.14), rgba(255, 185, 56, 0.16)),
			linear-gradient(180deg, rgba(255, 255, 255, 0.04), rgba(255, 255, 255, 0.03));
		border-color: rgba(255, 140, 185, 0.16);
	}

	:global(.page-root.theme-dark) .why-snapvie-avatar-count,
	:global(.page-root.theme-dark) .why-snapvie-cta-button {
		background: #ffffff;
		color: #2d1b36;
	}

	@media (max-width: 767px) {
		.why-snapvie-stage::before,
		.why-snapvie-stage::after {
			display: none;
		}
	}

	@media (min-width: 768px) {
		.why-snapvie-stage {
			padding: 1.55rem 1.65rem 1.6rem;
		}

		.why-snapvie-title {
			max-width: none;
		}

		.why-snapvie-features {
			grid-template-columns: repeat(4, minmax(0, 1fr));
			gap: 1rem;
			margin-top: 1.2rem;
		}

		.why-snapvie-proof {
			grid-template-columns: minmax(0, 1.3fr) minmax(0, 0.95fr) minmax(0, 0.95fr);
			grid-template-rows: auto auto;
			align-items: start;
			margin-top: 3.25rem;
		}

		.why-snapvie-proof-intro {
			grid-column: 1;
			grid-row: 1 / span 2;
			padding: 1.35rem;
		}

		.quote-sarah {
			grid-column: 2;
			grid-row: 1;
			transform: translateY(0.05rem);
		}

		.quote-mike {
			grid-column: 3;
			grid-row: 1;
			transform: translateY(0.05rem);
		}

		.why-snapvie-cta-card {
			grid-column: 2 / span 2;
			grid-row: 2;
			grid-template-columns: minmax(0, 1fr) auto auto;
			align-self: end;
			padding: 0.9rem 1.15rem;
		}

		.why-snapvie-cta-badge {
			display: block;
		}
	}
</style>
