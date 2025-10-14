# Selendra Network Website

The official website for Selendra Network - an EVM-compatible Layer 1 blockchain built on Substrate with hybrid consensus.

## Tech Stack

- **Framework**: React 18 with TypeScript
- **Build Tool**: Vite 5
- **Styling**: Tailwind CSS 3
- **Animations**: Framer Motion
- **Routing**: React Router DOM

## Prerequisites

- Node.js 18+ (recommended: 20+)
- npm or yarn

## Getting Started

### 1. Install Dependencies

```bash
npm install
```

### 2. Run Development Server

```bash
npm run dev
```

The site will be available at `http://localhost:3000` and will automatically open in your browser.

### 3. Build for Production

```bash
npm run build
```

The production-ready files will be in the `dist/` directory.

### 4. Preview Production Build

```bash
npm run preview
```

## Available Scripts

- `npm run dev` - Start development server on port 3000
- `npm run build` - Build for production (runs TypeScript check + Vite build)
- `npm run preview` - Preview production build locally
- `npm run lint` - Run ESLint to check code quality

## Project Structure

```
website/
├── public/              # Static assets
│   └── vite.svg
├── src/
│   ├── components/      # React components
│   │   ├── Navbar.tsx
│   │   ├── Hero.tsx
│   │   ├── Stats.tsx
│   │   ├── Features.tsx
│   │   ├── Technology.tsx
│   │   ├── Roadmap.tsx
│   │   ├── CTA.tsx
│   │   └── Footer.tsx
│   ├── styles/          # Global styles
│   │   └── index.css
│   ├── assets/          # Images, fonts, etc.
│   ├── App.tsx          # Main app component
│   └── main.tsx         # App entry point
├── index.html           # HTML template
├── vite.config.ts       # Vite configuration
├── tailwind.config.js   # Tailwind CSS configuration
├── postcss.config.js    # PostCSS configuration
└── tsconfig.json        # TypeScript configuration
```

## Configuration

### Path Aliases

The following path aliases are configured for cleaner imports:

- `@/` - Maps to `./src/`
- `@components/` - Maps to `./src/components/`
- `@pages/` - Maps to `./src/pages/`
- `@styles/` - Maps to `./src/styles/`
- `@assets/` - Maps to `./src/assets/`

Example usage:
```typescript
import Hero from '@components/Hero'
import '@styles/index.css'
```

### Tailwind Theme

The site uses a custom color palette:

- **Primary**: `#c6613f` (coral/terracotta) - Selendra brand color
- **Dark**: `#0f1419` - Main background
- **Slate**: Various shades for text and borders

### Custom Typography

Custom font sizes are configured:
- `text-display-xxl` - Extra large display text (5rem)
- `text-display-xl` - Large display text (4rem)
- `text-display-lg` - Medium display text (3.5rem)
- `text-heading-xl` - Extra large heading (2rem)
- And more...

## Features

### Current Pages

1. **Hero Section**: Main landing with key metrics
2. **Stats Section**: Performance metrics (1s block time, <2s finality)
3. **Features Section**: Key features of the network
4. **Technology Section**: Tech stack breakdown (Substrate, AlephBFT, Aura, Frontier)
5. **Roadmap Section**: Development timeline from Q4 2024 to Q4 2025
6. **CTA Section**: Call-to-action for developers
7. **Footer**: Links and social media

### Design System

The site includes a comprehensive design system with:

- Custom buttons (primary, secondary, outline, ghost)
- Card components with hover effects
- Gradient text and border effects
- Custom animations (fade-in, slide-in, scale-in)
- Responsive grid layouts
- Custom scrollbar styling

## Troubleshooting

### Development Server Won't Start

1. Make sure you're using Node.js 18+
2. Delete `node_modules/` and `package-lock.json`, then run `npm install` again
3. Clear Vite cache: `rm -rf node_modules/.vite`

### TypeScript Errors

The project uses strict TypeScript settings. If you encounter type errors:

1. Check that all dependencies are installed
2. Restart your IDE/editor
3. Run `npx tsc --noEmit` to see all type errors

### Build Fails

1. Ensure all environment variables are set (if any)
2. Check that all imports are correct
3. Run `npm run lint` to check for linting issues

## Browser Support

- Chrome/Edge (latest 2 versions)
- Firefox (latest 2 versions)
- Safari (latest 2 versions)

## License

See the main Selendra repository for license information.

## Contributing

This is part of the main Selendra Network repository. Please refer to the main repository's contributing guidelines.
