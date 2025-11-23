/**
 * Privacy-first telemetry for Node.js
 *
 * @packageDocumentation
 */

/**
 * Configuration for sync credentials
 */
export interface SyncConfig {
  organizationId: string;
  applicationId: string;
  token: string;
  secret: string;
  endpoint?: string;
}

/**
 * Configuration for privacy settings
 */
export interface PrivacyConfig {
  anonymizeEmails?: boolean;
  sanitizePaths?: boolean;
  respectDoNotTrack?: boolean;
}

/**
 * Configuration for consent management
 */
export interface ConsentConfig {
  required?: boolean;
  defaultGranted?: boolean;
}

/**
 * Builder configuration
 */
export interface BuilderConfig {
  serviceName: string;
  syncConfig?: SyncConfig;
  privacyConfig?: PrivacyConfig;
  consentConfig?: ConsentConfig;
  autoSync?: boolean;
  syncInterval?: number;
  syncOnShutdown?: boolean;
}

/**
 * Options for tracking commands
 */
export interface CommandOptions {
  success?: boolean;
  durationMs?: number;
  metadata?: Record<string, string | number | boolean>;
}

/**
 * Options for tracking features
 */
export interface FeatureOptions {
  enabled?: boolean;
  metadata?: Record<string, string | number | boolean>;
}

/**
 * Event statistics
 */
export interface EventStats {
  total: number;
  synced: number;
  unsynced: number;
}

/**
 * Main TelemetryKit class
 *
 * @example
 * ```typescript
 * const telemetry = new TelemetryKit({
 *   serviceName: 'my-app',
 *   autoSync: true
 * });
 *
 * await telemetry.trackCommand('deploy', {
 *   success: true,
 *   durationMs: 1234
 * });
 *
 * await telemetry.shutdown();
 * ```
 */
export class TelemetryKit {
  /**
   * Create a new TelemetryKit instance
   *
   * @param config - Configuration object
   */
  constructor(config: BuilderConfig);

  /**
   * Track a command execution
   *
   * @param name - Command name
   * @param options - Command options
   *
   * @example
   * ```typescript
   * await telemetry.trackCommand('deploy', {
   *   success: true,
   *   durationMs: 1234
   * });
   * ```
   */
  trackCommand(name: string, options?: CommandOptions): Promise<void>;

  /**
   * Track a feature usage
   *
   * @param name - Feature name
   * @param options - Feature options
   *
   * @example
   * ```typescript
   * await telemetry.trackFeature('dark-mode', {
   *   enabled: true
   * });
   * ```
   */
  trackFeature(name: string, options?: FeatureOptions): Promise<void>;

  /**
   * Manually trigger synchronization
   *
   * @example
   * ```typescript
   * await telemetry.sync();
   * ```
   */
  sync(): Promise<void>;

  /**
   * Get event statistics
   *
   * @returns Event statistics
   *
   * @example
   * ```typescript
   * const stats = await telemetry.stats();
   * console.log(`Total: ${stats.total}, Synced: ${stats.synced}`);
   * ```
   */
  stats(): Promise<EventStats>;

  /**
   * Gracefully shutdown telemetry
   *
   * This performs a final sync if configured and cleans up resources.
   *
   * @example
   * ```typescript
   * await telemetry.shutdown();
   * ```
   */
  shutdown(): Promise<void>;
}

/**
 * Builder for TelemetryKit
 *
 * @example
 * ```typescript
 * const telemetry = TelemetryKitBuilder.new('my-app')
 *   .withSyncCredentials(orgId, appId, token, secret)
 *   .autoSync(true)
 *   .build();
 * ```
 */
export class TelemetryKitBuilder {
  /**
   * Create a new builder
   *
   * @param serviceName - Service name
   */
  static new(serviceName: string): TelemetryKitBuilder;

  /**
   * Set sync credentials
   *
   * @param organizationId - Organization ID (UUID)
   * @param applicationId - Application ID (UUID)
   * @param token - Authentication token
   * @param secret - HMAC secret
   */
  withSyncCredentials(
    organizationId: string,
    applicationId: string,
    token: string,
    secret: string
  ): this;

  /**
   * Set custom sync endpoint
   *
   * @param endpoint - Endpoint URL
   */
  endpoint(endpoint: string): this;

  /**
   * Enable or disable auto-sync
   *
   * @param enabled - Whether to enable auto-sync
   */
  autoSync(enabled: boolean): this;

  /**
   * Set sync interval in seconds
   *
   * @param seconds - Sync interval
   */
  syncInterval(seconds: number): this;

  /**
   * Enable sync on shutdown
   *
   * @param enabled - Whether to sync on shutdown
   */
  syncOnShutdown(enabled: boolean): this;

  /**
   * Build the TelemetryKit instance
   *
   * @returns Configured TelemetryKit instance
   */
  build(): TelemetryKit;
}
