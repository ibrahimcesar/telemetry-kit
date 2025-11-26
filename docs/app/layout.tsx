import './global.css';
import { RootProvider } from 'fumadocs-ui/provider';
import { Inter } from 'next/font/google';
import type { ReactNode } from 'react';
import GoogleAnalytics from './components/GoogleAnalytics';

const inter = Inter({
  subsets: ['latin'],
});

export const metadata = {
  title: {
    default: 'telemetry-kit Documentation',
    template: '%s | telemetry-kit Docs',
  },
  description: 'Documentation for telemetry-kit - Privacy-first usage analytics for Rust open source projects',
  icons: {
    icon: '/favicon.ico',
  },
  openGraph: {
    title: 'telemetry-kit Documentation',
    description: 'Privacy-first usage analytics for Rust open source projects. Learn how to add telemetry to your CLI tools and libraries.',
    url: 'https://docs.telemetry-kit.dev',
    siteName: 'telemetry-kit Docs',
    locale: 'en_US',
    type: 'website',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'telemetry-kit Documentation',
    description: 'Privacy-first usage analytics for Rust open source projects.',
    creator: '@ibrahimcesar',
  },
};

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <html lang="en" className={inter.className} suppressHydrationWarning>
      <body>
        <GoogleAnalytics />
        <RootProvider>{children}</RootProvider>
      </body>
    </html>
  );
}
