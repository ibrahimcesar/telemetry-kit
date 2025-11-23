import type { DocsLayoutProps } from 'fumadocs-ui/layout';
import Image from 'next/image';

export const baseOptions: Omit<DocsLayoutProps, 'tree'> = {
  nav: {
    title: (
      <div className="flex items-center gap-2">
        <Image src="/logo.svg" alt="telemetry-kit" width={24} height={24} />
        <span className="font-semibold">telemetry-kit</span>
      </div>
    ),
  },
  links: [
    {
      text: 'Documentation',
      url: '/docs',
      active: 'nested-url',
    },
    {
      text: 'API Reference',
      url: '/docs/api',
    },
    {
      text: 'GitHub',
      url: 'https://github.com/ibrahimcesar/telemetry-kit',
      external: true,
    },
  ],
  githubUrl: 'https://github.com/ibrahimcesar/telemetry-kit',
};
