import Link from 'next/link';
import Image from 'next/image';

export default function HomePage() {
  return (
    <main className="flex h-screen flex-col items-center justify-center text-center px-4">
      <div className="max-w-4xl">
        <div className="mb-8 flex justify-center">
          <Image
            src="/logo.svg"
            alt="telemetry-kit logo"
            width={120}
            height={120}
            priority
          />
        </div>
        <h1 className="mb-4 text-5xl font-bold">
          telemetry-kit
        </h1>
        <p className="mb-8 text-xl text-muted-foreground">
          Privacy-first, batteries-included telemetry for Rust applications
        </p>

        <div className="flex flex-col sm:flex-row gap-4 justify-center mb-12">
          <Link
            href="/docs"
            className="inline-flex items-center justify-center rounded-lg bg-primary px-6 py-3 text-sm font-medium text-primary-foreground shadow transition-colors hover:bg-primary/90"
          >
            Get Started
          </Link>
          <Link
            href="/docs/api"
            className="inline-flex items-center justify-center rounded-lg border border-input bg-background px-6 py-3 text-sm font-medium shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground"
          >
            API Reference
          </Link>
          <a
            href="https://github.com/ibrahimcesar/telemetry-kit"
            className="inline-flex items-center justify-center rounded-lg border border-input bg-background px-6 py-3 text-sm font-medium shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground"
            target="_blank"
            rel="noopener noreferrer"
          >
            View on GitHub
          </a>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 text-left">
          <div className="rounded-lg border bg-card p-6">
            <h3 className="mb-2 font-semibold">🚀 Auto-Sync</h3>
            <p className="text-sm text-muted-foreground">
              Background synchronization with configurable intervals. No manual sync calls required.
            </p>
          </div>
          <div className="rounded-lg border bg-card p-6">
            <h3 className="mb-2 font-semibold">🔒 Privacy-First</h3>
            <p className="text-sm text-muted-foreground">
              Anonymous user IDs, GDPR compliant, DO_NOT_TRACK support built-in.
            </p>
          </div>
          <div className="rounded-lg border bg-card p-6">
            <h3 className="mb-2 font-semibold">🛠️ CLI Tool</h3>
            <p className="text-sm text-muted-foreground">
              Manage telemetry configuration and operations from the command line.
            </p>
          </div>
        </div>

        <div className="mt-12 text-sm text-muted-foreground">
          <code className="rounded bg-muted px-2 py-1">
            cargo add telemetry-kit
          </code>
        </div>
      </div>
    </main>
  );
}
