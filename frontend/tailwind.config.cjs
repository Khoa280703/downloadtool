/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				primary: '#FF4D8C',
				secondary: '#FFB938',
				accent: '#6C5CE7',
				plum: '#2D1B36',
				muted: '#8B7E96',
				'bg-page': '#FFF5F9',
				'bg-surface': '#FFFFFF',
				'background-light': '#FFF5F9',
				surface: '#FFFFFF',
				'text-main': '#2D1B36'
			},
			fontFamily: {
				heading: ['Fredoka', 'sans-serif'],
				body: ['Nunito', 'sans-serif'],
				display: ['Spline Sans', 'sans-serif']
			},
			borderRadius: {
				xl: '24px',
				'2xl': '32px',
				'3xl': '48px',
				full: '9999px',
				blob: '40% 60% 70% 30% / 40% 50% 60% 50%'
			},
			boxShadow: {
				float: '0 20px 40px -10px rgba(255, 77, 140, 0.3)',
				candy: '0 10px 25px -5px rgba(255, 77, 140, 0.4), 0 8px 10px -6px rgba(255, 77, 140, 0.1)',
				'input-focus': '0 0 0 4px rgba(255, 77, 140, 0.2)',
				card: '0 10px 30px -5px rgba(45, 27, 54, 0.05)',
				glow: '0 0 20px rgba(255, 77, 140, 0.4)'
			},
			animation: {
				bob: 'bob 3s ease-in-out infinite',
				'bob-delayed': 'bob 3s ease-in-out infinite 1.5s',
				wiggle: 'wiggle 1s ease-in-out infinite',
				'pulse-glow': 'pulse-glow 2s cubic-bezier(0.4, 0, 0.6, 1) infinite'
			},
			keyframes: {
				bob: {
					'0%, 100%': { transform: 'translateY(0)' },
					'50%': { transform: 'translateY(-15px)' }
				},
				wiggle: {
					'0%, 100%': { transform: 'rotate(-3deg)' },
					'50%': { transform: 'rotate(3deg)' }
				},
				'pulse-glow': {
					'0%, 100%': { opacity: 1, boxShadow: '0 0 0 0 rgba(255, 77, 140, 0.7)' },
					'50%': { opacity: 0.5, boxShadow: '0 0 0 10px rgba(255, 77, 140, 0)' }
				}
			}
		}
	},
	plugins: [require('@tailwindcss/forms'), require('@tailwindcss/container-queries')]
};
