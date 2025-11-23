/**
 * Basic example of using telemetry-kit in Node.js
 *
 * This example demonstrates:
 * - Creating a telemetry instance
 * - Tracking commands
 * - Tracking features
 * - Getting statistics
 * - Graceful shutdown
 */

// NOTE: When this is published, you would use:
// const { TelemetryKit } = require('@telemetry-kit/node');

// For now, this is a demonstration of the API
async function main() {
  console.log('🔭 telemetry-kit Node.js Example\n');

  // Create telemetry instance
  console.log('Creating telemetry instance...');
  const telemetry = new TelemetryKit({
    serviceName: 'example-node-app',
    autoSync: false, // Offline-only for this example
  });

  // Track a command
  console.log('\nTracking command: npm install');
  await telemetry.trackCommand('npm-install', {
    success: true,
    durationMs: 2534,
  });

  // Track multiple commands
  console.log('Tracking command: npm test');
  await telemetry.trackCommand('npm-test', {
    success: true,
    durationMs: 1523,
  });

  console.log('Tracking command: npm build');
  await telemetry.trackCommand('npm-build', {
    success: true,
    durationMs: 8921,
  });

  // Track features
  console.log('\nTracking feature usage...');
  await telemetry.trackFeature('typescript', {
    enabled: true,
  });

  await telemetry.trackFeature('dark-mode', {
    enabled: false,
  });

  // Get statistics
  console.log('\nGetting statistics...');
  const stats = await telemetry.stats();
  console.log(`  Total events: ${stats.total}`);
  console.log(`  Synced: ${stats.synced}`);
  console.log(`  Unsynced: ${stats.unsynced}`);

  // Shutdown
  console.log('\nShutting down...');
  await telemetry.shutdown();

  console.log('✅ Done!\n');
}

// Run if called directly
if (require.main === module) {
  main().catch((error) => {
    console.error('❌ Error:', error.message);
    process.exit(1);
  });
}

module.exports = { main };
