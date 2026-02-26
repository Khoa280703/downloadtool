<script lang="ts">
	import DownloadBtn from '$components/DownloadBtn.svelte';
	import FormatPicker from '$components/FormatPicker.svelte';
	import { extract, isValidVideoUrl } from '$lib/api';
	import type { ExtractResult, Stream } from '$lib/types';
	import { currentDownload } from '$stores/download';

	let inputUrl = $state('');
	let extractResult = $state<ExtractResult | null>(null);
	let selectedAudioStream = $state<Stream | null>(null);
	let isExtracting = $state(false);
	let extractError = $state('');

	function handleFormatSelect(videoStream: Stream, audioStream: Stream | null): void {
		currentDownload.update((state) => ({ ...state, selectedStream: videoStream }));
		selectedAudioStream = audioStream;
	}

	async function handleFetch(event?: SubmitEvent): Promise<void> {
		event?.preventDefault();
		const url = inputUrl.trim();

		if (!url) {
			extractError = 'Please paste a YouTube URL.';
			return;
		}

		if (!isValidVideoUrl(url)) {
			extractError = 'Please enter a valid YouTube URL.';
			return;
		}

		isExtracting = true;
		extractError = '';
		extractResult = null;
		selectedAudioStream = null;
		currentDownload.update((state) => ({ ...state, selectedStream: null, error: null }));
		requestAnimationFrame(() => {
			document
				.getElementById('download-options')
				?.scrollIntoView({ behavior: 'smooth', block: 'start' });
		});

		try {
			const result = await extract(url);
			if (!result.streams.length) {
				extractError = 'No downloadable streams were found for this video.';
				return;
			}

			extractResult = result;
			queueMicrotask(() => {
				document
					.getElementById('download-options')
					?.scrollIntoView({ behavior: 'smooth', block: 'start' });
			});
		} catch (error) {
			extractError = error instanceof Error ? error.message : 'Failed to fetch this URL.';
		} finally {
			isExtracting = false;
		}
	}

	function formatDuration(seconds?: number): string {
		if (!seconds || seconds <= 0) return 'N/A';
		const total = Math.floor(seconds);
		const h = Math.floor(total / 3600);
		const m = Math.floor((total % 3600) / 60);
		const s = total % 60;
		if (h > 0) return `${h}:${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
		return `${m}:${String(s).padStart(2, '0')}`;
	}

	function formatViews(views?: number): string {
		if (!views || views <= 0) return 'N/A';
		if (views >= 1_000_000_000) return `${(views / 1_000_000_000).toFixed(1)}B`;
		if (views >= 1_000_000) return `${(views / 1_000_000).toFixed(1)}M`;
		if (views >= 1_000) return `${(views / 1_000).toFixed(1)}K`;
		return `${views}`;
	}

	function shortDescription(text?: string, maxLength = 180): string {
		if (!text) return '';
		const normalized = text.replace(/\s+/g, ' ').trim();
		if (normalized.length <= maxLength) return normalized;
		return `${normalized.slice(0, maxLength).trimEnd()}...`;
	}

	function scrollToFetcher(): void {
		document.getElementById('home')?.scrollIntoView({ behavior: 'smooth', block: 'start' });
		queueMicrotask(() => {
			const input = document.getElementById('video-url-input') as HTMLInputElement | null;
			input?.focus();
		});
	}
</script>

<svelte:head>
<title>FetchTube - Vibrant Video Downloader</title>
<script src="https://cdn.tailwindcss.com?plugins=forms,container-queries"></script>
<link rel="preconnect" href="https://fonts.googleapis.com"/>
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
<link href="https://fonts.googleapis.com/css2?family=Fredoka:wght@300..700&amp;family=Nunito:ital,wght@0,200..1000;1,200..1000&amp;family=Spline+Sans:wght@300..700&amp;family=Material+Symbols+Outlined:wght,FILL@100..700,0..1&amp;display=swap" rel="stylesheet"/>
<script id="tailwind-config">
        tailwind.config = {
            theme: {
                extend: {
                    colors: {
                        "primary": "#FF4D8C", // Hot Pink
                        "secondary": "#FFB938", // Marigold
                        "accent": "#6C5CE7", // Periwinkle
                        "plum": "#2D1B36", // Deep Plum
                        "muted": "#8B7E96",
                        "bg-page": "#FFF5F9", // Pale Pink White
                        "bg-surface": "#FFFFFF",
                        "background-light": "#FFF5F9",
                        "surface": "#FFFFFF",
                        "text-main": "#2D1B36",
                    },
                    fontFamily: {
                        "heading": ["Fredoka", "sans-serif"],
                        "body": ["Nunito", "sans-serif"],
                        "display": ["Spline Sans", "sans-serif"]
                    },
                    borderRadius: {
                        "xl": "24px",
                        "2xl": "32px", // radius-xl
                        "3xl": "48px",
                        "full": "9999px",
                        "blob": "40% 60% 70% 30% / 40% 50% 60% 50%"
                    },
                    boxShadow: {
                        "float": "0 20px 40px -10px rgba(255, 77, 140, 0.3)",
                        "candy": "0 10px 25px -5px rgba(255, 77, 140, 0.4), 0 8px 10px -6px rgba(255, 77, 140, 0.1)",
                        "input-focus": "0 0 0 4px rgba(255, 77, 140, 0.2)",
                        "card": "0 10px 30px -5px rgba(45, 27, 54, 0.05)",
                        "glow": "0 0 20px rgba(255, 77, 140, 0.4)"
                    },
                    animation: {
                        'bob': 'bob 3s ease-in-out infinite',
                        'bob-delayed': 'bob 3s ease-in-out infinite 1.5s',
                        'wiggle': 'wiggle 1s ease-in-out infinite',
                        'pulse-glow': 'pulse-glow 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
                    },
                    keyframes: {
                        bob: {
                            '0%, 100%': { transform: 'translateY(0)' },
                            '50%': { transform: 'translateY(-15px)' },
                        },
                        wiggle: {
                            '0%, 100%': { transform: 'rotate(-3deg)' },
                            '50%': { transform: 'rotate(3deg)' },
                        },
                        'pulse-glow': {
                            '0%, 100%': { opacity: 1, boxShadow: '0 0 0 0 rgba(255, 77, 140, 0.7)' },
                            '50%': { opacity: .5, boxShadow: '0 0 0 10px rgba(255, 77, 140, 0)' },
                        }
                    }
                },
            },
        }
    </script>
<style>
        body {
            font-family: 'Nunito', sans-serif;
            background-color: #FFF5F9;
            color: #2D1B36;
        }
        h1, h2, h3, h4, h5, h6, button {
            font-family: 'Fredoka', sans-serif;
        }
        .glass-header {
            background: rgba(255, 245, 249, 0.8);
            backdrop-filter: blur(12px);
            -webkit-backdrop-filter: blur(12px);
        }
        .text-gradient {
            background: linear-gradient(135deg, #FF4D8C 0%, #FFB938 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        .bg-gradient-primary {
            background: linear-gradient(135deg, #FF4D8C 0%, #FFB938 100%);
        }
        .dashed-border-anim {
            background-image: url("data:image/svg+xml,%3csvg width='100%25' height='100%25' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='100%25' height='100%25' fill='none' rx='32' ry='32' stroke='%23FF4D8CFF' stroke-width='3' stroke-dasharray='12%2c 12' stroke-dashoffset='0' stroke-linecap='round'/%3e%3c/svg%3e");
        }html { scroll-behavior: smooth; }
        .bento-card {
            transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1), box-shadow 0.3s ease;
        }
        .bento-card:hover {
            transform: translateY(-8px);
            box-shadow: 0 25px 50px -12px rgba(255, 77, 140, 0.25);
        }
        .zig-zag-container > div:nth-child(even) {
            flex-direction: row-reverse;
        }
        .hide-scrollbar::-webkit-scrollbar {
            display: none;
        }
        .hide-scrollbar {
            -ms-overflow-style: none;
            scrollbar-width: none;
        }
		.defer-section {
			content-visibility: auto;
			contain-intrinsic-size: 1px 900px;
		}
		@media (max-width: 768px) {
			.hero-orb {
				display: none;
			}
		}
		@media (prefers-reduced-motion: reduce) {
			html {
				scroll-behavior: auto !important;
			}
			[class*="animate-"] {
				animation: none !important;
			}
			.bento-card,
			.bento-card:hover,
			button,
			a {
				transition: none !important;
				transform: none !important;
			}
		}
    </style>
</svelte:head>
<div class="bg-bg-page min-h-screen flex flex-col overflow-x-hidden text-plum selection:bg-primary/20">
<header class="glass-header sticky top-0 z-50 border-b border-white/50 px-6 py-3 lg:px-20 transition-all duration-300">
<div class="max-w-7xl mx-auto flex items-center justify-between">
<div class="flex items-center gap-3 group cursor-pointer">
<div class="flex size-10 items-center justify-center rounded-full bg-gradient-primary text-white shadow-candy group-hover:rotate-12 transition-transform">
<span class="material-symbols-outlined text-2xl">smart_toy</span>
</div>
<h2 class="text-plum text-2xl font-bold tracking-tight">FetchTube</h2>
</div>
<div class="hidden md:flex items-center gap-8">
<nav class="flex gap-6">
<a class="text-plum font-semibold hover:text-primary transition-colors text-base" href="#home">Home</a>
<a class="text-plum font-semibold hover:text-primary transition-colors text-base" href="#tools">Tools</a>
<a class="text-plum font-semibold hover:text-primary transition-colors text-base" href="#how-it-works">How it Works</a>
</nav>
<button class="flex h-10 px-6 items-center justify-center rounded-full bg-plum text-white text-sm font-bold shadow-lg hover:bg-plum/90 hover:scale-105 active:scale-95 transition-all duration-300 tracking-wide uppercase">
                Login
            </button>
</div>
<button class="md:hidden text-plum p-2 rounded-xl hover:bg-white/50 transition-colors">
<span class="material-symbols-outlined text-3xl">menu_open</span>
</button>
</div>
</header>
<main class="flex-1 w-full">
<section class="relative min-h-[50vh] flex flex-col items-center justify-center px-6 pt-12 pb-6 overflow-hidden" id="home">
<div class="hero-orb absolute top-[10%] left-[5%] w-24 h-24 rounded-full bg-accent/20 blur-xl animate-bob"></div>
<div class="hero-orb absolute bottom-[20%] right-[10%] w-32 h-32 rounded-3xl rotate-12 bg-primary/10 blur-xl animate-bob-delayed"></div>
<div class="hero-orb absolute top-[20%] right-[15%] w-16 h-16 rounded-full bg-secondary/30 blur-lg animate-bob"></div>
<div class="relative z-10 w-full max-w-4xl mx-auto text-center">
<div class="inline-flex items-center gap-2 bg-white px-3 py-1.5 rounded-full shadow-sm mb-4 animate-fade-in-up">
<span class="text-lg">✨</span>
<span class="text-xs font-bold text-plum/80 uppercase tracking-wide">The friendliest downloader ever</span>
</div>
<h1 class="text-4xl md:text-6xl lg:text-7xl font-bold text-plum mb-4 leading-[0.9] tracking-tight">
                Save videos in a <br/>
<span class="text-gradient inline-block hover:scale-105 transition-transform cursor-default">snap.</span>
</h1>
<p class="text-lg md:text-xl text-plum/70 max-w-xl mx-auto font-semibold mb-6">
                Paste a link, click the button, and get back to your life. No ads, no malware, just pure joy.
            </p>
<form class="relative w-full max-w-[700px] mx-auto group" onsubmit={handleFetch}>
<div class="absolute -inset-1 bg-gradient-to-r from-primary to-secondary rounded-full blur opacity-25 group-hover:opacity-50 transition duration-500"></div>
<div class="relative flex items-center bg-white rounded-full shadow-float p-2 h-[64px] transition-all duration-300 group-focus-within:ring-4 group-focus-within:ring-primary/20">
<div class="pl-6 text-plum/30">
<span class="material-symbols-outlined text-2xl">link</span>
</div>
<input id="video-url-input" class="w-full h-full bg-transparent border-none focus:ring-0 text-lg md:text-xl font-semibold placeholder:text-muted/50 text-plum px-4" placeholder="Paste that YouTube link here..." type="text" bind:value={inputUrl} disabled={isExtracting}/>
<button class="absolute right-1.5 top-1.5 bottom-1.5 bg-gradient-primary hover:brightness-110 text-white font-bold rounded-full px-6 md:px-10 text-base md:text-lg shadow-candy transition-all hover:scale-105 active:scale-95 flex items-center gap-2 disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:scale-100" type="submit" disabled={isExtracting}>
<span>{isExtracting ? 'FETCHING...' : 'FETCH IT!'}</span>
<span class="material-symbols-outlined font-bold text-lg">{isExtracting ? 'progress_activity' : 'bolt'}</span>
</button>
</div>
</form>
{#if extractError}
<p class="mt-4 text-sm font-bold text-red-500">{extractError}</p>
{/if}
{#if isExtracting}
<p class="mt-4 inline-flex items-center gap-2 rounded-full bg-white/80 px-4 py-2 text-sm font-bold text-primary shadow-sm">
<span class="material-symbols-outlined animate-spin text-base">progress_activity</span>
Analyzing your video link...
</p>
{/if}
<div class="mt-8 flex flex-wrap justify-center gap-3 opacity-60 grayscale hover:grayscale-0 transition-all duration-500">
<div class="flex items-center gap-2 bg-white/50 px-3 py-1.5 rounded-xl">
<span class="material-symbols-outlined text-green-500 text-lg">check_circle</span>
<span class="font-bold text-xs">Ad-Free Forever</span>
</div>
<div class="flex items-center gap-2 bg-white/50 px-3 py-1.5 rounded-xl">
<span class="material-symbols-outlined text-blue-500 text-lg">verified_user</span>
<span class="font-bold text-xs">Safe &amp; Secure</span>
</div>
<div class="flex items-center gap-2 bg-white/50 px-3 py-1.5 rounded-xl">
<span class="material-symbols-outlined text-purple-500 text-lg">rocket_launch</span>
<span class="font-bold text-xs">Super Fast</span>
</div>
</div>
</div>
</section>
{#if isExtracting}
<section class="py-8 px-6 lg:px-20" id="download-options">
<div class="max-w-7xl mx-auto">
<div class="bg-white rounded-[2rem] shadow-card border border-indigo-50 overflow-hidden flex flex-col lg:flex-row animate-pulse">
<div class="w-full lg:w-[42%] p-6 md:p-8 flex flex-col gap-5 bg-gradient-to-b from-indigo-50/50 to-white lg:border-r border-indigo-50">
<div class="w-full aspect-video rounded-3xl bg-slate-200"></div>
<div class="h-7 w-11/12 rounded-xl bg-slate-200"></div>
<div class="flex gap-2">
<div class="h-8 w-24 rounded-full bg-slate-200"></div>
<div class="h-8 w-24 rounded-full bg-slate-200"></div>
<div class="h-8 w-28 rounded-full bg-slate-200"></div>
</div>
<div class="h-20 rounded-2xl bg-slate-100"></div>
</div>
<div class="flex-1 flex flex-col bg-white">
<div class="p-6 md:p-8 flex flex-col gap-3">
<div class="h-11 rounded-2xl bg-slate-200"></div>
<div class="h-20 rounded-2xl bg-slate-100"></div>
<div class="h-20 rounded-2xl bg-slate-100"></div>
<div class="h-20 rounded-2xl bg-slate-100"></div>
</div>
<div class="p-6 md:p-8 border-t border-indigo-50 bg-indigo-50/20">
<div class="h-14 rounded-full bg-slate-200"></div>
</div>
</div>
</div>
</div>
</section>
{:else if extractResult}
<section class="py-8 px-6 lg:px-20" id="download-options">
<div class="max-w-7xl mx-auto">
<div class="bg-white rounded-[2rem] shadow-card border border-indigo-50 overflow-hidden flex flex-col lg:flex-row">
<div class="w-full lg:w-[42%] p-6 md:p-8 flex flex-col gap-5 bg-gradient-to-b from-indigo-50/50 to-white lg:border-r border-indigo-50">
<div class="relative w-full aspect-video rounded-3xl overflow-hidden shadow-lg border-4 border-white bg-slate-100">
{#if extractResult.thumbnail}
<img class="absolute inset-0 w-full h-full object-cover" src={extractResult.thumbnail} alt={extractResult.title}/>
{:else}
<div class="absolute inset-0 grid place-items-center text-slate-400">
<span class="material-symbols-outlined text-6xl">movie</span>
</div>
{/if}
<div class="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent"></div>
<div class="absolute bottom-4 right-4 bg-black/60 backdrop-blur-md px-3 py-1.5 rounded-full text-xs font-bold text-white border border-white/20">
Available
</div>
</div>
<h3 class="text-2xl md:text-3xl font-bold text-slate-900 leading-tight">{extractResult.title}</h3>
<div class="flex flex-wrap items-center gap-3">
<div class="flex items-center gap-2 bg-indigo-50 px-3 py-1.5 rounded-full text-indigo-600 text-sm font-bold">
<span class="material-symbols-outlined text-[18px]">schedule</span>
{formatDuration(extractResult.duration)}
</div>
{#if extractResult.viewCount}
<div class="flex items-center gap-2 bg-pink-50 px-3 py-1.5 rounded-full text-pink-600 text-sm font-bold">
<span class="material-symbols-outlined text-[18px]">visibility</span>
{formatViews(extractResult.viewCount)}
</div>
{/if}
{#if extractResult.channel}
<div class="flex items-center gap-2 bg-slate-100 px-3 py-1.5 rounded-full text-slate-600 text-sm font-bold">
<span class="material-symbols-outlined text-[18px]">person</span>
{extractResult.channel}
</div>
{/if}
</div>
{#if extractResult.description}
<div class="p-4 bg-slate-50 rounded-2xl border border-slate-100 text-sm text-slate-600 leading-relaxed">
<span class="font-bold text-slate-800">Description:</span> {shortDescription(extractResult.description)}
</div>
{/if}
<p class="text-slate-500 font-semibold">Choose format and quality, then download directly.</p>
</div>
<div class="flex-1 flex flex-col bg-white">
<div class="p-6 md:p-8">
<FormatPicker streams={extractResult.streams} onSelect={handleFormatSelect}/>
</div>
<div class="p-6 md:p-8 border-t border-indigo-50 bg-indigo-50/20">
<DownloadBtn stream={$currentDownload.selectedStream} audioStream={selectedAudioStream} title={extractResult.title}/>
</div>
</div>
</div>
</div>
</section>
{/if}
<section class="defer-section py-12 px-6 lg:px-20 relative bg-white/30" id="tools">
<div class="max-w-7xl mx-auto">
<div class="text-center mb-8">
<h2 class="text-3xl md:text-4xl font-bold text-plum mb-3">
                    Tired of <span class="bg-clip-text text-transparent bg-gradient-to-r from-primary to-secondary">Copy-Pasting?</span>
<span class="inline-block animate-bounce" style="animation-duration: 3s;">😴</span>
</h2>
<p class="text-base text-plum/70 max-w-lg mx-auto">
                    We built some shiny tools to make your life easier. Choose your fighter below.
                </p>
</div>
<div class="grid grid-cols-1 lg:grid-cols-3 gap-6 items-stretch">
<div class="bento-card group relative flex flex-col bg-white rounded-2xl p-6 border border-pink-50 shadow-float overflow-hidden h-full min-h-[360px]">
<div class="absolute top-4 right-4 bg-secondary text-white text-[10px] font-bold px-2.5 py-1 rounded-full shadow-sm z-10 tracking-widest uppercase">
                        Recommended
                    </div>
<div class="flex-1 flex flex-col items-center text-center z-10 mt-2">
<div class="size-24 mb-4 relative flex items-center justify-center">
<div class="absolute inset-0 bg-blue-100 rounded-full scale-110 opacity-0 group-hover:opacity-100 transition-opacity duration-500 blur-xl"></div>
<span class="material-symbols-outlined text-[60px] text-[#4285F4] drop-shadow-xl group-hover:scale-110 transition-transform duration-300">extension</span>
</div>
<h3 class="text-xl font-bold text-plum mb-2">FetchTube Extension</h3>
<p class="text-plum/60 font-medium mb-6 text-base">
                            The easiest way. Adds a cute "Fetch" button right under every video player.
                        </p>
</div>
<div class="mt-auto z-10">
<button class="w-full h-12 bg-primary hover:bg-primary/90 text-white font-bold rounded-full shadow-candy flex items-center justify-center gap-2 transition-all group-hover:scale-[1.02] tracking-wide uppercase text-xs">
<span class="material-symbols-outlined text-lg">add_to_queue</span>
                            Add to Chrome
                        </button>
</div>
<div class="absolute bottom-0 left-0 w-full h-1/2 bg-gradient-to-t from-pink-50 to-transparent opacity-50 pointer-events-none"></div>
</div>
<div class="bento-card group relative flex flex-col bg-white/50 rounded-2xl p-1.5 border-4 border-transparent h-full min-h-[360px]">
<div class="absolute inset-0 rounded-2xl dashed-border-anim pointer-events-none opacity-50 group-hover:opacity-100 transition-opacity"></div>
<div class="relative h-full flex flex-col p-5 bg-white/60 rounded-[18px] backdrop-blur-sm">
<div class="flex-1 flex flex-col items-center text-center">
<div class="w-16 h-16 bg-secondary/20 text-secondary rounded-2xl flex items-center justify-center mb-4 rotate-3 group-hover:rotate-12 transition-transform shadow-sm">
<span class="material-symbols-outlined text-4xl">bookmarks</span>
</div>
<h3 class="text-xl font-bold text-plum mb-2">The Magic Button</h3>
<p class="text-plum/60 font-medium mb-6 text-base">
                                No installation needed. Just drag this pill to your browser's bookmarks bar!
                            </p>
<div class="w-full py-6 px-3 bg-pink-50/50 rounded-xl border border-pink-100 border-dashed mb-3 flex justify-center items-center relative overflow-hidden">
<div class="absolute inset-0 bg-stripes opacity-5"></div>
<a class="cursor-grab active:cursor-grabbing inline-flex items-center gap-2 bg-gradient-to-r from-primary to-secondary text-white font-bold py-2.5 px-6 rounded-full shadow-lg hover:shadow-xl transform hover:scale-105 active:scale-95 transition-all select-none z-10 text-sm" href="https://download.khoadangbui.online">
<span class="material-symbols-outlined text-lg">touch_app</span>
                                    FETCH IT!
                                </a>
</div>
<div class="flex items-center gap-2 text-xs text-plum/50 font-bold bg-white px-3 py-1.5 rounded-lg shadow-sm">
<span class="material-symbols-outlined text-base animate-bounce">arrow_upward</span>
                                Drag up to bookmarks
                            </div>
</div>
</div>
</div>
<div class="bento-card group relative flex flex-col bg-plum rounded-2xl p-6 shadow-2xl h-full min-h-[360px] text-white overflow-hidden">
<div class="absolute top-0 right-0 w-48 h-48 bg-accent/20 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2"></div>
<div class="flex-1 flex flex-col items-start z-10 relative">
<div class="inline-flex items-center gap-2 bg-white/10 backdrop-blur-md px-2.5 py-1 rounded-lg mb-4 border border-white/10">
<span class="material-symbols-outlined text-accent text-xs">terminal</span>
<span class="text-[10px] font-bold tracking-wide uppercase">For Pros</span>
</div>
<h3 class="text-xl font-bold mb-2">Power User Script</h3>
<p class="text-white/70 font-medium mb-6 text-base">
                            Already use Tampermonkey? Get the raw script for maximum control.
                        </p>
<div class="w-full bg-black/30 rounded-xl p-4 font-mono text-xs text-accent mb-4 border border-white/10 shadow-inner">
<div class="flex gap-1.5 mb-2">
<div class="w-2.5 h-2.5 rounded-full bg-red-400"></div>
<div class="w-2.5 h-2.5 rounded-full bg-yellow-400"></div>
<div class="w-2.5 h-2.5 rounded-full bg-green-400"></div>
</div>
<p class="opacity-60">// install.js</p>
<p class="text-secondary">const</p> <span class="text-white">quality</span> = <span class="text-primary">'4k'</span>;
                            <p><span class="text-secondary">await</span> fetch.init();</p>
</div>
</div>
<div class="mt-auto z-10 relative">
<button class="w-full h-12 bg-accent hover:bg-accent/90 text-white font-bold rounded-full shadow-lg hover:shadow-accent/50 flex items-center justify-center gap-2 transition-all group-hover:translate-x-1 uppercase text-xs tracking-wide">
<span class="material-symbols-outlined text-lg">download</span>
                            Install Script
                        </button>
</div>
</div>
</div>
</div>
</section>
<section class="defer-section w-full max-w-4xl px-6 pt-20 pb-12 text-center mx-auto" id="how-it-works">
<span class="inline-block py-2 px-4 rounded-full bg-secondary/20 text-secondary font-heading font-bold text-sm mb-4 tracking-wider uppercase">Simple as 1-2-3</span>
<h2 class="font-heading text-4xl md:text-6xl font-bold text-text-main leading-tight mb-6">
                How the <span class="text-transparent bg-clip-text bg-gradient-to-r from-primary to-secondary">magic</span> happens
            </h2>
<p class="text-lg md:text-xl text-muted font-semibold max-w-2xl mx-auto">
                Saving your favorite videos shouldn't require a computer science degree. We made it as easy as fetching a stick!
            </p>
</section>
<div class="w-full max-w-7xl px-6 flex flex-col gap-24 md:gap-32 zig-zag-container mb-32 mx-auto">
<div class="flex flex-col md:flex-row items-center justify-between gap-12 md:gap-20">
<div class="flex-1 text-center md:text-left space-y-6">
<div class="size-16 rounded-2xl bg-primary/10 text-primary flex items-center justify-center font-heading text-3xl font-bold mx-auto md:mx-0 shadow-sm">
                        01
                    </div>
<h3 class="font-heading text-3xl md:text-4xl font-bold text-text-main">
                        Find a video you love
                    </h3>
<p class="text-lg text-muted font-medium leading-relaxed">
                        Spot a cooking recipe you need for dinner or a music video that hits just right? Simply copy the URL from your browser address bar. That's all the technical skill you need!
                    </p>
<div class="pt-4 flex justify-center md:justify-start gap-4">
<div class="flex items-center gap-2 text-sm font-bold text-accent bg-accent/10 px-4 py-2 rounded-full">
<span class="material-symbols-outlined text-lg">youtube_activity</span> YouTube
                        </div>
<div class="flex items-center gap-2 text-sm font-bold text-primary bg-primary/10 px-4 py-2 rounded-full">
<span class="material-symbols-outlined text-lg">music_note</span> TikTok
                        </div>
</div>
</div>
<div class="flex-1 w-full flex justify-center">
<div class="relative w-full max-w-md aspect-square">
<div class="absolute inset-0 bg-gradient-to-tr from-accent/20 to-primary/20 rounded-blob blur-2xl transform rotate-6"></div>
<div class="relative h-full w-full bg-surface rounded-3xl shadow-float overflow-hidden border-4 border-white flex flex-col transform hover:-rotate-2 transition-transform duration-500">
<div class="bg-background-light p-4 border-b border-primary/5 flex gap-2">
<div class="size-3 rounded-full bg-primary/40"></div>
<div class="size-3 rounded-full bg-secondary/40"></div>
<div class="flex-1 bg-white rounded-full h-3 ml-2"></div>
</div>
<div class="flex-1 relative overflow-hidden bg-background-light">
<img alt="Abstract 3D illustration of a character watching a colorful screen" class="w-full h-full object-cover" src="https://lh3.googleusercontent.com/aida-public/AB6AXuBUE52gefPx9GfaaiAVYCx9c75IjqlqTyvWaZ1XNQPkXQ2XPkECWD8XY4Z31Tjlya5CkEBBq6o98gEsW7TdGXVwWJ8LJOkmaH87xlhH95XVVAS1juqrAsIAP4_gn5ulok4TGsxJKMowAwd4zjZ9F9smdj9BcaaxGotZMo4sCEq1dtTAYhg3lbTfRHxkRGL2Emxt7wszmO70pYq_nxIobSLDzB-64rRmPzT9ybjNH6Vj4ogYsa-Q-3LlEAXIpGgpNIYb5nkP4VDKXFc" loading="lazy" decoding="async"/>
<div class="absolute bottom-6 left-6 right-6 bg-white/90 backdrop-blur-sm p-4 rounded-2xl shadow-lg flex items-center gap-3 animate-pulse">
<span class="material-symbols-outlined text-primary">link</span>
<div class="h-2 w-2/3 bg-gray-200 rounded-full"></div>
</div>
</div>
</div>
</div>
</div>
</div>
<div class="flex flex-col md:flex-row items-center justify-between gap-12 md:gap-20">
<div class="flex-1 text-center md:text-left space-y-6">
<div class="size-16 rounded-2xl bg-secondary/10 text-secondary flex items-center justify-center font-heading text-3xl font-bold mx-auto md:mx-0 shadow-sm">
                        02
                    </div>
<h3 class="font-heading text-3xl md:text-4xl font-bold text-text-main">
                        Paste &amp; Pop it in
                    </h3>
<p class="text-lg text-muted font-medium leading-relaxed">
                        Head over to our massive input bar and drop that link like it's hot. Our fetching engine analyzes the video instantly—no waiting around for spinning wheels.
                    </p>
<ul class="space-y-3 pt-2 inline-block text-left">
<li class="flex items-center gap-3 text-text-main font-bold">
<span class="material-symbols-outlined text-green-500">check_circle</span>
                            Instant analysis
                        </li>
<li class="flex items-center gap-3 text-text-main font-bold">
<span class="material-symbols-outlined text-green-500">check_circle</span>
                            100% Ad-free zone
                        </li>
</ul>
</div>
<div class="flex-1 w-full flex justify-center">
<div class="relative w-full max-w-md aspect-square">
<div class="absolute inset-0 bg-gradient-to-bl from-secondary/30 to-primary/10 rounded-full blur-3xl -z-10"></div>
<div class="relative h-full w-full rounded-3xl flex items-center justify-center">
<div class="relative w-64 h-20 bg-white rounded-full shadow-float flex items-center px-4 border-2 border-primary/10 z-20">
<div class="text-gray-300 text-sm font-bold truncate">https://youtube.com/watch...</div>
<div class="ml-auto bg-primary text-white rounded-full p-2 shadow-lg">
<span class="material-symbols-outlined text-sm">download</span>
</div>
<img alt="3D stylized hand interacting with a digital interface element" class="absolute -top-24 -right-12 w-48 h-48 object-cover rounded-full border-4 border-white shadow-xl z-30" src="https://lh3.googleusercontent.com/aida-public/AB6AXuDSiB2lQRivL_3C8SHYi3mXfiOG-SxLgKM-SB1ywVm8Et0-lZ5KhNtf1xEVtGI53bX8HHOcwqwN-DmNoRm_V7HSr3gIcb3ggiLkvozwClXDX56-Y0lKSeSXCdmqT17Bk-ir8nJWtuROT3YRCjgeh3mAfiE-Hr2_80jm-1VrsBBn6dnyLHr5w7xehIGRFqMxu9yuDHgZFwfSnAphrXCeR4lht9tofH5e-gN0vawA_YNcG_XsTTRLE9DuD4mztrFvAEGe3UpM5C9pOQk" loading="lazy" decoding="async"/>
</div>
<div class="absolute w-48 h-48 bg-secondary rounded-full -bottom-4 -left-4 z-10 opacity-20"></div>
<div class="absolute w-32 h-32 bg-primary rounded-full top-0 right-10 z-0 opacity-10"></div>
</div>
</div>
</div>
</div>
<div class="flex flex-col md:flex-row items-center justify-between gap-12 md:gap-20">
<div class="flex-1 text-center md:text-left space-y-6">
<div class="size-16 rounded-2xl bg-accent/10 text-accent flex items-center justify-center font-heading text-3xl font-bold mx-auto md:mx-0 shadow-sm">
                        03
                    </div>
<h3 class="font-heading text-3xl md:text-4xl font-bold text-text-main">
                        Watch offline, anywhere
                    </h3>
<p class="text-lg text-muted font-medium leading-relaxed">
                        Download the file to your device and enjoy your content on a plane, in a submarine, or in your secret fortress of solitude. No internet? No problem.
                    </p>
<div class="pt-4 flex flex-wrap justify-center md:justify-start gap-3">
<span class="px-4 py-2 bg-white border border-gray-100 shadow-sm rounded-xl font-bold text-sm text-gray-600">MP4 4K</span>
<span class="px-4 py-2 bg-white border border-gray-100 shadow-sm rounded-xl font-bold text-sm text-gray-600">MP3 Audio</span>
<span class="px-4 py-2 bg-white border border-gray-100 shadow-sm rounded-xl font-bold text-sm text-gray-600">GIF Maker</span>
</div>
</div>
<div class="flex-1 w-full flex justify-center">
<div class="relative w-full max-w-md aspect-square">
<div class="absolute inset-0 bg-gradient-to-r from-accent/20 to-secondary/20 rounded-blob blur-2xl transform -rotate-12"></div>
<div class="relative h-full w-full bg-surface rounded-[3rem] shadow-float overflow-hidden border-8 border-white group">
<img alt="Person relaxing by a pool looking at a tablet screen" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-700" src="https://lh3.googleusercontent.com/aida-public/AB6AXuDeEqX7ATwqRX3iw85ORD_oO7ZvZSNsaKjYmel3GwH8W7eemo3nMQPyZ7M4D3f0vNiu9g2-ejMvoBWTcN4NDcX_uCQyCIZAP3Y830M9s9IjtyIAMqcDq6WA5mT5p7kYpDvdyRzxw4gWYo6xZjHNR2SJPNUePHiCOVsFNRU4tA5jQ7zL4v1yQfV_EExDvoxc1z9tLBp1MzG0zXLD4MIbV8ZpzcbvaAwt9iYtpDQlSDKyVXliqf5JOE9uEb62ggrYxcLdbyRHKGpGEB0" loading="lazy" decoding="async"/>
<div class="absolute top-8 right-8 bg-white/80 backdrop-blur-md px-4 py-2 rounded-xl shadow-lg flex items-center gap-2">
<div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
<span class="text-xs font-bold text-text-main">Offline Mode</span>
</div>
</div>
</div>
</div>
</div>
</div>
<section class="defer-section w-full bg-surface rounded-t-[4rem] pt-20 pb-24 px-6 shadow-card overflow-hidden relative">
<div class="absolute top-0 left-0 w-64 h-64 bg-primary/5 rounded-full blur-3xl -translate-x-1/2 -translate-y-1/2"></div>
<div class="absolute bottom-0 right-0 w-96 h-96 bg-secondary/5 rounded-full blur-3xl translate-x-1/3 translate-y-1/3"></div>
<div class="max-w-7xl mx-auto relative z-10">
<div class="text-center mb-16">
<h2 class="font-heading text-3xl md:text-5xl font-bold text-text-main mb-6">Loved by Humans <br class="hidden md:block"/>(and some robots)</h2>
<div class="flex flex-wrap justify-center gap-6 md:gap-12 mb-12">
<div class="flex items-center gap-2 text-muted font-bold">
<span class="material-symbols-outlined text-primary">shield_lock</span>
                            Privacy Safe
                        </div>
<div class="flex items-center gap-2 text-muted font-bold">
<span class="material-symbols-outlined text-secondary">block</span>
                            No Pop-up Ads
                        </div>
<div class="flex items-center gap-2 text-muted font-bold">
<span class="material-symbols-outlined text-accent">bolt</span>
                            Lightning Fast
                        </div>
</div>
</div>
<div class="flex overflow-x-auto gap-6 pb-8 hide-scrollbar snap-x snap-mandatory px-4 md:justify-center">
<div class="snap-center shrink-0 w-[300px] md:w-[350px] bg-background-light p-8 rounded-3xl shadow-sm border border-gray-100 hover:-translate-y-2 transition-transform duration-300">
<div class="flex items-center gap-4 mb-4">
<div class="size-12 rounded-full overflow-hidden border-2 border-primary">
<img alt="Portrait of Sarah" class="w-full h-full object-cover" src="https://lh3.googleusercontent.com/aida-public/AB6AXuC3Y3A-H3EGvPU8zmqaikjguZFx0MdWCuAVz4FZ7BL-f0S8kmzaaMnFNde8chh1kdoCn-oaSF7Lx0F15djvpv4EZlhnDenwAcl9oNmyhvAOxw03HXO5cFt4IG723uHTi1UIUN9uktuYRDVdOXr8JjjLdFFN5gGE-32PYiKwFSkgcsVcKEaJR3x_yt1wi4120_C8xmqTy2i7x2mmdJnmwEXRiLCVJsjUu6gik5exKPKpfI-zjLRXtnc7JV0qMZDWoRqSOjJW_SmzdI4" loading="lazy" decoding="async"/>
</div>
<div>
<h4 class="font-heading font-bold text-text-main">Sarah Jenkins</h4>
<div class="flex text-secondary text-sm">
<span class="material-symbols-outlined text-[18px]">star</span>
<span class="material-symbols-outlined text-[18px]">star</span>
<span class="material-symbols-outlined text-[18px]">star</span>
<span class="material-symbols-outlined text-[18px]">star</span>
<span class="material-symbols-outlined text-[18px]">star</span>
</div>
</div>
</div>
<p class="text-text-main font-medium leading-relaxed italic">
                            "My grandma uses this! It's so pink and easy to understand. Finally, a downloader that doesn't feel like I'm installing a virus."
                        </p>
</div>
<div class="snap-center shrink-0 w-[300px] md:w-[350px] bg-primary text-white p-8 rounded-3xl shadow-float transform md:scale-105 hover:-translate-y-2 transition-transform duration-300">
<div class="flex items-center gap-4 mb-4">
<div class="size-12 rounded-full overflow-hidden border-2 border-white">
<img alt="Portrait of Mike" class="w-full h-full object-cover" src="https://lh3.googleusercontent.com/aida-public/AB6AXuC76jOxNNtx3Aysjd69Cxad7WEobtismgpD69h9FpOcD1oiwzQceZ8WnbAj7C0bIaV81x5YTNd7SzKorY0qKRd-QSTX44ds-jTUPzRaipVGnena-HEv8HoJFHEf-FnevgwaEcSEpy3aVWTB-X9GRLmAzWdXdfsKcYZuPd2dfoMLSYzccApsUCgo82fU-ZqSBZir02kNcmHE46EBnwy5-BRnbdSrI6_htYogzzZ8WZv4de8syjsAdKQ1-EpRQ-ZQEN0BhqHXoAMaBuY" loading="lazy" decoding="async"/>
</div>
<div>
<h4 class="font-heading font-bold text-white">Mike Chen</h4>
<div class="flex text-secondary text-sm">
<span class="material-symbols-outlined text-[18px] text-white">star</span>
<span class="material-symbols-outlined text-[18px] text-white">star</span>
<span class="material-symbols-outlined text-[18px] text-white">star</span>
<span class="material-symbols-outlined text-[18px] text-white">star</span>
<span class="material-symbols-outlined text-[18px] text-white">star</span>
</div>
</div>
</div>
<p class="text-white font-medium leading-relaxed italic">
                            "I use this for all my study lectures when the uni wifi is down. The MP3 converter is a lifesaver for my commute!"
                        </p>
</div>
<div class="snap-center shrink-0 w-[300px] md:w-[350px] bg-background-light p-8 rounded-3xl shadow-sm border border-gray-100 hover:-translate-y-2 transition-transform duration-300">
<div class="flex items-center gap-4 mb-4">
<div class="size-12 rounded-full overflow-hidden border-2 border-accent">
<img alt="Portrait of Jen" class="w-full h-full object-cover" src="https://lh3.googleusercontent.com/aida-public/AB6AXuCwRQbiiA6Y7bXK-6WZaKAwVf8k_zWHvy8Pmmk5DenO-jb0SrMgLBAJZ17iHWyn2gLlPspECc-fMFAMcR10e0yhJUVehY5SekEFrX4dE3RMZaDtPHyCqddQhEuOGuehKsGIrgY-F3aq6YJa15QkQGLvLVfyGgoBtoAKhGbVPMCYdjL1GV_Wl6Te7H7fnEEM4j4b08BO2MEx9rF8wFYjiL-9gHhf4b6bTfBfF39i5zB_PMhLy1VHKrt7Hby-vKVm7-Mfk0EsIKw9NM4" loading="lazy" decoding="async"/>
</div>
<div>
<h4 class="font-heading font-bold text-text-main">Jen Alston</h4>
<div class="flex text-secondary text-sm">
<span class="material-symbols-outlined text-[18px]">star</span>
<span class="material-symbols-outlined text-[18px]">star</span>
<span class="material-symbols-outlined text-[18px]">star</span>
<span class="material-symbols-outlined text-[18px]">star</span>
<span class="material-symbols-outlined text-[18px]">star_half</span>
</div>
</div>
</div>
<p class="text-text-main font-medium leading-relaxed italic">
                            "The design is just so cute. It makes a boring task actually kind of fun. Plus, no weird popups!"
                        </p>
</div>
</div>
<div class="mt-16 flex justify-center">
<button class="group relative inline-flex items-center justify-center px-12 py-4 font-heading font-bold text-white transition-all duration-200 bg-gradient-to-r from-primary to-secondary rounded-full shadow-float hover:shadow-lg hover:scale-105 focus:outline-none focus:ring-4 focus:ring-primary/30 text-lg" type="button" onclick={scrollToFetcher}>
<span class="mr-2">Start Fetching Now</span>
<span class="material-symbols-outlined group-hover:translate-x-1 transition-transform">arrow_forward</span>
</button>
</div>
</div>
</section>
</main>
<footer class="bg-white border-t border-pink-100 py-6 px-6 mt-6">
<div class="max-w-7xl mx-auto flex flex-col md:flex-row justify-between items-center gap-4">
<div class="flex items-center gap-2 opacity-50 grayscale hover:grayscale-0 transition-all">
<span class="material-symbols-outlined text-xl">smart_toy</span>
<span class="font-bold text-sm">FetchTube © 2023</span>
</div>
<div class="flex gap-4 text-plum/60 font-semibold text-xs">
<a class="hover:text-primary transition-colors" href="/privacy">Privacy Policy</a>
<a class="hover:text-primary transition-colors" href="/privacy">Terms of Service</a>
<a class="hover:text-primary transition-colors" href="/privacy">Contact</a>
</div>
</div>
</footer>
</div>
