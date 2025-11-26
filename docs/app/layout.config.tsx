import type { DocsLayoutProps } from 'fumadocs-ui/layout';
import Image from 'next/image';

export const baseOptions: Omit<DocsLayoutProps, 'tree'> = {
  nav: {
    title: (
      <Image src="/logo.svg" alt="telemetry-kit" width={140} height={32} />
    ),
  },
  links: [
    {
      text: 'Quick Start',
      url: '/quick-start',
    },
    {
      text: 'API Reference',
      url: '/api',
    },
    {
      text: 'Main Site',
      url: 'https://telemetry-kit.dev',
      external: true,
    },
    {
      text: 'GitHub',
      url: 'https://github.com/ibrahimcesar/telemetry-kit',
      external: true,
    },
  ],
  githubUrl: 'https://github.com/ibrahimcesar/telemetry-kit',
};
