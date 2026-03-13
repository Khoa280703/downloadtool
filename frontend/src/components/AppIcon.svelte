<script lang="ts">
	import ArrowDown from 'lucide-svelte/icons/arrow-down';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import AudioLines from 'lucide-svelte/icons/audio-lines';
	import BookMarked from 'lucide-svelte/icons/book-marked';
	import Bot from 'lucide-svelte/icons/bot';
	import Check from 'lucide-svelte/icons/check';
	import CircleCheck from 'lucide-svelte/icons/circle-check';
	import CircleUserRound from 'lucide-svelte/icons/circle-user-round';
	import ClipboardPaste from 'lucide-svelte/icons/clipboard-paste';
	import Clock3 from 'lucide-svelte/icons/clock-3';
	import Clapperboard from 'lucide-svelte/icons/clapperboard';
	import Database from 'lucide-svelte/icons/database';
	import Download from 'lucide-svelte/icons/download';
	import Eye from 'lucide-svelte/icons/eye';
	import Film from 'lucide-svelte/icons/film';
	import FileText from 'lucide-svelte/icons/file-text';
	import FolderOpen from 'lucide-svelte/icons/folder-open';
	import Headphones from 'lucide-svelte/icons/headphones';
	import Hand from 'lucide-svelte/icons/hand';
	import History from 'lucide-svelte/icons/history';
	import Languages from 'lucide-svelte/icons/languages';
	import LayoutDashboard from 'lucide-svelte/icons/layout-dashboard';
	import LayoutGrid from 'lucide-svelte/icons/layout-grid';
	import Link2 from 'lucide-svelte/icons/link-2';
	import ListMusic from 'lucide-svelte/icons/list-music';
	import ListPlus from 'lucide-svelte/icons/list-plus';
	import ListVideo from 'lucide-svelte/icons/list-video';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';
	import LogIn from 'lucide-svelte/icons/log-in';
	import LogOut from 'lucide-svelte/icons/log-out';
	import MonitorPlay from 'lucide-svelte/icons/monitor-play';
	import MoonStar from 'lucide-svelte/icons/moon-star';
	import Network from 'lucide-svelte/icons/network';
	import Plus from 'lucide-svelte/icons/plus';
	import Puzzle from 'lucide-svelte/icons/puzzle';
	import Rocket from 'lucide-svelte/icons/rocket';
	import Rows3 from 'lucide-svelte/icons/rows-3';
	import Save from 'lucide-svelte/icons/save';
	import Search from 'lucide-svelte/icons/search';
	import ShieldCheck from 'lucide-svelte/icons/shield-check';
	import Smartphone from 'lucide-svelte/icons/smartphone';
	import SquareArrowOutUpRight from 'lucide-svelte/icons/square-arrow-out-up-right';
	import SunMedium from 'lucide-svelte/icons/sun-medium';
	import Terminal from 'lucide-svelte/icons/terminal';
	import User from 'lucide-svelte/icons/user';
	import VideoOff from 'lucide-svelte/icons/video-off';
	import X from 'lucide-svelte/icons/x';
	import Zap from 'lucide-svelte/icons/zap';

	type IconComponent = typeof Bot;

	const iconMap: Record<string, IconComponent> = {
		account_circle: CircleUserRound,
		add: Plus,
		add_to_queue: ListPlus,
		arrow_downward: ArrowDown,
		arrow_forward: ArrowRight,
		bookmarks: BookMarked,
		bolt: Zap,
		check: Check,
		check_circle: CircleCheck,
		close: X,
		content_paste_go: ClipboardPaste,
		dark_mode: MoonStar,
		dashboard: LayoutDashboard,
		database: Database,
		description: FileText,
		download: Download,
		download_for_offline: Download,
		extension: Puzzle,
		folder_open: FolderOpen,
		grid_view: LayoutGrid,
		graphic_eq: AudioLines,
		headphones: Headphones,
		hub: Network,
		language: Languages,
		light_mode: SunMedium,
		link: Link2,
		login: LogIn,
		logout: LogOut,
		movie: Film,
		movie_edit: Clapperboard,
		open_in_new: SquareArrowOutUpRight,
		person: User,
		playlist_play: ListVideo,
		progress_activity: LoaderCircle,
		queue_music: ListMusic,
		rocket_launch: Rocket,
		save: Save,
		schedule: Clock3,
		search: Search,
		smart_display: MonitorPlay,
		smart_toy: Bot,
		smartphone: Smartphone,
		table_rows: Rows3,
		task_alt: CircleCheck,
		terminal: Terminal,
		touch_app: Hand,
		verified_user: ShieldCheck,
		videocam_off: VideoOff,
		visibility: Eye,
		work_history: History
	};

	const badgeMap: Record<string, string> = {
		'2k': '2K',
		'4k': '4K',
		'8k': '8K',
		full_hd: 'FHD',
		hd: 'HD',
		high_quality: 'HQ',
		sd: 'SD'
	};

	let {
		name,
		class: className = '',
		title,
		ariaLabel
	}: {
		name: string;
		class?: string;
		title?: string;
		ariaLabel?: string;
	} = $props();

	const iconClass = $derived(`app-icon ${className}`.trim());
	const Icon = $derived(iconMap[name] ?? null);
	const badge = $derived(badgeMap[name] ?? null);
</script>

{#if badge}
	<span
		class={`app-icon-badge ${className}`.trim()}
		title={title}
		aria-label={ariaLabel}
		aria-hidden={ariaLabel ? undefined : 'true'}
	>
		{badge}
	</span>
{:else if Icon}
	<Icon
		class={iconClass}
		aria-label={ariaLabel}
		aria-hidden={ariaLabel ? undefined : 'true'}
		strokeWidth={2}
	/>
{:else}
	<span
		class={`app-icon-placeholder ${className}`.trim()}
		title={title}
		aria-label={ariaLabel}
		aria-hidden={ariaLabel ? undefined : 'true'}
	></span>
{/if}

<style>
	.app-icon,
	.app-icon-placeholder,
	.app-icon-badge {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		vertical-align: middle;
	}

	.app-icon,
	.app-icon-placeholder {
		width: 1em;
		height: 1em;
	}

	.app-icon-badge {
		min-width: 1.5em;
		line-height: 1;
		font-size: 0.72em;
		font-weight: 800;
		letter-spacing: 0.04em;
		text-transform: uppercase;
	}
	
	:global(.animate-spin.app-icon) {
		animation: spin 1s linear infinite;
	}
</style>
