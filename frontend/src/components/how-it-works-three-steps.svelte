<script lang="ts">
	import AppIcon from '$components/AppIcon.svelte';
	import * as m from '$lib/paraglide/messages';

	let { sectionId = 'how-it-works' }: { sectionId?: string } = $props();
</script>

<section class="how-it-works-section defer-render-how-it-works py-8 px-6 lg:px-20 relative z-20" id={sectionId}>
	<div class="max-w-6xl mx-auto">
		<div class="grid grid-cols-1 md:grid-cols-3 gap-6 relative">
			<div class="relative group">
				<div class="how-it-works-card bg-white p-6 rounded-[2rem] shadow-sm border border-pink-50 hover:shadow-float transition-all duration-300 h-full flex flex-col items-center text-center relative overflow-hidden">
					<div class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-primary to-transparent opacity-50"></div>
					<div class="w-16 h-16 rounded-full bg-primary/10 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform duration-300">
						<AppIcon name="search" class="text-3xl text-primary" />
					</div>
					<h3 class="how-it-works-step-title text-xl font-bold text-plum mb-2">{m.home_step1_title()}</h3>
					<p class="how-it-works-step-desc text-plum/75 text-sm font-medium leading-snug">{m.home_step1_desc()}</p>
				</div>
				<div class="md:hidden flex justify-center py-2 text-plum/20">
					<AppIcon name="arrow_downward" />
				</div>
			</div>

			<div class="relative group md:-translate-y-4">
				<div class="how-it-works-step-paste-card bg-gradient-to-br from-white to-pink-50 p-6 rounded-[2rem] shadow-candy border border-pink-100 hover:-translate-y-1 transition-all duration-300 h-full flex flex-col items-center text-center relative overflow-hidden z-10">
					<div class="absolute -right-10 -top-10 w-24 h-24 bg-secondary/10 rounded-full blur-xl"></div>
					<div class="w-16 h-16 rounded-full bg-secondary text-white shadow-lg flex items-center justify-center mb-4 animate-bob">
						<AppIcon name="content_paste_go" class="text-3xl" />
					</div>
					<h3 class="how-it-works-step-title text-xl font-bold text-plum mb-2">{m.home_step2_title()}</h3>
					<p class="how-it-works-step-desc text-plum/75 text-sm font-medium leading-snug">{m.home_step2_desc()}</p>
				</div>
				<div class="md:hidden flex justify-center py-2 text-plum/20">
					<AppIcon name="arrow_downward" />
				</div>
			</div>

			<div class="relative group">
				<div class="how-it-works-card bg-white p-6 rounded-[2rem] shadow-sm border border-pink-50 hover:shadow-float transition-all duration-300 h-full flex flex-col items-center text-center relative overflow-hidden">
					<div class="absolute top-0 right-0 w-full h-1 bg-gradient-to-l from-accent to-transparent opacity-50"></div>
					<div class="w-16 h-16 rounded-full bg-accent/10 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform duration-300">
						<AppIcon name="download_for_offline" class="text-3xl text-accent" />
					</div>
					<h3 class="how-it-works-step-title text-xl font-bold text-plum mb-2">{m.home_step3_title()}</h3>
					<p class="how-it-works-step-desc text-plum/75 text-sm font-medium leading-snug">{m.home_step3_desc()}</p>
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.how-it-works-section {
		content-visibility: auto;
		contain-intrinsic-size: 580px;
	}

	.how-it-works-step-title {
		font-family: 'Fredoka', sans-serif;
		font-weight: 700;
	}

	.how-it-works-step-desc {
		font-weight: 600;
	}

	:global(.page-root.theme-dark) .how-it-works-step-paste-card {
		background: linear-gradient(135deg, rgba(38, 34, 56, 0.95), rgba(29, 26, 43, 0.95)) !important;
		border-color: rgba(255, 77, 140, 0.22) !important;
		box-shadow: 0 0 20px rgba(255, 77, 140, 0.28) !important;
	}

	:global(.page-root.theme-dark) .how-it-works-card {
		background: rgba(255, 255, 255, 0.04) !important;
		border-color: rgba(255, 77, 140, 0.16) !important;
	}

	:global(.page-root.theme-dark) .how-it-works-card:hover {
		box-shadow: 0 0 20px rgba(255, 77, 140, 0.35) !important;
	}

	:global(.page-root.theme-dark) .how-it-works-step-title {
		color: #ffffff !important;
	}

	:global(.page-root.theme-dark) .how-it-works-step-desc {
		color: rgba(224, 208, 245, 0.72) !important;
	}
</style>
