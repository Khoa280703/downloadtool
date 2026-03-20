<script lang="ts">
	import ArrowDown from 'lucide-svelte/icons/arrow-down';
	import ArrowLeft from 'lucide-svelte/icons/arrow-left';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import AudioLines from 'lucide-svelte/icons/audio-lines';
	import BadgeCheck from 'lucide-svelte/icons/badge-check';
	import BookMarked from 'lucide-svelte/icons/book-marked';
	import Bot from 'lucide-svelte/icons/bot';
	import Camera from 'lucide-svelte/icons/camera';
	import CameraOff from 'lucide-svelte/icons/camera-off';
	import Check from 'lucide-svelte/icons/check';
	import ChevronLeft from 'lucide-svelte/icons/chevron-left';
	import ChevronRight from 'lucide-svelte/icons/chevron-right';
	import CircleCheck from 'lucide-svelte/icons/circle-check';
	import CircleHelp from 'lucide-svelte/icons/circle-help';
	import CircleUserRound from 'lucide-svelte/icons/circle-user-round';
	import ClipboardPaste from 'lucide-svelte/icons/clipboard-paste';
	import Clock3 from 'lucide-svelte/icons/clock-3';
	import Clapperboard from 'lucide-svelte/icons/clapperboard';
	import Compass from 'lucide-svelte/icons/compass';
	import Database from 'lucide-svelte/icons/database';
	import Download from 'lucide-svelte/icons/download';
	import Eye from 'lucide-svelte/icons/eye';
	import Film from 'lucide-svelte/icons/film';
	import FileText from 'lucide-svelte/icons/file-text';
	import FolderOpen from 'lucide-svelte/icons/folder-open';
	import Headphones from 'lucide-svelte/icons/headphones';
	import Hand from 'lucide-svelte/icons/hand';
	import History from 'lucide-svelte/icons/history';
	import Home from 'lucide-svelte/icons/home';
	import Info from 'lucide-svelte/icons/info';
	import Languages from 'lucide-svelte/icons/languages';
	import LayoutDashboard from 'lucide-svelte/icons/layout-dashboard';
	import LayoutGrid from 'lucide-svelte/icons/layout-grid';
	import Link2 from 'lucide-svelte/icons/link-2';
	import ListChecks from 'lucide-svelte/icons/list-checks';
	import ListMusic from 'lucide-svelte/icons/list-music';
	import ListPlus from 'lucide-svelte/icons/list-plus';
	import ListVideo from 'lucide-svelte/icons/list-video';
	import LoaderCircle from 'lucide-svelte/icons/loader-circle';
	import LockOpen from 'lucide-svelte/icons/lock-open';
	import LogIn from 'lucide-svelte/icons/log-in';
	import LogOut from 'lucide-svelte/icons/log-out';
	import MemoryStick from 'lucide-svelte/icons/memory-stick';
	import Merge from 'lucide-svelte/icons/merge';
	import MonitorPlay from 'lucide-svelte/icons/monitor-play';
	import MonitorSmartphone from 'lucide-svelte/icons/monitor-smartphone';
	import MoonStar from 'lucide-svelte/icons/moon-star';
	import Network from 'lucide-svelte/icons/network';
	import Plus from 'lucide-svelte/icons/plus';
	import Puzzle from 'lucide-svelte/icons/puzzle';
	import Rocket from 'lucide-svelte/icons/rocket';
	import Rows3 from 'lucide-svelte/icons/rows-3';
	import Save from 'lucide-svelte/icons/save';
	import Search from 'lucide-svelte/icons/search';
	import ShieldCheck from 'lucide-svelte/icons/shield-check';
	import SlidersHorizontal from 'lucide-svelte/icons/sliders-horizontal';
	import Smartphone from 'lucide-svelte/icons/smartphone';
	import Sparkles from 'lucide-svelte/icons/sparkles';
	import SquareArrowOutUpRight from 'lucide-svelte/icons/square-arrow-out-up-right';
	import SunMedium from 'lucide-svelte/icons/sun-medium';
	import Terminal from 'lucide-svelte/icons/terminal';
	import User from 'lucide-svelte/icons/user';
	import Verified from 'lucide-svelte/icons/verified';
	import VideoOff from 'lucide-svelte/icons/video-off';
	import Waypoints from 'lucide-svelte/icons/waypoints';
	import X from 'lucide-svelte/icons/x';
	import Zap from 'lucide-svelte/icons/zap';

	type IconComponent = typeof Bot;

	const iconMap: Record<string, IconComponent> = {
		account_circle: CircleUserRound,
		add: Plus,
		add_to_queue: ListPlus,
		arrow_back: ArrowLeft,
		arrow_downward: ArrowDown,
		arrow_forward: ArrowRight,
		auto_awesome: Sparkles,
		bookmarks: BookMarked,
		bolt: Zap,
		check: Check,
		check_circle: CircleCheck,
		checklist: ListChecks,
		chevron_left: ChevronLeft,
		chevron_right: ChevronRight,
		close: X,
		content_paste_go: ClipboardPaste,
		dark_mode: MoonStar,
		dashboard: LayoutDashboard,
		database: Database,
		description: FileText,
		devices: MonitorSmartphone,
		download: Download,
		download_done: BadgeCheck,
		download_for_offline: Download,
		explore: Compass,
		extension: Puzzle,
		folder_open: FolderOpen,
		grid_view: LayoutGrid,
		graphic_eq: AudioLines,
		headphones: Headphones,
		home: Home,
		hub: Network,
		info: Info,
		language: Languages,
		light_mode: SunMedium,
		link: Link2,
		lock_open: LockOpen,
		login: LogIn,
		logout: LogOut,
		memory: MemoryStick,
		merge: Merge,
		movie: Film,
		movie_edit: Clapperboard,
		no_photography: CameraOff,
		open_in_new: SquareArrowOutUpRight,
		person: User,
		playlist_play: ListVideo,
		progress_activity: LoaderCircle,
		quiz: CircleHelp,
		queue_music: ListMusic,
		rocket_launch: Rocket,
		save: Save,
		schedule: Clock3,
		search: Search,
		smart_display: MonitorPlay,
		smart_toy: Bot,
		smartphone: Smartphone,
		speed: Zap,
		table_rows: Rows3,
		task_alt: CircleCheck,
		terminal: Terminal,
		touch_app: Hand,
		tune: SlidersHorizontal,
		verified_user: ShieldCheck,
		verified: Verified,
		videocam: Camera,
		videocam_off: VideoOff,
		visibility: Eye,
		volume_up: AudioLines,
		waypoints: Waypoints,
		work_history: History
	};

	const badgeMap: Record<string, string> = {
		'2k': '2K',
		'4k': '4K',
		'8k': '8K',
		full_hd: 'FHD',
		hd: 'HD',
		hdr_on: 'HDR',
		hdr_strong: 'HDR',
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
